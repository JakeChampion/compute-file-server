use clap::{arg, Command};
use fastly_api::apis::configuration::{ApiKey, Configuration};
use fastly_api::apis::version_api::{
    activate_service_version, list_service_versions, ActivateServiceVersionParams,
    ListServiceVersionsParams,
};
use fastly_api::apis::version_api::{clone_service_version, CloneServiceVersionParams};
use futures::{stream, StreamExt};
use reqwest::Client;
use simple_error::bail;
use std::error::Error;
use std::path::PathBuf;
use tokio;
use tokio::fs::File;
use walkdir::WalkDir;

const PARALLEL_REQUESTS: usize = 10;
const RETRY_REQUESTS: usize = 5;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStores {
    pub data: Vec<ObjectStore>,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStore {
    pub id: String,
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub limit: i64,
    pub total: i64,
}

async fn create_store(name: &str, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("creating store named {}", name);
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.fastly.com/resources/stores/object")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Fastly-Key", token)
        .body(format!("{{\"name\":\"{}\"}}", name))
        .send()
        .await?;
    if res.status() == 201 {
        Ok(res.json::<ObjectStore>().await?.id)
    } else {
        bail!(format!(
            "Failed to create Object Store named `{}`. Response body contained `{}`",
            name,
            res.text().await?
        ))
    }
}

async fn get_or_create_store(
    name: &str,
    token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // get all stores
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.fastly.com/resources/stores/object")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Fastly-Key", token)
        .send()
        .await?;
    // if no stores at all, create store
    if res.status() == 404 {
        create_store(name, token).await
    } else {
        // check if store already exists
        let res = res
            .json::<ObjectStores>()
            .await?
            .data
            .into_iter()
            .find_map(|store| {
                if store.name == name {
                    Some(store.id)
                } else {
                    None
                }
            });
        // if store does not exist, create store
        if res.is_none() {
            create_store(name, token).await
        } else {
            println!("store named `{}` already exists", name);
            Ok(res.unwrap())
        }
    }
}

async fn get_active_version_of_service(
    service_id: &str,
    token: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let cfg = &Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: token.to_owned(),
        }),
        ..Default::default()
    };

    let params = ListServiceVersionsParams {
        service_id: service_id.to_owned(),
        ..Default::default()
    };

    let result = list_service_versions(cfg, params)
        .await?
        .into_iter()
        .find_map(|v| {
            if v.active.unwrap() {
                Some(v.number.unwrap())
            } else {
                None
            }
        });

    return Ok(result.unwrap());
}

async fn clone_version_of_service(
    service_id: &str,
    token: &str,
    version: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let cfg = &Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: token.to_owned(),
        }),
        ..Default::default()
    };

    let params = CloneServiceVersionParams {
        service_id: service_id.to_owned(),
        version_id: version,
        ..Default::default()
    };

    Ok(clone_service_version(cfg, params).await?.number.unwrap())
}

async fn activate_version_of_service(
    service_id: &str,
    token: &str,
    version: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let cfg = &Configuration {
        api_key: Some(ApiKey {
            prefix: None,
            key: token.to_owned(),
        }),
        ..Default::default()
    };

    let params = ActivateServiceVersionParams {
        service_id: service_id.to_string(),
        version_id: version,
        ..Default::default()
    };

    Ok(activate_service_version(cfg, params).await?.number.unwrap())
}

fn cli() -> Command {
    Command::new("fastly-file-server")
        .about("Fastly File Server uploads files to Fastly for serving directly from within Compute@Edge applications. Upload any type of file: images, text, video etc and serve directly from Fastly. It is ideal for serving files built from a static site generator such as 11ty.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("upload")
                .about("Upload files")
                .arg(
                    arg!(path: [PATH])
                        .last(true)
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                )
                .arg_required_else_help(true)
                .arg(arg!(--name <NAME>).required(true))
                .arg(arg!(--token <TOKEN>)),
        )
        .subcommand(
            Command::new("local")
                .about("Setup files")
                .arg(
                    arg!(path: [PATH])
                        .last(true)
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                )
                .arg_required_else_help(true)
                .arg(arg!(--toml <TOML>).required(true).value_parser(clap::value_parser!(PathBuf)))
                .arg(arg!(--name <NAME>).required(true)),
        )
        .subcommand(
            Command::new("link")
                .about("link store to service")
                .arg(arg!(--name <NAME>).required(true))
                .arg(arg!(--token <TOKEN>))
                .arg(arg!(--"link-name" <LINK_NAME>).required(true))
                .arg(arg!(--"service-id" <SERVICE_ID>).required(true)),
        )
}

async fn link(sub_matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let service_id = sub_matches
        .get_one::<String>("service-id")
        .map(|s| s.as_str())
        .expect("required in clap");

    let link_name = sub_matches
        .get_one::<String>("link-name")
        .map(|s| s.as_str())
        .expect("required in clap");

    let name = sub_matches
        .get_one::<String>("name")
        .map(|s| s.as_str())
        .expect("required in clap");

    let token = sub_matches
        .get_one::<String>("token")
        .map(|s| s.to_owned())
        .or_else(|| match std::env::var("FASTLY_API_TOKEN") {
            Ok(x) => Some(x),
            Err(_) => None,
        });
    if token.is_none() {
        bail!("Missing Fastly API token. Please provide an API token via the --token argument or the FASTLY_API_TOKEN environment variable.")
    }
    let token = token.unwrap();

    let store_id = get_or_create_store(name, &token).await?;

    let version = get_active_version_of_service(service_id, &token).await?;
    println!("cloning version {}", version);
    let version = clone_version_of_service(service_id, &token, version).await?;

    // link
    let client = reqwest::Client::new();
    println!("connecting store named `{}` to service", name);
    let _res = client
        .post(format!(
            "https://api.fastly.com/service/{}/version/{}/resource",
            service_id, version
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Accept", "application/json")
        .header("Fastly-Key", &token)
        .body(format!("name={}&resource_id={}", link_name, store_id))
        .send()
        .await?;

    // activate
    println!("activating version {}", version);
    activate_version_of_service(service_id, &token, version).await?;

    Ok(())
}

async fn upload(sub_matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let name = sub_matches
        .get_one::<String>("name")
        .map(|s| s.as_str())
        .expect("required in clap");

    let token = sub_matches
        .get_one::<String>("token")
        .map(|s| s.to_owned())
        .or_else(|| match std::env::var("FASTLY_API_TOKEN") {
            Ok(x) => Some(x),
            Err(_) => None,
        });
    if token.is_none() {
        bail!("Missing Fastly API token. Please provide an API token via the --token argument or the FASTLY_API_TOKEN environment variable.")
    }
    let token = token.unwrap();
    let store_id = get_or_create_store(name, &token).await?;

    let path = sub_matches
        .get_one::<PathBuf>("path")
        .expect("required in clap");

    let entries = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .collect::<Vec<walkdir::DirEntry>>();

    let pb = indicatif::ProgressBar::new(entries.len().try_into().unwrap());
    let client = Client::new();

    let bodies = stream::iter(entries)
        .map(|entry| -> tokio::task::JoinHandle<Result<String, Box<dyn Error + Send + Sync>>> {
            let path = path.clone();
            let store_id = store_id.clone();
            let token = token.clone();
            let client = client.clone();
            tokio::spawn(async move {
                let normalised_entry = entry.path().strip_prefix(path).unwrap();
                let normalised_path = "/".to_owned() + &normalised_entry.to_string_lossy();
                let key = percent_encoding::utf8_percent_encode(
                    &normalised_path,
                    percent_encoding::NON_ALPHANUMERIC,
                );
                // println!(
                //     "https://api.fastly.com/resources/stores/object/{}/keys/{}",
                //     store_id, key
                // );
                let file = File::open(entry.path()).await?;
                let length = file.metadata().await?.len();
                let mut counter = 0;
                loop {
                    let res = client
                        .put(format!(
                            "https://api.fastly.com/resources/stores/object/{}/keys/{}",
                            store_id, key
                        ))
                        .header("Content-Type", "application/json")
                        .header("Content-Length", length)
                        .header("Accept", "application/json")
                        .header("Fastly-Key", &token)
                        .body(file.try_clone().await?)
                        .send()
                        .await?;
                    if res.status() != 200 {
                        counter = counter + 1;
                        if counter > RETRY_REQUESTS {
                            bail!(
                                "Error uploading file named `{}`: Response Status: {} Response Body: {}",
                                normalised_path,
                                res.status(),
                                res.text().await?
                            );
                        }
                    } else {
                        return Ok::<String, Box<dyn std::error::Error + Send + Sync>>(
                            normalised_path,
                        );
                    }
                }
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(Ok(_)) => {
                    // pb.println(format!("[+] uploaded {}", normalised_entry));
                    pb.inc(1);
                }
                Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;

    pb.finish_with_message("done");
    Ok(())
}

async fn local(sub_matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let name = sub_matches
        .get_one::<String>("name")
        .map(|s| s.as_str())
        .expect("required in clap");

    let path = sub_matches
        .get_one::<PathBuf>("path")
        .expect("required in clap");

    let toml_path = sub_matches
        .get_one::<PathBuf>("toml")
        .expect("required in clap");

    let entries = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .collect::<Vec<walkdir::DirEntry>>();

    let mut toml = std::fs::read_to_string(toml_path)?;

    for entry in entries {
        let path = path.clone();
        let normalised_entry = entry.path().strip_prefix(path).unwrap();
        let normalised_path = "/".to_owned() + &normalised_entry.to_string_lossy();
        let key = &normalised_path;
        toml = toml + "\n" + &format!("[[local_server.object_store.{}]]", name);
        toml = toml + "\n" + &format!("key = \"{}\"", key);
        toml = toml + "\n" + &format!("path = \"{}\"", entry.path().to_string_lossy());

        println!("{}", toml);
    }
    std::fs::write(toml_path, toml).expect("Unable to write file");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("link", sub_matches)) => link(sub_matches).await,
        Some(("local", sub_matches)) => local(sub_matches).await,
        Some(("upload", sub_matches)) => upload(sub_matches).await,
        _ => unreachable!(),
    }
}

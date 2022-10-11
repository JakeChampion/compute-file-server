# c-at-e-file-server

Fastly File Server uploads files to Fastly for serving directly from within Compute@Edge applications.

Upload any type of file: images, text, video etc and serve directly from Fastly.

It is ideal for serving files built from a static site generator such as 11ty.

## Install

### npm

Install pre-compiled binaries via `npm`

```sh
npm install c-at-e-file-server
```

### Cargo

Compile and install via `cargo`

```sh
git clone https://github.com/JakeChampion/c-at-e-file-server
cd c-at-e-file-server/cli
cargo install --path .
```

## Commands

### Upload

Upload files to a Fastly Object Store, creating the Object Store if it does not exist.

Example: `c-at-e-file-server upload --name website-static-files -- ./folder/of/files`

```sh
c-at-e-file-server upload
Upload files

Usage: c-at-e-file-server upload [OPTIONS] --name <NAME> -- <PATH>

Arguments:
  <PATH>  

Options:
      --name <NAME>    
      --token <TOKEN>  
  -h, --help           Print help information
```

### Link

Connect a Fastly Object Store to a Fastly Service.

Example: `c-at-e-file-server link --name website-static-files --link-name files --service-id xxyyzz`

```sh
Usage: c-at-e-file-server link [OPTIONS] --name <NAME> --link-name <LINK_NAME> --service-id <SERVICE_ID>

Options:
      --name <NAME>
      --token <TOKEN>
      --link-name <LINK_NAME>
      --service-id <SERVICE_ID>
  -h, --help                     Print help information
```

### Local

Update `fastly.toml` to contain a local Object Store containing the specified files.

Example: `c-at-e-file-server local --name files --toml fastly.toml -- ./folder/of/files`

```sh
Usage: c-at-e-file-server local --toml <TOML> --name <NAME> -- <PATH>

Arguments:
  <PATH>

Options:
      --toml <TOML>
      --name <NAME>
  -h, --help         Print help information
```
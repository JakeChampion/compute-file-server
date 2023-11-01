# compute-file-server

Compute File Server uploads files to Fastly for serving directly from within Fastly Compute applications.

Upload any type of file: images, text, video etc and serve directly from Fastly.

It is ideal for serving files built from a static site generator such as 11ty.

Serve the files from your Compute application.

## Usage

- You will need to install the CLI application used to upload files to Fastly. `cargo install compute-file-server-cli`
- Upload the files `compute-file-server upload --name 'my-site' --token "$(fastly profile token)" -- my-folder-of-files/`
- Create your Fastly Compute application. We have examples for [Rust](./examples/rust/) and [JavaScript](./examples/javascript/).
- Copy your Fastly Compute service_id from the `fastly.toml` file
- Link the files to your Fastly Compute application. `compute-file-server link --name my-site --link-name site --service-id "your-service_id" --token "$(fastly profile token)"`

- You are done!


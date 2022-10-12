# c-at-e-file-server

Compute@Edge File Server uploads files to Fastly for serving directly from within Compute@Edge applications.

Upload any type of file: images, text, video etc and serve directly from Fastly.

It is ideal for serving files built from a static site generator such as 11ty.

Serve the files from your Compute@Edge application.

## Usage

- You will need to install the CLI application used to upload files to Fastly.
- Upload the files `c-at-e-file-server upload --name 'my-site' --token "$(fastly profile token)" -- my-folder-of-files/`
- Create your Compute@Edge application. We have examples for [Rust](./examples/rust/) and [JavaScript](./examples/javascript/).
- Copy your Compute@Edge service_id from the `fastly.toml` file 
- Link the files to your Compute@Edge application. `c-at-e-file-server link --name my-site --link-name site --service-id "your-service_id" --token "$(fastly profile token)"`

- You are done!


# DJI Log Viewer

# Setup

## Native

- Install Rust (https://rustup.rs/)
- `$ cargo run` to run debug build.
- `$ cargo install --path .` to install.

## Web Assembly

- `$ cargo install trunk`
- `$ trunk serve --open` to start a local development server and open the result in the browser.
- `$ trunk build --release` to build release files. Copy `/dist` folder to webserver of choice.

# Paste server backend

## Build and run for production

```sh
cargo build --release
mkdir uploads
UPLOAD_DIR=./uploads ./target/release/paste
```

Server listens on 8000

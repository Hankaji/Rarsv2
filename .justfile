
export RUSTFLAGS := "-C target-cpu=native"

build:
  cargo build --release --target x86_64-unknown-linux-musl

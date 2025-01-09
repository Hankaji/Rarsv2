

export RUSTFLAGS := "-C target-cpu=native"
build:
  # cargo build --release --target x86_64-unknown-linux-musl
  cargo build --release --target x86_64-unknown-linux-gnu

dev backtrace="false":
  {{ if backtrace == "true" {"RUST_BACKTRACE=1"} else {""} }} cargo run
  

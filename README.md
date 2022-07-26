# rust-rocket-docker

Sample application for demonstrate rust CI/CD process for docker container and unikernel...

## run as unikernel

1. Install [ops](https://ops.city/)
1. Configure _x86_64-unknown-linux-musl_ toolchain
    1. Install toolchain: `rustup target add x86_64-unknown-linux-musl`
    1. Add to `~/.cargo/config`
      ```
      [target.x86_64-unknown-linux-musl]
      linker = "x86_64-linux-musl-gcc"
      ```
1. Build application: `cargo build --release --target=x86_64-unknown-linux-musl`
1. Run application as unikernel: `ops run -p 8000 target/x86_64-unknown-linux-musl/release/rocket-test`
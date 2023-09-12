# `cargo`

## `cargo build`

builds a binary in `target`

## `cargo run`

 builds a binary in `target` and runs it

## `cargo clean`

 removes `target` directory

## `cargo install`

 installs the binary under `~/.cargo/bin`. Example: `cargo install --path .`

## `cargo uninstall`

 uninstall the binary from `~/.cargo/bin`

## `cargo ... --release`

 run optimized artifacts with the release profile. Example: when running
 `cargo run --release`, computing fibonacci numbers is more than 2x faster compared to
 `cargo run`.

## `cargo run --bin=<binary name>`

 choose which binary to run. Options are: 
  - default binary: name of package
  - binaries under `src/bin`

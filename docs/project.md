# Project: Packages, Crates, Modules

## Packages

"A Cargo feature that lets you build, test, and share crates"

- A bundle of one ore more crates. 
- Contains a `Cargo.toml` file
- Can contain many `binary` crates
- Can only contain **one** `library` crate

## Crates

"A tree of modules that produces a library or a binary (executable)"

- `binary` crates
  - Must have a `main` function
  - Compile to an executable
  - `src/main.rs` is the crate root (crate will have same name as the package)
  - Example: CLI, server
- `library` crates
  - Don't have a `main` function
  - Don't compile to an executable
  - `src/lib.rs` is the crate root (crate will have same name as the package)
  - Define functionality to be shared with multiple projects

Usually, `crate` refers to library `crates`

Cargo passes the crate root files to `rustc` to build the library or binary.

## Modules

"Let you control the organization, scope, and privacy of paths"

[Modules cheat sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

## Paths

"A way of naming an item, such as a struct, function, or module"

## Sharing code between binaries
From: https://stackoverflow.com/questions/59912236/use-module-from-parent-directory-in-rust

To share code between binaries, use `src/lib.rs`.

All binaries automatically have access to anything accessible through `src/lib.rs`
as a separate crate.

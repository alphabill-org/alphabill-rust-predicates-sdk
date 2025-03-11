# Awesome Alphabill predicate `{{project-name}}` #

> TODO: write description

## Prerequisites ##

[cargo-generate](https://cargo-generate.github.io/cargo-generate/)

## Usage ##

    cargo generate \
      --git ssh://git@github.com:alphabill-org/alphabill-rust-predicates-sdk.git
      template

Edit: `src/lib.rs`.

Compile: `cargo build --release --target wasm32-unknown-unknown`

Output: `target/wasm32-unknown-unknown/release/{{crate_name}}.wasm`

## Authors ##

{{authors}}


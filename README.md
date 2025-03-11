# Rust SDK for Alphabill predicates

This is experimental / work-in-progress SDK for creating custom predicates
for [Alphabill](https://alphabill.org/) using [Rust](https://www.rust-lang.org/).

To see the documentation of the SDK run
```sh
cargo doc --all-features
```
in the repository root and open the generated doc in a web browser.


## Quick Start

The [cargo-generate](https://cargo-generate.github.io/cargo-generate/) tool can
be used to create predicate project from template:

```sh
cargo-generate generate --git https://github.com/alphabill-org/alphabill-rust-predicates-sdk template
```

### Predicates

Predicate project should be "library" project with
```toml
crate-type = ["cdylib"]
```
and export function(s) with signature
```rust
#[no_mangle]
pub extern "C" fn predicate_name() -> u64 {
}
```

Return values should follow convention:
 - `0`: predicate evaluated to "true";
 - `1`: predicate evaluated to "false";
 - any other value is considered to be "error code" (ie predicate failed);

SDK provides `predicate_result` macro which should be used to encode
the result of the predicate.


### Create WASM binary
- compile the predicate into WASM binary, ie in case of using Rust
```sh
cargo build --release --target wasm32-unknown-unknown
```

### Create Alphabill predicate

Use [Alhpabill CLI wallet tool](https://github.com/alphabill-org/alphabill-wallet)
to wrap the WASM binary into Alphabill predicate usable with Alphabill units.

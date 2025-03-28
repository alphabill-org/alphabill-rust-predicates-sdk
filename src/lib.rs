/*!
SDK for Alphabill predicate development.

Alphabill supports implementing predicates as WASM modules which export
function(s) with signature
```rust
# use alphabill::predicate_result;
#[unsafe(no_mangle)]
pub extern "C" fn predicate_name() -> u64 {
    predicate_result!(true)
}
```
Return value of the function must follow convention where:
 - `0`: predicate evaluated to "true";
 - `lowest byte of the value == 1`: predicate evaluated to "false", higher 7 bytes
    can be used to encode the reason why predicate evaluated to `false`;
 - any other value is "error code" (IOW predicate failed), predicate's documentation
   should list the meaning of each error code;

It is recommended to use [`predicate_result`] macro for encoding the return value
of the predicate.

## Project template

The [cargo-generate](https://cargo-generate.github.io/cargo-generate/) tool
can be used to start a new predicate project from template:
```shell
cargo-generate generate --git https://github.com/alphabill-org/alphabill-rust-predicates-sdk template
```
Write the predicate logic in the `src/lib.rs` file and compile it to WASM:
```shell
cargo build --release --target wasm32-unknown-unknown
```

Consult the [Alphabill documentation](https://docs.alphabill.org/docs/welcome)
on how to use the output
(`target/wasm32-unknown-unknown/release/crate_name.wasm`) as a
predicate with an Alphabill transaction system unit.

## Features

To help keep the binary size small some of the higher level data structures and functions
can be switched on/off by using feature flags. To enable specific types and functions
set the appropriate features in the `Cargo.toml`, ie

```toml
[dependencies.alphabill]
default-features = false
features = [ "nft-update", "nft-token-data" ]
```
Feature flags available are:
*/
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![no_std]

pub mod api;
pub mod cbor;
pub mod decoder;
pub mod error;
pub mod evaluation_ctx;
pub mod host;
pub mod memory;
pub mod txsystem;

#[cfg(not(test))]
#[cfg(target_arch = "wasm32")]
#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    core::arch::wasm32::unreachable()
}

/**
Helper to make it easier to write return statement for a predicate.

Macro helps to correctly encode the return value of a predicate, it has
three main forms:
 - `predicate_result!(true)` -- predicate returns "true" (encoded as `0`);
 - `predicate_result!(false, [code])` -- predicate returns "false", an
    optional information code (integer) can be sent as second parameter.
    Return value will have lowest byte set to `1`, higher bytes contain the
    optional code value. Ie `predicate_result!(false, 8)` would return `0x0801`.
    When the "code" is longer than 7 bytes higher bytes will be lost;
 - `predicate_result!(Error)` -- predicate returns error (ie abnormal end of
    evaluation), argument must be of type [`Error`]. The `code` value of the
    error is returned unless the lowest byte of the code equals to `1` -- in that
    case the value is shifted left 8 bits so the lowest byte would equal to `0`.
    In case of error code zero we will return max u64 (iow all bytes set to 0xFF).

## Examples
```
use alphabill::predicate_result;

#[unsafe(no_mangle)]
pub extern "C" fn my_predicate() -> u64 {
    if true {
        predicate_result!(true)
    }
    predicate_result!(false)
}
```
[`Error`]: crate::error::Error
 */
#[macro_export]
macro_rules! predicate_result {
    (true) => {
        return 0
    };
    (false) => {
        return 1
    };
    (false, $code:literal) => {
        return ($code << 8) | 1
    };
    (false, $code:expr) => {
        return ($code << 8) | 1
    };
    // when Error is returned make sure that the return value is not 0 or the
    // value of the low byte is not 0x01 (so we wouldn't incorrectly interpret
    // it as "false with informational code" on the host side).
    // in that case (last byte == 1) we just shift it to the left so return
    // value is 0xnn00. In case of error code zero we will return max u64!
    ($err:expr) => {{
        let c = $err.code();
        if c == 0 {
            return u64::MAX;
        }
        if c & 0xFF == 1 {
            return c << 8;
        }
        return c;
    };};
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::error::Error;

    fn pr_true() -> u64 {
        predicate_result!(true);
    }

    fn pr_false() -> u64 {
        predicate_result!(false);
    }

    fn pr_false_literal() -> u64 {
        predicate_result!(false, 0x12ff)
    }

    fn pr_false_expr(code: u64) -> u64 {
        predicate_result!(false, code)
    }

    fn pr_error(err: Error) -> u64 {
        predicate_result!(err);
    }

    #[test]
    fn predicate_result() {
        assert_eq!(0, pr_true());
        assert_eq!(1, pr_false());

        assert_eq!(0x12ff01, pr_false_literal());

        assert_eq!(0x0401, pr_false_expr(4));
        assert_eq!(0x11003301, pr_false_expr(0x110033));

        assert_eq!(0x04, pr_error(Error::new(4)));
        assert_eq!(0x020100, pr_error(Error::new(2).chain(1)));
        assert_eq!(u64::MAX, pr_error(Error::new(0)));
    }
}

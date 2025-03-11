/*!
Minimalist error type for predicates.
*/

extern crate alloc;
use alloc::boxed::Box;

/**
Error is a simple error type which uses u8 error codes.

Error value can keep track of last eight "chained" error codes - chaining
more error codes causes loss of information, ie the earliest error codes
are lost.

It doesn't implement all the methods / traits Rust error is supposed to implement!
*/
#[derive(Debug)]
pub struct Error(u64);

impl Error {
    pub fn new(code: u8) -> Self {
        Self(code as u64)
    }

    /// cain new "code" to the existing code and return it as a new Error instance
    pub fn chain(&self, code: u8) -> Self {
        if code == 0 {
            return Self(self.0);
        }
        Self((self.0 << 8) | (code as u64))
    }

    /// returns chained codes as single u64
    pub fn code(&self) -> u64 {
        self.0
    }
}

/**
Helper to chain error codes with ie [`or_else`](Result::or_else) method of [`Result`].

This function can be used to chain error code for `Result<T, Error>`
return values, ie
```
use alphabill::error::{error_code, Error};

fn foo() -> Result<String, Error> { Err(Error::new(1)) }

fn bar() -> Result<String, Error> {
    let r = foo().or_else(error_code(2))?;
    Ok(r)
}

assert_eq!(0x0102, bar().expect_err("huh?").code())
```
*/
pub fn error_code<T>(code: u8) -> Box<dyn FnOnce(Error) -> Result<T, Error>> {
    return Box::new(move |err: Error| Err(err.chain(code)));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new() {
        let err = Error::new(1);
        assert_eq!(1, err.code())
    }

    #[test]
    fn chain() {
        let err = Error::new(1);
        assert_eq!(0x0102, err.chain(2).code());
        assert_eq!(0x0103, err.chain(3).code());
    }

    fn return_err(code: u8) -> Error {
        if code == 1 {
            return Error::new(code);
        }
        return_err(code - 1).chain(code)
    }

    #[test]
    fn chained() {
        assert_eq!(0x010203, return_err(3).code());
        assert_eq!(0x0102030405060708, return_err(8).code());
        assert_eq!(0x030405060708090a, return_err(10).code());
    }

    fn result() -> Result<u32, Error> {
        Err(Error::new(1))
    }

    fn err(code: u8, x: Result<u32, Error>) -> Result<u32, Error> {
        x.or_else(error_code(code))
    }

    #[test]
    fn resulted() {
        let r = result().or_else(error_code(2));
        assert_eq!(0x0102, r.expect_err("expected error return value").code());

        let r = err(0, Ok(3)).unwrap();
        assert_eq!(3, r);

        let r = err(1, Err(Error::new(2))).expect_err("must return Err");
        assert_eq!(0x0201, r.code());
        let r = err(1, err(2, Err(Error::new(3)))).expect_err("must return Err");
        assert_eq!(0x030201, r.code())
    }
}

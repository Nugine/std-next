#![allow(unsafe_code)]

use crate::sealed::Sealed;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, thiserror::Error)]
#[error("invalid utf8 string")]
pub struct Utf8Error {
    bytes: Vec<u8>,
}

impl Utf8Error {
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

pub trait StringExt: Sealed {
    fn from_utf8_simd(bytes: Vec<u8>) -> Result<String, Utf8Error>;
}

impl Sealed for String {}

impl StringExt for String {
    fn from_utf8_simd(bytes: Vec<u8>) -> Result<String, Utf8Error> {
        match simdutf8::basic::from_utf8(&bytes) {
            Ok(_) => Ok(unsafe { String::from_utf8_unchecked(bytes) }),
            Err(_) => Err(Utf8Error { bytes }),
        }
    }
}

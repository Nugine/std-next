#![allow(unsafe_code)]

use crate::sealed::Sealed;

#[derive(Debug, thiserror::Error)]
#[error("invalid ascii str")]
pub struct AsciiError {}

#[derive(Debug, thiserror::Error)]
#[error("invalid utf8 str")]
pub struct Utf8Error {}

pub trait StrExt: Sealed {
    fn from_ascii_simd(bytes: &[u8]) -> Result<&str, AsciiError>;

    fn from_utf8_simd(bytes: &[u8]) -> Result<&str, Utf8Error>;
}

impl Sealed for str {}

impl StrExt for str {
    fn from_ascii_simd(bytes: &[u8]) -> Result<&str, AsciiError> {
        // TODO(blocking): use `unicode_simd::from_ascii`
        if bytes.is_ascii() {
            Ok(unsafe { core::str::from_utf8_unchecked(bytes) })
        } else {
            Err(AsciiError {})
        }
    }

    fn from_utf8_simd(bytes: &[u8]) -> Result<&str, Utf8Error> {
        simdutf8::basic::from_utf8(bytes).map_err(|_| Utf8Error {})
    }
}

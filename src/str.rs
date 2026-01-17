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

#[cfg(test)]
mod tests {
    use super::StrExt;

    #[test]
    fn test_str_simd_helpers() {
        let ascii = <str as StrExt>::from_ascii_simd(b"hello").expect("valid ascii");
        assert_eq!(ascii, "hello");
        assert!(<str as StrExt>::from_ascii_simd(&[0xff]).is_err());

        let utf8 = <str as StrExt>::from_utf8_simd("hé".as_bytes()).expect("valid utf8");
        assert_eq!(utf8, "hé");
        assert!(<str as StrExt>::from_utf8_simd(&[0xff, 0xff]).is_err());
    }
}

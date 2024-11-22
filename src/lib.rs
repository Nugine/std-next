#![deny(unsafe_code)]
#![deny(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::inline_always,
    clippy::missing_errors_doc, // TODO
)]
// ---
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

mod sealed {
    pub trait Sealed {}

    pub struct Internal {}
}

pub mod default;
pub mod mem;
pub mod ptr;
pub mod str;

#[cfg(feature = "alloc")]
cfg_group! {
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub mod string;
}

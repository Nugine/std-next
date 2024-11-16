#![deny(unsafe_code)]
#![deny(clippy::all, clippy::pedantic, clippy::cargo)]
// ---
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

pub mod default;
pub mod ptr;
pub mod sync;

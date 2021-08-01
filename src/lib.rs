//! A library to parse the `.srt` subtitle format.
//!
//! ### Note
//!
//! This library only supports the old subtitle format and not the new one
//! called [`WebVTT`], which is an extension that shares the
//! same file extension.
//!
//! [`WebVTT`]: https://en.wikipedia.org/wiki/WebVTT
//#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::module_inception, clippy::redundant_pub_crate)]
#![warn(missing_debug_implementations)]
#![feature(never_type)]
#![feature(result_flattening)]
#![feature(trait_alias)]
#![feature(maybe_uninit_uninit_array)]
#![feature(pattern)]
#![feature(array_map)]
#![feature(associated_type_bounds)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod parser;
mod subtitle;
mod subtitle_iterator;
pub mod text;

mod buffer;
mod serialize;
mod utils;

pub use crate::buffer::Buffer;

#[cfg(feature = "alloc")]
pub use crate::serialize::SerializeToString;

pub use crate::serialize::{ExactSerializedLength, Serialize, SerializeWithConfig};

pub use crate::subtitle::{InitError, Subtitle, SubtitleError, SubtitleInit};
pub use crate::text::Text;

pub use crate::parser::ParserError;
pub use crate::subtitle_iterator::SubtitleIterator;

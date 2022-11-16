//! This library provides the ability to read bytecode.
//!
//! # Usage
//!
//! ## Basic
//!
//! ```
//! use bytecode::ByteCode;
//!
//! let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
//!
//! bytes += 3;
//!
//! let _first = bytes[0];
//! let _second = bytes[1];
//!
//! let _subslice = &bytes[2..5];
//! ```
//!
//! ## Utility methods
//!
//! ```
//! use bytecode::ByteCode;
//!
//! let mut bytes = ByteCode::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
//!
//! match bytes.peek(3) {
//!     // omitted
//!     _ => {}
//! }
//!
//! if bytes.starts_with("foo".as_bytes()) {
//!     // omitted
//! }
//!
//! bytes.skip(2);
//!
//! let _subslice = bytes.take(4);
//! ```
//!
//! ```
//! use bytecode::ByteCode;
//!
//! let mut bytes = ByteCode::new(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00]);
//!
//! let _u16 = bytes.take_into_u16(); // u16::MAX
//! let _u32 = bytes.take_into_u32(); // u32::MAX
//! ```

mod core;
mod util;

pub use crate::core::ByteCode;

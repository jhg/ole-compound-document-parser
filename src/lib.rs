//             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyright (C) 2018 Thomas Bailleux <thomas@bailleux.me>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.
//
// Author: zadig <thomas chr(0x40) bailleux.me>

//! A simple parser and reader for Microsoft Compound Document File.
//!
//! This includes a basic parser, which validates the format of a given file
//! or a given stream.
//! It includes a reader too, for iterating over entries and for extracting
//! files inside the OLE storage.
//!
//! ## Example
//!
//! ```
//!
//! use ole::Reader;
//! use std::io::{Read, Write};
//!
//! let mut file = std::fs::File::open("assets/Thumbs.db").unwrap();
//! let mut parser = Reader::new(file).unwrap();
//!
//! // Iterate through the entries
//! for entry in parser.iterate() {
//!     println!("{}", entry);
//! }
//!
//! // We're going to extract a file from the OLE storage
//! let entry = parser.iterate().next().unwrap();
//! let mut slice = parser.get_entry_slice(entry).unwrap();
//! let mut buffer = std::vec::Vec::<u8>::with_capacity(slice.len());
//! slice.read_to_end(&mut buffer);
//!
//! // Saves the extracted file
//! let mut extracted_file = std::fs::File::create("./file.bin").unwrap();
//! extracted_file.write_all(&buffer[..]);
//! ```
//!
//! ## Compatibility
//!
//! The `ole` crate is tested for rust 1.9 or greater.


mod ole;
pub use ole::Reader;
pub(crate) mod iterator;
pub use iterator::OLEIterator;
mod error;
pub use error::Error;
pub(crate) mod header;
pub(crate) mod util;
pub(crate) mod sat;
pub(crate) mod constants;
pub(crate) mod entry;
pub use entry::Entry;
pub use entry::EntrySlice;
pub use entry::EntryType;
pub(crate) mod sector;

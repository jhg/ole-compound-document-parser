# OLE

[![Crates.io](https://img.shields.io/crates/v/ole.svg)](https://crates.io/crates/ole)
[![Crates.io](https://img.shields.io/crates/d/ole.svg)](https://crates.io/crates/ole)
[![license](http://img.shields.io/badge/license-WTFPL-blue.svg)](https://github.com/zadlg/olekit/blob/master/LICENSE)

A simple parser and reader for Microsoft Compound Document File.

This includes a basic parser, which validates the format of a given file or a
given stream. It includes a reader too, for iterating over entries and for
extracting files inside the OLE storage.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ole = "0.1.15"
```

and this to your crate root:

```rust
extern crate ole;
```

## Example

```rust
use ole::Reader;
use std::io::{Read, Write};
let mut file = std::fs::File::open("assets/Thumbs.db").unwrap();
let mut parser = Reader::new(file).unwrap();
// Iterate through the entries
for entry in parser.iterate() {
    println!("{}", entry);
}
// We're going to extract a file from the OLE storage
let entry = parser.iterate().next().unwrap();
let mut slice = parser.get_entry_slice(entry).unwrap();
let mut buffer = std::vec::Vec::<u8>::with_capacity(slice.len());
slice.read_to_end(&mut buffer);
// Saves the extracted file
let mut extracted_file = std::fs::File::create("./file.bin").unwrap();
extracted_file.write_all(&buffer[..]);
```



## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## Compatibility

`ole` seems to work for rust 1.9 and greater.

## License

<http://www.wtfpl.net/about/>

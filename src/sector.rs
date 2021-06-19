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


impl<'ole> super::ole::Reader<'ole> {
  pub(crate) fn read_sector(&self, sector_index: usize)
    -> Result<(&[u8]), super::error::Error> {
    let result: Result<(&[u8]), super::error::Error>;
    let sector_size = self.sec_size.unwrap();
    let offset = sector_size * sector_index;
    let max_size = offset + sector_size;

    let body_size: usize;
    if self.body.is_some() {
      body_size = self.body.as_ref().unwrap().len();
    } else {
      body_size = 0;
    }

    // Check if the sector has already been read
    let sector : &[u8];
    if body_size >= max_size {
      let body = self.body.as_ref().unwrap();
      sector = &body[offset .. offset + sector_size];
      result = Ok(sector);
    } else {
      result = Err(super::error::Error::BadSizeValue("File is too short"));
    }

    result
  }
}

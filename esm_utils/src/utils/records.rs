use bytes::{Buf, Bytes};
use nom::IResult;

use super::subrecords::Subrecord;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Record {
  pub s_type: String,
  pub size: usize,
  pub subrecords: Vec<Subrecord>,
}

impl Record {
  pub fn new(mut buffer: Bytes) -> IResult<Bytes, Self> {
    let header = buffer.split_to(24);

    let s_type = String::from_utf8(header.slice(..4).to_vec()).expect("Error");

    let size = header.slice(4..8).get_u32_le() as usize;

    let mut subrecords_buffer = buffer.split_to(size);

    let mut subrecords = Vec::new();

    while subrecords_buffer.len() > 0 {
      let subrecord_tuple = Subrecord::new(subrecords_buffer).unwrap();

      subrecords.push(subrecord_tuple.1);

      subrecords_buffer = subrecord_tuple.0;
    }

    Ok((
      buffer,
      Record {
        s_type,
        size,
        subrecords,
      },
    ))
  }
}
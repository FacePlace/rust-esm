use bytes::{Buf, Bytes};
use nom::IResult;

use super::{buffer::str_from_buffer, headers::Headers, subrecords::Subrecord};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Record {
  pub s_type: String,
  pub size: usize,
  pub subrecords: Vec<Subrecord>,
}

impl Record {
  pub fn new(
    mut buffer: Bytes,
    signature: String,
  ) -> IResult<Bytes, Self> {
    let header_size = Headers::new().record;

    let mut header = buffer.split_to(header_size);

    let s_type = str_from_buffer(header.split_to(4));

    if s_type == signature { // TODO: Handle subgroups
    }

    let size = header.split_to(4).get_u32_le() as usize;

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

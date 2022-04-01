use bytes::{Buf, Bytes};
use nom::IResult;

use super::headers::Headers;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Subrecord {
  pub s_type: String,
  pub size: usize,
  pub z_string: Bytes,
}

impl Subrecord {
  pub fn new(mut buffer: Bytes) -> IResult<Bytes, Self> {
    let header_size = Headers::new().subrecord;

    let header = buffer.split_to(header_size);

    let s_type = String::from_utf8(header.slice(..4).to_vec()).expect("Error");

    let size = header.slice(4..6).get_u16_le() as usize;

    let z_string = buffer.split_to(size);

    Ok((
      buffer,
      Subrecord {
        s_type,
        size,
        z_string,
      },
    ))
  }
}

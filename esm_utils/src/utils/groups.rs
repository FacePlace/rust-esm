use bytes::{Buf, Bytes};
use nom::IResult;

use super::records::Record;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Group {
  pub s_type: String,
  pub size: usize,
  pub signature: String,
  pub records: Vec<Record>,
}

impl Group {
  pub fn new(mut buffer: Bytes) -> IResult<Bytes, Self> {
    let header = buffer.split_to(24);

    let s_type = String::from_utf8(header.slice(..4).to_vec()).expect("Error");

    let size = header.slice(4..8).get_u32_le() as usize;

    let signature = String::from_utf8(header.slice(8..12).to_vec()).expect("Error");
    // println!("Processing group: {}.", signature);

    let mut records_buffer = buffer.split_to(size - 24);

    let mut records = Vec::new();

    if signature != "NAVI"
      && signature != "CELL"
      && signature != "WRLD"
      && signature != "DIAL"
      && signature != "NPC_"
    {
      while records_buffer.len() > 0 {
        let record_tuple = Record::new(records_buffer).unwrap();

        records.push(record_tuple.1);

        records_buffer = record_tuple.0;
      }
    }

    Ok((
      buffer,
      Group {
        s_type,
        size,
        signature,
        records,
      },
    ))
  }
}
use bytes::{Buf, Bytes};
use esm_utils::utils::{groups::Group, headers::Headers};
use std::{fs::File, io::Read, time::Instant};

pub fn parse_esm(path: String) -> Vec<Group> {
  let now = Instant::now();

  let filename = path.split("/").last().unwrap();

  let mut f = File::open(path.to_owned()).expect("Not found");

  let mut buf = Vec::new();

  f.read_to_end(&mut buf).expect("Err");

  let mut mem = Bytes::from(buf);

  let mut master_header = mem.split_to(Headers::new().record);

  let _master_buffer = mem.split_to(master_header.slice(4..8).get_u32_le() as usize); // skip master header's contents

  let mut groups = Vec::new();

  while mem.len() > 0 {
    let group_tuple = Group::new(mem).unwrap();

    let group = group_tuple.1;

    // println!("Processing group {} finished.", group.signature);

    groups.push(group);

    mem = group_tuple.0;
  }

  println!("Processed {} in {:.2?}", filename, now.elapsed());

  groups
}

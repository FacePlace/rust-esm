use bytes::{Buf, Bytes};
use esm_utils::utils::groups::Group;
use std::{fs::File, io::Read, time::Instant};

fn main() {
  let now = Instant::now();

  let file_path = "data/Skyrim/Skyrim.esm";

  let mut f = File::open(file_path).expect("Not found");

  let mut buf = Vec::new();

  f.read_to_end(&mut buf).expect("Err");

  let mut mem = Bytes::from(buf);

  mem.split_to(78);

  let mut groups = Vec::new();

  while mem.len() > 0 {
    let group_tuple = Group::new(mem).unwrap();

    let group = group_tuple.1;

    // println!("Processing group {} finished.", group.signature);

    groups.push(group);

    mem = group_tuple.0;
  }

  println!("{:?}", groups);

  println!("{:.2?}", now.elapsed());
}

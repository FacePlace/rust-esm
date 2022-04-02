use esm_parser::parser::parse_esm;
use std::time::Instant;

fn main() {
  let now = Instant::now();

  let file_path = "data/Skyrim/Skyrim.esm";

  let _groups = parse_esm(file_path.to_string());

  // println!("{:?}", groups);

  println!("Finished {:.2?}", now.elapsed());
}

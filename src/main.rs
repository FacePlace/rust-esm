use esm_parser::parser::parse_esm;
use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();

    let files = fs::read_dir("data").unwrap();

    for n in files {
        let path = n.unwrap().path().to_str().unwrap().to_owned();
        // println!("{}", path);
        let _groups = parse_esm(path);
    }

    // println!("{:?}", groups);

    println!("Finished in {:.2?}", now.elapsed());
}

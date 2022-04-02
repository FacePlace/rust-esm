use esm_parser::parser::parse_esm;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let files = vec![
        "data/Skyrim/Skyrim.esm",
        "data/Skyrim/Update.esm",
        "data/Skyrim/HearthFires.esm",
        "data/Skyrim/Dawnguard.esm",
        "data/Skyrim/Dragonborn.esm",
    ];

    for n in files.iter() {
        let _groups = parse_esm(n.to_string());
    }

    // println!("{:?}", groups);

    println!("Finished in {:.2?}", now.elapsed());
}

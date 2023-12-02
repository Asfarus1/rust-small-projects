extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    if dbg!(args()).len() != 3 {
        eprintln!("Usage: <input> <output>");
        return;
    }
    let mut br = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut br, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Sourse len: {:?}\nCompressed len: {:?}\nTime: {:?} ms",
        br.get_ref().metadata().unwrap().len(),
        output.metadata().unwrap().len(),
        start.elapsed()
    );
}

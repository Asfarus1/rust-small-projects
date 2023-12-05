use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

fn main() {

    let srcdir = PathBuf::from("./src/");
    println!("{:?}", fs::canonicalize(&srcdir));
    dbg!(std::env::current_dir().unwrap());
    if let Ok(lines) = read_lines(format!("src/main.rs")) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }      
        }   
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    dbg!(std::fs::canonicalize(Path::new(filename.as_ref())))?;
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

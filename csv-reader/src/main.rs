use std::error::Error;
use csv;

fn read_csv_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(filename)?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_csv_file("SampleCSVFile_11kb.csv") {
        eprint!("Error: {}", e);
    }
}

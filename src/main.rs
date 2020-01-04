use std::env;
use std::process;
fn analyze_records(csv_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    let mut count = 0;
    for result in rdr.byte_records() {
        let _record = result?;
        count += 1;
    }
    println!("Found {} records", count);
    Ok(())
}
fn main() {
    let path = env::args().nth(1);
    if let Some(path) = path {
        if let Err(err) = analyze_records(path) {
            println!("error running example: {}", err);
            process::exit(1);
        }
    } else {
        println!("Please provide mix.log example");
        process::exit(1);
    }
}

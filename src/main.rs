use std::env;
use std::process;
fn analyze_records(csv_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    let mut rampup_start = 0.0;
    let mut rampup_end = 0.0;
    for result in rdr.records() {
        let record = result?;
        if rampup_start == 0.0 {
            rampup_start = record[0].parse::<f32>().unwrap();
        }

        if &record[1] == "START" {
            rampup_end = record[0].parse::<f32>().unwrap();
        }
    }
    println!("{:01} second(s) ramping up", rampup_end - rampup_start);
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

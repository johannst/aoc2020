use std::fs::File;
use std::io::BufReader;

pub fn read_input_to_string(day: &str) -> String {
    let path = format!("{}/input/{}", std::env!("CARGO_MANIFEST_DIR"), day);
    match std::fs::read_to_string(&path) {
        Ok(input) => input,
        Err(e) => {
            println!("Failed to read input from '{}' with following error", path);
            println!("{}", e);
            std::process::exit(1);
        }
    }
}

pub fn input_bufreader(day: &str) -> BufReader<File> {
    let path = format!("{}/input/{}", std::env!("CARGO_MANIFEST_DIR"), day);
    match File::open(&path) {
        Ok(input) => BufReader::new(input),
        Err(e) => {
            println!("Failed to read input from '{}' with following error", path);
            println!("{}", e);
            std::process::exit(1);
        }
    }
}

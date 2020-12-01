pub fn read_input(day: &str) -> String {
    let path = format!("{}/input/{}", std::env!("CARGO_MANIFEST_DIR"), day);
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(e) => {
            println!("Failed to read input for '{}'!", day);
            println!("{}", e);
            std::process::exit(1);
        }
    }
}

use std::env;

fn main() {
    // Read in arguments
    let args: Vec<String> = env::args().collect();

    // Save values read in
    let query = &args[1];
    let filename = &args[2];
}

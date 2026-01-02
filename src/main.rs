use std::env;

use b64::process_args;


fn main() {
    let args: Vec<String> = env::args().collect();

    process_args(&args);
}


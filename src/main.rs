use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let pattern = &args[1];
    let file_path = &args[2];

    let contents =
        fs::read_to_string(file_path).expect(&format!("Error reading file {}.", file_path));

    println!("{contents}");
}

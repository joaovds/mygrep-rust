use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let pattern = &args[1];
    let file_path = &args[2];
}

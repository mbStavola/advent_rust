use std::io::Read;
use std::fs::File;

fn open_problem_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();

    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let len = buffer.len();

    buffer.truncate(len - 1);
    buffer
}

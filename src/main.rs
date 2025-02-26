mod lex;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("main.vb");
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    println!("{content}");
}

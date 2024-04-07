use std::fs;
use std::io;

fn main() {
    let file = fs::read_to_string("file.txt").expect("Unable to read contents of the file");
    let mut content = String::new();
    io::stdin().read_line(&mut content).expect("Unabel to read input");
    println!("The contents of the file are, \n{}", file);
}

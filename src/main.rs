use std::fs;
// use std::io;

fn main() {
    let file = fs::read_to_string("file.txt").unwrap();
    // let mut content = String::new();
    let content = "file";
    for item in file.lines(){
        println!("{}", item);
        // if item == content{
        //     println!("{} \n", item);
        // }
    }
    // io::stdin().read_line(&mut content).expect("Unabel to read input");
    // println!("The contents of the file are, \n{file}");
}

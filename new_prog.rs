// First rust program for data engineering 
// to run the script the file needs to be made as executable 
// first run rustc file_name.rs 
// then run ./filename without any extension to run the code 
// If you make changes to your file, you need to re-compile it with 
// rustc and then run the code to find the results!


// fn main() {
//     println!("Hello World!");
// }

use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();

    io::stdin().read_line(&mut name)
        .expect("Failed to read input.");

    println!("Hello, {}! Welcome to the world of Rust!", name.trim());
}

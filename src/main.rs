// Program to take a string and file name from command line and then
// do a search for the string in the file and print the results

// std::env::args function returns an iterator of the command line arguments
// collect() function on the iterator saves them in a vector

use std::env;
// use fs module to read contents of file
use std::fs;

fn main() {

    // Take the command line args and save it in a vector called args
    // Rust typically can infer the type but when the collect() function is used
    // Rust is not able to infer the types of data in the collection. So here 
    // we will use the Vec<String> to tell Rust that we are expecting strings to be
    // stored in the vector
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // Store the vector elements obtained from command line into variables
    // Note we are saving the references in these variables. 
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // Take the text in the file and store it as a string in the variable
    // contents
    let contents = fs::read_to_string(filename)
                    .expect("Something went wrong reading the file");
    println!("contents of the file are \n {}", contents);


}
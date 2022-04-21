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

    // call the new() function on Config to store the query and filename
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Take the text in the file and store it as a string in the variable
    // contents
    let contents = fs::read_to_string(config.filename)
                    .expect("Something went wrong reading the file");
    println!("contents of the file are \n {}", contents);


}

// We use config struct to group variables query and filename to better describe
// their relationship
// fields here will hold "owned" Strings
struct Config {
    query: String,
    filename: String
}

//Create a new method on config to create instances of Config
// this new function when called will store the query and filename

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename}
    }
}


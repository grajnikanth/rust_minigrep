// Program to take a string and file name from command line and then
// do a search for the string in the file and print the results

// std::env::args function returns an iterator of the command line arguments
// collect() function on the iterator saves them in a vector

use std::env;
// use fs module to read contents of file
use std::fs;

// Us the process::exit function to stop the program
use std::process;

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
    // unwrap_or_else() function will return the value inside the Result
    // type Ok() field if successful if not the error &str is passed as an argument
    // to the closure function
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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

// Create a new method on config to return a Result. Result will allow to handle
// the errors better in the main function

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {

        // Error handling - If sufficient arguments in command line were not 
        // provided panic and throw an error to user 
        if args.len() < 3 {
            // Return an Err of the type Result. The string literl being 
            // passed here can be referenced in the main program
            return Err("Sufficient arguments were not passed")
            
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        // Since we are returning a Result type, wrap the Config in Ok()
        Ok(Config { query, filename})
    }
}


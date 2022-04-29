// Chapter 12 - Rust book project
// Program to take a string and file name from command line and then
// do a search for the string in the file and print the results

// std::env::args function returns an iterator of the command line arguments
// collect() function on the iterator saves them in a vector

use std::env;
// use fs module to read contents of file


// Us the process::exit function to stop the program
use std::process;

// 
use minigrep::Config;


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
        // println! macro does not print errors to the standard error stream
        // So if you redirect output of this program to a file, the errors are also
        // redirected to the file. But typically errors should be redirected to 
        // standard error, we will use eprintln! macro
        // println!("Problem parsing arguments: {}", err);
        eprintln!("Problem parsing arguments: {}", err);

        
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

     // Error handling when run() function is used to read the file contents
     // if let here matches on the run(config) and if it returns an Err then
     // the value is assigned to "e" variable and we can use that in our funct//    
     // Note that the run function is prefixed with minigrep. I could use the "use"
     // syntax to bring in the "run" function but it is better to use the the
     // syntax shown below so that it is clear where the run function is coming from
    if let Err(e) = minigrep::run(config) {
        
        //println!("Application error: {}", e);
        eprintln!("Application error: {}", e);
        
        process::exit(1);
    }
    


}

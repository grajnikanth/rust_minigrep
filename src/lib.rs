use std::fs;
use std::error::Error;

// We use config struct to group variables query and filename to better describe
// their relationship
// fields here will hold "owned" Strings
pub struct Config {
    pub query: String,
    pub filename: String
}

// Create a new method on config to return a Result. Result will allow to handle
// the errors better in the main function

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

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

// run() function will have the functionality related to reading the file contents
// We are using smart pointer Box<> which will store the data in a heap and 
// provide a reference to this data 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Take the text in the file and store it as a string in the variable
    // contents

    let contents = fs::read_to_string(config.filename)?;
                    
    println!("contents of the file are \n {}", contents);
    Ok(())

}

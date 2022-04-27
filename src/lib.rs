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

// We want the search function to return a vector contianing the 
// lines which contain the query string
// We will use str references so we need a lifetime notation. In this case
// logically it make sense to match the lifetime of the contents which is a &str
// containing lines of the text to be searched and we want to return portions of this
// vector
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    // Mutable vector is needed to store the results from the for loop
    let mut results = Vec::new();
    // .lines() function provides a iterator for each of the line in the 
    // &str
    for line in contents.lines() {
        
        // contains() method checks if a given string contains the quereied string 
        // sent as an argument
        if line.contains(query) {
            // If the line contains the query store that in the results vector
            results.push(line);

        }
    }
    // results vector is returned. The results vector is defined in this search function
    // scope but sine it is a reference type, it is borrowed/referenced by the 
    // returned vector. The result vector will store the memory address of the 
    // lines in the string contents. So it will will stay in memory until contents
    // stays in memory. Also we are telling Rust that the returned value has to
    // stay in memory as long as contents are in the memory
    results
}


// writing tests to check the logic of the code
#[cfg(test)]
mod tests {
    // tests module can access the logic from above
    use super::*;

    #[test] 
    fn one_result() {
        let query = "duct";
        // The first \ makes sure that there is no newline at the begining of the
        // the text stored in contents variable
        // Note that the string in the contents variable has to be formatted to 
        // eliminate the extra spaces in front of each line. Other wise the empty
        // spaces are also considered to be part of the string/line
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}
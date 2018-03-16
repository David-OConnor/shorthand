use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::borrow::Cow;

use regex::Regex;


pub fn read_file(filename: &str) -> Result<String, io::Error> {
    // Opens a file, and returns its text content
    let mut file = File::open(filename)?;

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}


pub fn trailing_ed(contents: String) -> String {
    // Change trailing ed to D, trailing es to S etc.

    // todo: Handle if an s follows.
    let tail_letters = ['d', 'x', 's', 'l', 'r'];

    let mut result = contents;
    // Could handle this loop with a more complex regex instead.
    for letter in &tail_letters {
        let re_str = r"(e".to_string() + &letter.to_string() + r")\s+";
        let re = Regex::new(&re_str).unwrap();

        let replacement: &str = &(letter.to_uppercase().to_string() + " ");

        if let Cow::Owned(s) = re.replace_all(&result, replacement) {
           result = s.to_string();
       }
    }

    result
}


pub fn parse(filename: &str) -> String {
    let contents = read_file(filename);
    
    let mut contents = match contents {
        Ok(c) => c,
        Err(e) => panic!(
            "Problem reading the file, dude: {}", e 
            ),
    };

    contents = trailing_ed(contents);

    contents
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
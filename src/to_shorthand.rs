use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;

use regex::Regex;


// Note: This includes a trailing s, denoting plural.
// We use the named group t to include in the new pattern applied by
//Regex.replace_all().
const WORD_END: &str = r"(?P<tail>s?[\s+\.,;\?!-:$])";

pub fn read_file(filename: &str) -> Result<String, io::Error> {
    // Opens a file, and returns its text content
    let mut file = File::open(filename)?;

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn replace_all(re: Regex, words: &str, replacement: &str) -> Option<String> {
    // Parse the Cow here; we use it multiple places.
    if let Cow::Owned(s) = re.replace_all(words, replacement) {
        return Some(s.to_string());
    }
    return None
}

fn pattern_replacement(
    contents: String,
    replacements: HashMap<&str, &str>,
    preceding_match: &str,
) -> String {
    // Helper function to reduce repetition between word_replacements and suffixes.
    // preceding_match is what to match, but not replace, before the text to replace.
    // preceding_match must be a raw string including a <head> named capture group.

    let mut result = contents;

    for (original, repl) in replacements.iter() {
        let re_str = preceding_match.to_string() + original + WORD_END;
        let re = Regex::new(&re_str).unwrap();

        let replacement: &str = &("${head}".to_string() + repl + "$tail");

        match replace_all(re, &result, replacement) {
            None => (),
            Some(s) => result = s,
        }
    }
    result
}

pub fn word_replacements(contents: String) -> String {
    // Replace a few short words.
    let mut replacements = HashMap::new();

    // The most common English words here, in order.
    replacements.insert("the", "T");
    replacements.insert("be", "B");
    replacements.insert("to", "O");
    replacements.insert("of", "F");
    replacements.insert("and", "A");
    replacements.insert("in", "n");  // Could be handled with i replacement
    replacements.insert("that", "TT");
    replacements.insert("have", "");
    replacements.insert("it", "t");

    replacements.insert("because", "BC");
    
    replacements.insert("with", "W");
    replacements.insert("from", "FM");
    replacements.insert("when", "WN");
    replacements.insert("this", "TS");
    
    replacements.insert("are", "R");
    replacements.insert("you", "U");
    

    // The following may be more elegantly handled through the i-replacement logic
    
    
    replacements.insert("is", "s");
    replacements.insert("if", "f");

    pattern_replacement(contents, replacements, r"(?P<head>\s+)")
}

pub fn suffixes(contents: String) -> String {
    // Convert trailing ing to G, and tion to N.
    let mut replacements = HashMap::new();
    replacements.insert("ing", "G");
    replacements.insert("tion", "N");

    pattern_replacement(contents, replacements, "")
}

pub fn trailing_ed(contents: String) -> String {
    // Change trailing ed to D, trailing es to S etc.

    let tail_letters = ['d', 'x', 's', 'l', 'r', 't'];

    let mut result = contents;

    for letter in &tail_letters {
        let re_str = "e".to_string() + &letter.to_string() + WORD_END;
        let re = Regex::new(&re_str).unwrap();

        let replacement: &str = &(letter.to_uppercase().to_string() + "$tail");

        match replace_all(re, &result, replacement) {
            None => (),
            Some(s) => result = s,
        }
    }
    cleanup_double_s(result)
}

fn cleanup_double_s(contents: String) -> String {
    // We don't want words that end in ess to be converted to Ss - Run this
    // after trailing_ed to correct, ideally at the end of trailing_ed.

    let mut result = contents;

    let re_str = "Ss".to_string() + WORD_END;
    let re = Regex::new(&re_str).unwrap();

    let replacement: &str = &("ess".to_string() + "$tail");

    match replace_all(re, &result, replacement) {
        None => (),
        Some(s) => result = s,
    }
    
    result
}


pub fn run(filename: &str) -> String {
    let contents = read_file(filename);
    
    let mut contents = match contents {
        Ok(c) => c,
        Err(e) => panic!(
            "Problem reading the file, dude: {}", e 
            ),
    };

    contents = trailing_ed(contents);
    contents = suffixes(contents);
    contents = word_replacements(contents);

    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_letters() {
        // This test string should include multiple applicable letters,
        // and have spaces, newlines, and various punctuation after some of the 
        // test words.
        let test_str = String::from(
            "John is better at running while wearing her sweater.
It's complex, yet subtle!",
        );

        let expected = String::from(
            "John is bettR at running while wearing hR sweatR.
It's complX, yT subtle!",
        );

        assert_eq!(expected, trailing_ed(test_str));
    }

    #[test]
    fn trailing_plural() {
        let test_str = String::from("These punters dress in pastels.");

        let expected = String::from("These puntRs dress in pastLs.");

        assert_eq!(expected, trailing_ed(test_str));
    }

     #[test]
    fn ing() {
        let test_str = String::from(
            "Testing is a fun thing to do. So is swimming; playing
basketball. Fencing.",
        );

        let expected = String::from(
            "TestG is a fun thG to do. So is swimmG; playG
basketball. FencG.",
        );

        assert_eq!(expected, suffixes(test_str));
    }

    #[test]
    fn ing_plural() {
        let test_str = String::from("Things with clippings.");
        let expected = String::from("ThGs with clippGs.");

        assert_eq!(expected, suffixes(test_str));
    }

    #[test]
    fn tion() {
        let test_str = String::from("The function of your junction captions me.");

        let expected = String::from("The funcN of your juncN capNs me.");

        assert_eq!(expected, suffixes(test_str));
    }

    #[test]
    fn words() {
        let test_str = String::from(
            "Here, in this and that place, from where and when it is thoroughly
normal with all. Only if.",
        );

        let expected = String::from(
            "Here, n TS A TT place, FM where A WN t s thoroughly
normal W all. Only f.",
);

        assert_eq!(expected, word_replacements(test_str));
    }

    #[test]
    fn test_multiple() {
        let test_str = String::from(
            "Testing is a fun thing to do. So is swimming; playing
basketball. Fencing and formal functions. Croquet with loose rubber rods, in 
place of complexs more substantial. Label this; who are you?",
        );

        let expected = String::from(
            "TestG s a fun thG to do. So s swimmG; playG
basketball. FencG A formal funcNs. CroquT W loose rubbR rods, n 
place of complXs more substantial. LabL TS; who R U?",
        );

        let mut contents = trailing_ed(test_str);
        contents = suffixes(contents);
        contents = word_replacements(contents);
        assert_eq!(expected, contents);
    }
}
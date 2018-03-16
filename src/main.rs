extern crate regex;

mod to_shorthand;

fn main() {

    println!("{}", to_shorthand::parse("notes.txt"));
}

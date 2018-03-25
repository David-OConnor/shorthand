extern crate regex;

mod to_shorthand;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Use exactly one argument: The filename. Received {} args.",
               args.len() - 1);
    }

    let filename = &args[1];
    println!("{}", to_shorthand::run(filename));
}

extern crate exec;

use std::env;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    if argv.len() < 1 {
        println!("Must specify command to execute");
        process::exit(1);
    }

    // Exec the specified program.  If all goes well, this will never
    // return.  If it does return, it will always retun an error.
    let err = exec::Command::new(&argv[0]).args(&argv).exec();
    println!("Error: {}", err);
    process::exit(1);
}

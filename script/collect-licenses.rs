//usr/bin/env rustc $0 -o a.out && ./a.out ; rm -f ./a.out ; exit

fn main() {
    println!("Hello world");

    
}

// Next steps:
// 1a. Add wiring in Zed to check for a licenses markdown file
// 1b. Add wiring in Zed.dev for builds to publish licenses alongside releases as well as licenses for Zed.dev itself
//     (e.g. https://github.com/zed-industries/zed.dev/tree/main/content/licenses)
// 2. Figure out how to run those commands and get the license text for each MIT and Apache licensed software
// 3. Add in the configuration file:
//      a. and refactor this script to have types of licenses
//      b. add callback handlers for each type,
//      c. check if the handler succeeds

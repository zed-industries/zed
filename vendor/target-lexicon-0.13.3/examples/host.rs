extern crate target_lexicon;

use target_lexicon::HOST;

fn main() {
    println!(
        "{}",
        HOST.pointer_width()
            .expect("architecture should be known")
            .bytes()
    );
}

use pulldown_cmark::{Options, Parser};

fn main() {
    let text = "Here is a sentence. \nHere is another. `code` here";
    let mut parser = Parser::new_ext(text, Options::all()).into_offset_iter();
    while let Some((event, range)) = parser.next() {
        println!("{:?} {:?}", event, range);
    }
}

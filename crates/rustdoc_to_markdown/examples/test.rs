use rustdoc_to_markdown::convert_rustdoc_to_markdown;

pub fn main() {
    let html = include_str!("/Users/maxdeviant/projects/zed/target/doc/gpui/index.html");
    let markdown = convert_rustdoc_to_markdown(html).unwrap();

    println!("{markdown}");
}

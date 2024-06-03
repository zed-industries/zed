use indoc::indoc;
use rustdoc_to_markdown::convert_rustdoc_to_markdown;

pub fn main() {
    let html = indoc! {"
        <html>
            <body>
                <h1>Hello World</h1>
                <p>
                    Here is some content.
                </p>
                <h2>Some items</h2>
                <ul>
                    <li>One</li>
                    <li>Two</li>
                    <li>Three</li>
                </ul>
            </body>
        </html>
    "};
    // To test this out with some real input, try this:
    //
    // ```
    // let html = include_str!("/path/to/zed/target/doc/gpui/index.html");
    // ```
    let markdown = convert_rustdoc_to_markdown(html.as_bytes()).unwrap();

    println!("{markdown}");
}

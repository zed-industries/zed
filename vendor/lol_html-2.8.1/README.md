# LOL HTML

<p align="center">
    <a href="https://github.com/cloudflare/lol-html">
        <img src="media/logo.png" alt="The logo is generated from https://openmoji.org/data/color/svg/1F602.svg by Emily Jäger which is licensed under CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0/)" style="width:472px; height: 375px" />
    </a>
</p>


***L**ow **O**utput **L**atency streaming **HTML** rewriter/parser with CSS-selector based API.*

It is designed to modify HTML on the fly with minimal buffering. It can quickly handle very large
documents, and operate in environments with limited memory resources. More details can be found in the [blog post](https://blog.cloudflare.com/html-parsing-2/).

The crate serves as a back-end for the HTML rewriting functionality of
[Cloudflare Workers](https://www.cloudflare.com/en-gb/products/cloudflare-workers/), but can be used
as a standalone library with a convenient API for a wide variety of HTML rewriting/analysis tasks.

## Documentation

https://docs.rs/lol_html/

## Bindings for other programming languages
- [C](https://github.com/cloudflare/lol-html/tree/master/c-api)
- [Lua](https://github.com/jdesgats/lua-lolhtml)
- [Go](https://github.com/coolspring8/go-lolhtml) (unofficial, not coming from Cloudflare)
- [Ruby](https://github.com/gjtorikian/selma) (unofficial, not coming from Cloudflare)

## Example

Rewrite insecure hyperlinks:

```rust
use lol_html::{element, HtmlRewriter, Settings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = vec![];

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("a[href]", |el| {
                    let href = el
                        .get_attribute("href")
                        .expect("href was required")
                        .replace("http:", "https:");

                    el.set_attribute("href", &href)?;

                    Ok(())
                })
            ],
            ..Settings::new()
        },
        |c: &[u8]| output.extend_from_slice(c)
    );

    rewriter.write(b"<div><a href=")?;
    rewriter.write(b"http://example.com>")?;
    rewriter.write(b"</a></div>")?;
    rewriter.end()?;

    assert_eq!(
        String::from_utf8(output)?,
        r#"<div><a href="https://example.com"></a></div>"#
    );
    Ok(())
}
```

## License

BSD licensed. See the [LICENSE](LICENSE) file for details.

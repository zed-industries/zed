# Examples

The examples have been designed with [`cargo-script`](https://github.com/DanielKeep/cargo-script) in mind.

Here I'll just give broad overview how to install [`cargo script`] for Rust 1.5. For more details, check out [cargo-script repository](https://github.com/DanielKeep/cargo-script).

    cargo install cargo-script


# Token printer

The basis of xml5ever is its tokenizer and tree builder. Roughly speaking tokenizer
takes input and returns a set of tokens like comment, processing instruction, start
tag, end tag, etc.

First let's define our dependencies:

```toml
    [dependencies]
    xml5ever = "0.2.0"
    tendril = "0.1.3"
```

With dependencies declared, we can now make a simple tokenizer sink. First step is to
define a [`TokenSink`](https://docs.rs/xml5ever/latest/xml5ever/tokenizer/trait.TokenSink.html). [`TokenSink`](https://docs.rs/xml5ever/latest/xml5ever/tokenizer/trait.TokenSink.html) are traits that received stream of [`Tokens`](https://docs.rs/xml5ever/latest/xml5ever/tokenizer/enum.Token.html).

In our case we'll define a unit struct (i.e. a struct  without any fields).

```rust
    struct SimpleTokenPrinter;
```

To make `SimpleTokenPrinter` a [`TokenSink`](https://docs.rs/xml5ever/latest/xml5ever/tokenizer/trait.TokenSink.html), we need to implement [process_token](https://docs.rs/xml5ever/latest/xml5ever/tokenizer/trait.TokenSink.html#tymethod.process_token) method.

```rust
    impl TokenSink for SimpleTokenPrinter {
        fn process_token(&mut self, token: Token) {
            match token {
                CharacterTokens(b) => {
                    println!("TEXT: {}", &*b);
                },
                NullCharacterToken => print!("NULL"),
                TagToken(tag) => {
                    println!("{:?} {} ", tag.kind, &*tag.name.local);
                },
                ParseError(err) => {
                    println!("ERROR: {}", err);
                },
                PIToken(Pi{ref target, ref data}) => {
                    println!("PI : <?{} {}?>", &*target, &*data);
                },
                CommentToken(ref comment) => {
                    println!("<!--{:?}-->", &*comment);
                },
                EOFToken => {
                    println!("EOF");
                },
                DoctypeToken(Doctype{ref name, ref public_id, ..}) => {
                    println!("<!DOCTYPE {:?} {:?}>", &*name, &*public_id);
                }
            }
        }
    }
```

Now, we need some input to process. For input we'll use `stdin`. However, xml5ever `tokenize_to` method only takes `StrTendril`. So we need to construct a
[`ByteTendril`](https://docs.rs/tendril/latest/tendril/type.ByteTendril.html) using `ByteTendril::new()`, then read the `stdin` using [`read_to_tendril`](https://docs.rs/tendril/latest/tendril/trait.ReadExt.html#tymethod.read_to_tendril) extension.

Once that is set, to make `SimpleTokenPrinter` parse the input, call,
`tokenize_to` with it as the first parameter, input wrapped in Option for second parameter and XmlToke.

```rust
    fn main() {
        let sink = SimpleTokenPrinter;

        // We need a ByteTendril to read a file
        let mut input = ByteTendril::new();
        // Using SliceExt.read_to_tendril we read stdin
        io::stdin().read_to_tendril(&mut input).unwrap();
        // For xml5ever we need StrTendril, so we reinterpret it
        // into StrTendril.
        //
        // You might wonder, how does `try_reinterpret` know we
        // need StrTendril and the answer is type inference based
        // on `tokenize_xml_to` signature.
        let input = input.try_reinterpret().unwrap();
        // Here we create and run tokenizer
        let mut tok = XmlTokenizer::new(sink, Default::default());
        // We pass input to parsed.
        tok.feed(input);

        // tok.end must be invoked for final bytes to be processed.
        tok.end();
    }
```

NOTE: `unwrap` causes panic, it's only OK to use in simple examples.

For full source code check out: [`examples/simple_xml_tokenizer.rs`](https://github.com/servo/html5ever/blob/main/xml5ever/examples/simple_xml_tokenizer.rs)

Once we have successfully compiled the example we run the example with inline
xml

```bash
    cargo script simple_xml_tokenizer.rs <<< "<xml>Text with <b>bold words</b>!</xml>"
```

or by sending an [`examples/example.xml`](https://github.com/servo/html5ever/blob/main/xml5ever/examples/example.xml) located in same folder as examples.

```bash
    cargo script simple_xml_tokenizer.rs < example.xml
```

# Tree printer

To actually get an XML document tree from the xml5ever, you need to use a `TreeSink`.
`TreeSink` is in many way similar to the TokenSink. Basically, TokenSink takes data
and returns list of tokens, while TreeSink takes tokens and returns a tree of parsed
XML document. Do note, that this is a simplified explanation and consult
documentation for more info.

Ok, with that in mind, let's build us a TreePrinter. For example if we get an XML
file like:

```xml
    <student>
        <first-name>Bobby</first-name>
        <last-name>Tables</last-name>
    </student>
```

We'd want a structure similar to this:

```
#document
    student
        first-name
            #text Bobby
        last-name
            #text Tables

```
We won't print anything other than element names and text fields. So comments,
doctypes and other such elements are ignored.

First part is similar to making SimpleTokenPrinter:

```rust
    // We need to allocate an input tendril for xml5ever
    let mut input = ByteTendril::new();
    // Using SliceExt.read_to_tendril functions we can read stdin
    io::stdin().read_to_tendril(&mut input).unwrap();
    let input = input.try_reinterpret().unwrap();
```

This time, we need an implementation of [`TreeSink`](https://docs.rs/xml5ever/latest/xml5ever/tree_builder/trait.TreeSink.html). xml5ever comes with a
built-in `TreeSink` implementation called [`RcDom`](https://docs.rs/markup5ever_rcdom/latest/markup5ever_rcdom/struct.RcDom.html). To process input into
a `TreeSink` we use the following line:

```rust
    let dom: RcDom = parse(one_input(input), Default::default());
```

Let's analyze it a bit. First there is `let dom: RcDom`. We need this part,
because the type inferencer can't infer which TreeSink implementation we mean
in this scenario.

Function [`one_input`](https://ygg01.github.io/docs/xml5ever/xml5ever/fn.one_input.html) is a convenience function that turns any value into an iterator. In this case
it converts a StrTendril into an Iterator over itself.

Ok, so now that we parsed our tree what with it? Well, for that we might need some
kind of function that will help us traverse it. We shall call that function `walk`.

```rust
    fn walk(prefix: &str, handle: Handle) {
        let node = handle.borrow();

        // We print out the prefix before we start
        print!("{}", prefix);
        // We are only interested in following nodes:
        // Document, Text and Element, so our match
        // reflects that.
        match node.node {
            Document
                => println!("#document"),

            Text(ref text)  => {
                println!("#text {}", text.escape_default())
            },

            Element(ref name, _) => {
                println!("{}", name.local);
            },

            _ => {},

        }

        // We increase indent in child nodes
        let new_indent = {
            let mut temp = String::new();
            temp.push_str(prefix);
            temp.push_str("    ");
            temp
        };

        for child in node.children.iter()
            // In order to avoid weird indentation, we filter
            // only Text/Element nodes.
            // We don't need to filter Document since its guaranteed
            // child elements don't contain documents
            .filter(|child| match child.borrow().node {
                Text(_) | Element (_, _) => true,
                _ => false,
            }
        ) {
            // Recursion - Yay!
            walk(&new_indent, child.clone());
        }
    }
```

For full source code check out: [`examples/xml_tree_printer.rs`](https://github.com/servo/html5ever/blob/main/rcdom/examples/xml_tree_printer.rs)

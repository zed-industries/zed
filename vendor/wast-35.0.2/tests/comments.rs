use wast::parser::{self, Parse, ParseBuffer, Parser, Result};

pub struct Comments<'a> {
    comments: Vec<&'a str>,
}

impl<'a> Parse<'a> for Comments<'a> {
    fn parse(parser: Parser<'a>) -> Result<Comments<'a>> {
        let comments = parser.step(|mut cursor| {
            let mut comments = Vec::new();
            loop {
                let (comment, c) = match cursor.comment() {
                    Some(pair) => pair,
                    None => break,
                };
                cursor = c;
                comments.push(if comment.starts_with(";;") {
                    &comment[2..]
                } else {
                    &comment[2..comment.len() - 2]
                });
            }
            Ok((comments, cursor))
        })?;
        Ok(Comments { comments })
    }
}

pub struct Documented<'a, T> {
    comments: Comments<'a>,
    item: T,
}

impl<'a, T: Parse<'a>> Parse<'a> for Documented<'a, T> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let comments = parser.parse()?;
        let item = parser.parens(T::parse)?;
        Ok(Documented { comments, item })
    }
}

#[test]
fn parse_comments() -> anyhow::Result<()> {
    let buf = ParseBuffer::new(
        r#"
;; hello
(; again ;)
(module)
    "#,
    )?;

    let d: Documented<wast::Module> = parser::parse(&buf)?;
    assert_eq!(d.comments.comments, vec![" hello", " again "]);
    drop(d.item);

    let buf = ParseBuffer::new(
        r#"
;; this
(; is
on
multiple;)


;; lines
(func)
    "#,
    )?;

    let d: Documented<wast::Func> = parser::parse(&buf)?;
    assert_eq!(
        d.comments.comments,
        vec![" this", " is\non\nmultiple", " lines"]
    );
    drop(d.item);
    Ok(())
}

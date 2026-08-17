use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::errors::Result;
use crate::events::Event;
use crate::Writer;

impl<W: AsyncWrite + Unpin> Writer<W> {
    /// Writes the given event to the underlying writer. Async version of [`Writer::write_event`].
    pub async fn write_event_async<'a, E: AsRef<Event<'a>>>(&mut self, event: E) -> Result<()> {
        let mut next_should_line_break = true;
        let result = match *event.as_ref() {
            Event::Start(ref e) => {
                let result = self.write_wrapped_async(b"<", e, b">").await;
                if let Some(i) = self.indent.as_mut() {
                    i.grow();
                }
                result
            }
            Event::End(ref e) => {
                if let Some(i) = self.indent.as_mut() {
                    i.shrink();
                }
                self.write_wrapped_async(b"</", e, b">").await
            }
            Event::Empty(ref e) => self.write_wrapped_async(b"<", e, b"/>").await,
            Event::Text(ref e) => {
                next_should_line_break = false;
                self.write_async(e).await
            }
            Event::Comment(ref e) => self.write_wrapped_async(b"<!--", e, b"-->").await,
            Event::CData(ref e) => {
                next_should_line_break = false;
                self.write_async(b"<![CDATA[").await?;
                self.write_async(e).await?;
                self.write_async(b"]]>").await
            }
            Event::Decl(ref e) => self.write_wrapped_async(b"<?", e, b"?>").await,
            Event::PI(ref e) => self.write_wrapped_async(b"<?", e, b"?>").await,
            Event::DocType(ref e) => self.write_wrapped_async(b"<!DOCTYPE ", e, b">").await,
            Event::Eof => Ok(()),
        };
        if let Some(i) = self.indent.as_mut() {
            i.should_line_break = next_should_line_break;
        }
        result
    }

    /// Manually write a newline and indentation at the proper level. Async version of
    /// [`Writer::write_indent`].
    ///
    /// This method will do nothing if `Writer` was not constructed with [`Writer::new_with_indent`].
    pub async fn write_indent_async(&mut self) -> Result<()> {
        if let Some(ref i) = self.indent {
            self.writer.write_all(b"\n").await?;
            self.writer.write_all(i.current()).await?;
        }
        Ok(())
    }

    #[inline]
    async fn write_async(&mut self, value: &[u8]) -> Result<()> {
        self.writer.write_all(value).await.map_err(Into::into)
    }

    #[inline]
    async fn write_wrapped_async(
        &mut self,
        before: &[u8],
        value: &[u8],
        after: &[u8],
    ) -> Result<()> {
        if let Some(ref i) = self.indent {
            if i.should_line_break {
                self.writer.write_all(b"\n").await?;
                self.writer.write_all(i.current()).await?;
            }
        }
        self.write_async(before).await?;
        self.write_async(value).await?;
        self.write_async(after).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::*;
    use pretty_assertions::assert_eq;

    macro_rules! test {
        ($name: ident, $event: expr, $expected: expr) => {
            #[tokio::test]
            async fn $name() {
                let mut buffer = Vec::new();
                let mut writer = Writer::new(&mut buffer);

                writer
                    .write_event_async($event)
                    .await
                    .expect("write event failed");

                assert_eq!(std::str::from_utf8(&buffer).unwrap(), $expected,);
            }
        };
    }

    test!(
        xml_header,
        Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("no"))),
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#
    );

    test!(empty_tag, Event::Empty(BytesStart::new("tag")), r#"<tag/>"#);

    test!(
        comment,
        Event::Comment(BytesText::new("this is a comment")),
        r#"<!--this is a comment-->"#
    );

    test!(
        cdata,
        Event::CData(BytesCData::new("this is a cdata")),
        r#"<![CDATA[this is a cdata]]>"#
    );

    test!(
        pi,
        Event::PI(BytesText::new("this is a processing instruction")),
        r#"<?this is a processing instruction?>"#
    );

    test!(
        doctype,
        Event::DocType(BytesText::new("this is a doctype")),
        r#"<!DOCTYPE this is a doctype>"#
    );

    #[tokio::test]
    async fn full_tag() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new(&mut buffer);

        let start = Event::Start(BytesStart::new("tag"));
        let text = Event::Text(BytesText::new("inner text"));
        let end = Event::End(BytesEnd::new("tag"));
        for i in [start, text, end] {
            writer.write_event_async(i).await.expect("write tag failed");
        }

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<tag>inner text</tag>"#
        );
    }
}

#[cfg(test)]
mod indentation_async {
    use super::*;
    use crate::events::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn self_closed() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let tag = BytesStart::new("self-closed")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        writer
            .write_event_async(Event::Empty(tag))
            .await
            .expect("write tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<self-closed attr1="value1" attr2="value2"/>"#
        );
    }

    #[tokio::test]
    async fn empty_paired() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        writer
            .write_event_async(Event::Start(start.clone()))
            .await
            .expect("write start tag failed");
        writer
            .write_event_async(Event::End(end))
            .await
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
</paired>"#
        );
    }

    #[tokio::test]
    async fn paired_with_inner() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        let inner = BytesStart::new("inner");

        writer
            .write_event_async(Event::Start(start.clone()))
            .await
            .expect("write start tag failed");
        writer
            .write_event_async(Event::Empty(inner))
            .await
            .expect("write inner tag failed");
        writer
            .write_event_async(Event::End(end))
            .await
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
    <inner/>
</paired>"#
        );
    }

    #[tokio::test]
    async fn paired_with_text() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        let text = BytesText::new("text");

        writer
            .write_event_async(Event::Start(start.clone()))
            .await
            .expect("write start tag failed");
        writer
            .write_event_async(Event::Text(text))
            .await
            .expect("write text failed");
        writer
            .write_event_async(Event::End(end))
            .await
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">text</paired>"#
        );
    }

    #[tokio::test]
    async fn mixed_content() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        let text = BytesText::new("text");
        let inner = BytesStart::new("inner");

        writer
            .write_event_async(Event::Start(start.clone()))
            .await
            .expect("write start tag failed");
        writer
            .write_event_async(Event::Text(text))
            .await
            .expect("write text failed");
        writer
            .write_event_async(Event::Empty(inner))
            .await
            .expect("write inner tag failed");
        writer
            .write_event_async(Event::End(end))
            .await
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">text<inner/>
</paired>"#
        );
    }

    #[tokio::test]
    async fn nested() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let start = BytesStart::new("paired")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = start.to_end();
        let inner = BytesStart::new("inner");

        writer
            .write_event_async(Event::Start(start.clone()))
            .await
            .expect("write start 1 tag failed");
        writer
            .write_event_async(Event::Start(start.clone()))
            .await
            .expect("write start 2 tag failed");
        writer
            .write_event_async(Event::Empty(inner))
            .await
            .expect("write inner tag failed");
        writer
            .write_event_async(Event::End(end.clone()))
            .await
            .expect("write end tag 2 failed");
        writer
            .write_event_async(Event::End(end))
            .await
            .expect("write end tag 1 failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
    <paired attr1="value1" attr2="value2">
        <inner/>
    </paired>
</paired>"#
        );
    }
}

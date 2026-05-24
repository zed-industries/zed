use anyhow::Result;
use quick_xml::events::Event;

struct StripForeignObject<I> {
    inner: I,
    depth: usize,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for StripForeignObject<I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = self.inner.next()?;
            let event = match event {
                Ok(event) => event,
                Err(e) => return Some(Err(e)),
            };

            match &event {
                Event::Start(e) if e.name().as_ref() == b"foreignObject" => {
                    self.depth += 1;
                    continue;
                }
                Event::Start(_) if self.depth > 0 => {
                    self.depth += 1;
                    continue;
                }
                Event::End(_) if self.depth > 0 => {
                    self.depth -= 1;
                    continue;
                }
                Event::Empty(e) if e.name().as_ref() == b"foreignObject" => {
                    continue;
                }
                _ if self.depth > 0 => {
                    continue;
                }
                _ => return Some(Ok(event)),
            }
        }
    }
}

pub(super) fn process<'a>(
    events: impl Iterator<Item = Result<Event<'a>>>,
) -> impl Iterator<Item = Result<Event<'a>>> {
    StripForeignObject {
        inner: events,
        depth: 0,
    }
}

use base64::{engine::general_purpose::STANDARD as base64_standard, Engine};
use quick_xml::{escape::resolve_xml_entity, events::Event as XmlEvent, Error as XmlReaderError, Reader as EventReader};
use std::io::{self, BufRead};

use crate::{
    error::{Error, ErrorKind, FilePosition},
    stream::{Event, OwnedEvent},
    Date, Integer,
};

pub struct XmlReader<R: BufRead> {
    buffer: Vec<u8>,
    started: bool,
    finished: bool,
    state: ReaderState<R>,
}

struct ReaderState<R: BufRead>(EventReader<R>);

enum ReadResult {
    XmlDecl,
    Event(OwnedEvent),
    Eof,
}

impl<R: BufRead> XmlReader<R> {
    pub fn new(reader: R) -> XmlReader<R> {
        let mut xml_reader = EventReader::from_reader(reader);
        let config = xml_reader.config_mut();
        config.trim_text(false);
        config.check_end_names = true;
        config.expand_empty_elements = true;

        XmlReader {
            buffer: Vec::new(),
            started: false,
            finished: false,
            state: ReaderState(xml_reader),
        }
    }

    pub fn into_inner(self) -> R {
        self.state.0.into_inner()
    }

    pub(crate) fn xml_doc_started(&self) -> bool {
        self.started
    }
}

impl From<XmlReaderError> for ErrorKind {
    fn from(err: XmlReaderError) -> Self {
        match err {
            XmlReaderError::Io(err) if err.kind() == io::ErrorKind::UnexpectedEof => {
                ErrorKind::UnexpectedEof
            }
            XmlReaderError::Io(err) => match std::sync::Arc::try_unwrap(err) {
                Ok(err) => ErrorKind::Io(err),
                Err(err) => ErrorKind::Io(std::io::Error::from(err.kind())),
            },
            XmlReaderError::Syntax(_) => ErrorKind::UnexpectedEof,
            XmlReaderError::IllFormed(_)
            | XmlReaderError::InvalidAttr(_)
            | XmlReaderError::Escape(_)
            | XmlReaderError::Namespace(_) => {
                ErrorKind::InvalidXmlSyntax
            },
            XmlReaderError::Encoding(_) => ErrorKind::InvalidXmlUtf8,
        }
    }
}

impl<R: BufRead> Iterator for XmlReader<R> {
    type Item = Result<OwnedEvent, Error>;

    fn next(&mut self) -> Option<Result<OwnedEvent, Error>> {
        if self.finished {
            return None;
        }

        loop {
            match self.state.read_next(&mut self.buffer) {
                Ok(ReadResult::XmlDecl) => {
                    self.started = true;
                }
                Ok(ReadResult::Event(event)) => {
                    self.started = true;
                    return Some(Ok(event));
                }
                Ok(ReadResult::Eof) => {
                    self.started = true;
                    self.finished = true;
                    return None;
                }
                Err(err) => {
                    self.finished = true;
                    return Some(Err(err));
                }
            }
        }
    }
}

impl<R: BufRead> ReaderState<R> {
    fn xml_reader_pos(&self) -> FilePosition {
        let pos = self.0.buffer_position();
        FilePosition(pos)
    }

    fn with_pos(&self, kind: ErrorKind) -> Error {
        kind.with_position(self.xml_reader_pos())
    }

    fn read_xml_event<'buf>(&mut self, buffer: &'buf mut Vec<u8>) -> Result<XmlEvent<'buf>, Error> {
        let event = self.0.read_event_into(buffer);
        let pos = self.xml_reader_pos();
        event.map_err(|err| ErrorKind::from(err).with_position(pos))
    }

    fn read_content(&mut self, buffer: &mut Vec<u8>) -> Result<String, Error> {
        let mut content = String::new();
        loop {
            match self.read_xml_event(buffer)? {
                XmlEvent::Text(text) => {
                    let decoded = text
                        .decode()
                        .map_err(|err| self.with_pos(ErrorKind::from(err)))?;
                    content.push_str(&decoded);
                }
                XmlEvent::GeneralRef(bytes) => {
                    if let Some(ch) = bytes
                        .resolve_char_ref()
                        .map_err(|err| self.with_pos(ErrorKind::from(err)))?
                    {
                        content.push(ch);
                    } else {
                        let decoded = bytes
                            .decode()
                            .map_err(|err| self.with_pos(ErrorKind::from(err)))?;
                        if let Some(entity) = resolve_xml_entity(&decoded) {
                            content.push_str(entity);
                        }
                    }
                }
                XmlEvent::End(_) => {
                    break;
                }
                XmlEvent::Eof => return Err(self.with_pos(ErrorKind::UnclosedXmlElement)),
                XmlEvent::Start(_) => return Err(self.with_pos(ErrorKind::UnexpectedXmlOpeningTag)),
                XmlEvent::PI(_)
                | XmlEvent::Empty(_)
                | XmlEvent::Comment(_)
                | XmlEvent::CData(_)
                | XmlEvent::Decl(_)
                | XmlEvent::DocType(_) => {
                    // skip
                }
            }
        }
        Ok(content)
    }

    fn read_next(&mut self, buffer: &mut Vec<u8>) -> Result<ReadResult, Error> {
        loop {
            match self.read_xml_event(buffer)? {
                XmlEvent::Decl(_) | XmlEvent::DocType(_) => return Ok(ReadResult::XmlDecl),
                XmlEvent::Start(name) => {
                    match name.local_name().as_ref() {
                        b"plist" => {}
                        b"array" => return Ok(ReadResult::Event(Event::StartArray(None))),
                        b"dict" => return Ok(ReadResult::Event(Event::StartDictionary(None))),
                        b"key" => {
                            return Ok(ReadResult::Event(Event::String(
                                self.read_content(buffer)?.into(),
                            )))
                        }
                        b"data" => {
                            let mut encoded = self.read_content(buffer)?;
                            // Strip whitespace and line endings from input string
                            encoded.retain(|c| !c.is_ascii_whitespace());
                            let data = base64_standard
                                .decode(&encoded)
                                .map_err(|_| self.with_pos(ErrorKind::InvalidDataString))?;
                            return Ok(ReadResult::Event(Event::Data(data.into())));
                        }
                        b"date" => {
                            let s = self.read_content(buffer)?;
                            let date = Date::from_xml_format(&s)
                                .map_err(|_| self.with_pos(ErrorKind::InvalidDateString))?;
                            return Ok(ReadResult::Event(Event::Date(date)));
                        }
                        b"integer" => {
                            let s = self.read_content(buffer)?;
                            match Integer::from_str(&s) {
                                Ok(i) => return Ok(ReadResult::Event(Event::Integer(i))),
                                Err(_) => {
                                    return Err(self.with_pos(ErrorKind::InvalidIntegerString))
                                }
                            }
                        }
                        b"real" => {
                            let s = self.read_content(buffer)?;
                            match s.parse() {
                                Ok(f) => return Ok(ReadResult::Event(Event::Real(f))),
                                Err(_) => return Err(self.with_pos(ErrorKind::InvalidRealString)),
                            }
                        }
                        b"string" => {
                            return Ok(ReadResult::Event(Event::String(
                                self.read_content(buffer)?.into(),
                            )))
                        }
                        b"true" => return Ok(ReadResult::Event(Event::Boolean(true))),
                        b"false" => return Ok(ReadResult::Event(Event::Boolean(false))),
                        _ => return Err(self.with_pos(ErrorKind::UnknownXmlElement)),
                    }
                }
                XmlEvent::End(name) => match name.local_name().as_ref() {
                    b"array" | b"dict" => return Ok(ReadResult::Event(Event::EndCollection)),
                    _ => (),
                },
                XmlEvent::Eof => return Ok(ReadResult::Eof),
                XmlEvent::Text(text) => {
                    let decoded = text
                        .decode()
                        .map_err(|err| self.with_pos(ErrorKind::from(err)))?;

                    if !decoded.chars().all(char::is_whitespace) {
                        return Err(
                            self.with_pos(ErrorKind::UnexpectedXmlCharactersExpectedElement)
                        );
                    }
                }
                XmlEvent::GeneralRef(bytes) => {
                    if let Some(ch) = bytes
                        .resolve_char_ref()
                        .map_err(|err| self.with_pos(ErrorKind::from(err)))?
                    {
                        if ch.is_whitespace() {
                            continue;
                        }
                    } else {
                        let decoded = bytes
                            .decode()
                            .map_err(|err| self.with_pos(ErrorKind::from(err)))?;
                        if let Some(entity) = resolve_xml_entity(&decoded) {
                            if entity.chars().all(char::is_whitespace) {
                                continue;
                            }
                        }
                    }
                    return Err(
                        self.with_pos(ErrorKind::UnexpectedXmlCharactersExpectedElement)
                    );
                }
                XmlEvent::PI(_)
                | XmlEvent::CData(_)
                | XmlEvent::Comment(_)
                | XmlEvent::Empty(_) => {
                    // skip
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;
    use crate::stream::Event::*;

    #[test]
    fn streaming_parser() {
        let reader = File::open("./tests/data/xml.plist").unwrap();
        let streaming_parser = XmlReader::new(BufReader::new(reader));
        let events: Result<Vec<_>, _> = streaming_parser.collect();

        let comparison = &[
            StartDictionary(None),
            String("Author".into()),
            String("William Shakespeare".into()),
            String("Lines".into()),
            StartArray(None),
            String("It is a tale told by an idiot,     ".into()),
            String("Full of sound and fury, signifying nothing.".into()),
            EndCollection,
            String("Death".into()),
            Integer(1564.into()),
            String("Height".into()),
            Real(1.60),
            String("Data".into()),
            Data(vec![0, 0, 0, 190, 0, 0, 0, 3, 0, 0, 0, 30, 0, 0, 0].into()),
            String("Birthdate".into()),
            Date(super::Date::from_xml_format("1981-05-16T11:32:06Z").unwrap()),
            String("Blank".into()),
            String("".into()),
            String("BiggestNumber".into()),
            Integer(18446744073709551615u64.into()),
            String("SmallestNumber".into()),
            Integer((-9223372036854775808i64).into()),
            String("HexademicalNumber".into()),
            Integer(0xdead_beef_u64.into()),
            String("IsTrue".into()),
            Boolean(true),
            String("IsNotFalse".into()),
            Boolean(false),
            String("Pets".into()),
            String("A cat & a dog.".into()),
            EndCollection,
        ];

        assert_eq!(events.unwrap(), comparison);
    }

    #[test]
    fn bad_data() {
        let reader = File::open("./tests/data/xml_error.plist").unwrap();
        let streaming_parser = XmlReader::new(BufReader::new(reader));
        let events: Vec<_> = streaming_parser.collect();

        assert!(events.last().unwrap().is_err());
    }

    #[test]
    fn entity_error() {
        let reader = File::open("./tests/data/xml_entity_error.plist").unwrap();
        let streaming_parser = XmlReader::new(BufReader::new(reader));
        let events: Vec<_> = streaming_parser.collect();
        let event = events.last().unwrap();

        assert!(event.is_err());
        assert_eq!(
            event.as_ref().unwrap_err().to_string(),
            "UnexpectedXmlCharactersExpectedElement (offset 174)".to_string()
        );
    }
}

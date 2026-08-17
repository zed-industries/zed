use crate::error::Error;
use std::io::BufRead;

const SPLIT: &str = "  ";

#[derive(Debug, PartialEq, Eq)]
pub enum Event<'a> {
    Vendor {
        id: &'a str,
        name: &'a str,
    },
    Device {
        id: &'a str,
        name: &'a str,
    },
    Subdevice {
        subvendor: &'a str,
        subdevice: &'a str,
        subsystem_name: &'a str,
    },
    Class {
        id: &'a str,
        name: &'a str,
    },
    SubClass {
        id: &'a str,
        name: &'a str,
    },
    ProgIf {
        id: &'a str,
        name: &'a str,
    },
}

pub struct Parser<R> {
    reader: R,
    buf: String,
    section: Section,
}

enum Section {
    Devices,
    Classes,
}

impl<R: BufRead> Parser<R> {
    pub(crate) fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::new(),
            section: Section::Devices,
        }
    }

    pub fn next_event(&mut self) -> Result<Option<Event>, Error> {
        self.buf.clear();

        while self.reader.read_line(&mut self.buf)? != 0 {
            if self.buf.is_empty() || self.buf.starts_with('#') || self.buf == "\n" {
                self.buf.clear();
                continue;
            }

            let buf = &self.buf[..self.buf.len() - 1];

            let event = if let Some(buf) = buf.strip_prefix("C ") {
                self.section = Section::Classes;

                let (id, name) = parse_split(buf)?;
                Event::Class { id, name }
            } else if let Some(buf) = buf.strip_prefix("\t\t") {
                // Subdevice
                let (prefix, name) = parse_split(buf)?;

                if let Some((subvendor, subdevice)) = prefix.split_once(' ') {
                    Event::Subdevice {
                        subvendor,
                        subdevice,
                        subsystem_name: name,
                    }
                } else {
                    Event::ProgIf { id: prefix, name }
                }
            } else if let Some(buf) = buf.strip_prefix('\t') {
                let (id, name) = parse_split(buf)?;

                match self.section {
                    Section::Devices => Event::Device { id, name },
                    Section::Classes => Event::SubClass { id, name },
                }
            } else {
                let (id, name) = parse_split(buf)?;
                Event::Vendor { id, name }
            };
            return Ok(Some(event));
        }

        Ok(None)
    }
}

fn parse_split(buf: &str) -> Result<(&str, &str), Error> {
    buf.split_once(SPLIT)
        .ok_or_else(|| Error::Parse(format!("missing delimiter in line {buf}")))
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::parser::{Event, Section};
    use pretty_assertions::assert_eq;
    use std::{
        fs::File,
        io::{BufReader, Cursor},
    };

    #[test]
    fn first_events() {
        let file = File::open("./tests/pci.ids").unwrap();
        let mut parser = Parser::new(BufReader::new(file));

        assert_eq!(
            Event::Vendor {
                id: "0001",
                name: "SafeNet (wrong ID)"
            },
            parser.next_event().unwrap().unwrap()
        );
        assert_eq!(
            Event::Vendor {
                id: "0010",
                name: "Allied Telesis, Inc (Wrong ID)"
            },
            parser.next_event().unwrap().unwrap()
        );
        assert_eq!(
            Event::Device {
                id: "8139",
                name: "AT-2500TX V3 Ethernet"
            },
            parser.next_event().unwrap().unwrap()
        );
    }

    #[test]
    fn parse_class_line() {
        let mut parser = Parser::new(Cursor::new("C 00  Unclassified device\n"));
        parser.section = Section::Classes;
        assert_eq!(
            Event::Class {
                id: "00",
                name: "Unclassified device"
            },
            parser.next_event().unwrap().unwrap()
        );
    }

    #[test]
    fn parse_subclass_line() {
        let buf = "	01  IDE interface\n";
        let mut parser = Parser::new(Cursor::new(buf));
        parser.section = Section::Classes;
        assert_eq!(
            Event::SubClass {
                id: "01",
                name: "IDE interface"
            },
            parser.next_event().unwrap().unwrap()
        );
    }

    #[test]
    fn parse_prog_if_line() {
        let buf = "		00  ISA Compatibility mode-only controller\n";
        let mut parser = Parser::new(Cursor::new(buf));
        parser.section = Section::Classes;
        assert_eq!(
            Event::ProgIf {
                id: "00",
                name: "ISA Compatibility mode-only controller"
            },
            parser.next_event().unwrap().unwrap()
        );
    }
}

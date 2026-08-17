use compression_core::util::PartialBuffer;
use std::io;

#[derive(Debug, Default)]
struct Flags {
    _ascii: bool,
    crc: bool,
    extra: bool,
    filename: bool,
    comment: bool,
}

#[derive(Debug, Default)]
pub(super) struct Header {
    flags: Flags,
}

#[derive(Debug)]
enum State {
    Fixed(PartialBuffer<[u8; 10]>),
    ExtraLen(PartialBuffer<[u8; 2]>),
    Extra(PartialBuffer<Vec<u8>>),
    Filename(Vec<u8>),
    Comment(Vec<u8>),
    Crc(PartialBuffer<[u8; 2]>),
    Done,
}

impl Default for State {
    fn default() -> Self {
        State::Fixed(<_>::default())
    }
}

#[derive(Debug, Default)]
pub(super) struct Parser {
    state: State,
    header: Header,
}

impl Header {
    fn parse(input: &[u8; 10]) -> io::Result<Self> {
        if input[0..3] != [0x1f, 0x8b, 0x08] {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid gzip header",
            ));
        }

        let flag = input[3];

        let flags = Flags {
            _ascii: (flag & 0b0000_0001) != 0,
            crc: (flag & 0b0000_0010) != 0,
            extra: (flag & 0b0000_0100) != 0,
            filename: (flag & 0b0000_1000) != 0,
            comment: (flag & 0b0001_0000) != 0,
        };

        Ok(Header { flags })
    }
}

impl Parser {
    pub(super) fn input(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
    ) -> io::Result<Option<Header>> {
        loop {
            match &mut self.state {
                State::Fixed(data) => {
                    data.copy_unwritten_from(input);

                    if data.unwritten().is_empty() {
                        self.header = Header::parse(&data.take().into_inner())?;
                        self.state = State::ExtraLen(<_>::default());
                    } else {
                        return Ok(None);
                    }
                }

                State::ExtraLen(data) => {
                    if !self.header.flags.extra {
                        self.state = State::Filename(<_>::default());
                        continue;
                    }

                    data.copy_unwritten_from(input);

                    if data.unwritten().is_empty() {
                        let len = u16::from_le_bytes(data.take().into_inner());
                        self.state = State::Extra(vec![0; usize::from(len)].into());
                    } else {
                        return Ok(None);
                    }
                }

                State::Extra(data) => {
                    data.copy_unwritten_from(input);

                    if data.unwritten().is_empty() {
                        self.state = State::Filename(<_>::default());
                    } else {
                        return Ok(None);
                    }
                }

                State::Filename(data) => {
                    if !self.header.flags.filename {
                        self.state = State::Comment(<_>::default());
                        continue;
                    }

                    if let Some(len) = memchr::memchr(0, input.unwritten()) {
                        data.extend_from_slice(&input.unwritten()[..len]);
                        input.advance(len + 1);
                        self.state = State::Comment(<_>::default());
                    } else {
                        data.extend_from_slice(input.unwritten());
                        input.advance(input.unwritten().len());
                        return Ok(None);
                    }
                }

                State::Comment(data) => {
                    if !self.header.flags.comment {
                        self.state = State::Crc(<_>::default());
                        continue;
                    }

                    if let Some(len) = memchr::memchr(0, input.unwritten()) {
                        data.extend_from_slice(&input.unwritten()[..len]);
                        input.advance(len + 1);
                        self.state = State::Crc(<_>::default());
                    } else {
                        data.extend_from_slice(input.unwritten());
                        input.advance(input.unwritten().len());
                        return Ok(None);
                    }
                }

                State::Crc(data) => {
                    if !self.header.flags.crc {
                        self.state = State::Done;
                        return Ok(Some(std::mem::take(&mut self.header)));
                    }

                    data.copy_unwritten_from(input);

                    if data.unwritten().is_empty() {
                        self.state = State::Done;
                        return Ok(Some(std::mem::take(&mut self.header)));
                    } else {
                        return Ok(None);
                    }
                }

                State::Done => {
                    return Err(io::Error::other("parser used after done"));
                }
            };
        }
    }
}

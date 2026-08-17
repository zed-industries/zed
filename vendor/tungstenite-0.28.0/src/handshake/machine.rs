//! WebSocket handshake machine.

use bytes::Buf;
use log::*;
use std::io::{Cursor, Read, Write};

use crate::{
    error::{Error, ProtocolError, Result},
    util::NonBlockingResult,
    ReadBuffer,
};

/// A generic handshake state machine.
#[derive(Debug)]
pub struct HandshakeMachine<Stream> {
    stream: Stream,
    state: HandshakeState,
}

impl<Stream> HandshakeMachine<Stream> {
    /// Start reading data from the peer.
    pub fn start_read(stream: Stream) -> Self {
        Self { stream, state: HandshakeState::Reading(ReadBuffer::new(), AttackCheck::new()) }
    }
    /// Start writing data to the peer.
    pub fn start_write<D: Into<Vec<u8>>>(stream: Stream, data: D) -> Self {
        HandshakeMachine { stream, state: HandshakeState::Writing(Cursor::new(data.into())) }
    }
    /// Returns a shared reference to the inner stream.
    pub fn get_ref(&self) -> &Stream {
        &self.stream
    }
    /// Returns a mutable reference to the inner stream.
    pub fn get_mut(&mut self) -> &mut Stream {
        &mut self.stream
    }
}

impl<Stream: Read + Write> HandshakeMachine<Stream> {
    /// Perform a single handshake round.
    pub fn single_round<Obj: TryParse>(mut self) -> Result<RoundResult<Obj, Stream>> {
        trace!("Doing handshake round.");
        match self.state {
            HandshakeState::Reading(mut buf, mut attack_check) => {
                let read = buf.read_from(&mut self.stream).no_block()?;
                match read {
                    Some(0) => Err(Error::Protocol(ProtocolError::HandshakeIncomplete)),
                    Some(count) => {
                        attack_check.check_incoming_packet_size(count)?;
                        // TODO: this is slow for big headers with too many small packets.
                        // The parser has to be reworked in order to work on streams instead
                        // of buffers.
                        Ok(if let Some((size, obj)) = Obj::try_parse(Buf::chunk(&buf))? {
                            buf.advance(size);
                            RoundResult::StageFinished(StageResult::DoneReading {
                                result: obj,
                                stream: self.stream,
                                tail: buf.into_vec(),
                            })
                        } else {
                            RoundResult::Incomplete(HandshakeMachine {
                                state: HandshakeState::Reading(buf, attack_check),
                                ..self
                            })
                        })
                    }
                    None => Ok(RoundResult::WouldBlock(HandshakeMachine {
                        state: HandshakeState::Reading(buf, attack_check),
                        ..self
                    })),
                }
            }
            HandshakeState::Writing(mut buf) => {
                assert!(buf.has_remaining());
                if let Some(size) = self.stream.write(Buf::chunk(&buf)).no_block()? {
                    assert!(size > 0);
                    buf.advance(size);
                    Ok(if buf.has_remaining() {
                        RoundResult::Incomplete(HandshakeMachine {
                            state: HandshakeState::Writing(buf),
                            ..self
                        })
                    } else {
                        RoundResult::Incomplete(HandshakeMachine {
                            state: HandshakeState::Flushing,
                            ..self
                        })
                    })
                } else {
                    Ok(RoundResult::WouldBlock(HandshakeMachine {
                        state: HandshakeState::Writing(buf),
                        ..self
                    }))
                }
            }
            HandshakeState::Flushing => Ok(match self.stream.flush().no_block()? {
                Some(()) => RoundResult::StageFinished(StageResult::DoneWriting(self.stream)),
                None => RoundResult::WouldBlock(HandshakeMachine {
                    state: HandshakeState::Flushing,
                    ..self
                }),
            }),
        }
    }
}

/// The result of the round.
#[derive(Debug)]
pub enum RoundResult<Obj, Stream> {
    /// Round not done, I/O would block.
    WouldBlock(HandshakeMachine<Stream>),
    /// Round done, state unchanged.
    Incomplete(HandshakeMachine<Stream>),
    /// Stage complete.
    StageFinished(StageResult<Obj, Stream>),
}

/// The result of the stage.
#[derive(Debug)]
pub enum StageResult<Obj, Stream> {
    /// Reading round finished.
    #[allow(missing_docs)]
    DoneReading { result: Obj, stream: Stream, tail: Vec<u8> },
    /// Writing round finished.
    DoneWriting(Stream),
}

/// The parseable object.
pub trait TryParse: Sized {
    /// Return Ok(None) if incomplete, Err on syntax error.
    fn try_parse(data: &[u8]) -> Result<Option<(usize, Self)>>;
}

/// The handshake state.
#[derive(Debug)]
enum HandshakeState {
    /// Reading data from the peer.
    Reading(ReadBuffer, AttackCheck),
    /// Sending data to the peer.
    Writing(Cursor<Vec<u8>>),
    /// Flushing data to ensure that all intermediately buffered contents reach their destination.
    Flushing,
}

/// Attack mitigation. Contains counters needed to prevent DoS attacks
/// and reject valid but useless headers.
#[derive(Debug)]
pub(crate) struct AttackCheck {
    /// Number of HTTP header successful reads (TCP packets).
    number_of_packets: usize,
    /// Total number of bytes in HTTP header.
    number_of_bytes: usize,
}

impl AttackCheck {
    /// Initialize attack checking for incoming buffer.
    fn new() -> Self {
        Self { number_of_packets: 0, number_of_bytes: 0 }
    }

    /// Check the size of an incoming packet. To be called immediately after `read()`
    /// passing its returned bytes count as `size`.
    fn check_incoming_packet_size(&mut self, size: usize) -> Result<()> {
        self.number_of_packets += 1;
        self.number_of_bytes += size;

        // TODO: these values are hardcoded. Instead of making them configurable,
        // rework the way HTTP header is parsed to remove this check at all.
        const MAX_BYTES: usize = 65536;
        const MAX_PACKETS: usize = 512;
        const MIN_PACKET_SIZE: usize = 128;
        const MIN_PACKET_CHECK_THRESHOLD: usize = 64;

        if self.number_of_bytes > MAX_BYTES {
            return Err(Error::AttackAttempt);
        }

        if self.number_of_packets > MAX_PACKETS {
            return Err(Error::AttackAttempt);
        }

        if self.number_of_packets > MIN_PACKET_CHECK_THRESHOLD
            && self.number_of_packets * MIN_PACKET_SIZE > self.number_of_bytes
        {
            return Err(Error::AttackAttempt);
        }

        Ok(())
    }
}

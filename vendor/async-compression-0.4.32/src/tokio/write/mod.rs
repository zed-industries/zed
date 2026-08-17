//! Types which operate over [`AsyncWrite`](tokio::io::AsyncWrite) streams, both encoders and
//! decoders for various formats.

#[macro_use]
mod macros;
mod generic;

mod buf_write;
mod buf_writer;

use self::{
    buf_write::AsyncBufWrite,
    buf_writer::BufWriter,
    generic::{Decoder, Encoder},
};

algos!(tokio::write<W>);

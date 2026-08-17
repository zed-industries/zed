//! Implement push-based [`Write`] trait for both compressing and decompressing.
use std::io::{self, Write};

use zstd_safe;

use crate::dict::{DecoderDictionary, EncoderDictionary};
use crate::stream::{raw, zio};

#[cfg(test)]
mod tests;

/// An encoder that compress and forward data to another writer.
///
/// This allows to compress a stream of data
/// (good for files or heavy network stream).
///
/// Don't forget to call [`finish()`] before dropping it!
///
/// Alternatively, you can call [`auto_finish()`] to use an
/// [`AutoFinishEncoder`] that will finish on drop.
///
/// Note: The zstd library has its own internal input buffer (~128kb).
///
/// [`finish()`]: #method.finish
/// [`auto_finish()`]: #method.auto_finish
/// [`AutoFinishEncoder`]: AutoFinishEncoder
pub struct Encoder<'a, W: Write> {
    // output writer (compressed data)
    writer: zio::Writer<W, raw::Encoder<'a>>,
}

/// A decoder that decompress and forward data to another writer.
///
/// Note that you probably want to `flush()` after writing your stream content.
/// You can use [`auto_flush()`] to automatically flush the writer on drop.
///
/// [`auto_flush()`]: Decoder::auto_flush
pub struct Decoder<'a, W: Write> {
    // output writer (decompressed data)
    writer: zio::Writer<W, raw::Decoder<'a>>,
}

/// A wrapper around an `Encoder<W>` that finishes the stream on drop.
///
/// This can be created by the [`auto_finish()`] method on the [`Encoder`].
///
/// [`auto_finish()`]: Encoder::auto_finish
/// [`Encoder`]: Encoder
pub struct AutoFinishEncoder<
    'a,
    W: Write,
    F: FnMut(io::Result<W>) = Box<dyn Send + FnMut(io::Result<W>)>,
> {
    // We wrap this in an option to take it during drop.
    encoder: Option<Encoder<'a, W>>,

    on_finish: Option<F>,
}

/// A wrapper around a `Decoder<W>` that flushes the stream on drop.
///
/// This can be created by the [`auto_flush()`] method on the [`Decoder`].
///
/// [`auto_flush()`]: Decoder::auto_flush
/// [`Decoder`]: Decoder
pub struct AutoFlushDecoder<
    'a,
    W: Write,
    F: FnMut(io::Result<()>) = Box<dyn Send + FnMut(io::Result<()>)>,
> {
    // We wrap this in an option to take it during drop.
    decoder: Option<Decoder<'a, W>>,

    on_flush: Option<F>,
}

impl<'a, W: Write, F: FnMut(io::Result<()>)> AutoFlushDecoder<'a, W, F> {
    fn new(decoder: Decoder<'a, W>, on_flush: F) -> Self {
        AutoFlushDecoder {
            decoder: Some(decoder),
            on_flush: Some(on_flush),
        }
    }

    /// Acquires a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        self.decoder.as_ref().unwrap().get_ref()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that mutation of the writer may result in surprising results if
    /// this decoder is continued to be used.
    ///
    /// Mostly used for testing purposes.
    pub fn get_mut(&mut self) -> &mut W {
        self.decoder.as_mut().unwrap().get_mut()
    }
}

impl<W, F> Drop for AutoFlushDecoder<'_, W, F>
where
    W: Write,
    F: FnMut(io::Result<()>),
{
    fn drop(&mut self) {
        let mut decoder = self.decoder.take().unwrap();
        let result = decoder.flush();
        if let Some(mut on_finish) = self.on_flush.take() {
            on_finish(result);
        }
    }
}

impl<W: Write, F: FnMut(io::Result<()>)> Write for AutoFlushDecoder<'_, W, F> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.decoder.as_mut().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.decoder.as_mut().unwrap().flush()
    }
}

impl<'a, W: Write, F: FnMut(io::Result<W>)> AutoFinishEncoder<'a, W, F> {
    fn new(encoder: Encoder<'a, W>, on_finish: F) -> Self {
        AutoFinishEncoder {
            encoder: Some(encoder),
            on_finish: Some(on_finish),
        }
    }

    /// Acquires a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        self.encoder.as_ref().unwrap().get_ref()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that mutation of the writer may result in surprising results if
    /// this encoder is continued to be used.
    ///
    /// Mostly used for testing purposes.
    pub fn get_mut(&mut self) -> &mut W {
        self.encoder.as_mut().unwrap().get_mut()
    }
}

impl<W: Write, F: FnMut(io::Result<W>)> Drop for AutoFinishEncoder<'_, W, F> {
    fn drop(&mut self) {
        let result = self.encoder.take().unwrap().finish();
        if let Some(mut on_finish) = self.on_finish.take() {
            on_finish(result);
        }
    }
}

impl<W: Write, F: FnMut(io::Result<W>)> Write for AutoFinishEncoder<'_, W, F> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.encoder.as_mut().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.encoder.as_mut().unwrap().flush()
    }
}

impl<W: Write> Encoder<'static, W> {
    /// Creates a new encoder.
    ///
    /// `level`: compression level (1-21).
    ///
    /// A level of `0` uses zstd's default (currently `3`).
    pub fn new(writer: W, level: i32) -> io::Result<Self> {
        Self::with_dictionary(writer, level, &[])
    }

    /// Creates a new encoder, using an existing dictionary.
    ///
    /// (Provides better compression ratio for small files,
    /// but requires the dictionary to be present during decompression.)
    ///
    /// A level of `0` uses zstd's default (currently `3`).
    pub fn with_dictionary(
        writer: W,
        level: i32,
        dictionary: &[u8],
    ) -> io::Result<Self> {
        let encoder = raw::Encoder::with_dictionary(level, dictionary)?;
        let writer = zio::Writer::new(writer, encoder);
        Ok(Encoder { writer })
    }
}

impl<'a, W: Write> Encoder<'a, W> {
    /// Creates a new encoder, using an existing prepared `EncoderDictionary`.
    ///
    /// (Provides better compression ratio for small files,
    /// but requires the dictionary to be present during decompression.)
    pub fn with_prepared_dictionary<'b>(
        writer: W,
        dictionary: &EncoderDictionary<'b>,
    ) -> io::Result<Self>
    where
        'b: 'a,
    {
        let encoder = raw::Encoder::with_prepared_dictionary(dictionary)?;
        let writer = zio::Writer::new(writer, encoder);
        Ok(Encoder { writer })
    }

    /// Returns a wrapper around `self` that will finish the stream on drop.
    ///
    /// # Panic
    ///
    /// Panics on drop if an error happens when finishing the stream.
    pub fn auto_finish(self) -> AutoFinishEncoder<'a, W> {
        self.on_finish(Box::new(|result| {
            result.unwrap();
        }))
    }

    /// Returns an encoder that will finish the stream on drop.
    ///
    /// Calls the given callback with the result from `finish()`.
    pub fn on_finish<F: FnMut(io::Result<W>)>(
        self,
        f: F,
    ) -> AutoFinishEncoder<'a, W, F> {
        AutoFinishEncoder::new(self, f)
    }

    /// Acquires a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        self.writer.writer()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that mutation of the writer may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut W {
        self.writer.writer_mut()
    }

    /// **Required**: Finishes the stream.
    ///
    /// You *need* to finish the stream when you're done writing, either with
    /// this method or with [`try_finish(self)`](#method.try_finish).
    ///
    /// This returns the inner writer in case you need it.
    ///
    /// To get back `self` in case an error happened, use `try_finish`.
    ///
    /// **Note**: If you don't want (or can't) call `finish()` manually after
    ///           writing your data, consider using `auto_finish()` to get an
    ///           `AutoFinishEncoder`.
    pub fn finish(self) -> io::Result<W> {
        self.try_finish().map_err(|(_, err)| err)
    }

    /// **Required**: Attempts to finish the stream.
    ///
    /// You *need* to finish the stream when you're done writing, either with
    /// this method or with [`finish(self)`](#method.finish).
    ///
    /// This returns the inner writer if the finish was successful, or the
    /// object plus an error if it wasn't.
    ///
    /// `write` on this object will panic after `try_finish` has been called,
    /// even if it fails.
    pub fn try_finish(mut self) -> Result<W, (Self, io::Error)> {
        match self.writer.finish() {
            // Return the writer, because why not
            Ok(()) => Ok(self.writer.into_inner().0),
            Err(e) => Err((self, e)),
        }
    }

    /// Attemps to finish the stream.
    ///
    /// You *need* to finish the stream when you're done writing, either with
    /// this method or with [`finish(self)`](#method.finish).
    pub fn do_finish(&mut self) -> io::Result<()> {
        self.writer.finish()
    }

    /// Return a recommendation for the size of data to write at once.
    pub fn recommended_input_size() -> usize {
        zstd_safe::CCtx::in_size()
    }

    crate::encoder_common!(writer);
}

impl<'a, W: Write> Write for Encoder<'a, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> Decoder<'static, W> {
    /// Creates a new decoder.
    pub fn new(writer: W) -> io::Result<Self> {
        Self::with_dictionary(writer, &[])
    }

    /// Creates a new decoder, using an existing dictionary.
    ///
    /// (Provides better compression ratio for small files,
    /// but requires the dictionary to be present during decompression.)
    pub fn with_dictionary(writer: W, dictionary: &[u8]) -> io::Result<Self> {
        let decoder = raw::Decoder::with_dictionary(dictionary)?;
        let writer = zio::Writer::new(writer, decoder);
        Ok(Decoder { writer })
    }
}

impl<'a, W: Write> Decoder<'a, W> {
    /// Creates a new decoder, using an existing prepared `DecoderDictionary`.
    ///
    /// (Provides better compression ratio for small files,
    /// but requires the dictionary to be present during decompression.)
    pub fn with_prepared_dictionary<'b>(
        writer: W,
        dictionary: &DecoderDictionary<'b>,
    ) -> io::Result<Self>
    where
        'b: 'a,
    {
        let decoder = raw::Decoder::with_prepared_dictionary(dictionary)?;
        let writer = zio::Writer::new(writer, decoder);
        Ok(Decoder { writer })
    }

    /// Acquires a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        self.writer.writer()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that mutation of the writer may result in surprising results if
    /// this decoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut W {
        self.writer.writer_mut()
    }

    /// Returns the inner `Write`.
    pub fn into_inner(self) -> W {
        self.writer.into_inner().0
    }

    /// Return a recommendation for the size of data to write at once.
    pub fn recommended_input_size() -> usize {
        zstd_safe::DCtx::in_size()
    }

    /// Returns a wrapper around `self` that will flush the stream on drop.
    ///
    /// # Panic
    ///
    /// Panics on drop if an error happens when flushing the stream.
    pub fn auto_flush(self) -> AutoFlushDecoder<'a, W> {
        self.on_flush(Box::new(|result| {
            result.unwrap();
        }))
    }

    /// Returns a decoder that will flush the stream on drop.
    ///
    /// Calls the given callback with the result from `flush()`.
    pub fn on_flush<F: FnMut(io::Result<()>)>(
        self,
        f: F,
    ) -> AutoFlushDecoder<'a, W, F> {
        AutoFlushDecoder::new(self, f)
    }

    crate::decoder_common!(writer);
}

impl<W: Write> Write for Decoder<'_, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

fn _assert_traits() {
    fn _assert_send<T: Send>(_: T) {}

    _assert_send(Decoder::new(Vec::new()));
    _assert_send(Encoder::new(Vec::new(), 1));
    _assert_send(Decoder::new(Vec::new()).unwrap().auto_flush());
    _assert_send(Encoder::new(Vec::new(), 1).unwrap().auto_finish());
}

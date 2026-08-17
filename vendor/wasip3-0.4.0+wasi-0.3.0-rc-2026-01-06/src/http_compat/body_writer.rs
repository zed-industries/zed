use crate::{
    http::types::{ErrorCode, HeaderError, Trailers},
    wit_bindgen::{FutureReader, FutureWriter, StreamReader, StreamWriter},
    wit_future, wit_stream,
};
use http::HeaderMap;
use http_body::{Body as _, Frame};
use std::future::poll_fn;
use std::{fmt::Debug, pin};

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type BodyResult = Result<Option<Trailers>, ErrorCode>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The [`http_body::Body`] returned an error.
    #[error("body error: {0}")]
    HttpBody(#[source] BoxError),

    /// Received trailers were rejected by [`Trailers::from_list`].
    #[error("invalid trailers: {0}")]
    InvalidTrailers(#[source] HeaderError),

    /// The result future reader end was closed (dropped).
    ///
    /// The result that couldn't be written is returned.
    #[error("result future reader closed")]
    ResultReaderClosed(BodyResult),

    /// The stream reader end was closed (dropped).
    ///
    /// The number of bytes written successfully is returned as `written` and
    /// the bytes that couldn't be written are returned as `unwritten`.
    #[error("stream reader closed")]
    StreamReaderClosed { written: usize, unwritten: Vec<u8> },
}

/// BodyWriter coordinates a [`StreamWriter`] and [`FutureWriter`] associated
/// with the write end of a `wasi:http` `Request` or `Response` body.
pub struct BodyWriter {
    pub stream_writer: StreamWriter<u8>,
    pub result_writer: FutureWriter<BodyResult>,
    pub trailers: HeaderMap,
}

impl BodyWriter {
    /// Returns a new writer and the matching stream and result future readers,
    /// which will typically be used to create a `wasi:http` `Request` or
    /// `Response`.
    pub fn new() -> (Self, StreamReader<u8>, FutureReader<BodyResult>) {
        let (stream_writer, stream_reader) = wit_stream::new();
        let (result_writer, result_reader) =
        // TODO: is there a more appropriate ErrorCode?
            wit_future::new(|| Err(ErrorCode::InternalError(Some("body writer dropped".into()))));
        (
            Self {
                stream_writer,
                result_writer,
                trailers: Default::default(),
            },
            stream_reader,
            result_reader,
        )
    }

    /// Sends the given [`http_body::Body`] to this writer.
    ///
    /// This copies all data frames from the body to this writer's stream and
    /// then writes any trailers from the body to the result future. On success
    /// the number of data bytes written to the stream (which does not including
    /// trailers) is returned.
    ///
    /// If there is an error it is written to the result future.
    pub async fn send_http_body<T>(mut self, mut body: &mut T) -> Result<u64, Error>
    where
        T: http_body::Body + Unpin,
        T::Data: Into<Vec<u8>>,
        T::Error: Into<BoxError>,
    {
        let mut total_written = 0;

        loop {
            let frame = poll_fn(|cx| pin::Pin::new(&mut body).poll_frame(cx)).await;

            match frame {
                Some(Ok(frame)) => {
                    let written = self.send_frame(frame).await?;
                    total_written += written as u64;
                }
                Some(Err(err)) => {
                    let err = err.into();
                    // TODO: consider if there are better ErrorCode mappings
                    let error_code = ErrorCode::InternalError(Some(err.to_string()));
                    // TODO: log result_writer.write errors?
                    _ = self.result_writer.write(Err(error_code)).await;
                    return Err(Error::HttpBody(err));
                }
                None => break,
            }
        }
        drop(self.stream_writer);
        let maybe_trailers = if self.trailers.is_empty() {
            None
        } else {
            Some(self.trailers.try_into().map_err(Error::InvalidTrailers)?)
        };
        match self.result_writer.write(Ok(maybe_trailers)).await {
            Ok(()) => Ok(total_written),
            Err(err) => Err(Error::ResultReaderClosed(err.value)),
        }
    }

    /// Sends a [`http_body::Frame`].
    ///
    /// - If the frame contains data, the data is written to this writer's
    ///   stream and the size of the written data is returned.
    /// - If the frame contains trailers they are added to [`Self::trailers`]
    ///   and `Ok(0)` is returned.
    pub async fn send_frame<T>(&mut self, frame: Frame<T>) -> Result<usize, Error>
    where
        T: Into<Vec<u8>>,
    {
        // Frame is a pseudo-enum which is either 'data' or 'trailers'
        if frame.is_data() {
            let data = frame.into_data().unwrap_or_else(|_| unreachable!()).into();
            let data_len = data.len();
            // write_all returns any unwritten data if the read end is dropped
            let unwritten = self.stream_writer.write_all(data).await;
            if !unwritten.is_empty() {
                return Err(Error::StreamReaderClosed {
                    written: data_len - unwritten.len(),
                    unwritten,
                });
            }
            Ok(data_len)
        } else if frame.is_trailers() {
            let trailers = frame.into_trailers().unwrap_or_else(|_| unreachable!());
            self.trailers.extend(trailers);
            Ok(0)
        } else {
            unreachable!("Frames are data or trailers");
        }
    }
}

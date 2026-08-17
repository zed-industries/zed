//! Split read/write halves for WebSocket connections.
//!
//! # Architecture Layer: Fragment Assembly
//!
//! This module implements the **middle layer** of the WebSocket processing stack,
//! sitting between the codec layer and the WebSocket layer. Its primary responsibility
//! is **fragment assembly** according to RFC 6455.
//!
//! ## ReadHalf Responsibilities
//!
//! [`ReadHalf`] handles fragmented message assembly:
//!
//! - **Fragment accumulation**: Collects continuation frames into complete messages
//! - **State tracking**: Maintains fragment state (opcode, compression flag, accumulated data)
//! - **Fragment timeout**: Enforces timeouts on incomplete fragmented messages
//! - **Size limits**: Enforces maximum message size after assembly
//! - **Masking removal**: Removes frame masks to prevent accidental echo
//!
//! ## What ReadHalf Does NOT Handle
//!
//! - **Frame decoding**: Handled by [`Codec`](crate::codec::Codec)
//! - **Decompression**: Handled by [`WebSocket`](crate::WebSocket)
//! - **UTF-8 validation**: Handled by [`WebSocket`](crate::WebSocket)
//!
//! ## Data Flow Example
//!
//! **Receiving a fragmented message:**
//!
//! ```text
//! Codec → ReadHalf:
//!   Frame(OpCode::Text, FIN=0, "Hello")
//!     → ReadHalf: Start accumulating, return None
//!
//!   Frame(OpCode::Continuation, FIN=0, " Wor")
//!     → ReadHalf: Continue accumulating, return None
//!
//!   Frame(OpCode::Continuation, FIN=1, "ld!")
//!     → ReadHalf: Assemble complete message
//!     → Returns Frame(OpCode::Text, FIN=1, "Hello World!")
//! ```
//!
//! ## WriteHalf Responsibilities
//!
//! [`WriteHalf`] handles frame transmission with compression support:
//!
//! - **Compression**: Compresses frames when permessage-deflate is enabled
//! - **Masking**: Applies masks to client frames (required by RFC 6455)
//! - **Connection closure**: Manages graceful WebSocket closure protocol
//!
//! # Sans-IO Design
//!
//! This module implements a **sans-io** (I/O-free) design pattern where the protocol logic
//! is completely separated from the I/O operations.
//!
//! ## Benefits
//!
//! 1. **Testability**: Protocol logic can be tested without actual network I/O
//! 2. **Transport Agnostic**: Works with TCP, Unix sockets, in-memory buffers, or custom transports
//! 3. **Flexibility**: Users can implement custom flow control and buffering strategies
//! 4. **Performance**: Enables zero-copy optimizations in user code
//!
//! # Thread Safety Through `split()`
//!
//! WebSocket connections can be split into separate read and write halves for concurrent
//! operation. This is achieved through the `split()` method on `WebSocket`:
//!
//! ## Using `futures::StreamExt::split()` (Recommended)
//!
//! ```no_run
//! use futures::{StreamExt, SinkExt};
//! use yawc::{WebSocket, Frame};
//!
//! # async fn example() -> yawc::Result<()> {
//! let ws = WebSocket::connect("wss://example.com".parse()?).await?;
//!
//! // Split into read and write halves
//! let (mut write, mut read) = ws.split();
//!
//! // Spawn a task to read messages
//! tokio::spawn(async move {
//!     while let Some(frame) = read.next().await {
//!         println!("Received: {:?}", frame.opcode());
//!     }
//! });
//!
//! // Write from the main task
//! write.send(Frame::text("Hello!")).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Thread Safety Guarantees
//!
//! 1. **No Shared Mutable State**: Read and write halves operate independently with no
//!    shared mutable state between them.
//!
//! 2. **Borrowing Rules**: Rust's ownership system ensures only one task can access each
//!    half at a time, preventing data races at compile time.
//!
//! 3. **Send + Sync**: Both halves implement `Send`, allowing them to be moved across
//!    thread boundaries safely.
//!
//! 4. **Protocol Correctness**: Each half maintains its own protocol state (fragmentation,
//!    compression windows) ensuring WebSocket protocol correctness even with concurrent
//!    read/write operations.
//!
//! ## Low-Level `split_stream()` (Advanced)
//!
//! For advanced use cases requiring direct access to the underlying stream and protocol
//! handlers, use the unsafe `split_stream()` method:
//!
//! ```no_run
//! # use yawc::WebSocket;
//! # async fn example() -> yawc::Result<()> {
//! let ws = WebSocket::connect("wss://example.com".parse()?).await?;
//!
//! // SAFETY: User must ensure the stream is not used after splitting
//! let (mut stream, mut read_half, mut write_half) = unsafe { ws.split_stream() };
//!
//! // Manual protocol handling required
//! # Ok(())
//! # }
//! ```
//!
//! **Warning**: This is unsafe because it requires manual protocol handling. Users must:
//! - Correctly handle control frames (Ping, Pong, Close)
//! - Maintain proper message ordering
//! - Handle compression state correctly
//!
//! Prefer `futures::StreamExt::split()` unless you have specific low-level requirements.

use std::{
    future::poll_fn,
    task::{ready, Context, Poll},
};

use futures::SinkExt;

use crate::{
    close::CloseCode,
    frame::{Frame, OpCode},
    Result, WebSocketError,
};

// ================ ReadHalf ====================

/// The read half of a WebSocket connection, responsible for receiving and processing incoming messages.
///
/// [`ReadHalf`] follows a sans-io design, meaning it does not handle I/O operations directly.
/// Instead, the user must provide a [`futures::Stream`](https://docs.rs/futures/latest/futures/stream/trait.Stream.html)
/// of frames on each function call. This allows the ReadHalf to be I/O agnostic and work with any underlying transport.
///
/// [`ReadHalf`] handles decompression and message fragmentation but does not manage WebSocket control frames.
/// Users are responsible for handling frames with [`OpCode::Ping`], [`OpCode::Pong`], and [`OpCode::Close`] codes.
///
/// After a [`OpCode::Close`] frame is received, the [`ReadHalf`] will no longer accept reads and will
/// return a [`WebSocketError::ConnectionClosed`] error for all subsequent read attempts.
///
/// # Warning
///
/// In most cases, you should **not** use [`ReadHalf`] directly. Instead, use
/// [`futures::StreamExt::split`](https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html#method.split)
/// on the [`WebSocket`](super::WebSocket) to obtain a stream split that maintains all WebSocket protocol handling.
/// Direct use of [`ReadHalf`] bypasses important protocol management like automatic control frame handling.
///
///
/// # Example
/// ```no_run
/// use tokio::net::TcpStream;
/// use yawc::{WebSocket, frame::OpCode};
/// use futures::StreamExt;
/// use std::task::Context;
/// use std::pin::Pin;
/// use tokio_rustls::TlsConnector;
/// use url::Url;
///
/// #[tokio::main]
/// async fn main() -> yawc::Result<()> {
///     let url = "wss://api.example.com/ws".parse()?;
///     let ws = WebSocket::connect(url).await?;
///
///     let (mut stream, mut read_half, write_half) = unsafe { ws.split_stream() };
///
///     std::future::poll_fn(|cx| {
///         read_half.poll_frame(&mut stream, cx)
///     }).await?;
///
///     Ok(())
/// }
/// ```
pub struct ReadHalf {
    /// This flag is true when the first message is NOT FIN and RSV1 is set.
    pub(super) compressed_stream: bool,
    /// Indicates if the connection has been closed.
    pub(super) is_closed: bool,
}

impl ReadHalf {
    pub(super) fn new() -> Self {
        Self {
            compressed_stream: false,
            is_closed: false,
        }
    }

    /// Polls the WebSocket stream for the next frame, managing frame processing and protocol compliance.
    ///
    /// This method continuously polls the stream until one of three conditions is met:
    /// 1. A complete message frame is ready
    /// 2. An error occurs during frame processing
    /// 3. The WebSocket connection is closed
    ///
    /// # Message Types
    ///
    /// ## Data Frames
    /// Text and binary frames are processed according to the WebSocket protocol:
    /// - Complete frames are returned immediately
    /// - Fragmented frames are accumulated until the final fragment arrives
    /// - All fragments are validated for proper sequencing
    ///
    /// ## Control Frames
    /// Ping, Pong and Close frames receive special handling:
    /// - Always processed immediately when received
    /// - Never fragmented according to protocol
    /// - Close frames set internal closed state
    ///
    /// # Parameters
    /// - `stream`: The WebSocket stream to poll frames from
    /// - `cx`: Task context for asynchronous polling
    ///
    /// # Returns
    /// - `Poll::Ready(Ok(Frame))`: A complete frame was successfully received
    /// - `Poll::Ready(Err(WebSocketError))`: An error occurred during frame processing
    /// - `Poll::Pending`: More data needed to complete current frame
    ///
    /// On connection closure (receiving Close frame or stream end),
    /// returns `Poll::Ready(Err(WebSocketError::ConnectionClosed))`.
    pub fn poll_frame<S>(&mut self, stream: &mut S, cx: &mut Context<'_>) -> Poll<Result<Frame>>
    where
        S: futures::Stream<Item = Result<Frame>> + Unpin,
    {
        use futures::StreamExt;

        if self.is_closed {
            return Poll::Ready(Err(WebSocketError::ConnectionClosed));
        }

        let frame = ready!(stream.poll_next_unpin(cx));
        let res = match frame {
            Some(res) => res,
            None => {
                self.is_closed = true;
                return Poll::Ready(Err(WebSocketError::ConnectionClosed));
            }
        };

        match res {
            Ok(mut frame) => {
                #[cfg(test)]
                println!(
                    "<<Incoming<< OpCode={:?} Fin={} Payload={}",
                    frame.opcode,
                    frame.fin,
                    frame.payload.len()
                );

                // Remove mask immediately
                frame.mask = None;

                // Mark connection as closed if we receive a close frame
                if frame.opcode == OpCode::Close {
                    // don't do `self.is_closed = frame.opcode == OpCode::Close`
                    self.is_closed = true;
                }

                // the frame is considered compressed if it is compressed (of course)
                // OR it's not a control frame and it's within a compressed frame.
                frame.is_compressed =
                    frame.is_compressed || (!frame.opcode.is_control() && self.compressed_stream);

                // If the ocde is Text or Binary, and it is the first of a compressed sequence of fragmented frames
                if (frame.opcode == OpCode::Text || frame.opcode == OpCode::Binary)
                    && !frame.is_fin()
                    && frame.is_compressed
                {
                    self.compressed_stream = true;
                }
                // If the continuation frame is the last.
                if (frame.opcode == OpCode::Continuation) && frame.is_fin() {
                    self.compressed_stream = false;
                }

                Poll::Ready(Ok(frame))
            }
            Err(err) => Poll::Ready(Err(err)),
        }
    }

    /// Convenience method to read the next frame from the stream.
    ///
    /// This is a higher-level alternative to [`poll_frame`](Self::poll_frame) that handles
    /// the polling loop for you.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use yawc::WebSocket;
    /// # async fn example() -> yawc::Result<()> {
    /// let ws = WebSocket::connect("wss://example.com".parse()?).await?;
    /// let (mut stream, mut read_half, _write_half) = unsafe { ws.split_stream() };
    ///
    /// // Read frames one by one
    /// while let Ok(frame) = read_half.next_frame(&mut stream).await {
    ///     println!("Received: {:?}", frame.opcode());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn next_frame<S>(&mut self, stream: &mut S) -> Result<Frame>
    where
        S: futures::Stream<Item = Result<Frame>> + Unpin,
    {
        poll_fn(|cx| self.poll_frame(stream, cx)).await
    }
}

// ================ WriteHalf ====================

/// Write half of the WebSocket connection.
///
/// Keeps track of the sender's state.
pub struct WriteHalf {
    // Whether the user is streaming frames or not.
    //
    // This is used to prevent the RSV1 bit to be set on Continuation frames.
    pub(super) streaming: bool,
    close_state: Option<CloseState>,
}

/// Represents the various states involved in gracefully closing a WebSocket connection.
///
/// The `CloseState` enum defines the sequential steps taken to close the WebSocket connection:
///
/// - `Sending(Frame)`: The WebSocket is in the process of sending an `OpCode::Close` frame to the peer.
/// - `Flushing`: The `Close` frame has been sent, and the connection is now flushing any remaining data.
/// - `Closing`: The WebSocket is closing the underlying stream, ensuring all resources are properly released.
/// - `Done`: The connection is fully closed, and no further actions are required.
enum CloseState {
    /// The WebSocket is sending the `Close` frame to signal the end of the connection.
    Sending(Frame),
    /// The WebSocket is flushing the remaining data after the `Close` frame has been sent.
    Flushing,
    /// The WebSocket is in the process of closing the underlying network stream.
    Closing,
    /// The connection is fully closed, and all necessary shutdown steps have been completed.
    Done,
}

impl WriteHalf {
    pub(super) fn new() -> Self {
        Self {
            streaming: false,
            close_state: None,
        }
    }

    /// Polls the readiness of the `WriteHalf` to send a new frame.
    ///
    /// If the WebSocket connection is already closed, this will return an error. Otherwise, it
    /// checks if the underlying stream is ready to accept a new frame.
    ///
    /// # Parameters
    /// - `stream`: The WebSocket stream to check readiness on
    /// - `cx`: The polling context
    ///
    /// # Returns
    /// - `Poll::Ready(Ok(()))` if the `WriteHalf` is ready to send.
    /// - `Poll::Ready(Err(WebSocketError::ConnectionClosed))` if the connection is closed.
    pub fn poll_ready<S>(&mut self, stream: &mut S, cx: &mut Context<'_>) -> Poll<Result<()>>
    where
        S: futures::Sink<Frame, Error = WebSocketError> + Unpin,
    {
        if matches!(self.close_state, Some(CloseState::Done)) {
            return Poll::Ready(Err(WebSocketError::ConnectionClosed));
        }

        // ready if the underlying is
        stream.poll_ready_unpin(cx)
    }

    /// Begins sending a frame through the `WriteHalf`.
    ///
    /// This method takes a Frame representing the frame to be sent and prepares it for
    /// transmission according to the WebSocket protocol rules:
    ///
    /// - For non-control frames with compression enabled, the payload is compressed
    /// - For client connections, the frame is automatically masked per protocol requirements
    /// - Close frames trigger a state change to handle connection shutdown
    ///
    /// The method handles frame preparation but does not wait for the actual transmission to complete.
    /// The prepared frame is queued for sending in the underlying stream.
    ///
    /// # Parameters
    /// - `stream`: The WebSocket sink that will transmit the frame
    /// - `view`: A Frame containing the frame payload and metadata
    ///
    /// # Returns
    /// - `Ok(())` if the frame was successfully prepared and queued
    /// - `Err(WebSocketError)` if frame preparation or queueing failed
    ///
    /// # Protocol Details
    /// - Non-control frames are compressed if compression is enabled
    /// - Client frames are masked per WebSocket protocol requirement
    /// - Close frames transition the connection to closing state
    pub fn start_send<S>(&mut self, stream: &mut S, frame: Frame) -> Result<()>
    where
        S: futures::Sink<Frame, Error = WebSocketError> + Unpin,
    {
        if frame.opcode == OpCode::Close {
            self.close_state = Some(CloseState::Flushing);
        }

        // We are streaming if the frame is not control
        // and the FIN flag is not set.
        //
        // If the FIN flag was set on a Continuation frame, then streaming has finished
        if !frame.opcode.is_control() {
            self.streaming = !frame.is_fin();
        }

        #[cfg(test)]
        println!(
            ">>Outgoing>> OpCode={:?} Fin={} Payload={}",
            frame.opcode,
            frame.fin,
            frame.payload.len()
        );

        // Send directly to the underlying stream (fragmentation happens in WebSocket layer now)
        use futures::SinkExt;
        stream.start_send_unpin(frame)?;

        Ok(())
    }

    /// Polls to flush all pending frames in the `WriteHalf`.
    ///
    /// Ensures that any frames waiting to be sent are fully flushed to the network.
    ///
    /// # Parameters
    /// - `stream`: The WebSocket stream to flush frames on
    /// - `cx`: The polling context
    ///
    /// # Returns
    /// - `Poll::Ready(Ok(()))` if all pending frames have been flushed.
    /// - `Poll::Pending` if there are still frames waiting to be flushed.
    pub fn poll_flush<S>(&mut self, stream: &mut S, cx: &mut Context<'_>) -> Poll<Result<()>>
    where
        S: futures::Sink<Frame, Error = WebSocketError> + Unpin,
    {
        // Just flush the underlying stream (no queue to drain anymore)
        use futures::SinkExt;
        stream.poll_flush_unpin(cx)
    }

    /// Polls the connection to close the [`WriteHalf`] by initiating a graceful shutdown.
    ///
    /// The `poll_close` function guides the closure process through several stages:
    /// 1. **Sending**: Sends a `Close` frame to the peer.
    /// 2. **Flushing**: Ensures the `Close` frame and any pending frames are fully flushed.
    /// 3. **Closing**: Closes the underlying network stream once the frames are flushed.
    /// 4. **Done**: Marks the connection as closed.
    ///
    /// This sequence ensures a clean shutdown, allowing all necessary data to be sent before the connection closes.
    ///
    /// # Parameters
    /// - `stream`: The WebSocket stream to close
    /// - `cx`: The polling context
    ///
    /// # Returns
    /// - `Poll::Ready(Ok(()))` once the connection is fully closed.
    /// - `Poll::Pending` if the closure is still in progress.
    pub fn poll_close<S>(&mut self, stream: &mut S, cx: &mut Context<'_>) -> Poll<Result<()>>
    where
        S: futures::Sink<Frame, Error = WebSocketError> + Unpin,
    {
        loop {
            match self.close_state.take() {
                None => {
                    let frame = Frame::close(CloseCode::Normal, []);
                    self.close_state = Some(CloseState::Sending(frame));
                }
                Some(CloseState::Sending(frame)) => {
                    if stream.poll_ready_unpin(cx).is_pending() {
                        self.close_state = Some(CloseState::Sending(frame));
                        break Poll::Pending;
                    }

                    stream.start_send_unpin(frame)?;

                    self.close_state = Some(CloseState::Flushing);
                }
                Some(CloseState::Flushing) => {
                    if stream.poll_flush_unpin(cx).is_pending() {
                        self.close_state = Some(CloseState::Flushing);
                        break Poll::Pending;
                    }

                    self.close_state = Some(CloseState::Closing);
                }
                // TODO: we should probably wait for the read half close frame response
                Some(CloseState::Closing) => {
                    if stream.poll_close_unpin(cx).is_pending() {
                        self.close_state = Some(CloseState::Closing);
                        break Poll::Pending;
                    }

                    self.close_state = Some(CloseState::Done);
                }
                Some(CloseState::Done) => break Poll::Ready(Ok(())),
            }
        }
    }

    /// Convenience method to send a frame to the stream.
    ///
    /// This is a higher-level alternative to the poll-based methods that handles
    /// the polling loop for you. It ensures the sink is ready, sends the frame,
    /// and flushes it.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use yawc::{WebSocket, Frame};
    /// # async fn example() -> yawc::Result<()> {
    /// let ws = WebSocket::connect("wss://example.com".parse()?).await?;
    /// let (mut stream, _read_half, mut write_half) = unsafe { ws.split_stream() };
    ///
    /// // Send a frame
    /// write_half.send_frame(&mut stream, Frame::text("Hello")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_frame<S>(&mut self, stream: &mut S, frame: Frame) -> Result<()>
    where
        S: futures::Sink<Frame, Error = crate::WebSocketError> + Unpin,
    {
        // Wait until ready
        poll_fn(|cx| self.poll_ready(stream, cx)).await?;

        // Send the frame
        self.start_send(stream, frame)?;

        // Flush to ensure it's sent
        poll_fn(|cx| self.poll_flush(stream, cx)).await?;

        Ok(())
    }

    /// Convenience method to close the connection.
    ///
    /// This sends a close frame and waits for the connection to close gracefully.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use yawc::WebSocket;
    /// # async fn example() -> yawc::Result<()> {
    /// let ws = WebSocket::connect("wss://example.com".parse()?).await?;
    /// let (mut stream, _read_half, mut write_half) = unsafe { ws.split_stream() };
    ///
    /// // Close the connection
    /// write_half.close(&mut stream).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn close<S>(&mut self, stream: &mut S) -> Result<()>
    where
        S: futures::Sink<Frame, Error = crate::WebSocketError> + Unpin,
    {
        poll_fn(|cx| self.poll_close(stream, cx)).await
    }
}

//! WebSocket connection configuration options.

use std::time::Duration;

use crate::compression::WebSocketExtensions;

/// Type alias for the compression level used in WebSocket compression settings.
///
/// This alias refers to `flate2::Compression`, allowing easy configuration of compression levels
/// when creating compressors and decompressors for WebSocket frames.
pub type CompressionLevel = flate2::Compression;

/// Configuration options for a WebSocket connection.
///
/// `Options` allows comprehensive control over WebSocket connection behavior, including:
/// - **Payload size limits**: Control memory usage and prevent abuse
/// - **Compression**: Reduce bandwidth using permessage-deflate (RFC 7692)
/// - **Fragmentation**: Handle large messages
/// - **UTF-8 validation**: Ensure text frame compliance
/// - **TCP options**: Fine-tune network behavior
///
/// # Protocol Layers
///
/// The WebSocket implementation is organized into distinct processing layers:
///
/// ## 1. Fragmentation Layer
/// Handles splitting and reassembling messages across multiple frames. Controlled by
/// [`Options::fragmentation`] (see [`Fragmentation`] for details).
///
/// - **Outgoing**: Automatically fragments large messages when [`Fragmentation::fragment_size`] is set
/// - **Incoming**: Automatically reassembles fragments into complete messages before delivery
///
/// ## 2. Compression Layer
/// Applies permessage-deflate compression to reduce bandwidth. Controlled by
/// [`Options::compression`] (see [`DeflateOptions`] for details).
///
/// - Operates on complete messages (after fragmentation/before defragmentation)
/// - Can be configured with different compression levels and context takeover modes
/// - Negotiated during the WebSocket handshake
///
/// # Common Patterns
///
/// ## Basic Configuration
/// ```rust
/// use yawc::Options;
///
/// let options = Options::default()
///     .with_max_payload_read(1024 * 1024)  // 1 MiB max incoming
///     .with_utf8();                         // Validate text frames
/// ```
///
/// ## CPU and Memory-Constrained Environment
/// ```rust
/// use yawc::Options;
///
/// let options = Options::default()
///     .with_limits(128 * 1024, 256 * 1024)  // Small payload/buffer limits
///     .without_compression();                 // Avoid compression overhead
/// ```
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// Maximum allowed payload size for incoming messages, in bytes.
    ///
    /// If a message exceeds this size, the connection will be closed immediately
    /// to prevent overloading the receiving end.
    ///
    /// Default: 1 MiB (1,048,576 bytes) as defined in [`super::MAX_PAYLOAD_READ`]
    pub max_payload_read: Option<usize>,

    /// Maximum size allowed for the buffer that accumulates fragmented messages.
    ///
    /// WebSocket messages can be split into multiple fragments for transmission. These fragments
    /// are accumulated in a read buffer until the complete message is received. To prevent memory
    /// exhaustion attacks using extremely large fragmented messages, once the total size of
    /// accumulated fragments exceeds this limit, the connection is closed.
    ///
    /// Default: 2 MiB (2,097,152 bytes) as defined in [`super::MAX_READ_BUFFER`], or twice the
    /// configured `max_payload_read` value if that is set.
    pub max_read_buffer: Option<usize>,

    /// Compression settings for the WebSocket connection.
    ///
    /// Compression is based on the Deflate algorithm, with additional configuration available
    /// through [`DeflateOptions`] for finer control over the compression strategy.
    pub compression: Option<DeflateOptions>,

    /// Configuration for message fragmentation behavior.
    ///
    /// Controls how the WebSocket handles fragmented messages, including whether to
    /// reassemble fragments, timeouts for receiving fragments, and automatic fragmentation
    /// of outgoing messages.
    ///
    /// See [`Fragmentation`] for available options.
    pub fragmentation: Option<Fragmentation>,

    /// Flag to determine whether incoming messages should be validated for UTF-8 encoding.
    ///
    /// If `true`, the [`super::ReadHalf`] will validate that received text frames contain valid UTF-8
    /// data, closing the connection on any validation failure.
    ///
    /// Default: `false`
    pub check_utf8: bool,

    /// Flag to disable [Nagle's Algorithm](https://en.wikipedia.org/wiki/Nagle%27s_algorithm) on the TCP socket.
    ///
    /// If `true`, outgoing WebSocket messages will be sent immediately without waiting
    /// for additional data to be buffered, potentially improving latency.
    ///
    /// Default: `false`
    pub no_delay: bool,

    /// Backpressure boundary for the write buffer in bytes.
    ///
    /// When the write buffer exceeds this size, backpressure is applied to
    /// prevent unbounded memory growth. This helps with flow control when
    /// sending large amounts of data.
    ///
    /// Default: `None` (uses tokio-util default)
    pub max_backpressure_write_boundary: Option<usize>,
}

/// Configuration for WebSocket message fragmentation.
///
/// The WebSocket protocol allows large messages to be split into multiple fragments
/// for transmission. This struct controls how both incoming and outgoing fragmented
/// messages are handled.
///
/// # Fragmentation Modes
///
/// ## Standard Mode (default)
/// By default, incoming fragments are automatically reassembled into complete messages
/// before being returned to the application. This provides a simple interface where
/// applications always receive complete messages.
///
/// # Automatic Fragmentation
/// For outgoing messages, the `fragment_size` option enables automatic fragmentation
/// of large messages. This is useful for:
/// - Preventing large messages from blocking the send queue
/// - Ensuring fair interleaving of multiple concurrent messages
/// - Working with size constraints imposed by intermediaries
///
/// # Fragment Timeout
/// The `timeout` option protects against incomplete messages by setting a maximum
/// duration to wait for all fragments to arrive.
///
/// # Example
/// ```
/// use std::time::Duration;
/// use yawc::Options;
///
/// let options = Options::default()
///     .with_fragment_timeout(Duration::from_secs(30))
///     .with_max_fragment_size(64 * 1024); // 64 KiB fragments
/// ```
#[derive(Clone, Default, Debug)]
pub struct Fragmentation {
    /// Maximum time allowed to receive all fragments of a fragmented message.
    ///
    /// When receiving a fragmented WebSocket message, this timeout limits how long
    /// the connection will wait for all fragments to arrive. If the timeout expires
    /// before the final fragment is received, the connection returns a
    /// [`FragmentTimeout`](crate::WebSocketError::FragmentTimeout) error.
    ///
    /// This protects against slow-loris style attacks where a peer sends fragments
    /// very slowly to hold resources.
    ///
    /// Default: `None` (no timeout)
    pub timeout: Option<Duration>,

    /// Maximum size for each fragment when automatically splitting outgoing messages.
    ///
    /// When set, outgoing messages that exceed this size will be automatically
    /// fragmented into multiple frames. Each fragment will have a payload size
    /// at or below this limit.
    ///
    /// This is useful for:
    /// - Preventing large messages from blocking smaller ones
    /// - Working with intermediaries that have message size limits
    /// - Controlling memory usage in the send buffer
    ///
    /// Default: `None` (no automatic fragmentation)
    pub fragment_size: Option<usize>,
}

impl Options {
    /// Configures payload and buffer size limits.
    ///
    /// This is a convenience method to set both limits at once instead of calling
    /// `with_max_payload_read()` and `with_max_read_buffer()` separately.
    ///
    /// # Parameters
    /// - `max_payload`: Maximum size for a single frame's payload
    /// - `max_buffer`: Maximum size for accumulated fragmented message data
    ///
    /// # Example
    /// ```rust
    /// use yawc::Options;
    ///
    /// let options = Options::default()
    ///     .with_limits(2 * 1024 * 1024, 4 * 1024 * 1024); // 2MB payload, 4MB buffer
    /// ```
    pub fn with_limits(self, max_payload: usize, max_buffer: usize) -> Self {
        Self {
            max_payload_read: Some(max_payload),
            max_read_buffer: Some(max_buffer),
            ..self
        }
    }

    /// Configures compression using a preset profile optimized for low latency.
    ///
    /// This is equivalent to using `DeflateOptions::low_latency()`.
    /// Use this for real-time applications where response time is critical.
    ///
    /// # Example
    /// ```rust
    /// use yawc::Options;
    ///
    /// let options = Options::default().with_low_latency_compression();
    /// ```
    pub fn with_low_latency_compression(self) -> Self {
        Self {
            compression: Some(DeflateOptions::low_latency()),
            ..self
        }
    }

    /// Configures compression using a preset profile optimized for maximum compression ratio.
    ///
    /// This is equivalent to using `DeflateOptions::high_compression()`.
    /// Use this when bandwidth is limited and CPU resources are available.
    ///
    /// # Example
    /// ```rust
    /// use yawc::Options;
    ///
    /// let options = Options::default().with_high_compression();
    /// ```
    pub fn with_high_compression(self) -> Self {
        Self {
            compression: Some(DeflateOptions::high_compression()),
            ..self
        }
    }

    /// Configures compression using a balanced preset profile.
    ///
    /// This is equivalent to using `DeflateOptions::balanced()`.
    /// This is a good default for most applications.
    ///
    /// # Example
    /// ```rust
    /// use yawc::Options;
    ///
    /// let options = Options::default().with_balanced_compression();
    /// ```
    pub fn with_balanced_compression(self) -> Self {
        Self {
            compression: Some(DeflateOptions::balanced()),
            ..self
        }
    }

    /// Sets the compression level for outgoing messages.
    ///
    /// Adjusts the compression level used for message transmission, allowing a balance between
    /// compression efficiency and CPU usage. This option is useful for controlling bandwidth
    /// while managing resource consumption.
    ///
    /// The WebSocket protocol supports two main compression approaches:
    /// - Per-message compression (RFC 7692) using the permessage-deflate extension
    /// - Context takeover, which maintains compression state between messages
    ///
    /// Compression levels range from 0-9:
    /// - 0: No compression (fastest)
    /// - 1-3: Low compression, minimal CPU usage
    /// - 4-6: Balanced compression/CPU trade-off
    /// - 7-9: Maximum compression, highest CPU usage
    ///
    /// For real-time applications with latency constraints, lower compression levels (1-3)
    /// are recommended. For bandwidth-constrained scenarios where CPU usage is less critical,
    /// higher levels (7-9) may be preferable.
    ///
    /// # Parameters
    /// - `level`: The desired compression level, based on the [`CompressionLevel`] type.
    ///
    /// # Returns
    /// A modified `Options` instance with the specified compression level.
    ///
    /// # Example
    /// ```rust
    /// use yawc::{Options, CompressionLevel};
    ///
    /// let options = Options::default()
    ///     .with_compression_level(CompressionLevel::new(6)) // Balanced compression
    ///     .with_utf8(); // Enable UTF-8 validation
    /// ```
    pub fn with_compression_level(self, level: CompressionLevel) -> Self {
        let mut compression = self.compression.unwrap_or_default();
        compression.level = level;

        Self {
            compression: Some(compression),
            ..self
        }
    }

    /// Disables compression for the WebSocket connection.
    ///
    /// Removes any previously configured compression settings, ensuring that
    /// messages sent over this connection will not be compressed. This is useful
    /// when compression would add unnecessary overhead, such as when sending
    /// already-compressed data or small messages.
    ///
    /// # Returns
    /// A modified `Options` instance with compression disabled.
    pub fn without_compression(self) -> Self {
        Self {
            compression: None,
            ..self
        }
    }

    /// Sets the maximum allowed payload size for incoming messages.
    ///
    /// Specifies the maximum size of messages that the WebSocket connection will accept.
    /// If an incoming message exceeds this size, the connection will be terminated to avoid
    /// overloading the receiver.
    ///
    /// # Parameters
    /// - `size`: The maximum payload size in bytes.
    ///
    /// # Returns
    /// A modified `Options` instance with the specified payload size limit.
    pub fn with_max_payload_read(self, size: usize) -> Self {
        Self {
            max_payload_read: Some(size),
            ..self
        }
    }

    /// Sets the maximum read buffer size for accumulated fragmented messages.
    ///
    /// When receiving fragmented WebSocket messages, data is accumulated in a read buffer.
    /// Once this buffer exceeds the specified size limit, it will be reset back to the
    /// initial 8 KiB capacity to prevent unbounded memory growth from very large fragmented
    /// messages.
    ///
    /// # Parameters
    /// - `size`: Maximum size in bytes allowed for the read buffer
    ///
    /// # Returns
    /// A modified `Options` instance with the specified read buffer size limit.
    pub fn with_max_read_buffer(self, size: usize) -> Self {
        Self {
            max_read_buffer: Some(size),
            ..self
        }
    }

    /// Enables UTF-8 validation for incoming text messages.
    ///
    /// When enabled, the WebSocket connection will verify that all received text messages
    /// contain valid UTF-8 data. This is particularly useful for cases where the source of messages
    /// is untrusted, ensuring data integrity.
    ///
    /// # Returns
    /// A modified `Options` instance with UTF-8 validation enabled.
    pub fn with_utf8(self) -> Self {
        Self {
            check_utf8: true,
            ..self
        }
    }

    /// Enables TCP_NODELAY on the TCP stream.
    ///
    /// When enabled, [Nagle's Algorithm](https://en.wikipedia.org/wiki/Nagle%27s_algorithm) will be
    /// disabled on the TCP socket. WebSocket messages will be sent immediately without waiting
    /// for additional data to be buffered, potentially improving latency.
    ///
    /// # Returns
    /// A modified `Options` instance with TCP_NODELAY enabled.
    pub fn with_no_delay(self) -> Self {
        Self {
            no_delay: true,
            ..self
        }
    }

    /// Sets the maximum time allowed to receive all fragments of a fragmented message.
    ///
    /// This protects against slow-loris style attacks where a peer sends fragments
    /// very slowly to hold connection resources.
    ///
    /// # Parameters
    /// - `timeout`: Maximum duration to wait for all fragments
    ///
    /// # Returns
    /// A modified `Options` instance with the specified fragment timeout.
    ///
    /// # Example
    /// ```rust
    /// use std::time::Duration;
    /// use yawc::Options;
    ///
    /// let options = Options::default()
    ///     .with_fragment_timeout(Duration::from_secs(30));
    /// ```
    pub fn with_fragment_timeout(self, timeout: Duration) -> Self {
        let mut fragmentation = self.fragmentation.unwrap_or_default();
        fragmentation.timeout = Some(timeout);
        Self {
            fragmentation: Some(fragmentation),
            ..self
        }
    }

    /// Sets the maximum fragment size for automatic fragmentation of outgoing messages.
    ///
    /// When set, outgoing messages that exceed this size will be automatically
    /// fragmented into multiple frames. Each fragment will have a payload size
    /// at or below this limit.
    ///
    /// # Parameters
    /// - `size`: Maximum fragment size in bytes
    ///
    /// # Returns
    /// A modified `Options` instance with the specified fragment size.
    ///
    /// # Example
    /// ```rust
    /// use yawc::Options;
    ///
    /// let options = Options::default()
    ///     .with_max_fragment_size(64 * 1024); // 64 KiB max per fragment
    /// ```
    pub fn with_max_fragment_size(self, size: usize) -> Self {
        let mut fragmentation = self.fragmentation.unwrap_or_default();
        fragmentation.fragment_size = Some(size);
        Self {
            fragmentation: Some(fragmentation),
            ..self
        }
    }

    /// Sets the backpressure boundary for the write buffer.
    ///
    /// When the write buffer exceeds this size, backpressure is applied to
    /// prevent unbounded memory growth. This is useful for flow control when
    /// sending large amounts of data.
    ///
    /// # Parameters
    /// - `size`: Backpressure boundary in bytes
    ///
    /// # Returns
    /// A modified `Options` instance with the specified backpressure boundary.
    ///
    /// # Example
    /// ```rust
    /// use yawc::Options;
    ///
    /// let options = Options::default()
    ///     .with_backpressure_boundary(128 * 1024); // 128 KiB boundary
    /// ```
    pub fn with_backpressure_boundary(self, size: usize) -> Self {
        Self {
            max_backpressure_write_boundary: Some(size),
            ..self
        }
    }

    /// Sets the maximum window size for the client's decompression (LZ77) window.
    ///
    /// Limits the size of the client's decompression window, used during message decompression.
    /// This option is available only when compiled with the `zlib` feature.
    ///
    /// # Parameters
    /// - `max_window_bits`: The maximum number of bits for the client decompression window.
    ///
    /// # Returns
    /// A modified `Options` instance with the specified client max window size.
    #[cfg(feature = "zlib")]
    pub fn with_client_max_window_bits(self, max_window_bits: u8) -> Self {
        let mut compression = self.compression.unwrap_or_default();
        compression.client_max_window_bits = Some(max_window_bits);
        Self {
            compression: Some(compression),
            ..self
        }
    }

    /// Configures the maximum window size for server-side compression.
    ///
    /// The window size determines how much historical data the compressor can reference
    /// when encoding new data. Larger windows generally provide better compression but
    /// require more memory.
    ///
    /// Available only with the `zlib` feature enabled.
    ///
    /// # Parameters
    /// - `max_window_bits`: The maximum number of bits for the server compression window (9-15).
    ///
    /// # Returns
    /// A modified `Options` instance with the specified server max window size.
    #[cfg(feature = "zlib")]
    pub fn with_server_max_window_bits(self, max_window_bits: u8) -> Self {
        let mut compression = self.compression.unwrap_or_default();
        compression.server_max_window_bits = Some(max_window_bits);
        Self {
            compression: Some(compression),
            ..self
        }
    }

    /// Disables context takeover for server-side compression.
    ///
    /// In the WebSocket permessage-deflate extension, "context takeover" refers to maintaining
    /// the compression dictionary between messages. With context takeover enabled (default),
    /// the compression algorithm builds and reuses a dictionary of repeated patterns across
    /// multiple messages, potentially achieving better compression ratios for similar data.
    ///
    /// When disabled via this option:
    /// - The compression dictionary is reset after each message
    /// - Memory usage remains constant since the dictionary isn't preserved
    /// - Compression ratio may be lower since patterns can't be reused
    /// - Particularly useful for long-lived connections where memory growth is a concern
    ///
    /// This setting corresponds to the "server_no_context_takeover" extension parameter
    /// in the WebSocket protocol negotiation.
    ///
    /// # Returns
    /// A modified `Options` instance with server context takeover disabled.
    pub fn server_no_context_takeover(self) -> Self {
        let mut compression = self.compression.unwrap_or_default();
        compression.server_no_context_takeover = true;
        Self {
            compression: Some(compression),
            ..self
        }
    }

    /// Disables context takeover for client-side compression.
    ///
    /// In the WebSocket permessage-deflate extension, "context takeover" refers to maintaining
    /// the compression dictionary between messages. The client's compression context is separate
    /// from the server's, allowing asymmetric configuration based on each endpoint's capabilities.
    ///
    /// When disabled via this option:
    /// - The client's compression dictionary is reset after each message
    /// - Client memory usage remains constant since the dictionary isn't preserved
    /// - May reduce compression efficiency but prevents memory growth on clients
    /// - Useful for memory-constrained clients like mobile devices or browsers
    ///
    /// This setting corresponds to the "client_no_context_takeover" extension parameter
    /// in the WebSocket protocol negotiation.
    ///
    /// # Returns
    /// A modified `Options` instance with client context takeover disabled.
    pub fn client_no_context_takeover(self) -> Self {
        let mut compression = self.compression.unwrap_or_default();
        compression.client_no_context_takeover = true;
        Self {
            compression: Some(compression),
            ..self
        }
    }
}

/// Configuration options for WebSocket message compression using the Deflate algorithm.
///
/// The WebSocket protocol supports per-message compression using a Deflate-based algorithm
/// (RFC 7692) to reduce bandwidth usage. This struct allows fine-grained control over
/// compression behavior and memory usage through several key settings:
///
/// # Compression Level
/// Controls the tradeoff between compression ratio and CPU usage via the `level` field.
/// Higher levels provide better compression but require more processing time.
///
/// # Context Management
/// Offers two modes for managing the compression context:
///
/// - **Context Takeover** (default): Maintains compression state between messages,
///   providing better compression ratios at the cost of increased memory usage.
///   Ideal for applications prioritizing bandwidth efficiency.
///
/// - **No Context Takeover**: Resets compression state after each message,
///   reducing memory usage at the expense of compression efficiency.
///   Better suited for memory-constrained environments.
///
/// # Memory Window Size
/// When the `zlib` feature is enabled, allows precise control over the compression
/// window size for both client and server, enabling further optimization of the
/// memory-compression tradeoff.
///
/// # Example
/// ```
/// use yawc::{DeflateOptions, CompressionLevel};
///
/// let opts = DeflateOptions {
///     level: CompressionLevel::default(),
///     server_no_context_takeover: true,
///     ..Default::default()
/// };
/// ```
#[derive(Clone, Debug, Default)]
pub struct DeflateOptions {
    /// Sets the compression level (0-9), balancing compression ratio against CPU usage.
    ///
    /// - 0: No compression (fastest)
    /// - 1-3: Low compression (fast)
    /// - 4-6: Medium compression (default)
    /// - 7-9: High compression (slow)
    pub level: CompressionLevel,

    /// Controls the compression window size (in bits) for server-side compression.
    ///
    /// Larger windows improve compression but use more memory. Available only with
    /// the `zlib` feature enabled. Valid range: 8-15 bits.
    #[cfg(feature = "zlib")]
    pub server_max_window_bits: Option<u8>,

    /// Controls the compression window size (in bits) for client-side compression.
    ///
    /// Larger windows improve compression but use more memory. Available only with
    /// the `zlib` feature enabled. Valid range: 8-15 bits.
    #[cfg(feature = "zlib")]
    pub client_max_window_bits: Option<u8>,

    /// Controls server-side compression context management.
    ///
    /// When `true`, compression state is reset after each message, reducing
    /// memory usage at the cost of compression efficiency.
    pub server_no_context_takeover: bool,

    /// Controls client-side compression context management.
    ///
    /// When `true`, compression state is reset after each message, reducing
    /// memory usage at the cost of compression efficiency.
    pub client_no_context_takeover: bool,
}

impl DeflateOptions {
    /// Creates compression settings optimized for low latency.
    ///
    /// This profile uses:
    /// - Fast compression level (level 1)
    /// - No context takeover on both sides to minimize memory and processing
    ///
    /// Best for real-time applications where latency is critical.
    pub fn low_latency() -> Self {
        Self {
            level: CompressionLevel::fast(),
            #[cfg(feature = "zlib")]
            server_max_window_bits: None,
            #[cfg(feature = "zlib")]
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        }
    }

    /// Creates compression settings optimized for maximum compression.
    ///
    /// This profile uses:
    /// - Best compression level (level 9)
    /// - Context takeover enabled for better compression ratios
    ///
    /// Best for bandwidth-constrained scenarios where CPU is available.
    pub fn high_compression() -> Self {
        Self {
            level: CompressionLevel::best(),
            #[cfg(feature = "zlib")]
            server_max_window_bits: None,
            #[cfg(feature = "zlib")]
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        }
    }

    /// Creates balanced compression settings.
    ///
    /// This profile uses:
    /// - Default compression level (level 6)
    /// - Moderate window sizes
    /// - No context takeover to prevent memory growth
    ///
    /// Best general-purpose configuration for most applications.
    pub fn balanced() -> Self {
        Self {
            level: CompressionLevel::default(),
            #[cfg(feature = "zlib")]
            server_max_window_bits: None,
            #[cfg(feature = "zlib")]
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        }
    }

    /// Called by the server when upgrading.
    pub(super) fn merge(&self, offered: &WebSocketExtensions) -> WebSocketExtensions {
        WebSocketExtensions {
            // Accept client's no_context_takeover settings
            client_no_context_takeover: offered.client_no_context_takeover
                || self.client_no_context_takeover,
            server_no_context_takeover: offered.server_no_context_takeover
                || self.server_no_context_takeover,
            // For window bits, take the minimum of what client offers and what server supports
            #[cfg(feature = "zlib")]
            client_max_window_bits: match (
                offered.client_max_window_bits,
                self.client_max_window_bits,
            ) {
                // Client offers specific value, server has preference: take minimum
                (Some(Some(c)), Some(s)) => Some(Some(c.min(s))),
                // Client offers specific value, server has no preference: use client's
                (Some(Some(c)), None) => Some(Some(c)),
                // Client offers parameter without value, server has preference: use server's
                (Some(None), Some(s)) => Some(Some(s)),
                // Client offers parameter without value, server has no preference: use the minimum value
                (Some(None), None) => Some(Some(9)),
                // Client doesn't offer, use server's preference (if any)
                (None, s) => s.map(Some),
            },
            #[cfg(feature = "zlib")]
            server_max_window_bits: match (
                offered.server_max_window_bits,
                self.server_max_window_bits,
            ) {
                // Client offers specific value, server has preference: take minimum
                (Some(Some(c)), Some(s)) => Some(Some(c.min(s))),
                // Client offers specific value, server has no preference: use client's
                (Some(Some(c)), None) => Some(Some(c)),
                // Client offers parameter without value, server has preference: use server's
                (Some(None), Some(s)) => Some(Some(s)),
                // Client offers parameter without value, server has no preference: use the minimum value
                (Some(None), None) => Some(Some(9)),
                // Client doesn't offer, use server's preference (if any)
                (None, s) => s.map(Some),
            },
            #[cfg(not(feature = "zlib"))]
            client_max_window_bits: None,
            #[cfg(not(feature = "zlib"))]
            server_max_window_bits: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_no_context_takeover() {
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            #[cfg(feature = "zlib")]
            server_max_window_bits: None,
            #[cfg(feature = "zlib")]
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let client_offer = WebSocketExtensions {
            server_max_window_bits: None,
            client_max_window_bits: None,
            server_no_context_takeover: true,
            client_no_context_takeover: true,
        };

        let merged = server.merge(&client_offer);

        assert!(merged.server_no_context_takeover);
        assert!(merged.client_no_context_takeover);
    }

    #[test]
    fn test_merge_no_context_takeover_server_requires() {
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            #[cfg(feature = "zlib")]
            server_max_window_bits: None,
            #[cfg(feature = "zlib")]
            client_max_window_bits: None,
            server_no_context_takeover: true,
            client_no_context_takeover: false,
        };

        let client_offer = WebSocketExtensions {
            server_max_window_bits: None,
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let merged = server.merge(&client_offer);

        // Server requires no_context_takeover, so it's enabled even if client doesn't request it
        assert!(merged.server_no_context_takeover);
        assert!(!merged.client_no_context_takeover);
    }

    #[cfg(feature = "zlib")]
    #[test]
    fn test_merge_window_bits_takes_minimum() {
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            server_max_window_bits: Some(15),
            client_max_window_bits: Some(15),
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let client_offer = WebSocketExtensions {
            server_max_window_bits: Some(Some(12)),
            client_max_window_bits: Some(Some(10)),
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let merged = server.merge(&client_offer);

        // Should take the minimum of client and server values
        assert_eq!(merged.server_max_window_bits, Some(Some(12)));
        assert_eq!(merged.client_max_window_bits, Some(Some(10)));
    }

    #[cfg(feature = "zlib")]
    #[test]
    fn test_merge_window_bits_client_only() {
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            #[cfg(feature = "zlib")]
            server_max_window_bits: None,
            #[cfg(feature = "zlib")]
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let client_offer = WebSocketExtensions {
            server_max_window_bits: Some(Some(12)),
            client_max_window_bits: Some(Some(10)),
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let merged = server.merge(&client_offer);

        // Server accepts client's offered values when server has no preference
        assert_eq!(merged.server_max_window_bits, Some(Some(12)));
        assert_eq!(merged.client_max_window_bits, Some(Some(10)));
    }

    #[cfg(feature = "zlib")]
    #[test]
    fn test_merge_window_bits_server_only() {
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            #[cfg(feature = "zlib")]
            server_max_window_bits: Some(14),
            #[cfg(feature = "zlib")]
            client_max_window_bits: Some(13),
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let client_offer = WebSocketExtensions {
            server_max_window_bits: None,
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let merged = server.merge(&client_offer);

        // When client doesn't specify, use server's preference
        assert_eq!(merged.server_max_window_bits, Some(Some(14)));
        assert_eq!(merged.client_max_window_bits, Some(Some(13)));
    }

    #[cfg(feature = "zlib")]
    #[test]
    fn test_merge_window_bits_none_both() {
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            #[cfg(feature = "zlib")]
            server_max_window_bits: None,
            #[cfg(feature = "zlib")]
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let client_offer = WebSocketExtensions {
            server_max_window_bits: None,
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let merged = server.merge(&client_offer);

        // When neither specifies, should be None
        assert_eq!(merged.server_max_window_bits, None);
        assert_eq!(merged.client_max_window_bits, None);
    }

    #[test]
    fn test_merge_mixed_options() {
        #[cfg(feature = "zlib")]
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            server_max_window_bits: Some(15),
            client_max_window_bits: Some(14),
            server_no_context_takeover: true,
            client_no_context_takeover: false,
        };

        #[cfg(not(feature = "zlib"))]
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            server_no_context_takeover: true,
            client_no_context_takeover: false,
        };

        #[cfg(feature = "zlib")]
        let client_offer = WebSocketExtensions {
            server_max_window_bits: Some(Some(12)),
            client_max_window_bits: Some(Some(13)),
            server_no_context_takeover: false,
            client_no_context_takeover: true,
        };

        #[cfg(not(feature = "zlib"))]
        let client_offer = WebSocketExtensions {
            server_max_window_bits: None,
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: true,
        };

        let merged = server.merge(&client_offer);

        // Both server and client request no_context_takeover for their respective sides
        assert!(merged.server_no_context_takeover);
        assert!(merged.client_no_context_takeover);

        #[cfg(feature = "zlib")]
        {
            // Window bits should take minimum
            assert_eq!(merged.server_max_window_bits, Some(Some(12)));
            assert_eq!(merged.client_max_window_bits, Some(Some(13)));
        }

        #[cfg(not(feature = "zlib"))]
        {
            // Without zlib feature, window bits should be None
            assert_eq!(merged.server_max_window_bits, None);
            assert_eq!(merged.client_max_window_bits, None);
        }
    }

    #[cfg(feature = "zlib")]
    #[test]
    fn test_merge_client_offers_no_value() {
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            server_max_window_bits: Some(14),
            client_max_window_bits: Some(13),
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let client_offer = WebSocketExtensions {
            server_max_window_bits: Some(None), // Client offers parameter without value
            client_max_window_bits: Some(None), // Client offers parameter without value
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let merged = server.merge(&client_offer);

        // When client offers without value, use server's preference
        assert_eq!(merged.server_max_window_bits, Some(Some(14)));
        assert_eq!(merged.client_max_window_bits, Some(Some(13)));
    }

    #[cfg(feature = "zlib")]
    #[test]
    fn test_merge_client_offers_no_value_server_no_preference() {
        let server = DeflateOptions {
            level: CompressionLevel::default(),
            server_max_window_bits: None,
            client_max_window_bits: None,
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let client_offer = WebSocketExtensions {
            server_max_window_bits: Some(None), // Client offers parameter without value
            client_max_window_bits: Some(None), // Client offers parameter without value
            server_no_context_takeover: false,
            client_no_context_takeover: false,
        };

        let merged = server.merge(&client_offer);

        // When client offers without value and server has no preference, leave unspecified
        assert_eq!(merged.server_max_window_bits, Some(Some(9)));
        assert_eq!(merged.client_max_window_bits, Some(Some(9)));
    }

    #[test]
    fn test_parse_and_merge_client_offers_client_max_window_bits_no_value() {
        use std::str::FromStr;

        // Parse client offer: "permessage-deflate; client_max_window_bits"
        let client_offer =
            WebSocketExtensions::from_str("permessage-deflate; client_max_window_bits").unwrap();

        // Verify client offered the parameter without a value
        assert_eq!(client_offer.client_max_window_bits, Some(None));
        assert_eq!(client_offer.server_max_window_bits, None);

        #[cfg(feature = "zlib")]
        {
            // Server has a preference for client_max_window_bits
            let server = DeflateOptions {
                level: CompressionLevel::default(),
                server_max_window_bits: Some(15),
                client_max_window_bits: Some(12), // Server prefers 12
                server_no_context_takeover: false,
                client_no_context_takeover: false,
            };

            let merged = server.merge(&client_offer);

            // Server should respond with its preference
            assert_eq!(merged.client_max_window_bits, Some(Some(12)));
            assert_eq!(merged.server_max_window_bits, Some(Some(15)));

            // Verify the response string includes the value
            let response = merged.to_string();
            assert!(response.contains("client_max_window_bits=12"));
            assert!(response.contains("server_max_window_bits=15"));
        }

        #[cfg(not(feature = "zlib"))]
        {
            // Without zlib, server can't negotiate window bits
            let server = DeflateOptions {
                level: CompressionLevel::default(),
                server_no_context_takeover: false,
                client_no_context_takeover: false,
            };

            let merged = server.merge(&client_offer);

            // Should be None without zlib feature
            assert_eq!(merged.client_max_window_bits, None);
            assert_eq!(merged.server_max_window_bits, None);
        }
    }
}

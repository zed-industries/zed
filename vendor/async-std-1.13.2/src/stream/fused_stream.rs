use crate::stream::Stream;

/// A stream that always continues to yield `None` when exhausted.
///
/// Calling next on a fused stream that has returned `None` once is guaranteed
/// to return [`None`] again. This trait should be implemented by all streams
/// that behave this way because it allows optimizing [`Stream::fuse`].
///
/// Note: In general, you should not use `FusedStream` in generic bounds if
/// you need a fused stream. Instead, you should just call [`Stream::fuse`]
/// on the stream. If the stream is already fused, the additional [`Fuse`]
/// wrapper will be a no-op with no performance penalty.
///
/// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
/// [`Stream::fuse`]: trait.Stream.html#method.fuse
/// [`Fuse`]: struct.Fuse.html
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
pub trait FusedStream: Stream {}

impl<S: FusedStream + ?Sized + Unpin> FusedStream for &mut S {}

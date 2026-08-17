use client::Client;
use tracing_subscriber::fmt::format::DefaultFields;
use tracing_subscriber::fmt::FormatFields;

/// Configuration of the [`TracyLayer`](super::TracyLayer) behaviour.
///
/// For most users [`DefaultConfig`] is going to be a good default choice, however advanced users
/// can implement this trait manually to override the formatter used or to otherwise modify the
/// behaviour of the `TracyLayer`.
///
/// # Examples
///
/// ```
/// use tracing_subscriber::fmt::format::DefaultFields;
///
/// struct TracyLayerConfig {
///     fmt: DefaultFields,
/// }
/// impl tracing_tracy::Config for TracyLayerConfig {
///     type Formatter = DefaultFields;
///     fn formatter(&self) -> &Self::Formatter {
///         &self.fmt
///     }
///     // The boilerplate ends here
///
///     /// Collect 32 frames in stack traces.
///     fn stack_depth(&self, _: &tracing::Metadata) -> u16 {
///         32
///     }
///
///     /// Do not format fields into zone names.
///     fn format_fields_in_zone_name(&self) -> bool {
///         false
///     }
///
///     // etc.
/// }
/// ```
///
/// With this configuration `TracyLayer` will collect some call stacks and the formatting of the
/// zone names is different from the `DefaultConfig`.
pub trait Config {
    type Formatter: for<'writer> FormatFields<'writer> + 'static;

    /// Use a custom field formatting implementation.
    fn formatter(&self) -> &Self::Formatter;

    /// Specify the maximum number of stack frames that will be collected.
    ///
    /// Note that enabling callstack collection can and will introduce a non-trivial overhead at
    /// every instrumentation point. Specifying 0 frames will disable stack trace collection.
    ///
    /// Default implementation returns `0`.
    fn stack_depth(&self, metadata: &tracing_core::Metadata<'_>) -> u16 {
        let _ = metadata;
        0
    }

    /// Specify whether or not to include tracing span fields in the tracy zone name, or to emit
    /// them as zone text.
    ///
    /// The former enables zone analysis along unique span field invocations, while the latter
    /// aggregates every invocation of a given span into a single zone, irrespective of field
    /// values.
    ///
    /// Default implementation returns `true`.
    fn format_fields_in_zone_name(&self) -> bool {
        true
    }

    /// Apply handling for errors detected by the [`TracyLayer`](super::TracyLayer).
    ///
    /// Fundamentally the way the tracing crate and the Tracy profiler work are somewhat
    /// incompatible in certain ways. For instance, a `tracing::Span` can be created on one
    /// thread and moved to another, where it is cleaned up. Tracy on the other hand expects that
    /// its eqvivalent concept of zone remains entirely within a thread.
    ///
    /// Another example a limitation in `Tracy` where the message length or zone name cannot exceed
    /// a certain (low) limit of bytes.
    ///
    /// Although `tracing_tracy` does it best to paper over these sorts of differences, it can’t
    /// always make them invisible. In certain cases detecting these sorts of issues is
    /// straightforward, and it is when `tracing_tracy` will invoke this method to enable users to
    /// report the issues in whatever way they wish to.
    ///
    /// By default a message coloured in red is emitted to the tracy client.
    fn on_error(&self, client: &Client, error: &'static str) {
        client.color_message(error, 0xFF000000, 0);
    }
}

/// A default configuration of the [`TracyLayer`](super::TracyLayer).
///
/// This type does not allow for any adjustment of the configuration. In order to customize
/// the behaviour of the layer implement the [`Config`] trait for your own type.
#[derive(Default)]
pub struct DefaultConfig(DefaultFields);

impl Config for DefaultConfig {
    type Formatter = DefaultFields;
    fn formatter(&self) -> &Self::Formatter {
        &self.0
    }
}

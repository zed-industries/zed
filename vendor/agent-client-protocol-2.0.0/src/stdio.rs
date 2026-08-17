//! Stdio transport for connecting ACP components via standard input/output.

use crate::acp_agent::LineDirection;
use crate::{ByteStreams, ConnectTo, Role};
use std::sync::Arc;

/// A transport that connects to an ACP peer via standard input/output.
///
/// This is useful for building agents or proxies that communicate over stdio,
/// which is the standard transport for MCP and ACP subprocess communication.
pub struct Stdio {
    debug_callback: Option<Arc<dyn Fn(&str, LineDirection) + Send + Sync + 'static>>,
}

impl std::fmt::Debug for Stdio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stdio").finish_non_exhaustive()
    }
}

impl Stdio {
    /// Create a new `Stdio` transport.
    #[must_use]
    pub fn new() -> Self {
        Self {
            debug_callback: None,
        }
    }

    /// Add a debug callback that will be invoked for each line sent/received.
    #[must_use]
    pub fn with_debug<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str, LineDirection) + Send + Sync + 'static,
    {
        self.debug_callback = Some(Arc::new(callback));
        self
    }
}

impl Default for Stdio {
    fn default() -> Self {
        Self::new()
    }
}

impl<Counterpart: Role> ConnectTo<Counterpart> for Stdio {
    async fn connect_to(
        self,
        client: impl ConnectTo<Counterpart::Counterpart>,
    ) -> Result<(), crate::Error> {
        let stdin = blocking::Unblock::new(std::io::stdin());
        let stdout = blocking::Unblock::new(std::io::stdout());

        if let Some(callback) = self.debug_callback {
            use futures::io::BufReader;
            use futures::{AsyncBufReadExt, StreamExt};

            let incoming_callback = callback.clone();
            let incoming_lines = Box::pin(BufReader::new(stdin).lines().inspect(move |result| {
                if let Ok(line) = result {
                    incoming_callback(line, LineDirection::Stdin);
                }
            }))
                as std::pin::Pin<Box<dyn futures::Stream<Item = std::io::Result<String>> + Send>>;

            let outgoing_sink = Box::pin(futures::sink::unfold(
                (stdout, callback),
                async move |(mut writer, callback), line: String| {
                    callback(&line, LineDirection::Stdout);
                    crate::jsonrpc::write_line(&mut writer, line).await?;
                    Ok::<_, std::io::Error>((writer, callback))
                },
            ))
                as std::pin::Pin<Box<dyn futures::Sink<String, Error = std::io::Error> + Send>>;

            ConnectTo::<Counterpart>::connect_to(
                crate::Lines::new(outgoing_sink, incoming_lines),
                client,
            )
            .await
        } else {
            ConnectTo::<Counterpart>::connect_to(ByteStreams::new(stdout, stdin), client).await
        }
    }
}

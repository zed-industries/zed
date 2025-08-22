/// Some helpers for structured error handling.
///
/// The helpers defined here allow you to pass type-safe error codes from
/// the collab server to the client; and provide a mechanism for additional
/// structured data alongside the message.
///
/// When returning an error, it can be as simple as:
///
/// `return Err(Error::Forbidden.into())`
///
/// If you'd like to log more context, you can set a message. These messages
/// show up in our logs, but are not shown visibly to users.
///
/// `return Err(Error::Forbidden.message("not an admin").into())`
///
/// If you'd like to provide enough context that the UI can render a good error
/// message (or would be helpful to see in a structured format in the logs), you
/// can use .with_tag():
///
/// `return Err(Error::WrongReleaseChannel.with_tag("required", "stable").into())`
///
/// When handling an error you can use .error_code() to match which error it was
/// and .error_tag() to read any tags.
///
/// ```
/// match err.error_code() {
///   ErrorCode::Forbidden => alert("I'm sorry I can't do that.")
///   ErrorCode::WrongReleaseChannel =>
///     alert(format!("You need to be on the {} release channel.", err.error_tag("required").unwrap()))
///   ErrorCode::Internal => alert("Sorry, something went wrong")
/// }
/// ```
///
pub use crate::ErrorCode;

/// ErrorCodeExt provides some helpers for structured error handling.
///
/// The primary implementation is on the proto::ErrorCode to easily convert
/// that into an anyhow::Error, which we use pervasively.
///
/// The RpcError struct provides support for further metadata if needed.
pub trait ErrorCodeExt {
    /// Return an anyhow::Error containing this.
    /// (useful in places where .into() doesn't have enough type information)
    fn anyhow(self) -> anyhow::Error;

    /// Add a message to the error (by default the error code is used)
    fn message(self, msg: String) -> RpcError;

    /// Add a tag to the error. Tags are key value pairs that can be used
    /// to send semi-structured data along with the error.
    fn with_tag(self, k: &str, v: &str) -> RpcError;
}

impl ErrorCodeExt for ErrorCode {
    fn anyhow(self) -> anyhow::Error {
        self.into()
    }

    fn message(self, msg: String) -> RpcError {
        let err: RpcError = self.into();
        err.message(msg)
    }

    fn with_tag(self, k: &str, v: &str) -> RpcError {
        let err: RpcError = self.into();
        err.with_tag(k, v)
    }
}

/// ErrorExt provides helpers for structured error handling.
///
/// The primary implementation is on the anyhow::Error, which is
/// what we use throughout our codebase. Though under the hood this
pub trait ErrorExt {
    /// error_code() returns the ErrorCode (or ErrorCode::Internal if there is none)
    fn error_code(&self) -> ErrorCode;
    /// error_tag() returns the value of the tag with the given key, if any.
    fn error_tag(&self, k: &str) -> Option<&str>;
    /// to_proto() converts the error into a crate::Error
    fn to_proto(&self) -> crate::Error;
    /// Clones the error and turns into an [anyhow::Error].
    fn cloned(&self) -> anyhow::Error;
}

impl ErrorExt for anyhow::Error {
    fn error_code(&self) -> ErrorCode {
        if let Some(rpc_error) = self.downcast_ref::<RpcError>() {
            rpc_error.code
        } else {
            ErrorCode::Internal
        }
    }

    fn error_tag(&self, k: &str) -> Option<&str> {
        if let Some(rpc_error) = self.downcast_ref::<RpcError>() {
            rpc_error.error_tag(k)
        } else {
            None
        }
    }

    fn to_proto(&self) -> crate::Error {
        if let Some(rpc_error) = self.downcast_ref::<RpcError>() {
            rpc_error.to_proto()
        } else {
            ErrorCode::Internal
                .message(
                    format!("{self:#}")
                        .lines()
                        .fold(String::new(), |mut message, line| {
                            if !message.is_empty() {
                                message.push(' ');
                            }
                            message.push_str(line);
                            message
                        }),
                )
                .to_proto()
        }
    }

    fn cloned(&self) -> anyhow::Error {
        if let Some(rpc_error) = self.downcast_ref::<RpcError>() {
            rpc_error.cloned()
        } else {
            anyhow::anyhow!("{self}")
        }
    }
}

impl From<ErrorCode> for anyhow::Error {
    fn from(value: ErrorCode) -> Self {
        RpcError {
            request: None,
            code: value,
            msg: format!("{:?}", value),
            tags: Default::default(),
        }
        .into()
    }
}

#[derive(Clone, Debug)]
pub struct RpcError {
    request: Option<String>,
    msg: String,
    code: ErrorCode,
    tags: Vec<String>,
}

/// RpcError is a structured error type that is returned by the collab server.
/// In addition to a message, it lets you set a specific ErrorCode, and attach
/// small amounts of metadata to help the client handle the error appropriately.
///
/// This struct is not typically used directly, as we pass anyhow::Error around
/// in the app; however it is useful for chaining .message() and .with_tag() on
/// ErrorCode.
impl RpcError {
    /// from_proto converts a crate::Error into an anyhow::Error containing
    /// an RpcError.
    pub fn from_proto(error: &crate::Error, request: &str) -> anyhow::Error {
        RpcError {
            request: Some(request.to_string()),
            code: error.code(),
            msg: error.message.clone(),
            tags: error.tags.clone(),
        }
        .into()
    }
}

impl ErrorCodeExt for RpcError {
    fn message(mut self, msg: String) -> RpcError {
        self.msg = msg;
        self
    }

    fn with_tag(mut self, k: &str, v: &str) -> RpcError {
        self.tags.push(format!("{}={}", k, v));
        self
    }

    fn anyhow(self) -> anyhow::Error {
        self.into()
    }
}

impl ErrorExt for RpcError {
    fn error_tag(&self, k: &str) -> Option<&str> {
        for tag in &self.tags {
            let mut parts = tag.split('=');
            if let Some(key) = parts.next()
                && key == k
            {
                return parts.next();
            }
        }
        None
    }

    fn error_code(&self) -> ErrorCode {
        self.code
    }

    fn to_proto(&self) -> crate::Error {
        crate::Error {
            code: self.code as i32,
            message: self.msg.clone(),
            tags: self.tags.clone(),
        }
    }

    fn cloned(&self) -> anyhow::Error {
        self.clone().into()
    }
}

impl std::error::Error for RpcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(request) = &self.request {
            write!(f, "RPC request {} failed: {}", request, self.msg)?
        } else {
            write!(f, "{}", self.msg)?
        }
        for tag in &self.tags {
            write!(f, " {}", tag)?
        }
        Ok(())
    }
}

impl From<ErrorCode> for RpcError {
    fn from(code: ErrorCode) -> Self {
        RpcError {
            request: None,
            code,
            msg: format!("{:?}", code),
            tags: Default::default(),
        }
    }
}

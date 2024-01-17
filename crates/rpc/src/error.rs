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
use crate::proto;
pub use proto::ErrorCode;

/// ErrorCodeExt provides some helpers for structured error handling.
///
/// The primary implementation is on the proto::ErrorCode to easily convert
/// that into an anyhow::Error, which we use pervasively.
///
/// The RPCError struct provides support for further metadata if needed.
pub trait ErrorCodeExt {
    /// Return an anyhow::Error containing this.
    /// (useful in places where .into() doesn't have enough type information)
    fn anyhow(self) -> anyhow::Error;

    /// Add a message to the error (by default the error code is used)
    fn message(self, msg: String) -> RPCError;

    /// Add a tag to the error. Tags are key value pairs that can be used
    /// to send semi-structured data along with the error.
    fn with_tag(self, k: &str, v: &str) -> RPCError;
}

impl ErrorCodeExt for proto::ErrorCode {
    fn anyhow(self) -> anyhow::Error {
        self.into()
    }

    fn message(self, msg: String) -> RPCError {
        let err: RPCError = self.into();
        err.message(msg)
    }

    fn with_tag(self, k: &str, v: &str) -> RPCError {
        let err: RPCError = self.into();
        err.with_tag(k, v)
    }
}

/// ErrorExt provides helpers for structured error handling.
///
/// The primary implementation is on the anyhow::Error, which is
/// what we use throughout our codebase. Though under the hood this
pub trait ErrorExt {
    /// error_code() returns the ErrorCode (or ErrorCode::Internal if there is none)
    fn error_code(&self) -> proto::ErrorCode;
    /// error_tag() returns the value of the tag with the given key, if any.
    fn error_tag(&self, k: &str) -> Option<&str>;
    /// to_proto() convers the error into a proto::Error
    fn to_proto(&self) -> proto::Error;
}

impl ErrorExt for anyhow::Error {
    fn error_code(&self) -> proto::ErrorCode {
        if let Some(rpc_error) = self.downcast_ref::<RPCError>() {
            rpc_error.code
        } else {
            proto::ErrorCode::Internal
        }
    }

    fn error_tag(&self, k: &str) -> Option<&str> {
        if let Some(rpc_error) = self.downcast_ref::<RPCError>() {
            rpc_error.error_tag(k)
        } else {
            None
        }
    }

    fn to_proto(&self) -> proto::Error {
        if let Some(rpc_error) = self.downcast_ref::<RPCError>() {
            rpc_error.to_proto()
        } else {
            ErrorCode::Internal.message(format!("{}", self)).to_proto()
        }
    }
}

impl From<proto::ErrorCode> for anyhow::Error {
    fn from(value: proto::ErrorCode) -> Self {
        RPCError {
            request: None,
            code: value,
            msg: format!("{:?}", value).to_string(),
            tags: Default::default(),
        }
        .into()
    }
}

#[derive(Clone, Debug)]
pub struct RPCError {
    request: Option<String>,
    msg: String,
    code: proto::ErrorCode,
    tags: Vec<String>,
}

/// RPCError is a structured error type that is returned by the collab server.
/// In addition to a message, it lets you set a specific ErrorCode, and attach
/// small amounts of metadata to help the client handle the error appropriately.
///
/// This struct is not typically used directly, as we pass anyhow::Error around
/// in the app; however it is useful for chaining .message() and .with_tag() on
/// ErrorCode.
impl RPCError {
    /// from_proto converts a proto::Error into an anyhow::Error containing
    /// an RPCError.
    pub fn from_proto(error: &proto::Error, request: &str) -> anyhow::Error {
        RPCError {
            request: Some(request.to_string()),
            code: error.code(),
            msg: error.message.clone(),
            tags: error.tags.clone(),
        }
        .into()
    }
}

impl ErrorCodeExt for RPCError {
    fn message(mut self, msg: String) -> RPCError {
        self.msg = msg;
        self
    }

    fn with_tag(mut self, k: &str, v: &str) -> RPCError {
        self.tags.push(format!("{}={}", k, v));
        self
    }

    fn anyhow(self) -> anyhow::Error {
        self.into()
    }
}

impl ErrorExt for RPCError {
    fn error_tag(&self, k: &str) -> Option<&str> {
        for tag in &self.tags {
            let mut parts = tag.split('=');
            if let Some(key) = parts.next() {
                if key == k {
                    return parts.next();
                }
            }
        }
        None
    }

    fn error_code(&self) -> proto::ErrorCode {
        self.code
    }

    fn to_proto(&self) -> proto::Error {
        proto::Error {
            code: self.code as i32,
            message: self.msg.clone(),
            tags: self.tags.clone(),
        }
    }
}

impl std::error::Error for RPCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for RPCError {
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

impl From<proto::ErrorCode> for RPCError {
    fn from(code: proto::ErrorCode) -> Self {
        RPCError {
            request: None,
            code,
            msg: format!("{:?}", code).to_string(),
            tags: Default::default(),
        }
    }
}

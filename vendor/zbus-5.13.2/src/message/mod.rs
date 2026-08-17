//! D-Bus Message.
use std::{borrow::Cow, fmt, sync::Arc};

use zbus_names::{ErrorName, InterfaceName, MemberName};
use zvariant::{Endian, serialized};

use crate::{Error, Result, utils::padding_for_8_bytes, zvariant::ObjectPath};

mod builder;
pub use builder::Builder;

mod field_code;
pub(crate) use field_code::FieldCode;

mod fields;
pub(crate) use fields::Fields;
use fields::QuickFields;

mod body;
pub use body::Body;

pub(crate) mod header;
pub use header::{EndianSig, Flags, Header, NATIVE_ENDIAN_SIG, PrimaryHeader, Type};
use header::{MIN_MESSAGE_SIZE, PRIMARY_HEADER_SIZE};

/// A position in the stream of [`Message`] objects received by a single [`zbus::Connection`].
///
/// Note: the relative ordering of values obtained from distinct [`zbus::Connection`] objects is
/// not specified; only sequence numbers originating from the same connection should be compared.
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Sequence {
    recv_seq: u64,
}

impl Sequence {
    /// A sequence number that is higher than any other; used by errors that terminate a stream.
    pub(crate) const LAST: Self = Self { recv_seq: u64::MAX };
}

/// A D-Bus Message.
///
/// The contents of the message are stored in serialized format. To get the body of the message, use
/// the [`Message::body`] method, and use [`Body`] methods to deserialize it. You may also access
/// the header and other details with the various other getters.
///
/// Also provided are constructors for messages of different types. These will mainly be useful for
/// very advanced use cases as typically you will want to create a message for immediate dispatch
/// and hence use the API provided by [`crate::connection::Connection::call_method`], even when
/// using the low-level API.
///
/// **Note**: The message owns the received FDs and will close them when dropped. You can
/// deserialize the body (that you get using [`Message::body`]) to [`zvariant::OwnedFd`] if you want
/// to keep the FDs around after the containing message is dropped.
#[derive(Clone)]
pub struct Message {
    pub(super) inner: Arc<Inner>,
}

pub(super) struct Inner {
    pub(crate) primary_header: PrimaryHeader,
    pub(crate) quick_fields: std::sync::OnceLock<QuickFields>,
    pub(crate) bytes: serialized::Data<'static, 'static>,
    pub(crate) body_offset: usize,
    pub(crate) recv_seq: Sequence,
}

impl Message {
    /// Create a builder for a message of type [`Type::MethodCall`].
    pub fn method_call<'b, 'p: 'b, 'm: 'b, P, M>(path: P, method_name: M) -> Result<Builder<'b>>
    where
        P: TryInto<ObjectPath<'p>>,
        M: TryInto<MemberName<'m>>,
        P::Error: Into<Error>,
        M::Error: Into<Error>,
    {
        Builder::new(Type::MethodCall)
            .path(path)?
            .member(method_name)
    }

    /// Create a builder for a message of type [`Type::Signal`].
    pub fn signal<'b, 'p: 'b, 'i: 'b, 'm: 'b, P, I, M>(
        path: P,
        iface: I,
        signal_name: M,
    ) -> Result<Builder<'b>>
    where
        P: TryInto<ObjectPath<'p>>,
        I: TryInto<InterfaceName<'i>>,
        M: TryInto<MemberName<'m>>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
        M::Error: Into<Error>,
    {
        Builder::new(Type::Signal)
            .path(path)?
            .interface(iface)?
            .member(signal_name)
    }

    /// Create a builder for a message of type [`Type::MethodReturn`].
    pub fn method_return(reply_to: &Header<'_>) -> Result<Builder<'static>> {
        Builder::new(Type::MethodReturn).reply_to(reply_to)
    }

    /// Create a builder for a message of type [`Type::Error`].
    pub fn error<'b, 'e: 'b, E>(reply_to: &Header<'_>, name: E) -> Result<Builder<'b>>
    where
        E: TryInto<ErrorName<'e>>,
        E::Error: Into<Error>,
    {
        Builder::new(Type::Error)
            .reply_to(reply_to)?
            .error_name(name)
    }

    /// Create a message from bytes.
    ///
    /// **Note:** Since the message is not constructed by zbus, the receive sequence,
    /// which can be acquired from [`Message::recv_position`], is not applicable and hence set
    /// to `0`.
    ///
    /// # Safety
    ///
    /// This method is unsafe as bytes may have an invalid encoding.
    pub unsafe fn from_bytes(bytes: serialized::Data<'static, 'static>) -> Result<Self> {
        Self::from_raw_parts(bytes, 0)
    }

    /// Create a message from its full contents.
    pub(crate) fn from_raw_parts(
        bytes: serialized::Data<'static, 'static>,
        recv_seq: u64,
    ) -> Result<Self> {
        let endian = Endian::from(EndianSig::try_from(bytes[0])?);
        if endian != bytes.context().endian() {
            return Err(Error::IncorrectEndian);
        }

        let (primary_header, fields_len) = PrimaryHeader::read_from_data(&bytes)?;
        let fields_bytes = bytes.slice(PRIMARY_HEADER_SIZE..);
        let (fields, _) = fields_bytes.deserialize()?;
        let header = Header::new(primary_header.clone(), fields);

        let header_len = MIN_MESSAGE_SIZE + fields_len as usize;
        let body_offset = header_len + padding_for_8_bytes(header_len);
        let quick_fields = QuickFields::new(&bytes, &header).into();

        Ok(Self {
            inner: Arc::new(Inner {
                primary_header,
                quick_fields,
                bytes,
                body_offset,
                recv_seq: Sequence { recv_seq },
            }),
        })
    }

    pub fn primary_header(&self) -> &PrimaryHeader {
        &self.inner.primary_header
    }

    /// The message header.
    pub fn header(&self) -> Header<'_> {
        let quick_fields = self.quick_fields();
        let fields = Fields {
            path: quick_fields.path(self),
            interface: quick_fields.interface(self),
            member: quick_fields.member(self),
            error_name: quick_fields.error_name(self),
            reply_serial: quick_fields.reply_serial(),
            destination: quick_fields.destination(self),
            sender: quick_fields.sender(self),
            signature: Cow::Borrowed(quick_fields.signature()),
            unix_fds: quick_fields.unix_fds(),
        };

        Header::new(self.inner.primary_header.clone(), fields)
    }

    /// The message type.
    pub fn message_type(&self) -> Type {
        self.inner.primary_header.msg_type()
    }

    /// The body that you can deserialize using [`Body::deserialize`].
    ///
    /// # Example
    ///
    /// ```
    /// # use zbus::message::Message;
    /// # (|| -> zbus::Result<()> {
    /// let send_body = (7i32, (2i32, "foo"), vec!["bar"]);
    /// let message = Message::method_call("/", "ping")?
    ///     .destination("zbus.test")?
    ///     .interface("zbus.test")?
    ///     .build(&send_body)?;
    /// let header = message.header();
    /// let body = message.body();
    /// let body: zbus::zvariant::Structure = body.deserialize()?;
    /// let fields = body.fields();
    /// assert!(matches!(fields[0], zvariant::Value::I32(7)));
    /// assert!(matches!(fields[1], zvariant::Value::Structure(_)));
    /// assert!(matches!(fields[2], zvariant::Value::Array(_)));
    ///
    /// let reply_body = Message::method_return(&header)?.build(&body)?.body();
    /// let reply_value : (i32, (i32, &str), Vec<String>) = reply_body.deserialize()?;
    ///
    /// assert_eq!(reply_value.0, 7);
    /// assert_eq!(reply_value.2.len(), 1);
    /// # Ok(()) })().unwrap()
    /// ```
    pub fn body(&self) -> Body {
        Body::new(
            self.inner.bytes.slice(self.inner.body_offset..),
            self.clone(),
        )
    }

    /// Get a reference to the underlying byte encoding of the message.
    pub fn data(&self) -> &serialized::Data<'static, 'static> {
        &self.inner.bytes
    }

    /// Get the receive ordering of a message.
    ///
    /// This may be used to identify how two events were ordered on the bus. It only produces a
    /// useful ordering for messages that were produced by the same [`zbus::Connection`].
    ///
    /// This is completely unrelated to the serial number on the message, which is set by the peer
    /// and might not be ordered at all.
    pub fn recv_position(&self) -> Sequence {
        self.inner.recv_seq
    }

    fn quick_fields(&self) -> &QuickFields {
        self.inner.quick_fields.get_or_init(|| {
            let bytes = &self.inner.bytes;
            // SAFETY: We ensure that by the time `quick_fields` is called, the header has already
            // been checked.
            let (header, _): (Header<'_>, _) = bytes.deserialize().unwrap();

            QuickFields::new(bytes, &header)
        })
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = f.debug_struct("Msg");
        let h = self.header();
        msg.field("type", &h.message_type());
        msg.field("serial", &self.primary_header().serial_num());
        if let Some(sender) = h.sender() {
            msg.field("sender", &sender);
        }
        if let Some(serial) = h.reply_serial() {
            msg.field("reply-serial", &serial);
        }
        if let Some(path) = h.path() {
            msg.field("path", &path);
        }
        if let Some(iface) = h.interface() {
            msg.field("iface", &iface);
        }
        if let Some(member) = h.member() {
            msg.field("member", &member);
        }
        match self.body().signature() {
            zvariant::Signature::Unit => (),
            s => {
                msg.field("body", &s);
            }
        }
        #[cfg(unix)]
        {
            msg.field("fds", &self.data().fds());
        }
        msg.finish()
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = self.header();
        let (ty, error_name, sender, member) = (
            header.message_type(),
            header.error_name(),
            header.sender(),
            header.member(),
        );

        match ty {
            Type::MethodCall => {
                write!(f, "Method call")?;
                if let Some(m) = member {
                    write!(f, " {m}")?;
                }
            }
            Type::MethodReturn => {
                write!(f, "Method return")?;
            }
            Type::Error => {
                write!(f, "Error")?;
                if let Some(e) = error_name {
                    write!(f, " {e}")?;
                }

                let body = self.body();
                let msg = body.deserialize_unchecked::<&str>();
                if let Ok(msg) = msg {
                    write!(f, ": {msg}")?;
                }
            }
            Type::Signal => {
                write!(f, "Signal")?;
                if let Some(m) = member {
                    write!(f, " {m}")?;
                }
            }
        }

        if let Some(s) = sender {
            write!(f, " from {s}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use std::os::fd::{AsFd, AsRawFd};
    use test_log::test;
    #[cfg(unix)]
    use zvariant::Fd;
    use zvariant::Signature;

    use super::Message;
    use crate::Error;

    #[test]
    fn test() {
        #[cfg(unix)]
        let stdout = std::io::stdout();
        let m = Message::method_call("/", "do")
            .unwrap()
            .sender(":1.72")
            .unwrap()
            .build(&(
                #[cfg(unix)]
                Fd::from(&stdout),
                "foo",
            ))
            .unwrap();
        #[cfg(unix)]
        assert_eq!(
            m.body().signature(),
            &Signature::static_structure(&[&Signature::Fd, &Signature::Str]),
        );
        #[cfg(not(unix))]
        assert_eq!(m.body().signature(), &Signature::Str,);
        #[cfg(unix)]
        {
            let fds = m.data().fds();
            assert_eq!(fds.len(), 1);
            // FDs get dup'ed so it has to be a different FD now.
            assert_ne!(fds[0].as_fd().as_raw_fd(), stdout.as_raw_fd());
        }

        let body: Result<u32, Error> = m.body().deserialize();
        assert!(matches!(
            body.unwrap_err(),
            Error::Variant(zvariant::Error::SignatureMismatch { .. })
        ));

        assert_eq!(m.to_string(), "Method call do from :1.72");
        let r = Message::method_return(&m.header())
            .unwrap()
            .build(&("all fine!"))
            .unwrap();
        assert_eq!(r.to_string(), "Method return");
        let e = Message::error(&m.header(), "org.freedesktop.zbus.Error")
            .unwrap()
            .build(&("kaboom!", 32))
            .unwrap();
        assert_eq!(e.to_string(), "Error org.freedesktop.zbus.Error: kaboom!");
    }
}

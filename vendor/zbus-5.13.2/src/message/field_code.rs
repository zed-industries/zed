use serde_repr::{Deserialize_repr, Serialize_repr};

use zvariant::Type;

/// The message field code.
///
/// Every [`Field`] has an associated code. This is mostly an internal D-Bus protocol detail
/// that you would not need to ever care about when using the high-level API. When using the
/// low-level API, this is how you can [retrieve a specific field] from [`Fields`].
///
/// [`Field`]: enum.Field.html
/// [retrieve a specific field]: struct.Fields.html#method.get_field
/// [`Fields`]: struct.Fields.html
#[repr(u8)]
#[derive(Copy, Clone, Debug, Deserialize_repr, PartialEq, Eq, Serialize_repr, Type)]
pub(crate) enum FieldCode {
    /// Code for [`Field::Path`](enum.Field.html#variant.Path).
    Path = 1,
    /// Code for [`Field::Interface`](enum.Field.html#variant.Interface).
    Interface = 2,
    /// Code for [`Field::Member`](enum.Field.html#variant.Member).
    Member = 3,
    /// Code for [`Field::ErrorName`](enum.Field.html#variant.ErrorName).
    ErrorName = 4,
    /// Code for [`Field::ReplySerial`](enum.Field.html#variant.ReplySerial).
    ReplySerial = 5,
    /// Code for [`Field::Destination`](enum.Field.html#variant.Destination).
    Destination = 6,
    /// Code for [`Field::Sender`](enum.Field.html#variant.Sender).
    Sender = 7,
    /// Code for [`Field::Signature`](enum.Field.html#variant.Signature).
    Signature = 8,
    /// Code for [`Field::UnixFDs`](enum.Field.html#variant.UnixFDs).
    UnixFDs = 9,
}

use std::io::Read;
use std::io::Write;

use crate::coded_output_stream::with::WithCodedOutputStream;
use crate::error::ProtobufError;
use crate::wire_format::check_message_size;
use crate::CodedInputStream;
use crate::CodedOutputStream;
use crate::SpecialFields;
use crate::UnknownFields;

/// Trait which is implemented by all generated message.
///
/// Note, by default all generated messages also implement [`MessageFull`](crate::MessageFull)
/// trait which provides access to reflection and features which depend on reflection
/// (text format and JSON serialization).
pub trait Message: Default + Clone + Send + Sync + Sized + PartialEq + 'static {
    /// Message name as specified in `.proto` file.
    ///
    /// Message name can be accessed using
    /// [`MessageFull::descriptor`](crate::MessageFull::descriptor),
    /// but when lite runtime is requested, this field can be used.
    const NAME: &'static str;

    /// True iff all required fields are initialized.
    /// Always returns `true` for protobuf 3.
    fn is_initialized(&self) -> bool;

    /// Update this message object with fields read from given stream.
    fn merge_from(&mut self, is: &mut CodedInputStream) -> crate::Result<()>;

    /// Parse message from stream.
    fn parse_from(is: &mut CodedInputStream) -> crate::Result<Self> {
        let mut r: Self = Message::new();
        r.merge_from(is)?;
        r.check_initialized()?;
        Ok(r)
    }

    /// Write message to the stream.
    ///
    /// Sizes of this messages and nested messages must be cached
    /// by calling `compute_size` prior to this call.
    fn write_to_with_cached_sizes(&self, os: &mut CodedOutputStream) -> crate::Result<()>;

    /// Compute and cache size of this message and all nested messages.
    ///
    /// Note if the computation overflows u32, the cached size is stored truncated.
    fn compute_size(&self) -> u64;

    /// Get size previously computed by `compute_size`.
    ///
    /// Note if message size exceeds u32, the cached size is stored truncated.
    fn cached_size(&self) -> u32 {
        self.special_fields().cached_size().get()
    }

    /// Write the message to the stream.
    ///
    /// Results in error if message is not fully initialized.
    fn write_to(&self, os: &mut CodedOutputStream) -> crate::Result<()> {
        self.check_initialized()?;

        // cache sizes
        let size = self.compute_size();
        let size = check_message_size(size)?;
        os.reserve_additional(size as u32, Self::NAME)?;
        self.write_to_with_cached_sizes(os)?;

        Ok(())
    }

    /// Write the message to the stream prepending the message with message length
    /// encoded as varint.
    fn write_length_delimited_to(&self, os: &mut CodedOutputStream) -> crate::Result<()> {
        let size = self.compute_size();
        let size = check_message_size(size)?;

        os.reserve_additional_for_length_delimited(size, Self::NAME)?;

        os.write_raw_varint32(size)?;

        let written = os.total_bytes_written();

        self.write_to_with_cached_sizes(os)?;

        // Self-check.
        assert_eq!(
            written + size as u64,
            os.total_bytes_written(),
            "Expected to write {}, actually wrote {}",
            size,
            os.total_bytes_written() - written
        );

        Ok(())
    }

    /// Write the message to the vec, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_vec(&self, vec: &mut Vec<u8>) -> crate::Result<()> {
        let mut os = CodedOutputStream::vec(vec);
        self.write_length_delimited_to(&mut os)?;
        os.flush()?;
        Ok(())
    }

    /// Update this message object with fields read from given stream.
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> crate::Result<()> {
        let mut is = CodedInputStream::from_bytes(bytes);
        self.merge_from(&mut is)
    }

    /// Parse message from reader.
    /// Parse stops on EOF or when error encountered.
    fn parse_from_reader(reader: &mut dyn Read) -> crate::Result<Self> {
        let mut is = CodedInputStream::new(reader);
        let r = Message::parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }

    /// Parse message from byte array.
    fn parse_from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        let mut is = CodedInputStream::from_bytes(bytes);
        let r = Message::parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }

    /// Parse message from `Bytes` object.
    /// Resulting message may share references to the passed bytes object.
    #[cfg(feature = "bytes")]
    fn parse_from_tokio_bytes(bytes: &bytes::Bytes) -> crate::Result<Self> {
        let mut is = CodedInputStream::from_tokio_bytes(bytes);
        let r = Self::parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }

    /// Check if all required fields of this object are initialized.
    fn check_initialized(&self) -> crate::Result<()> {
        if !self.is_initialized() {
            Err(ProtobufError::MessageNotInitialized(Self::NAME.to_owned()).into())
        } else {
            Ok(())
        }
    }

    /// Write the message to the writer.
    fn write_to_writer(&self, w: &mut dyn Write) -> crate::Result<()> {
        w.with_coded_output_stream(|os| self.write_to(os))
    }

    /// Write the message to bytes vec.
    fn write_to_vec(&self, v: &mut Vec<u8>) -> crate::Result<()> {
        v.with_coded_output_stream(|os| self.write_to(os))
    }

    /// Write the message to bytes vec.
    ///
    /// > **Note**: You can use [`Message::parse_from_bytes`]
    /// to do the reverse.
    fn write_to_bytes(&self) -> crate::Result<Vec<u8>> {
        self.check_initialized()?;

        let size = self.compute_size() as usize;
        let mut v = Vec::with_capacity(size);
        let mut os = CodedOutputStream::vec(&mut v);
        self.write_to_with_cached_sizes(&mut os)?;
        os.flush()?;
        drop(os);
        Ok(v)
    }

    /// Write the message to the writer, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_writer(&self, w: &mut dyn Write) -> crate::Result<()> {
        w.with_coded_output_stream(|os| self.write_length_delimited_to(os))
    }

    /// Write the message to the bytes vec, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_bytes(&self) -> crate::Result<Vec<u8>> {
        let mut v = Vec::new();
        v.with_coded_output_stream(|os| self.write_length_delimited_to(os))?;
        Ok(v)
    }

    /// Special fields (unknown fields and cached size).
    fn special_fields(&self) -> &SpecialFields;
    /// Special fields (unknown fields and cached size).
    fn mut_special_fields(&mut self) -> &mut SpecialFields;

    /// Get a reference to unknown fields.
    fn unknown_fields(&self) -> &UnknownFields {
        &self.special_fields().unknown_fields()
    }
    /// Get a mutable reference to unknown fields.
    fn mut_unknown_fields(&mut self) -> &mut UnknownFields {
        self.mut_special_fields().mut_unknown_fields()
    }

    /// Create an empty message object.
    ///
    /// ```
    /// # use protobuf::MessageFull;
    /// # fn foo<MyMessage: MessageFull>() {
    /// let m = MyMessage::new();
    /// # }
    /// ```
    fn new() -> Self;

    /// Reset all fields.
    fn clear(&mut self) {
        *self = Self::new();
    }

    /// Return a pointer to default immutable message with static lifetime.
    ///
    /// ```
    /// # use protobuf::MessageFull;
    /// # fn foo<MyMessage: MessageFull>() {
    /// let m: &MyMessage = MyMessage::default_instance();
    /// # }
    /// ```
    fn default_instance() -> &'static Self;
}

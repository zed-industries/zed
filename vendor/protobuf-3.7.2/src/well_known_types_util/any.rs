use crate::message_dyn::MessageDyn;
use crate::reflect::MessageDescriptor;
use crate::well_known_types::any::Any;
use crate::MessageFull;

impl Any {
    fn type_url(type_url_prefix: &str, descriptor: &MessageDescriptor) -> String {
        format!("{}/{}", type_url_prefix, descriptor.full_name())
    }

    fn type_name_from_type_url(type_url: &str) -> Option<&str> {
        match type_url.rfind('/') {
            Some(i) => Some(&type_url[i + 1..]),
            None => None,
        }
    }

    /// Pack any message into `well_known_types::Any` value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use protobuf::MessageFull;
    /// use protobuf::well_known_types::any::Any;
    ///
    /// # fn the_test<MyMessage: MessageFull>(message: &MyMessage) -> protobuf::Result<()> {
    /// let message: &MyMessage = message;
    /// let any = Any::pack(message)?;
    /// assert!(any.is::<MyMessage>());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn pack<M: MessageFull>(message: &M) -> crate::Result<Any> {
        Any::pack_dyn(message)
    }

    /// Pack any message into `well_known_types::Any` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use protobuf::{MessageFull, MessageDyn};
    /// use protobuf::well_known_types::any::Any;
    ///
    /// # fn the_test(message: &dyn MessageDyn) -> protobuf::Result<()> {
    /// let message: &dyn MessageDyn = message;
    /// let any = Any::pack_dyn(message)?;
    /// assert!(any.is_dyn(&message.descriptor_dyn()));
    /// #   Ok(())
    /// # }
    /// ```
    pub fn pack_dyn(message: &dyn MessageDyn) -> crate::Result<Any> {
        Any::pack_with_type_url_prefix(message, "type.googleapis.com")
    }

    fn pack_with_type_url_prefix(
        message: &dyn MessageDyn,
        type_url_prefix: &str,
    ) -> crate::Result<Any> {
        Ok(Any {
            type_url: Any::type_url(type_url_prefix, &message.descriptor_dyn()),
            value: message.write_to_bytes_dyn()?,
            ..Default::default()
        })
    }

    /// Check if `Any` contains a message of given type.
    pub fn is<M: MessageFull>(&self) -> bool {
        self.is_dyn(&M::descriptor())
    }

    /// Check if `Any` contains a message of given type.
    pub fn is_dyn(&self, descriptor: &MessageDescriptor) -> bool {
        match Any::type_name_from_type_url(&self.type_url) {
            Some(type_name) => type_name == descriptor.full_name(),
            None => false,
        }
    }

    /// Extract a message from this `Any`.
    ///
    /// # Returns
    ///
    /// * `Ok(None)` when message type mismatch
    /// * `Err` when parse failed
    pub fn unpack<M: MessageFull>(&self) -> crate::Result<Option<M>> {
        if !self.is::<M>() {
            return Ok(None);
        }
        Ok(Some(M::parse_from_bytes(&self.value)?))
    }

    /// Extract a message from this `Any`.
    ///
    /// # Returns
    ///
    /// * `Ok(None)` when message type mismatch
    /// * `Err` when parse failed
    pub fn unpack_dyn(
        &self,
        descriptor: &MessageDescriptor,
    ) -> crate::Result<Option<Box<dyn MessageDyn>>> {
        if !self.is_dyn(descriptor) {
            return Ok(None);
        }
        let mut message = descriptor.new_instance();
        message.merge_from_bytes_dyn(&self.value)?;
        message.check_initialized_dyn()?;
        Ok(Some(message))
    }
}

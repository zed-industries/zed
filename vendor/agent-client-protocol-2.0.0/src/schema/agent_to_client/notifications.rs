#[cfg(feature = "unstable_elicitation")]
use crate::schema::v1::CompleteElicitationNotification;
use crate::schema::v1::SessionNotification;

impl_jsonrpc_notification!(SessionNotification, "session/update");
#[cfg(feature = "unstable_elicitation")]
impl_jsonrpc_notification!(CompleteElicitationNotification, "elicitation/complete");

use crate::schema::v1::CancelNotification;

impl_jsonrpc_notification!(CancelNotification, "session/cancel");

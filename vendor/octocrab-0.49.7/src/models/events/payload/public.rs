use serde::{Deserialize, Serialize};

/// The payload in a [`super::EventPayload::PublicEvent`] type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PublicEventPayload {}

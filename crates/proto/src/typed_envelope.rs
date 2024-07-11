use crate::{PeerId, RequestMessage};
use anyhow::{anyhow, Result};
use std::{marker::PhantomData, time::Instant};

pub struct Receipt<T> {
    pub sender_id: PeerId,
    pub message_id: u32,
    payload_type: PhantomData<T>,
}

impl<T> Clone for Receipt<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Receipt<T> {}

#[derive(Clone, Debug)]
pub struct TypedEnvelope<T> {
    pub sender_id: PeerId,
    pub original_sender_id: Option<PeerId>,
    pub message_id: u32,
    pub payload: T,
    pub received_at: Instant,
}

impl<T> TypedEnvelope<T> {
    pub fn original_sender_id(&self) -> Result<PeerId> {
        self.original_sender_id
            .ok_or_else(|| anyhow!("missing original_sender_id"))
    }
}

impl<T: RequestMessage> TypedEnvelope<T> {
    pub fn receipt(&self) -> Receipt<T> {
        Receipt {
            sender_id: self.sender_id,
            message_id: self.message_id,
            payload_type: PhantomData,
        }
    }
}

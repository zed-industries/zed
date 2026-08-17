use bytes::Bytes;

use std::collections::vec_deque::{Iter, VecDeque};
use std::convert::{From, TryFrom};
use std::fmt;

#[derive(Debug)]
pub struct ZmqEmptyMessageError;

impl fmt::Display for ZmqEmptyMessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to construct an empty ZmqMessage")
    }
}

#[derive(Debug, Clone)]
pub struct ZmqMessage {
    frames: VecDeque<Bytes>,
}

impl ZmqMessage {
    pub fn push_back(&mut self, frame: Bytes) {
        self.frames.push_back(frame);
    }

    pub fn push_front(&mut self, frame: Bytes) {
        self.frames.push_front(frame);
    }

    pub fn iter(&self) -> Iter<'_, Bytes> {
        self.frames.iter()
    }

    pub(crate) fn pop_front(&mut self) -> Option<Bytes> {
        self.frames.pop_front()
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Bytes> {
        self.frames.get(index)
    }

    pub fn into_vec(self) -> Vec<Bytes> {
        Vec::from(self.frames)
    }

    pub fn into_vecdeque(self) -> VecDeque<Bytes> {
        self.frames
    }

    pub fn prepend(&mut self, message: &ZmqMessage) {
        for frame in message.iter().rev() {
            self.push_front(frame.clone());
        }
    }

    pub fn split_off(&mut self, at: usize) -> ZmqMessage {
        let frames = self.frames.split_off(at);
        ZmqMessage { frames }
    }
}

impl TryFrom<Vec<Bytes>> for ZmqMessage {
    type Error = ZmqEmptyMessageError;
    fn try_from(v: Vec<Bytes>) -> Result<Self, Self::Error> {
        if v.is_empty() {
            Err(ZmqEmptyMessageError)
        } else {
            Ok(Self { frames: v.into() })
        }
    }
}

impl TryFrom<VecDeque<Bytes>> for ZmqMessage {
    type Error = ZmqEmptyMessageError;
    fn try_from(v: VecDeque<Bytes>) -> Result<Self, Self::Error> {
        if v.is_empty() {
            Err(ZmqEmptyMessageError)
        } else {
            Ok(Self { frames: v })
        }
    }
}

impl From<Vec<u8>> for ZmqMessage {
    fn from(v: Vec<u8>) -> Self {
        ZmqMessage::from(Bytes::from(v))
    }
}

impl From<Bytes> for ZmqMessage {
    fn from(b: Bytes) -> Self {
        Self {
            frames: vec![b].into(),
        }
    }
}

impl From<String> for ZmqMessage {
    fn from(s: String) -> Self {
        let b: Bytes = s.into();
        ZmqMessage::from(b)
    }
}

impl From<&str> for ZmqMessage {
    fn from(s: &str) -> Self {
        ZmqMessage::from(s.to_owned())
    }
}

impl TryFrom<ZmqMessage> for Vec<u8> {
    type Error = &'static str;

    fn try_from(z: ZmqMessage) -> Result<Self, Self::Error> {
        if z.len() != 1 {
            return Err("Message must have only 1 frame to convert to Vec<u8>");
        }
        Ok(z.into_vecdeque().pop_front().unwrap().to_vec())
    }
}

impl TryFrom<ZmqMessage> for String {
    type Error = &'static str;

    fn try_from(z: ZmqMessage) -> Result<Self, Self::Error> {
        if z.len() != 1 {
            return Err("Message must have only 1 frame to convert to String");
        }
        match String::from_utf8(z.into_vecdeque().pop_front().unwrap().to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => Err("Could not parse string from message"),
        }
    }
}

// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{fmt::Debug, str::Utf8Error};

use serde::Deserialize;
use thiserror::Error;

use crate::{imp::data_channel as dc_imp, rtp_parameters::Priority};

#[derive(Clone, Debug)]
pub struct DataChannelInit {
    pub ordered: bool,
    pub max_retransmit_time: Option<i32>,
    pub max_retransmits: Option<i32>,
    pub protocol: String,
    pub negotiated: bool,
    pub id: i32,
    pub priority: Option<Priority>,
}

impl Default for DataChannelInit {
    fn default() -> Self {
        Self {
            ordered: true,
            max_retransmit_time: None,
            max_retransmits: None,
            protocol: String::new(),
            negotiated: false,
            id: -1,
            priority: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum DataChannelError {
    #[error("failed to send data, dc not open? send buffer is full ?")]
    Send,
    #[error("only utf8 strings can be sent")]
    Utf8(#[from] Utf8Error),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataChannelState {
    Connecting,
    Open,
    Closing,
    Closed,
}

#[derive(Debug)]
pub struct DataBuffer<'a> {
    pub data: &'a [u8],
    pub binary: bool,
}

pub type OnStateChange = Box<dyn FnMut(DataChannelState) + Send + Sync>;
pub type OnMessage = Box<dyn FnMut(DataBuffer) + Send + Sync>;
pub type OnBufferedAmountChange = Box<dyn FnMut(u64) + Send + Sync>;

#[derive(Clone)]
pub struct DataChannel {
    pub(crate) handle: dc_imp::DataChannel,
}

impl DataChannel {
    pub fn send(&self, data: &[u8], binary: bool) -> Result<(), DataChannelError> {
        self.handle.send(data, binary)
    }

    pub fn id(&self) -> i32 {
        self.handle.id()
    }

    pub fn label(&self) -> String {
        self.handle.label()
    }

    pub fn state(&self) -> DataChannelState {
        self.handle.state()
    }

    pub fn close(&self) {
        self.handle.close()
    }

    pub fn buffered_amount(&self) -> u64 {
        self.handle.buffered_amount()
    }

    pub fn on_state_change(&self, callback: Option<OnStateChange>) {
        self.handle.on_state_change(callback)
    }

    pub fn on_message(&self, callback: Option<OnMessage>) {
        self.handle.on_message(callback)
    }

    pub fn on_buffered_amount_change(&self, callback: Option<OnBufferedAmountChange>) {
        self.handle.on_buffered_amount_change(callback)
    }
}

impl Debug for DataChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataChannel")
            .field("id", &self.id())
            .field("label", &self.label())
            .field("state", &self.state())
            .finish()
    }
}

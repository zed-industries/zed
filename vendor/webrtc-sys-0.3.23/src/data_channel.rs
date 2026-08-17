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

use std::sync::Arc;

use crate::impl_thread_safety;

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {
    #[derive(Debug)]
    #[repr(i32)]
    pub enum Priority {
        VeryLow,
        Low,
        Medium,
        High,
    }

    #[derive(Debug)]
    pub struct DataChannelInit {
        pub ordered: bool,
        pub has_max_retransmit_time: bool,
        pub max_retransmit_time: i32,
        pub has_max_retransmits: bool,
        pub max_retransmits: i32,
        pub protocol: String,
        pub negotiated: bool,
        pub id: i32,
        pub has_priority: bool,
        pub priority: Priority,
    }

    #[derive(Debug)]
    pub struct DataBuffer {
        pub ptr: *const u8,
        pub len: usize,
        pub binary: bool,
    }

    #[derive(Debug)]
    #[repr(i32)]
    pub enum DataState {
        Connecting,
        Open,
        Closing,
        Closed,
    }

    unsafe extern "C++" {
        include!("livekit/data_channel.h");

        type DataChannel;

        fn register_observer(self: &DataChannel, observer: Box<DataChannelObserverWrapper>);
        fn unregister_observer(self: &DataChannel);

        fn send(self: &DataChannel, data: &DataBuffer) -> bool;
        fn id(self: &DataChannel) -> i32;
        fn label(self: &DataChannel) -> String;
        fn state(self: &DataChannel) -> DataState;
        fn close(self: &DataChannel);
        fn buffered_amount(self: &DataChannel) -> u64;

        fn _shared_data_channel() -> SharedPtr<DataChannel>; // Ignore
    }

    extern "Rust" {
        type DataChannelObserverWrapper;

        fn on_state_change(self: &DataChannelObserverWrapper, state: DataState);
        fn on_message(self: &DataChannelObserverWrapper, buffer: DataBuffer);
        fn on_buffered_amount_change(self: &DataChannelObserverWrapper, sent_data_size: u64);
    }
}

impl_thread_safety!(ffi::DataChannel, Send + Sync);

pub trait DataChannelObserver: Send + Sync {
    fn on_state_change(&self, state: ffi::DataState);
    fn on_message(&self, data: &[u8], is_binary: bool);
    fn on_buffered_amount_change(&self, sent_data_size: u64);
}

pub struct DataChannelObserverWrapper {
    observer: Arc<dyn DataChannelObserver>,
}

impl DataChannelObserverWrapper {
    pub fn new(observer: Arc<dyn DataChannelObserver>) -> Self {
        Self { observer }
    }

    fn on_state_change(&self, state: ffi::DataState) {
        self.observer.on_state_change(state);
    }

    fn on_message(&self, buffer: ffi::DataBuffer) {
        unsafe {
            let data = std::slice::from_raw_parts(buffer.ptr, buffer.len);
            self.observer.on_message(data, buffer.binary);
        }
    }

    fn on_buffered_amount_change(&self, sent_data_size: u64) {
        self.observer.on_buffered_amount_change(sent_data_size);
    }
}

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

use core::str;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, RtcDataChannelEvent, RtcDataChannelState};

use crate::data_channel::{
    DataChannelError, DataChannelTrait, DataState, OnBufferedAmountChange, OnMessage, OnStateChange,
};

impl From<RtcDataChannelState> for DataState {
    fn from(value: RtcDataChannelState) -> Self {
        match value {
            RtcDataChannelState::Connecting => Self::Connecting,
            RtcDataChannelState::Open => Self::Open,
            RtcDataChannelState::Closing => Self::Closing,
            RtcDataChannelState::Closed => Self::Closed,
            _ => panic!("unknown data channel state"),
        }
    }
}

#[derive(Clone)]
pub struct DataChannel {
    sys_handle: web_sys::RtcDataChannel,
    on_closing: Rc<RefCell<Option<JsValue>>>,
}

impl DataChannelTrait for DataChannel {
    fn send(&self, data: &[u8], binary: bool) -> Result<(), DataChannelError> {
        if binary {
            self.sys_handle
                .send_with_u8_array(data)
                .map_err(|_| DataChannelError::Send)
        } else {
            let utf8 = str::from_utf8(data)?;
            self.sys_handle
                .send_with_str(utf8)
                .map_err(|_| DataChannelError::Send)
        }
    }

    fn label(&self) -> String {
        self.sys_handle.label()
    }

    fn state(&self) -> DataState {
        self.sys_handle.ready_state().into()
    }

    fn close(&self) {
        self.sys_handle.close();
    }

    fn on_state_change(&self, callback: Option<OnStateChange>) {
        if let Some(mut callback) = callback {
            let dc = self.clone();
            let js_callback = Closure::new(move |_: RtcDataChannelEvent| {
                callback(dc.state());
            });
            let js_callback = js_callback.into_js_value();
            self.sys_handle
                .set_onopen(Some(js_callback.unchecked_ref()));
            self.sys_handle
                .set_onclose(Some(js_callback.unchecked_ref()));
            self.sys_handle
                .add_event_listener_with_callback("closing", js_callback.unchecked_ref())
                .unwrap();

            self.on_closing.replace(Some(js_callback));
        } else {
            self.sys_handle.set_onopen(None);
            self.sys_handle.set_onclose(None);
            if let Some(on_closing) = self.on_closing.take() {
                self.sys_handle
                    .remove_event_listener_with_callback("closing", on_closing.unchecked_ref())
                    .unwrap();
            }
            self.on_closing.replace(None);
        }
    }

    fn on_message(&self, callback: Option<OnMessage>) {
        let js_callback = callback.map(|mut callback| {
            Closure::new(move |event: MessageEvent| {
                if let Some(str) = event.as_string() {
                    callback(str.as_bytes(), false);
                }
            })
            .into_js_value()
        });

        self.sys_handle.set_onmessage(
            js_callback
                .as_ref()
                .map(|callback| callback.unchecked_ref()),
        );
    }

    fn on_buffered_amount_change(&self, _callback: Option<OnBufferedAmountChange>) {
        todo!("onbufferedamountlow instead?")
    }
}

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

use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Weak},
};

use lazy_static::lazy_static;
use libwebrtc::prelude::*;
use parking_lot::Mutex;

lazy_static! {
    static ref LK_RUNTIME: Mutex<Weak<LkRuntime>> = Mutex::new(Weak::new());
}

pub struct LkRuntime {
    pc_factory: PeerConnectionFactory,
}

impl Debug for LkRuntime {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("LkRuntime").finish()
    }
}

impl LkRuntime {
    pub fn instance() -> Arc<LkRuntime> {
        let mut lk_runtime_ref = LK_RUNTIME.lock();
        if let Some(lk_runtime) = lk_runtime_ref.upgrade() {
            lk_runtime
        } else {
            log::debug!("LkRuntime::new()");
            let new_runtime = Arc::new(Self { pc_factory: PeerConnectionFactory::default() });
            *lk_runtime_ref = Arc::downgrade(&new_runtime);
            new_runtime
        }
    }

    pub fn pc_factory(&self) -> &PeerConnectionFactory {
        &self.pc_factory
    }
}

impl Drop for LkRuntime {
    fn drop(&mut self) {
        log::debug!("LkRuntime::drop()");
    }
}

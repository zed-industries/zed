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

#[cfg(any(
    all(feature = "tokio", feature = "async"),
    all(feature = "tokio", feature = "dispatcher"),
    all(feature = "dispatcher", feature = "async")
))]
compile_error!("Cannot compile livekit with multiple runtimes");

#[cfg(feature = "tokio")]
mod tokio;
#[cfg(feature = "tokio")]
pub use tokio::*;

#[cfg(feature = "async")]
mod async_std;
#[cfg(feature = "async")]
pub use async_std::*;

#[cfg(feature = "dispatcher")]
mod dispatcher;
#[cfg(feature = "dispatcher")]
pub use dispatcher::*;

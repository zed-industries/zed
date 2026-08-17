// Copyright 2025 FastLabs Developers
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

#![doc(hidden)]

pub use stacker;

#[cfg(debug_assertions)]
thread_local! {
    static PROTECTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[inline(always)]
pub fn is_protected() -> bool {
    #[cfg(debug_assertions)]
    {
        PROTECTED.get()
    }

    #[cfg(not(debug_assertions))]
    {
        true
    }
}

#[inline(always)]
pub fn with_protected<R>(callback: impl FnOnce() -> R) -> impl FnOnce() -> R {
    move || {
        #[cfg(debug_assertions)]
        {
            let old = PROTECTED.replace(true);
            let ret = callback();
            PROTECTED.set(old);
            ret
        }

        #[cfg(not(debug_assertions))]
        {
            callback()
        }
    }
}

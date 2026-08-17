// Copyright 2021 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

mod context;
mod filters;
mod node;
mod text;
mod util;
mod window_handle;

mod adapter;
pub use adapter::{Adapter, QueuedEvents};

mod subclass;
pub use subclass::SubclassingAdapter;

pub use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

#[cfg(test)]
mod tests;

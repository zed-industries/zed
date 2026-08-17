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

use parking_lot::RwLock;
use std::{fmt::Debug, sync::Arc};

/// A thread-safe cell that holds a value which can be taken exactly once.
/// After taking the value, subsequent attempts to take it will return `None`.
pub struct TakeCell<T> {
    value: Arc<RwLock<Option<T>>>,
}

impl<T> TakeCell<T> {
    pub(crate) fn new(value: T) -> Self {
        Self { value: Arc::new(RwLock::new(Some(value))) }
    }

    /// Take ownership of the value in the cell if it matches some predicate.
    ///
    /// This method will only take the value if the provided predicate returns `true` when called with the current value.
    /// If the predicate returns `false` or the value has already been taken, this method returns `None`.
    pub(crate) fn take_if_raw(&self, predicate: impl FnOnce(&T) -> bool) -> Option<T> {
        if self.value.read().as_ref().map_or(false, |v| predicate(v)) {
            self.take()
        } else {
            None
        }
    }

    /// Take ownership of the value in the cell. If the value has,
    /// already been taken, the result is `None`.
    pub fn take(&self) -> Option<T> {
        self.value.write().take()
    }

    /// Returns whether or not the value has been taken.
    pub fn is_taken(&self) -> bool {
        self.value.read().is_none()
    }
}

impl<T> Debug for TakeCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.value.read();
        f.debug_struct("TakeCell")
            .field(
                "value",
                &inner.as_ref().map(|v| v as &dyn Debug).unwrap_or(&"<taken>" as &dyn Debug),
            )
            .finish()
    }
}

impl<T> Clone for TakeCell<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_take() {
        let cell = TakeCell::new(1);
        assert_eq!(cell.is_taken(), false);
        assert_eq!(cell.take(), Some(1));
        assert_eq!(cell.take(), None);
        assert_eq!(cell.is_taken(), true);
    }

    #[test]
    fn test_take_if_raw() {
        let cell = TakeCell::new(1);
        assert_eq!(cell.take_if_raw(|value| *value == 2), None);
        assert_eq!(cell.take_if_raw(|value| *value == 1), Some(1));
        assert_eq!(cell.take_if_raw(|value| *value == 1), None);
    }

    #[test]
    fn test_debug() {
        let cell = TakeCell::new(1);
        assert_eq!(format!("{:?}", cell), "TakeCell { value: 1 }");

        cell.take();
        assert_eq!(format!("{:?}", cell), "TakeCell { value: \"<taken>\" }");
    }
}

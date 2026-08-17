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

use tokio::sync::{oneshot, Mutex, RwLock};

pub struct Promise<T> {
    tx: Mutex<Option<oneshot::Sender<T>>>,
    rx: Mutex<Option<oneshot::Receiver<T>>>,
    result: RwLock<Option<T>>,
}

impl<T: Clone> Promise<T> {
    pub fn new() -> Self {
        let (tx, rx) = oneshot::channel();
        Self { tx: Mutex::new(Some(tx)), rx: Mutex::new(Some(rx)), result: Default::default() }
    }

    pub fn resolve(&self, result: T) -> Result<(), &'static str> {
        let mut tx = self.tx.try_lock().unwrap();
        if tx.is_some() {
            let _ = tx.take().unwrap().send(result);
            Ok(())
        } else {
            Err("promise already used")
        }
    }

    pub async fn result(&self) -> T {
        {
            let result_read = self.result.read().await;
            if let Some(result) = result_read.clone() {
                return result;
            }
        }

        let mut rx = self.rx.lock().await;
        if let Some(rx) = rx.take() {
            let result = rx.await.unwrap();
            *self.result.write().await = Some(result.clone());
            result
        } else {
            self.result.read().await.clone().unwrap()
        }
    }

    pub fn try_result(&self) -> Option<T> {
        self.result.try_read().ok().and_then(|result| result.clone())
    }
}

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
    collections::HashMap,
    ffi::{c_char, c_void, CString},
    pin::Pin,
    sync::{Arc, LazyLock},
    task::{Context, Poll},
    time::Duration,
};

use futures_util::Stream;
use libloading::{Library, Symbol};
use libwebrtc::{audio_stream::native::NativeAudioStream, prelude::AudioFrame};
use parking_lot::RwLock;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("dylib error: {0}")]
    Library(#[from] libloading::Error),
    #[error("dylib error: {0}")]
    NotImplemented(String),
    #[error("on_load failed with error: {0}")]
    OnLoad(i32),
}

type OnLoadFn = unsafe extern "C" fn(options: *const c_char) -> i32;
type CreateFn = unsafe extern "C" fn(
    sampling_rate: u32,
    options: *const c_char,
    stream_info: *const c_char,
) -> *mut c_void;
type DestroyFn = unsafe extern "C" fn(*const c_void);
type ProcessI16Fn = unsafe extern "C" fn(*const c_void, usize, *const i16, *mut i16);
type ProcessF32Fn = unsafe extern "C" fn(*const c_void, usize, *const f32, *mut f32);
type UpdateStreamInfoFn = unsafe extern "C" fn(*const c_void, *const c_char);
type UpdateRefreshedTokenFn = unsafe extern "C" fn(*const c_char, *const c_char);

static REGISTERED_PLUGINS: LazyLock<RwLock<HashMap<String, Arc<AudioFilterPlugin>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn register_audio_filter_plugin(id: String, plugin: Arc<AudioFilterPlugin>) {
    REGISTERED_PLUGINS.write().insert(id, plugin);
}

pub fn registered_audio_filter_plugin(id: &str) -> Option<Arc<AudioFilterPlugin>> {
    REGISTERED_PLUGINS.read().get(id).cloned()
}

pub fn registered_audio_filter_plugins() -> Vec<Arc<AudioFilterPlugin>> {
    REGISTERED_PLUGINS.read().values().map(|v| v.clone()).collect()
}

pub struct AudioFilterPlugin {
    lib: Library,
    dependencies: Vec<Library>,
    on_load_fn_ptr: *const c_void,
    create_fn_ptr: *const c_void,
    destroy_fn_ptr: *const c_void,
    process_i16_fn_ptr: *const c_void,
    process_f32_fn_ptr: *const c_void,
    update_stream_info_fn_ptr: *const c_void,
    update_token_fn_ptr: *const c_void,
}

impl AudioFilterPlugin {
    pub fn new<P: AsRef<str>>(path: P) -> Result<Arc<Self>, PluginError> {
        Ok(Arc::new(Self::_new(path)?))
    }

    pub fn new_with_dependencies<P: AsRef<str>>(
        path: P,
        dependencies: Vec<P>,
    ) -> Result<Arc<Self>, PluginError> {
        let mut libs = vec![];
        for path in dependencies {
            let lib = unsafe { Library::new(path.as_ref()) }?;
            libs.push(lib);
        }
        let mut this = Self::_new(path)?;
        this.dependencies = libs;
        Ok(Arc::new(this))
    }

    fn _new<P: AsRef<str>>(path: P) -> Result<Self, PluginError> {
        let lib = unsafe { Library::new(path.as_ref()) }?;

        let on_load_fn_ptr = unsafe {
            lib.get::<Symbol<OnLoadFn>>(b"audio_filter_on_load")?.try_as_raw_ptr().unwrap()
        };

        let create_fn_ptr = unsafe {
            lib.get::<Symbol<CreateFn>>(b"audio_filter_create")?.try_as_raw_ptr().unwrap()
        };
        if create_fn_ptr.is_null() {
            return Err(PluginError::NotImplemented(
                "audio_filter_create is not implemented".into(),
            ));
        }
        let destroy_fn_ptr = unsafe {
            lib.get::<Symbol<DestroyFn>>(b"audio_filter_destroy")?.try_as_raw_ptr().unwrap()
        };
        if destroy_fn_ptr.is_null() {
            return Err(PluginError::NotImplemented(
                "audio_filter_destroy is not implemented".into(),
            ));
        }
        let process_i16_fn_ptr = unsafe {
            lib.get::<Symbol<ProcessI16Fn>>(b"audio_filter_process_int16")?
                .try_as_raw_ptr()
                .unwrap()
        };
        if process_i16_fn_ptr.is_null() {
            return Err(PluginError::NotImplemented(
                "audio_filter_process_int16 is not implemented".into(),
            ));
        }
        let process_f32_fn_ptr = unsafe {
            lib.get::<Symbol<ProcessF32Fn>>(b"audio_filter_process_float")?
                .try_as_raw_ptr()
                .unwrap()
        };
        let update_stream_info_fn_ptr = unsafe {
            lib.get::<Symbol<UpdateStreamInfoFn>>(b"audio_filter_update_stream_info")?
                .try_as_raw_ptr()
                .unwrap()
        };
        let update_token_fn_ptr = unsafe {
            // treat as optional function for now
            match lib.get::<Symbol<UpdateRefreshedTokenFn>>(b"audio_filter_update_token") {
                Ok(sym) => sym.try_as_raw_ptr().unwrap(),
                Err(_) => std::ptr::null(),
            }
        };

        Ok(Self {
            lib,
            dependencies: Default::default(),
            on_load_fn_ptr,
            create_fn_ptr,
            destroy_fn_ptr,
            process_i16_fn_ptr,
            process_f32_fn_ptr,
            update_stream_info_fn_ptr,
            update_token_fn_ptr,
        })
    }

    pub fn on_load<S: AsRef<str>>(&self, url: S, token: S) -> Result<(), PluginError> {
        if self.on_load_fn_ptr.is_null() {
            // on_load is optional function
            return Ok(());
        }

        let options_json = json!({
            "url": url.as_ref().to_string(),
            "token": token.as_ref().to_string(),
        });
        let options = serde_json::to_string(&options_json).map_err(|e| {
            eprintln!("failed to serialize option: {}", e);
            PluginError::OnLoad(-1)
        })?;

        let options = CString::new(options).unwrap_or(CString::new("").unwrap());
        let on_load_fn: OnLoadFn = unsafe { std::mem::transmute(self.on_load_fn_ptr) };

        let res = unsafe { on_load_fn(options.as_ptr()) };
        if res == 0 {
            Ok(())
        } else {
            Err(PluginError::OnLoad(res))
        }
    }

    pub fn update_token(&self, url: String, token: String) {
        if self.update_token_fn_ptr.is_null() {
            return;
        }
        let update_token_fn: UpdateRefreshedTokenFn =
            unsafe { std::mem::transmute(self.update_token_fn_ptr) };
        let url = CString::new(url).unwrap();
        let token = CString::new(token).unwrap();
        unsafe { update_token_fn(url.as_ptr(), token.as_ptr()) }
    }

    pub fn new_session<S: AsRef<str>>(
        self: Arc<Self>,
        sampling_rate: u32,
        options: S,
        stream_info: AudioFilterStreamInfo,
    ) -> Option<AudioFilterSession> {
        let create_fn: CreateFn = unsafe { std::mem::transmute(self.create_fn_ptr) };

        let options = CString::new(options.as_ref()).unwrap_or(CString::new("").unwrap());

        let stream_info = serde_json::to_string(&stream_info).unwrap();
        let stream_info = CString::new(stream_info).unwrap_or(CString::new("").unwrap());

        let ptr = unsafe { create_fn(sampling_rate, options.as_ptr(), stream_info.as_ptr()) };
        if ptr.is_null() {
            return None;
        }

        Some(AudioFilterSession { plugin: self.clone(), ptr })
    }
}

pub struct AudioFilterSession {
    plugin: Arc<AudioFilterPlugin>,
    ptr: *const c_void,
}

impl AudioFilterSession {
    pub fn destroy(&self) {
        let destroy: DestroyFn = unsafe { std::mem::transmute(self.plugin.destroy_fn_ptr) };
        unsafe { destroy(self.ptr) };
    }

    pub fn process_i16(&self, num_samples: usize, input: &[i16], output: &mut [i16]) {
        let process: ProcessI16Fn = unsafe { std::mem::transmute(self.plugin.process_i16_fn_ptr) };
        unsafe { process(self.ptr, num_samples, input.as_ptr(), output.as_mut_ptr()) };
    }

    pub fn process_f32(&self, num_samples: usize, input: &[f32], output: &mut [f32]) {
        let process: ProcessF32Fn = unsafe { std::mem::transmute(self.plugin.process_f32_fn_ptr) };
        unsafe { process(self.ptr, num_samples, input.as_ptr(), output.as_mut_ptr()) };
    }

    pub fn update_stream_info(&self, info: AudioFilterStreamInfo) {
        if self.plugin.update_stream_info_fn_ptr.is_null() {
            return;
        }
        let update_stream_info_fn: UpdateStreamInfoFn =
            unsafe { std::mem::transmute(self.plugin.update_stream_info_fn_ptr) };
        let info_json = serde_json::to_string(&info).unwrap();
        let info_json = CString::new(info_json).unwrap_or(CString::new("").unwrap());
        unsafe { update_stream_info_fn(self.ptr, info_json.as_ptr()) }
    }
}

impl Drop for AudioFilterSession {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.destroy();
        }
    }
}

pub struct AudioFilterAudioStream {
    inner: NativeAudioStream,
    session: AudioFilterSession,
    buffer: Vec<i16>,
    sample_rate: u32,
    num_channels: u32,
    frame_size: usize,
}

impl AudioFilterAudioStream {
    pub fn new(
        inner: NativeAudioStream,
        session: AudioFilterSession,
        duration: Duration,
        sample_rate: u32,
        num_channels: u32,
    ) -> Self {
        let frame_size =
            ((sample_rate as f64) * duration.as_secs_f64() * num_channels as f64) as usize;
        Self {
            inner,
            session,
            buffer: Vec::with_capacity(frame_size),
            sample_rate,
            num_channels,
            frame_size,
        }
    }

    pub fn update_stream_info(&mut self, info: AudioFilterStreamInfo) {
        self.session.update_stream_info(info);
    }
}

impl Stream for AudioFilterAudioStream {
    type Item = AudioFrame<'static>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        while let Poll::Ready(frame) = Pin::new(&mut this.inner).poll_next(cx) {
            let Some(frame) = frame else {
                return Poll::Ready(None);
            };
            this.buffer.extend_from_slice(&frame.data);

            if this.buffer.len() >= this.frame_size {
                let data = this.buffer.drain(..this.frame_size).collect::<Vec<_>>();
                let mut out: Vec<i16> = vec![0; this.frame_size];

                this.session.process_i16(this.frame_size, &data, &mut out);

                return Poll::Ready(Some(AudioFrame {
                    data: out.into(),
                    sample_rate: this.sample_rate,
                    num_channels: this.num_channels,
                    samples_per_channel: (this.frame_size / this.num_channels as usize) as u32,
                }));
            }
        }

        Poll::Pending
    }
}

#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioFilterStreamInfo {
    pub url: String,
    pub room_id: String,
    pub room_name: String,
    pub participant_identity: String,
    pub participant_id: String,
    pub track_id: String,
}

// The function pointers in this struct are initialized only once during construction
// and remain read-only throughout the lifetime of the struct, ensuring thread safety.
unsafe impl Send for AudioFilterPlugin {}
unsafe impl Sync for AudioFilterPlugin {}
unsafe impl Send for AudioFilterSession {}
unsafe impl Sync for AudioFilterSession {}

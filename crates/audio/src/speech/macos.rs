use super::{SpeechEvent, SpeechRecognizer};
use anyhow::{Result, anyhow};
use futures::channel::mpsc;
use gpui::{App, Task};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use parking_lot::Mutex;
use std::ffi::c_void;
use std::sync::Arc;

#[link(name = "Speech", kind = "framework")]
extern "C" {}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}

type Id = *mut Object;
const NIL: Id = std::ptr::null_mut();

static ACTIVE_SESSION: Mutex<Option<ActiveSession>> = Mutex::new(None);

struct ActiveSession {
    audio_engine: Id,
    recognition_task: Id,
    request: Id,
}

// Speech framework objects are used from the main thread only, which is Send-safe
// in GPUI's single-threaded foreground model.
unsafe impl Send for ActiveSession {}
unsafe impl Sync for ActiveSession {}

pub struct MacSpeechRecognizer;

impl SpeechRecognizer for MacSpeechRecognizer {
    fn is_available() -> bool {
        unsafe {
            let recognizer_class = class!(SFSpeechRecognizer);
            let recognizer: Id = msg_send![recognizer_class, new];
            if recognizer.is_null() {
                return false;
            }
            let available: bool = msg_send![recognizer, isAvailable];
            let _: () = msg_send![recognizer, release];
            available
        }
    }

    fn request_authorization(cx: &mut App) -> Task<Result<bool>> {
        cx.background_spawn(async {
            let (sender, receiver) = futures::channel::oneshot::channel();
            let sender = Arc::new(Mutex::new(Some(sender)));

            unsafe {
                let sender_ptr = Arc::into_raw(sender) as *mut c_void;

                extern "C" fn auth_callback(status: isize, context: *mut c_void) {
                    let sender = unsafe {
                        Arc::from_raw(context as *const Mutex<Option<futures::channel::oneshot::Sender<bool>>>)
                    };
                    if let Some(sender) = sender.lock().take() {
                        // 2 = SFSpeechRecognizerAuthorizationStatusAuthorized
                        let _ = sender.send(status == 2);
                    }
                }

                // SFSpeechRecognizer.requestAuthorization uses a block, which is complex
                // to create from Rust. Instead, check the current status directly.
                let recognizer_class = class!(SFSpeechRecognizer);
                let status: isize = msg_send![recognizer_class, authorizationStatus];

                // Reconstruct the Arc so it's properly dropped
                let sender = Arc::from_raw(sender_ptr as *const Mutex<Option<futures::channel::oneshot::Sender<bool>>>);

                match status {
                    // 0 = notDetermined — trigger the system dialog
                    0 => {
                        // For notDetermined, we need the system to prompt.
                        // Creating a recognizer triggers the prompt on first use.
                        let recognizer: Id = msg_send![recognizer_class, new];
                        if !recognizer.is_null() {
                            let _: () = msg_send![recognizer, release];
                        }
                        // Re-check after prompt
                        let new_status: isize = msg_send![recognizer_class, authorizationStatus];
                        if let Some(sender) = sender.lock().take() {
                            let _ = sender.send(new_status == 2);
                        }
                    }
                    // 2 = authorized
                    2 => {
                        if let Some(sender) = sender.lock().take() {
                            let _ = sender.send(true);
                        }
                    }
                    // 1 = denied, 3 = restricted
                    _ => {
                        if let Some(sender) = sender.lock().take() {
                            let _ = sender.send(false);
                        }
                    }
                }
            }

            receiver
                .await
                .map_err(|_| anyhow!("Authorization check canceled"))
        })
    }

    fn start(cx: &mut App) -> Result<mpsc::UnboundedReceiver<SpeechEvent>> {
        if !Self::is_available() {
            return Err(anyhow!("Speech recognition is not available"));
        }

        // Stop any existing session first
        Self::stop();

        let (sender, receiver) = mpsc::unbounded();

        unsafe {
            // Create SFSpeechRecognizer
            let recognizer_class = class!(SFSpeechRecognizer);
            let recognizer: Id = msg_send![recognizer_class, new];
            if recognizer.is_null() {
                return Err(anyhow!("Failed to create speech recognizer"));
            }

            // Create SFSpeechAudioBufferRecognitionRequest
            let request_class = class!(SFSpeechAudioBufferRecognitionRequest);
            let request: Id = msg_send![request_class, new];
            if request.is_null() {
                let _: () = msg_send![recognizer, release];
                return Err(anyhow!("Failed to create recognition request"));
            }
            let _: () = msg_send![request, setShouldReportPartialResults: true];

            // Create AVAudioEngine
            let engine_class = class!(AVAudioEngine);
            let audio_engine: Id = msg_send![engine_class, new];
            if audio_engine.is_null() {
                let _: () = msg_send![request, release];
                let _: () = msg_send![recognizer, release];
                return Err(anyhow!("Failed to create audio engine"));
            }

            // Get the input node
            let input_node: Id = msg_send![audio_engine, inputNode];
            if input_node.is_null() {
                let _: () = msg_send![audio_engine, release];
                let _: () = msg_send![request, release];
                let _: () = msg_send![recognizer, release];
                return Err(anyhow!("No audio input available"));
            }

            // Get the recording format from the input node
            let bus: u64 = 0;
            let format: Id = msg_send![input_node, outputFormatForBus: bus];

            // Install a tap on the input node to feed audio to the request
            let request_for_tap = request;
            let tap_block = block::ConcreteBlock::new(
                move |buffer: Id, _when: Id| {
                    let _: () = msg_send![request_for_tap, appendAudioPCMBuffer: buffer];
                },
            );
            let tap_block = tap_block.copy();
            let buffer_size: u32 = 1024;
            let _: () = msg_send![
                input_node,
                installTapOnBus: bus
                bufferSize: buffer_size
                format: format
                block: &*tap_block
            ];

            // Start the recognition task with a result handler block
            let sender_for_block = sender.clone();
            let result_block = block::ConcreteBlock::new(
                move |result: Id, error: Id| {
                    if !error.is_null() {
                        let description: Id = msg_send![error, localizedDescription];
                        let utf8: *const u8 = msg_send![description, UTF8String];
                        if !utf8.is_null() {
                            let msg =
                                std::ffi::CStr::from_ptr(utf8 as *const std::ffi::c_char)
                                    .to_string_lossy()
                                    .to_string();
                            let _ = sender_for_block.unbounded_send(SpeechEvent::Error(msg));
                        }
                        return;
                    }

                    if !result.is_null() {
                        let best: Id = msg_send![result, bestTranscription];
                        if !best.is_null() {
                            let formatted: Id = msg_send![best, formattedString];
                            let utf8: *const u8 = msg_send![formatted, UTF8String];
                            if !utf8.is_null() {
                                let text =
                                    std::ffi::CStr::from_ptr(utf8 as *const std::ffi::c_char)
                                        .to_string_lossy()
                                        .to_string();
                                let is_final: bool = msg_send![result, isFinal];
                                if is_final {
                                    let _ = sender_for_block
                                        .unbounded_send(SpeechEvent::FinalResult(text));
                                } else {
                                    let _ = sender_for_block
                                        .unbounded_send(SpeechEvent::PartialResult(text));
                                }
                            }
                        }
                    }
                },
            );
            let result_block = result_block.copy();

            let recognition_task: Id = msg_send![
                recognizer,
                recognitionTaskWithRequest: request
                resultHandler: &*result_block
            ];

            // Start the audio engine
            let mut error: Id = NIL;
            let started: bool = msg_send![audio_engine, startAndReturnError: &mut error];
            if !started {
                let _: () = msg_send![recognition_task, cancel];
                let _: () = msg_send![audio_engine, release];
                let _: () = msg_send![request, release];
                let _: () = msg_send![recognizer, release];
                return Err(anyhow!("Failed to start audio engine"));
            }

            let _: () = msg_send![recognizer, release];

            *ACTIVE_SESSION.lock() = Some(ActiveSession {
                audio_engine,
                recognition_task,
                request,
            });
        }

        Ok(receiver)
    }

    fn stop() {
        let session = ACTIVE_SESSION.lock().take();
        if let Some(session) = session {
            unsafe {
                let input_node: Id = msg_send![session.audio_engine, inputNode];
                if !input_node.is_null() {
                    let _: () = msg_send![input_node, removeTapOnBus: 0u64];
                }
                let _: () = msg_send![session.audio_engine, stop];
                let _: () = msg_send![session.request, endAudio];
                let _: () = msg_send![session.recognition_task, cancel];
                let _: () = msg_send![session.audio_engine, release];
                let _: () = msg_send![session.request, release];
            }
        }
    }
}

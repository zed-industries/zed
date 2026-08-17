/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <jni.h>

#include <memory>

#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"

namespace livekit_ffi {
typedef JavaVM JavaVM;
}  // namespace livekit_ffi
#include "webrtc-sys/src/android.rs.h"

namespace livekit_ffi {
void init_android(JavaVM* jvm);

std::unique_ptr<webrtc::VideoEncoderFactory> CreateAndroidVideoEncoderFactory();
std::unique_ptr<webrtc::VideoDecoderFactory> CreateAndroidVideoDecoderFactory();

}  // namespace livekit_ffi

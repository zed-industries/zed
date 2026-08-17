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

#include "livekit/android.h"

#include <jni.h>

#include <memory>

#include "api/video_codecs/video_decoder_factory.h"
#include "sdk/android/native_api/base/init.h"
#include "sdk/android/native_api/codecs/wrapper.h"
#include "sdk/android/native_api/jni/class_loader.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace livekit_ffi {

void init_android(JavaVM* jvm) {
  webrtc::InitAndroid(jvm);
}

std::unique_ptr<webrtc::VideoEncoderFactory>
CreateAndroidVideoEncoderFactory() {
  JNIEnv* env = webrtc::AttachCurrentThreadIfNeeded();
  webrtc::ScopedJavaLocalRef<jclass> factory_class =
      webrtc::GetClass(env, "org/webrtc/DefaultVideoEncoderFactory");

  jmethodID ctor = env->GetMethodID(factory_class.obj(), "<init>",
                                    "(Lorg/webrtc/EglBase$Context;ZZ)V");

  jobject encoder_factory =
      env->NewObject(factory_class.obj(), ctor, nullptr, true, false);

  return webrtc::JavaToNativeVideoEncoderFactory(env, encoder_factory);
}

std::unique_ptr<webrtc::VideoDecoderFactory>
CreateAndroidVideoDecoderFactory() {
  JNIEnv* env = webrtc::AttachCurrentThreadIfNeeded();

  webrtc::ScopedJavaLocalRef<jclass> factory_class =
      webrtc::GetClass(env, "org/webrtc/WrappedVideoDecoderFactory");

  jmethodID ctor = env->GetMethodID(factory_class.obj(), "<init>",
                                    "(Lorg/webrtc/EglBase$Context;)V");

  jobject decoder_factory = env->NewObject(factory_class.obj(), ctor, nullptr);
  return webrtc::JavaToNativeVideoDecoderFactory(env, decoder_factory);
}

}  // namespace livekit_ffi

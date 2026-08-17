/*
 * Copyright 2022 LiveKit
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

#ifndef SDK_ANDROID_SRC_JNI_PC_FRAME_CRYPTOR_H_
#define SDK_ANDROID_SRC_JNI_PC_FRAME_CRYPTOR_H_

#include <jni.h>

#include "api/crypto/frame_crypto_transformer.h"
#include "sdk/android/native_api/jni/scoped_java_ref.h"

namespace webrtc {
namespace jni {

webrtc::FrameCryptorTransformer::Algorithm AlgorithmFromIndex(int index);

ScopedJavaLocalRef<jobject> NativeToJavaFrameCryptor(
    JNIEnv* env,
    rtc::scoped_refptr<FrameCryptorTransformer> cryptor);

class FrameCryptorObserverJni : public FrameCryptorTransformerObserver {
 public:
  FrameCryptorObserverJni(JNIEnv* jni, const JavaRef<jobject>& j_observer);
  ~FrameCryptorObserverJni() override;

 protected:
  void OnFrameCryptionStateChanged(const std::string participant_id,
                                   FrameCryptionState state) override;

 private:
  const ScopedJavaGlobalRef<jobject> j_observer_global_;
  const ScopedJavaGlobalRef<jobject> j_observer_;
};

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_FRAME_CRYPTOR_H_

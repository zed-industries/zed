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
#ifndef API_ANDROID_JNI_EXTERNALAUDIOPROCESSORINTERFACE_H_
#define API_ANDROID_JNI_EXTERNALAUDIOPROCESSORINTERFACE_H_

namespace webrtc {

class ExternalAudioProcessingInterface {
 public:
  virtual void Initialize(int sample_rate_hz, int num_channels) = 0;
  virtual void Reset(int new_rate) = 0;
  virtual void Process(int num_bands, int num_frames, int buffer_size, float* buffer) = 0;

 protected:
  virtual ~ExternalAudioProcessingInterface() = default;
};

}  // namespace webrtc

#endif  // API_ANDROID_JNI_EXTERNALAUDIOPROCESSORINTERFACE_H_

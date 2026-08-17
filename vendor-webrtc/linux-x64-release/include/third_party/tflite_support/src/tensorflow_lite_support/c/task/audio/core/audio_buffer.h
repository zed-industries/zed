/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_AUDIO_AUDIO_BUFFER_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_AUDIO_AUDIO_BUFFER_H_

// Defines C structs for holding the audio buffer.

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

// Holds audio format metadata.
typedef struct TfLiteAudioFormat {
  // The number of channels in the audio buffer.
  int channels;
  // The sample rate of the audio buffer.
  int sample_rate;
} TfLiteAudioFormat;

// A `TfLiteAudioBuffer` provides a view into the provided backing buffer and
// the audio format metadata.. TfLiteAudioBuffer doesn't take ownership of the
// provided backing buffer. The caller is responsible to manage the backing
// buffer lifecycle for the lifetime of the TfLiteAudioBuffer.
typedef struct TfLiteAudioBuffer {
  TfLiteAudioFormat format;

  // Backing buffer that holds the audio samples which are to be processed. For
  // muti channel data array is expected to be interleaved .
  float* data;

  // Size of the audio buffer. This size can be used to loop through the
  // audio_buffer.
  int size;
} TfLiteAudioBuffer;

void TfLiteAudioBufferDelete(TfLiteAudioBuffer *buffer);

void TfLiteAudioBufferDeleteData(const TfLiteAudioBuffer audio_buffer);

void TfLiteAudioFormatDelete(TfLiteAudioFormat *format);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_AUDIO_AUDIO_BUFFER_H_

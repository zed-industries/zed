/* Copyright 2016 The TensorFlow Authors. All Rights Reserved.

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

// Functions to write audio in WAV format.
// This file is forked from `tensorflow/core/lib/wav/wav_io.h`.

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_UTILS_WAV_IO_H_

#define TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_UTILS_WAV_IO_H_

#include <string>
#include <vector>
#include <cstdint>

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/status_macros.h"

namespace tflite {
namespace task {
namespace audio {

// Byte order definition provided by gcc. MSVC doesn't define those so
// we define them here.
// We assume that all windows platform out there are little endian.
#if defined(_MSC_VER) && !defined(__clang__)
#define __ORDER_LITTLE_ENDIAN__ 0x4d2
#define __ORDER_BIG_ENDIAN__ 0x10e1
#define __BYTE_ORDER__ __ORDER_LITTLE_ENDIAN__
#endif

namespace port {

// TODO(jeff,sanjay): Make portable
constexpr bool kLittleEndian = __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__;

}  // namespace port

// Decodes the little-endian signed 16-bit PCM WAV file data (aka LIN16
// encoding) into a float Tensor. The channels are encoded as the lowest
// dimension of the tensor, with the number of frames as the second. This means
// that a four frame stereo signal will have the shape [4, 2]. The sample rate
// is read from the file header, and an error is returned if the format is not
// supported.
// The results are output as floats within the range -1 to 1,
absl::Status DecodeLin16WaveAsFloatVector(const std::string& wav_string,
                                          std::vector<float>* float_values,
                                          uint32_t* offset,
                                          uint32_t* sample_count,
                                          uint16_t* channel_count,
                                          uint32_t* sample_rate);

// Everything below here is only exposed publicly for testing purposes.

// Handles moving the data index forward, validating the arguments, and avoiding
// overflow or underflow.
absl::Status IncrementOffset(uint32_t old_offset, size_t increment,
                             size_t max_size, uint32_t* new_offset);

// This function is only exposed in the header for testing purposes, as a
// template that needs to be instantiated. Reads a typed numeric value from a
// stream of data.
template <class T>
absl::Status ReadValue(const std::string& data, T* value, uint32_t* offset) {
  uint32_t new_offset;
  TFLITE_RETURN_IF_ERROR(
      IncrementOffset(*offset, sizeof(T), data.size(), &new_offset));
  if (port::kLittleEndian) {
    memcpy(value, data.data() + *offset, sizeof(T));
  } else {
    *value = 0;
    const uint8_t* data_buf =
        reinterpret_cast<const uint8_t*>(data.data() + *offset);
    int shift = 0;
    for (int i = 0; i < sizeof(T); ++i, shift += 8) {
      *value = *value | (data_buf[i] << shift);
    }
  }
  *offset = new_offset;
  return absl::OkStatus();
}

// Load the content of the file into std::string.
std::string ReadFile(const std::string filepath);

}  // namespace audio
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_AUDIO_UTILS_WAV_IO_H_

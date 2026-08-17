/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// WAV to string libraries with c++ support.

#ifndef AUDIO_DSP_PORTABLE_WRITE_WAV_STRING_H_
#define AUDIO_DSP_PORTABLE_WRITE_WAV_STRING_H_

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <string>

namespace audio_dsp {

/* Write a WAV file with int16_t samples. For a multichannel signal, the channel
 * samples should be interleaved, and num_samples must be an integer multiple of
 * num_channels. Returns 1 on success, 0 on failure.
 */
int WriteWavToString(const int16_t* samples,
                     size_t num_samples,
                     int sample_rate_hz,
                     int num_channels,
                     std::string* string_data);

int WriteWavToString(const float* samples,
                     size_t num_samples,
                     int sample_rate_hz,
                     int num_channels,
                     std::string* string_data);

}  // namespace audio_dsp
#endif  // AUDIO_DSP_PORTABLE_WRITE_WAV_STRING_H_

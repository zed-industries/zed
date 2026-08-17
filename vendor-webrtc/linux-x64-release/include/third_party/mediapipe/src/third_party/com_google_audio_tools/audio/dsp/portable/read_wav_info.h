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

#ifndef AUDIO_DSP_PORTABLE_READ_WAV_INFO_H_
#define AUDIO_DSP_PORTABLE_READ_WAV_INFO_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

/* As the caller of a WAV reader, you should never modify any these members. */
struct ReadWavInfo {
  /* Number of channels, for example 2 for stereo. */
  int num_channels;
  /* Sample rate in Hz. */
  int sample_rate_hz;
  /* Number of remaining samples to read, a multiple of num_channels. */
  size_t remaining_samples;
  /* Number of bits per sample in the WAV file. */
  int bit_depth;

  /* The number of bytes in a single sample. After a call to ReadWavHeader(),
   * this is set to the minimum number of bytes needed to represent the type
   * (always 2 or 4). If it is 2, it may be increased to 4 prior to reading
   * samples. */
  int destination_alignment_bytes;

  /* The encoded format of the WAV file read. */
  enum EncodingType {
    kPcm16Encoding,
    kPcm24Encoding,
    kMuLawEncoding,
    kIeeeFloat32Encoding,
    kIeeeFloat64Encoding,
  } encoding;

  /* The sample format of the read samples (the type of the data as returned by
   * ReadWavSamplesGeneric(). */
  enum SampleType {
    kInt16,
    kInt32,
    kFloat,
  } sample_format;
};
typedef struct ReadWavInfo ReadWavInfo;

#ifdef __cplusplus
}  /* extern "C" */
#endif

/* NOLINTNEXTLINE(build/header_guard) */
#endif  /* AUDIO_DSP_PORTABLE_READ_WAV_INFO_H_ */

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

/* A WAV reader with support for 16 or 24-bit linear PCM format, mu-law format,
 * or IEEE floating point format (32 or 64 bits).
 *
 * The simplest usage of this API is to use the ReadWavFile function, which
 * reads all samples into memory.
 *
 * The API also supports streaming using the following pattern:
 * FILE* f = fopen(filename, "rb");
 * ReadWavInfo info;
 * ReadWavHeader(f, &info);
 * int16_t buffer[kBufferSize];
 * Read16BitWavSamples(f, &info, buffer, kBufferSize);
 * Read16BitWavSamples(f, &info, buffer, kBufferSize);
 * ...
 * fclose(f);
 */

#ifndef AUDIO_DSP_PORTABLE_READ_WAV_FILE_H_
#define AUDIO_DSP_PORTABLE_READ_WAV_FILE_H_

#include <stdint.h>
#include <stdio.h>

#include "audio/dsp/portable/read_wav_file_generic.h"
#include "audio/dsp/portable/read_wav_info.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Read WAV header and fmt chunk, filling the fields of the info struct. Returns
 * 1 on success, 0 on failure.
 */
int ReadWavHeader(FILE* f, ReadWavInfo* info);

/* Read up to num_samples samples from a WAV file in 16-bit PCM or mu-law
 * format. Returns the number of samples actually read, a multiple of
 * info->num_channels. Samples are in interleaved order.
 */
size_t Read16BitWavSamples(FILE* f, ReadWavInfo* info, int16_t* samples,
                           size_t num_samples);

/* Read up to num_samples samples from a WAV file in either 16- or 24-bit PCM
 * format or mu-law format into a 32-bit container. Data is shifted into the
 * most significant bits such that full scale values of 16- or 24-bit samples
 * will fill the [2^31 - 1, -2^31] range and leave the least significant bits
 * equal to zero.
 */
size_t ReadWavSamples(FILE* f, ReadWavInfo* info, int32_t* samples,
                      size_t num_samples);

/* Read a WAV file, which must be 16-bit PCM or mu-law. The function allocates
 * memory and returns a pointer to the read samples. It is the caller's
 * responsibility to free this pointer. The number of samples read is
 * *num_samples, a multiple of *num_channels.
 */
int16_t* Read16BitWavFile(const char* file_name, size_t* num_samples,
                          int* num_channels, int* sample_rate_hz);

/* Same as above, but also handles 24-bit PCM, and reads the samples into a
 * 32-bit container. Data is shifted into the most significant bits such that
 * full scale values in 16 or 24-bit WAV files will fill the [2^31 - 1, -2^31]
 * range and leave the least significant bits equal to zero.
 */
int32_t* ReadWavFile(const char* file_name, size_t* num_samples,
                     int* num_channels, int* sample_rate_hz);

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif  /* AUDIO_DSP_PORTABLE_READ_WAV_FILE_H_ */

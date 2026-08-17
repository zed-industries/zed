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

/* C library to write 16-bit WAV files.
 *
 * The simplest usage of this API is to use the WriteWavFile function, which
 * requires the samples to be buffered at the application layer.
 *
 * The API also supports streaming.  If the length is known in advance, use the
 * following pattern:
 * FILE* f = fopen(filename, "wb");
 * WriteWavHeader(f, ...);
 * WriteWavSamples(...);
 * WriteWavSamples(...);
 * ...
 * fclose(f);
 *
 * The API also supports streaming without knowing the length in advance by
 * first writing a dummy header, using the following pattern.  The number of
 * channels must be correct in the dummy header:
 * FILE* f = fopen(filename, "wb");
 * WriteWavHeader(f, 0, 0, nchannels);
 * WriteWavSamples(...);
 * WriteWavSamples(...);
 * ...
 * fseek(f, 0, SEEK_SET);
 * WriteWavHeader(f, ...);
 * fclose(f);
 *
 */

#ifndef AUDIO_DSP_PORTABLE_WRITE_WAV_FILE_H_
#define AUDIO_DSP_PORTABLE_WRITE_WAV_FILE_H_

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Write a WAV file header at the beginning of a WAV file.  Returns 1 on
 * success, 0 on failure.
 */
int WriteWavHeader(FILE* f, size_t num_samples, int sample_rate_hz,
                   int num_channels);

/* Write samples into a WAV file.  samples should be interleaved, and
 * num_samples must be an integer multiple of num_channels. Returns 1 on
 * success, 0 on failure.
 */
int WriteWavSamples(FILE* f, const int16_t* samples, size_t num_samples);

/* Write a WAV file with int16_t samples. For a multichannel signal, the channel
 * samples should be interleaved, and num_samples must be an integer multiple of
 * num_channels. Returns 1 on success, 0 on failure.
 */
int WriteWavFile(const char* file_name, const int16_t* samples,
                 size_t num_samples, int sample_rate_hz, int num_channels);

#ifdef __cplusplus
} /* extern "C" */
#endif
#endif /* AUDIO_DSP_PORTABLE_WRITE_WAV_FILE_H_ */

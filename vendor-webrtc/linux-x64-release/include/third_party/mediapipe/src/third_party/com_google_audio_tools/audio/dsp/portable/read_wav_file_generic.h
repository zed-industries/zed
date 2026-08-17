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

#ifndef AUDIO_DSP_PORTABLE_READ_WAV_FILE_GENERIC_H_
#define AUDIO_DSP_PORTABLE_READ_WAV_FILE_GENERIC_H_

/* A WAV reader with support for 16 or 24-bit linear PCM format, mu-law format,
 * or IEEE floating point format (32 or 64 bits).
 *
 * Don't use this file directly unless you are adding support for a different
 * kind of filesystem. See read_wav_file.h or audio/util/wavfile.
 */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "audio/dsp/portable/read_wav_info.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Allows callers to provide custom IO callbacks for reading WAV files.
 * Note that only io_ptr and read_fun *must* be set to non NULL values.
 */
struct WavReader {
  /* Pointer to user-defined struct of state needed for read_fun. */
  void* io_ptr;

  /* A function for reading bytes. Return the number of bytes read. */
  size_t (*read_fun)(void* bytes, size_t num_bytes, void* io_ptr);

  /* A function for seeking forward in the file. num_bytes is always relative
   * to the read pointer (for fseek, this means using SEEK_CUR). Return 1 on
   * error. In some systems, seeking is more efficient than reading, but if
   * no function is provided for seek_fun (set it to NULL), read_fun will be
   * used.
   */
  int (*seek_fun)(size_t num_bytes, void* io_ptr);

  /* Function for distinguishing between an error and the end-of-file when
   * read_fun returns fewer bytes than expected. You may set this to NULL.
   * Return 1 on EOF. */
  int (*eof_fun)(void* io_ptr);

  /* The RIFF format is forward compatible, supporting arbitrary data block
   * definitions. This callback, if defined, will provide the caller with
   * each chunk of data that is not recognized by the WavReader.
   * Typically, you do not need to provide this.
   *
   * id is the name of the chunk to be verified with memcmp(*id, ..., 4) == 0
   * data points to the first byte following the id and the chunk size
   * num_bytes is the chunk size in bytes
   */
  void (*custom_chunk_fun)(
      char (*id)[4], const void * data, size_t num_bytes, void* io_ptr);

  /* 1 for error, 0 for no error. This is used internally. No need to set it. */
  int has_error;
};
typedef struct WavReader WavReader;

/* Read WAV header and fmt chunk, filling the fields of the info struct. Returns
 * 1 on success, 0 on failure.
 */
int ReadWavHeaderGeneric(WavReader* w, ReadWavInfo* info);

/* Read up to num_samples samples from a WAV file in 16-bit PCM or mu-law
 * format. Returns the number of samples actually read, a multiple of
 * info->num_channels. Samples are in interleaved order.
 *
 * Prefer ReadWavSamplesGeneric. The only advantage to this function is that it
 * saves a factor of 2 in memory for WAV files that are known to have a bit
 * depth of at most 16.
 */
size_t Read16BitWavSamplesGeneric(WavReader* w, ReadWavInfo* info,
                                  int16_t* samples, size_t num_samples);

/* Read up to num_samples samples from a WAV file regardless of WAV file
 * formatting. The actual format of the data is described by *info.
 * Returns the number of samples actually read, a multiple of
 * info->num_channels. Samples are in interleaved order.
 *
 * The samples pointer must be aligned to a 4-byte word boundary.
 */
size_t ReadWavSamplesGeneric(WavReader* w, ReadWavInfo* info, void* samples,
                             size_t num_samples);

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif  /* AUDIO_DSP_PORTABLE_READ_WAV_FILE_GENERIC_H_ */

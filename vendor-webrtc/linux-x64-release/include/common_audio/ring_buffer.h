/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// A ring buffer to hold arbitrary data. Provides no thread safety. Unless
// otherwise specified, functions return 0 on success and -1 on error.

#ifndef COMMON_AUDIO_RING_BUFFER_H_
#define COMMON_AUDIO_RING_BUFFER_H_

// TODO(https://issues.webrtc.org/379542219): Remove when AECm gets removed.

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>  // size_t

enum Wrap { SAME_WRAP, DIFF_WRAP };

typedef struct RingBuffer {
  size_t read_pos;
  size_t write_pos;
  size_t element_count;
  size_t element_size;
  enum Wrap rw_wrap;
  char* data;
} RingBuffer;

// Creates and initializes the buffer. Returns null on failure.
RingBuffer* WebRtc_CreateBuffer(size_t element_count, size_t element_size);
void WebRtc_InitBuffer(RingBuffer* handle);
void WebRtc_FreeBuffer(void* handle);

// Reads data from the buffer. Returns the number of elements that were read.
// The `data_ptr` will point to the address where the read data is located.
// If no data can be read, `data_ptr` is set to `NULL`. If all data can be read
// without buffer wrap around then `data_ptr` will point to the location in the
// buffer. Otherwise, the data will be copied to `data` (memory allocation done
// by the user) and `data_ptr` points to the address of `data`. `data_ptr` is
// only guaranteed to be valid until the next call to WebRtc_WriteBuffer().
//
// To force a copying to `data`, pass a null `data_ptr`.
//
// Returns number of elements read.
size_t WebRtc_ReadBuffer(RingBuffer* handle,
                         void** data_ptr,
                         void* data,
                         size_t element_count);

// Writes `data` to buffer and returns the number of elements written.
size_t WebRtc_WriteBuffer(RingBuffer* handle,
                          const void* data,
                          size_t element_count);

// Moves the buffer read position and returns the number of elements moved.
// Positive `element_count` moves the read position towards the write position,
// that is, flushing the buffer. Negative `element_count` moves the read
// position away from the the write position, that is, stuffing the buffer.
// Returns number of elements moved.
int WebRtc_MoveReadPtr(RingBuffer* handle, int element_count);

// Returns number of available elements to read.
size_t WebRtc_available_read(const RingBuffer* handle);

// Returns number of available elements for write.
size_t WebRtc_available_write(const RingBuffer* handle);

#ifdef __cplusplus
}
#endif

#endif  // COMMON_AUDIO_RING_BUFFER_H_

/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_PUBLIC_STREAM_WRITER_H_
#define INCLUDE_PERFETTO_PUBLIC_STREAM_WRITER_H_

#include <assert.h>
#include <string.h>

#include "perfetto/public/abi/stream_writer_abi.h"
#include "perfetto/public/compiler.h"

// Returns the number of bytes available for writing in the current chunk.
static inline size_t PerfettoStreamWriterAvailableBytes(
    const struct PerfettoStreamWriter* w) {
  return PERFETTO_STATIC_CAST(size_t, w->end - w->write_ptr);
}

// Writes `size` bytes from `src` to the writer.
//
// WARNING: PerfettoStreamWriterAvailableBytes(`w`) must be >= `size`.
static inline void PerfettoStreamWriterAppendBytesUnsafe(
    struct PerfettoStreamWriter* w,
    const uint8_t* src,
    size_t size) {
  assert(size <= PerfettoStreamWriterAvailableBytes(w));
  memcpy(w->write_ptr, src, size);
  w->write_ptr += size;
}

// Writes `size` bytes from `src` to the writer.
static inline void PerfettoStreamWriterAppendBytes(
    struct PerfettoStreamWriter* w,
    const uint8_t* src,
    size_t size) {
  if (PERFETTO_LIKELY(size <= PerfettoStreamWriterAvailableBytes(w))) {
    PerfettoStreamWriterAppendBytesUnsafe(w, src, size);
  } else {
    PerfettoStreamWriterAppendBytesSlowpath(w, src, size);
  }
}

// Writes the single byte `value` to the writer.
static inline void PerfettoStreamWriterAppendByte(
    struct PerfettoStreamWriter* w,
    uint8_t value) {
  if (PERFETTO_UNLIKELY(PerfettoStreamWriterAvailableBytes(w) < 1)) {
    PerfettoStreamWriterNewChunk(w);
  }
  *w->write_ptr++ = value;
}

// Returns a pointer to an area of the chunk long `size` for writing. The
// returned area is considered already written by the writer (it will not be
// used again).
//
// WARNING: PerfettoStreamWriterAvailableBytes(`w`) must be >= `size`.
static inline uint8_t* PerfettoStreamWriterReserveBytesUnsafe(
    struct PerfettoStreamWriter* w,
    size_t size) {
  uint8_t* ret = w->write_ptr;
  assert(size <= PerfettoStreamWriterAvailableBytes(w));
  w->write_ptr += size;
  return ret;
}

// Returns a pointer to an area of the chunk long `size` for writing. The
// returned area is considered already written by the writer (it will not be
// used again).
//
// WARNING: `size` should be smaller than the chunk size returned by the
// `delegate`.
static inline uint8_t* PerfettoStreamWriterReserveBytes(
    struct PerfettoStreamWriter* w,
    size_t size) {
  if (PERFETTO_LIKELY(size <= PerfettoStreamWriterAvailableBytes(w))) {
    return PerfettoStreamWriterReserveBytesUnsafe(w, size);
  }
  PerfettoStreamWriterReserveBytesSlowpath(w, size);
  return w->write_ptr - size;
}

// Returns the number of bytes written to the stream writer from the start.
static inline size_t PerfettoStreamWriterGetWrittenSize(
    const struct PerfettoStreamWriter* w) {
  return w->written_previously +
         PERFETTO_STATIC_CAST(size_t, w->write_ptr - w->begin);
}

#endif  // INCLUDE_PERFETTO_PUBLIC_STREAM_WRITER_H_

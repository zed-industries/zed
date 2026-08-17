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

#ifndef INCLUDE_PERFETTO_PUBLIC_ABI_STREAM_WRITER_ABI_H_
#define INCLUDE_PERFETTO_PUBLIC_ABI_STREAM_WRITER_ABI_H_

#include <stddef.h>
#include <stdint.h>

#include "perfetto/public/abi/export.h"

#ifdef __cplusplus
extern "C" {
#endif

// An opaque structure used to represent the internal implementation of a
// protozero stream writer.
struct PerfettoStreamWriterImpl;

// A `PerfettoStreamWriter` owns a chunk of memory that the user can write
// to. The section from `begin` (inclusive) to `write_ptr` (non inclusive)
// already contains valid data. The section from `write_ptr` (inclusive) to
// `end` (non inclusive) is empty and can be used for new data.
//
// --------------------------------------------------------------------------
// |wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww                                    |
// --------------------------------------------------------------------------
//  ^                                   ^                                   ^
//   \                                  |                                  /
//    `begin`                       `write_ptr`                       `end`
struct PerfettoStreamWriter {
  struct PerfettoStreamWriterImpl* impl;

  // Points to the first byte of the current chunk.
  uint8_t* begin;

  // Points to the first byte after the end of the current chunk (STL-style).
  uint8_t* end;

  // Write pointer: points to the first not-yet-written byte of the current
  // chunk.
  uint8_t* write_ptr;

  // Keeps track of all the bytes written in previous chunks (bytes written in
  // the current chunk are not included here).
  size_t written_previously;
};

// Tells the writer that the current chunk has been written until `write_ptr`
// (non inclusive).
//
// Returns the new state of the writer.
PERFETTO_SDK_EXPORT void PerfettoStreamWriterUpdateWritePtr(
    struct PerfettoStreamWriter*);

// Commits the current chunk and gets a new chunk.
PERFETTO_SDK_EXPORT void PerfettoStreamWriterNewChunk(
    struct PerfettoStreamWriter*);

// Appends `size` from `src` to the writer.
PERFETTO_SDK_EXPORT void PerfettoStreamWriterAppendBytesSlowpath(
    struct PerfettoStreamWriter*,
    const uint8_t* src,
    size_t size);

#define PERFETTO_STREAM_WRITER_PATCH_SIZE 4

// Tells the stream writer that the part of the current chunk pointed by
// `patch_addr` (until `patch_addr`+`PERFETTO_STREAM_WRITER_PATCH_SIZE`) needs
// to be changed after the current chunk goes away.
//
// The caller can write to the returned location (which may have been redirected
// by the stream writer) after the current chunk has gone away. The caller
// **must write a non-zero value as the first byte** eventually.
//
// The stream writer can also return NULL, in which case the caller should not
// write anything.
//
// This can be used to backfill the size of a protozero message.
PERFETTO_SDK_EXPORT uint8_t* PerfettoStreamWriterAnnotatePatch(
    struct PerfettoStreamWriter*,
    uint8_t* patch_addr);

// Returns a pointer to an area of the chunk long `size` for writing. The
// returned area is considered already written by the writer (it will not be
// used again).
//
// WARNING: `size` should be smaller than the chunk size returned by the
// `delegate`.
PERFETTO_SDK_EXPORT
void PerfettoStreamWriterReserveBytesSlowpath(struct PerfettoStreamWriter*,
                                              size_t size);

#ifdef __cplusplus
}
#endif

#endif  // INCLUDE_PERFETTO_PUBLIC_ABI_STREAM_WRITER_ABI_H_

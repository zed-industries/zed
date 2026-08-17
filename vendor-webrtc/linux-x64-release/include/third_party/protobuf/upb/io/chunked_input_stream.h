// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_IO_CHUNKED_INPUT_STREAM_H_
#define UPB_IO_CHUNKED_INPUT_STREAM_H_

#include "upb/io/zero_copy_input_stream.h"
#include "upb/mem/arena.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// A ZeroCopyInputStream which wraps a flat buffer and limits the number of
// bytes that can be returned by a single call to Next().
upb_ZeroCopyInputStream* upb_ChunkedInputStream_New(const void* data,
                                                    size_t size, size_t limit,
                                                    upb_Arena* arena);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_IO_CHUNKED_INPUT_STREAM_H_ */

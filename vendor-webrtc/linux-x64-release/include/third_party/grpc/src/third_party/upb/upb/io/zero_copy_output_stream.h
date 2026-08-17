// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_IO_ZERO_COPY_OUTPUT_STREAM_H_
#define UPB_IO_ZERO_COPY_OUTPUT_STREAM_H_

#include "upb/base/status.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct upb_ZeroCopyOutputStream upb_ZeroCopyOutputStream;

typedef struct {
  // Obtains a buffer into which data can be written. Any data written
  // into this buffer will eventually (maybe instantly, maybe later on)
  // be written to the output.
  //
  // Preconditions:
  //   "count" and "status" are not NULL.
  //
  // Postconditions:
  //   All errors are permanent. If an error occurs then:
  //     - NULL will be returned to the caller.
  //     - *count will be set to zero.
  //     - *status will be set to the error.
  //   EOF is permanent. If EOF is reached then:
  //     - NULL will be returned to the caller.
  //     - *count will be set to zero.
  //     - *status will not be touched.
  //   Otherwise:
  //     - The returned value will point to a buffer containing the bytes read.
  //     - *count will be set to the number of bytes read.
  //     - *status will not be touched.
  //
  // Ownership of this buffer remains with the stream, and the buffer
  // remains valid only until some other method of the stream is called
  // or the stream is destroyed.
  //
  // Any data which the caller stores in this buffer will eventually be
  // written to the output (unless BackUp() is called).
  void* (*Next)(struct upb_ZeroCopyOutputStream* z, size_t* count,
                upb_Status* status);

  // Backs up a number of bytes, so that the end of the last buffer returned
  // by Next() is not actually written. This is needed when you finish
  // writing all the data you want to write, but the last buffer was bigger
  // than you needed. You don't want to write a bunch of garbage after the
  // end of your data, so you use BackUp() to back up.
  //
  // Preconditions:
  // * The last method called must have been Next().
  // * count must be less than or equal to the size of the last buffer
  //   returned by Next().
  // * The caller must not have written anything to the last "count" bytes
  //   of that buffer.
  //
  // Postconditions:
  // * The last "count" bytes of the last buffer returned by Next() will be
  //   ignored.
  //
  // This method can be called with `count = 0` to finalize (flush) any
  // previously returned buffer. For example, a file output stream can
  // flush buffers returned from a previous call to Next() upon such
  // BackUp(0) invocations. ZeroCopyOutputStream callers should always
  // invoke BackUp() after a final Next() call, even if there is no
  // excess buffer data to be backed up to indicate a flush point.
  void (*BackUp)(struct upb_ZeroCopyOutputStream* z, size_t count);

  // Returns the total number of bytes written since this object was created.
  size_t (*ByteCount)(const struct upb_ZeroCopyOutputStream* z);
} _upb_ZeroCopyOutputStream_VTable;

struct upb_ZeroCopyOutputStream {
  const _upb_ZeroCopyOutputStream_VTable* vtable;
};

UPB_INLINE void* upb_ZeroCopyOutputStream_Next(upb_ZeroCopyOutputStream* z,
                                               size_t* count,
                                               upb_Status* status) {
  void* out = z->vtable->Next(z, count, status);
  UPB_ASSERT((out == NULL) == (*count == 0));
  return out;
}

UPB_INLINE void upb_ZeroCopyOutputStream_BackUp(upb_ZeroCopyOutputStream* z,
                                                size_t count) {
  return z->vtable->BackUp(z, count);
}

UPB_INLINE size_t
upb_ZeroCopyOutputStream_ByteCount(const upb_ZeroCopyOutputStream* z) {
  return z->vtable->ByteCount(z);
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_IO_ZERO_COPY_OUTPUT_STREAM_H_ */

// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_IO_ZERO_COPY_INPUT_STREAM_H_
#define UPB_IO_ZERO_COPY_INPUT_STREAM_H_

#include "upb/base/status.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct upb_ZeroCopyInputStream upb_ZeroCopyInputStream;

typedef struct {
  // Obtains a chunk of data from the stream.
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
  const void* (*Next)(struct upb_ZeroCopyInputStream* z, size_t* count,
                      upb_Status* status);

  // Backs up a number of bytes, so that the next call to Next() returns
  // data again that was already returned by the last call to Next().  This
  // is useful when writing procedures that are only supposed to read up
  // to a certain point in the input, then return.  If Next() returns a
  // buffer that goes beyond what you wanted to read, you can use BackUp()
  // to return to the point where you intended to finish.
  //
  // Preconditions:
  // * The last method called must have been Next().
  // * count must be less than or equal to the size of the last buffer
  //   returned by Next().
  //
  // Postconditions:
  // * The last "count" bytes of the last buffer returned by Next() will be
  //   pushed back into the stream.  Subsequent calls to Next() will return
  //   the same data again before producing new data.
  void (*BackUp)(struct upb_ZeroCopyInputStream* z, size_t count);

  // Skips a number of bytes. Returns false if the end of the stream is
  // reached or some input error occurred. In the end-of-stream case, the
  // stream is advanced to the end of the stream (so ByteCount() will return
  // the total size of the stream).
  bool (*Skip)(struct upb_ZeroCopyInputStream* z, size_t count);

  // Returns the total number of bytes read since this object was created.
  size_t (*ByteCount)(const struct upb_ZeroCopyInputStream* z);
} _upb_ZeroCopyInputStream_VTable;

struct upb_ZeroCopyInputStream {
  const _upb_ZeroCopyInputStream_VTable* vtable;
};

UPB_INLINE const void* upb_ZeroCopyInputStream_Next(upb_ZeroCopyInputStream* z,
                                                    size_t* count,
                                                    upb_Status* status) {
  const void* out = z->vtable->Next(z, count, status);
  UPB_ASSERT((out == NULL) == (*count == 0));
  return out;
}

UPB_INLINE void upb_ZeroCopyInputStream_BackUp(upb_ZeroCopyInputStream* z,
                                               size_t count) {
  return z->vtable->BackUp(z, count);
}

UPB_INLINE bool upb_ZeroCopyInputStream_Skip(upb_ZeroCopyInputStream* z,
                                             size_t count) {
  return z->vtable->Skip(z, count);
}

UPB_INLINE size_t
upb_ZeroCopyInputStream_ByteCount(const upb_ZeroCopyInputStream* z) {
  return z->vtable->ByteCount(z);
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_IO_ZERO_COPY_INPUT_STREAM_H_ */

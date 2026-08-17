// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_INCLUDE_SRC_REFERENCE_DRIVERS_HANDLE_EINTR_H_
#define IPCZ_INCLUDE_SRC_REFERENCE_DRIVERS_HANDLE_EINTR_H_

// Helper to ignore EINTR errors when making interruptible system calls. The
// expression `x` is retried until it produces a non-error result or a non-EINTR
// error.
#define HANDLE_EINTR(x)                                     \
  ({                                                        \
    decltype(x) eintr_wrapper_result;                       \
    do {                                                    \
      eintr_wrapper_result = (x);                           \
    } while (eintr_wrapper_result == -1 && errno == EINTR); \
    eintr_wrapper_result;                                   \
  })

#endif  // IPCZ_INCLUDE_SRC_REFERENCE_DRIVERS_HANDLE_EINTR_H_

// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_API_CONTEXT_H_
#define IPCZ_SRC_IPCZ_API_CONTEXT_H_

namespace ipcz {

// An APIContext is constructed on the stack immediately upon entry to any
// IpczAPI function. This is used to surface reentrancy to trap event handlers.
class APIContext {
 public:
  APIContext();
  APIContext(const APIContext&) = delete;
  APIContext& operator=(const APIContext&) = delete;
  ~APIContext();

  // Indicates whether the current call stack contains an IpczAPI entry point.
  static bool IsCurrentThreadWithinAPICall();

 private:
  // Whether or not the current thread already had an APIContext on it when this
  // one was constructed.
  const bool was_thread_within_api_call_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_API_CONTEXT_H_

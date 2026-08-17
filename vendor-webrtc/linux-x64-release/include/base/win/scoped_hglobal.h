// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/40284755): Remove this and spanify to fix the errors.
#pragma allow_unsafe_buffers
#endif

#ifndef BASE_WIN_SCOPED_HGLOBAL_H_
#define BASE_WIN_SCOPED_HGLOBAL_H_

#include <windows.h>

#include <stddef.h>

#include <utility>

namespace base {
namespace win {

// Like ScopedHandle except for HGLOBAL.
template <class Ptr>
class ScopedHGlobal {
 public:
  explicit ScopedHGlobal(HGLOBAL glob)
      : glob_(glob), data_(static_cast<Ptr>(GlobalLock(glob_))) {}

  ScopedHGlobal(const ScopedHGlobal&) = delete;
  ScopedHGlobal& operator=(const ScopedHGlobal&) = delete;

  ~ScopedHGlobal() { GlobalUnlock(glob_); }

  Ptr data() { return data_; }
  size_t size() const { return GlobalSize(glob_); }

  Ptr operator->() const {
    assert(data_ != 0);
    return data_;
  }

  Ptr release() { return std::exchange(data_, nullptr); }

  Ptr begin() { return data(); }
  Ptr end() { return data() + size(); }

 private:
  HGLOBAL glob_;

  Ptr data_;
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_HGLOBAL_H_

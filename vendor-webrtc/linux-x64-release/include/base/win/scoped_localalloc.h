// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SCOPED_LOCALALLOC_H_
#define BASE_WIN_SCOPED_LOCALALLOC_H_

#include <memory>
#include <utility>

#include "base/win/windows_types.h"

namespace base {
namespace win {

// unique_ptr deleter for LocalAlloc memory.
struct LocalAllocDeleter {
  void operator()(void* ptr) const { ::LocalFree(ptr); }
};

template <typename T>
using ScopedLocalAllocTyped = std::unique_ptr<T, LocalAllocDeleter>;

using ScopedLocalAlloc = ScopedLocalAllocTyped<void>;

// Make a typed ScopedLocalAlloc class and clear the original pointer.
template <typename T>
ScopedLocalAllocTyped<T> TakeLocalAlloc(T*& ptr) {
  return ScopedLocalAllocTyped<T>(std::exchange(ptr, nullptr));
}

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_LOCALALLOC_H_

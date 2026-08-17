// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_WINDOWS_HANDLE_UTIL_H_
#define BASE_WIN_WINDOWS_HANDLE_UTIL_H_

#include <stdint.h>

#include "base/win/windows_types.h"

namespace base::win {

inline bool IsPseudoHandle(HANDLE h) {
  // Note that there appears to be no official documentation covering the
  // existence of specific pseudo handle values. In practice it's clear that
  // e.g. -1 is the current process, -2 is the current thread, etc. The largest
  // negative value known to be an issue with DuplicateHandle in fuzzers is
  // -12.
  //
  // Note that there is virtually no risk of a real handle value falling within
  // this range and being misclassified as a pseudo handle.
  //
  // Cast through uintptr_t and then unsigned int to make the truncation to
  // 32 bits explicit. Handles are size of-pointer but are always 32-bit values.
  // https://msdn.microsoft.com/en-us/library/aa384203(VS.85).aspx says:
  // 64-bit versions of Windows use 32-bit handles for interoperability.
  static constexpr int kMinimumKnownPseudoHandleValue = -12;
  const auto value = static_cast<int32_t>(reinterpret_cast<uintptr_t>(h));
  return value < 0 && value >= kMinimumKnownPseudoHandleValue;
}

inline uint32_t HandleToUint32(HANDLE h) {
  // Cast through uintptr_t and then unsigned int to make the truncation to
  // 32 bits explicit. Handles are size of-pointer but are always 32-bit values.
  // https://msdn.microsoft.com/en-us/library/aa384203(VS.85).aspx says:
  // 64-bit versions of Windows use 32-bit handles for interoperability.
  return static_cast<uint32_t>(reinterpret_cast<uintptr_t>(h));
}

inline HANDLE Uint32ToHandle(uint32_t h) {
  return reinterpret_cast<HANDLE>(
      static_cast<uintptr_t>(static_cast<int32_t>(h)));
}

}  // namespace base::win

#endif  // BASE_WIN_WINDOWS_HANDLE_UTIL_H_

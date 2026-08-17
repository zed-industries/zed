//===-- sanitizer/rtsan_interface.h -----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is a part of RealtimeSanitizer.
//
// Public interface header.
//===----------------------------------------------------------------------===//

#ifndef SANITIZER_RTSAN_INTERFACE_H
#define SANITIZER_RTSAN_INTERFACE_H

#include <sanitizer/common_interface_defs.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

// Disable all RTSan error reporting.
// Must be paired with a call to `__rtsan_enable`
void SANITIZER_CDECL __rtsan_disable(void);

// Re-enable all RTSan error reporting.
// Must follow a call to `__rtsan_disable`.
void SANITIZER_CDECL __rtsan_enable(void);

#ifdef __cplusplus
} // extern "C"

namespace __rtsan {
#if defined(__has_feature) && __has_feature(realtime_sanitizer)

class ScopedDisabler {
public:
  ScopedDisabler() { __rtsan_disable(); }
  ~ScopedDisabler() { __rtsan_enable(); }

#if __cplusplus >= 201103L
  ScopedDisabler(const ScopedDisabler &) = delete;
  ScopedDisabler &operator=(const ScopedDisabler &) = delete;
  ScopedDisabler(ScopedDisabler &&) = delete;
  ScopedDisabler &operator=(ScopedDisabler &&) = delete;
#else
private:
  ScopedDisabler(const ScopedDisabler &);
  ScopedDisabler &operator=(const ScopedDisabler &);
#endif // __cplusplus >= 201103L
};

#else

class ScopedDisabler {
public:
  ScopedDisabler() {}
#if __cplusplus >= 201103L
  ScopedDisabler(const ScopedDisabler &) = delete;
  ScopedDisabler &operator=(const ScopedDisabler &) = delete;
  ScopedDisabler(ScopedDisabler &&) = delete;
  ScopedDisabler &operator=(ScopedDisabler &&) = delete;
#else
private:
  ScopedDisabler(const ScopedDisabler &);
  ScopedDisabler &operator=(const ScopedDisabler &);
#endif // __cplusplus >= 201103L
};

#endif // defined(__has_feature) && __has_feature(realtime_sanitizer)
} // namespace __rtsan
#endif // __cplusplus

#endif // SANITIZER_RTSAN_INTERFACE_H

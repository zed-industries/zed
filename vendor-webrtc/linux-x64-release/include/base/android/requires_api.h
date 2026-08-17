// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_REQUIRES_API_H_
#define BASE_ANDROID_REQUIRES_API_H_

// Sets the API version of a symbol. Analogous to how @RequiresApi sets the API
// version of Java symbols.
//
// A compiler warning (-Wunguarded-availability) is emitted when symbols with
// this annotation are reference by code that has a lower API version.
//
// The default API version is set by the default_min_sdk_version GN arg.
//
// To reference a symbol from code with a lower api level, you must use:
// if (__builtin_available(android API_VERSION, *)) { ... }
//
// See also:
// https://android.googlesource.com/platform/ndk/+/master/docs/BuildSystemMaintainers.md#weak-symbols-for-api-definitions
#define REQUIRES_ANDROID_API(x) \
  __attribute__((__availability__(android, introduced = x)))

// Sets the default API version for all symbols.
//
// Usage:
// #pragma clang attribute push DEFAULT_REQUIRES_ANDROID_API(29)
// ...
// #pragma clang attribute pop
//
// For use only within .cc files so that declarations within header files are
// clearly labeled.
#define DEFAULT_REQUIRES_ANDROID_API(x)                                    \
  (REQUIRES_ANDROID_API(x),                                                \
   apply_to = any(enum, enum_constant, field, function, namespace, record, \
                  type_alias, variable))

#endif  // BASE_ANDROID_REQUIRES_API_H_

// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_NATIVE_LIBRARY_TEST_UTILS_H_
#define BASE_TEST_NATIVE_LIBRARY_TEST_UTILS_H_

#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#define NATIVE_LIBRARY_TEST_ALWAYS_EXPORT __declspec(dllexport)
#else
#define NATIVE_LIBRARY_TEST_ALWAYS_EXPORT __attribute__((visibility("default")))
#endif

extern "C" {

extern NATIVE_LIBRARY_TEST_ALWAYS_EXPORT int g_native_library_exported_value;

// A function which increments an internal counter value and returns its value.
// The first call returns 1, then 2, etc.
NATIVE_LIBRARY_TEST_ALWAYS_EXPORT int NativeLibraryTestIncrement();

// A function which resets the internal counter value to 0.
NATIVE_LIBRARY_TEST_ALWAYS_EXPORT void NativeLibraryResetCounter();

}  // extern "C"

#endif  // BASE_TEST_NATIVE_LIBRARY_TEST_UTILS_H_

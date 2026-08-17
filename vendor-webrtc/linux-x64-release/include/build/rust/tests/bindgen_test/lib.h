// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BUILD_RUST_TESTS_BINDGEN_TEST_LIB_H_
#define BUILD_RUST_TESTS_BINDGEN_TEST_LIB_H_

#include "build/rust/tests/bindgen_test/lib2.h"

#include <stdint.h>

// The following is equivalent to //base/base_export.h.

#if defined(COMPONENT_BUILD)
#if defined(WIN32)

#if defined(COMPONENT_IMPLEMENTATION)
#define COMPONENT_EXPORT __declspec(dllexport)
#else
#define COMPONENT_EXPORT __declspec(dllimport)
#endif  // defined(COMPONENT_IMPLEMENTATION)

#else  // defined(WIN32)
#if defined(COMPONENT_IMPLEMENTATION)
#define COMPONENT_EXPORT __attribute__((visibility("default")))
#else
#define COMPONENT_EXPORT
#endif  // defined(COMPONENT_IMPLEMENTATION)
#endif

#else  // defined(COMPONENT_BUILD)
#define COMPONENT_EXPORT
#endif

#ifdef __cplusplus
extern "C" {
#endif

COMPONENT_EXPORT uint32_t add_two_numbers(uint32_t a, uint32_t b);

#ifdef __cplusplus
}
#endif

#endif  //  BUILD_RUST_TESTS_BINDGEN_TEST_LIB_H_

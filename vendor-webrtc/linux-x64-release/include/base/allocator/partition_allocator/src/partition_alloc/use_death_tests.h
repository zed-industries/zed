// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_USE_DEATH_TESTS_H_
#define PARTITION_ALLOC_USE_DEATH_TESTS_H_

// Test-only header. This must be separated from the main body of
// `partition_alloc_config.h` because the preprocessor evaluates it too
// early, leaving `GTEST_HAS_DEATH_TEST` undefined. This makes the
// corresponding `PA_CONFIG()` yield the wrong result.

#include "partition_alloc/build_config.h"

// `GTEST_HAS_DEATH_TEST` is `#define`d by Googletest headers.
// Therefore, Googletest headers must be textually evaluated before this
// one, or else `GTEST_HAS_DEATH_TEST` will probably remain undefined.
#include "testing/gtest/include/gtest/gtest.h"

// An informal CQ survey
// (https://chromium-review.googlesource.com/c/chromium/src/+/5493422/1?tab=checks)
// tells us that iOS doesn't define `GTEST_HAS_DEATH_TEST`.
//
// In addition, Android is generally thought not to support death tests.
// See also:
// *  crbug.com/40483507
// *  crbug.com/40785222
// *  crbug.com/41372437
#if defined(GTEST_HAS_DEATH_TEST)
#define PA_USE_DEATH_TESTS() (!PA_BUILDFLAG(IS_ANDROID))
#else
#define PA_USE_DEATH_TESTS() 0
#endif

#endif  // PARTITION_ALLOC_USE_DEATH_TESTS_H_

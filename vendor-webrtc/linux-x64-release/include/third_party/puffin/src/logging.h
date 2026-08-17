// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_LOGGING_H_
#define SRC_LOGGING_H_

#include "base/check.h"
#include "base/logging.h"

#define TEST_AND_RETURN_FALSE(_x)    \
  do {                               \
    if (!(_x)) {                     \
      VLOG(1) << #_x " failed.";     \
      return false;                  \
    }                                \
  } while (0)

#define TEST_AND_RETURN_VALUE(_x, _v) \
  do {                                \
    if (!(_x)) {                      \
      VLOG(1) << #_x " failed.";      \
      return (_v);                    \
    }                                 \
  } while (0)

#endif  // SRC_LOGGING_H_

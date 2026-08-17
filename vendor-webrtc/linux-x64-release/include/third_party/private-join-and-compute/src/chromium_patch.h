// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <ostream>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/logging.h"

#ifndef THIRD_PARTY_PRIVATE_JOIN_AND_COMPUTE_CHROMIUM_PATCH_H_
#define THIRD_PARTY_PRIVATE_JOIN_AND_COMPUTE_CHROMIUM_PATCH_H_

namespace chromium_patch {

// Replacement for glog macro.
template <typename T>
inline T CheckNotNull(const char* names, T&& t) {
  CHECK(t) << names;
  return std::forward<T>(t);
}

}  // namespace chromium_patch

#define CHECK_NOTNULL(val) \
  ::chromium_patch::CheckNotNull("'" #val "' Must be non nullptr", (val))

#endif  // THIRD_PARTY_PRIVATE_JOIN_AND_COMPUTE_CHROMIUM_PATCH_H_

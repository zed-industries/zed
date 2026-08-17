// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_TENSORFLOW_TEXT_SHIMS_TENSORFLOW_CORE_LIB_CORE_STATUS_H_
#define THIRD_PARTY_TENSORFLOW_TEXT_SHIMS_TENSORFLOW_CORE_LIB_CORE_STATUS_H_

#include "absl/status/status.h"

namespace tensorflow {
using Status = ::absl::Status;
}

#endif  // THIRD_PARTY_TENSORFLOW_TEXT_SHIMS_TENSORFLOW_CORE_LIB_CORE_STATUS_H_

// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
#ifndef GOOGLE_PROTOBUF_INTERNAL_VISIBILITY_FOR_TESTING_H__
#define GOOGLE_PROTOBUF_INTERNAL_VISIBILITY_FOR_TESTING_H__

#include "google/protobuf/internal_visibility.h"

namespace google {
namespace protobuf {
namespace internal {

class InternalVisibilityForTesting {
 public:
  operator InternalVisibility() const {  // NOLINT
    return InternalVisibility{};
  }
};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_INTERNAL_VISIBILITY_FOR_TESTING_H__

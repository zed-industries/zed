/*
 * Copyright 2022 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_FLEX_FLAT_UTIL_H_
#define FLATBUFFERS_FLEX_FLAT_UTIL_H_

#include "flatbuffers/flatbuffers.h"
#include "flatbuffers/flexbuffers.h"

namespace flexbuffers {

// Verifies the `nested` flexbuffer within a flatbuffer vector is valid.
inline bool VerifyNestedFlexBuffer(
    const flatbuffers::Vector<uint8_t> *const nested,
    flatbuffers::Verifier &verifier) {
  if (!nested) return true;
  return verifier.Check(flexbuffers::VerifyBuffer(
      nested->data(), nested->size(), verifier.GetFlexReuseTracker()));
}

}  // namespace flexbuffers

#endif  // FLATBUFFERS_FLEX_FLAT_UTIL_H_

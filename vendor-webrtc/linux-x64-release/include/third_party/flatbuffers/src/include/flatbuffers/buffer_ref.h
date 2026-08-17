/*
 * Copyright 2021 Google Inc. All rights reserved.
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

#ifndef FLATBUFFERS_BUFFER_REF_H_
#define FLATBUFFERS_BUFFER_REF_H_

#include "flatbuffers/base.h"
#include "flatbuffers/verifier.h"

namespace flatbuffers {

// Convenient way to bundle a buffer and its length, to pass it around
// typed by its root.
// A BufferRef does not own its buffer.
struct BufferRefBase {};  // for std::is_base_of

template<typename T> struct BufferRef : BufferRefBase {
  BufferRef() : buf(nullptr), len(0), must_free(false) {}
  BufferRef(uint8_t *_buf, uoffset_t _len)
      : buf(_buf), len(_len), must_free(false) {}

  ~BufferRef() {
    if (must_free) free(buf);
  }

  const T *GetRoot() const { return flatbuffers::GetRoot<T>(buf); }

  bool Verify() {
    Verifier verifier(buf, len);
    return verifier.VerifyBuffer<T>(nullptr);
  }

  uint8_t *buf;
  uoffset_t len;
  bool must_free;
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_BUFFER_REF_H_

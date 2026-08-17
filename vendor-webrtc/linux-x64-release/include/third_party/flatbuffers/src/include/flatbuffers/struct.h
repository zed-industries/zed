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

#ifndef FLATBUFFERS_STRUCT_H_
#define FLATBUFFERS_STRUCT_H_

#include "flatbuffers/base.h"

namespace flatbuffers {

// "structs" are flat structures that do not have an offset table, thus
// always have all members present and do not support forwards/backwards
// compatible extensions.

class Struct FLATBUFFERS_FINAL_CLASS {
 public:
  template<typename T> T GetField(uoffset_t o) const {
    return ReadScalar<T>(&data_[o]);
  }

  template<typename T> T GetStruct(uoffset_t o) const {
    return reinterpret_cast<T>(&data_[o]);
  }

  const uint8_t *GetAddressOf(uoffset_t o) const { return &data_[o]; }
  uint8_t *GetAddressOf(uoffset_t o) { return &data_[o]; }

 private:
  // private constructor & copy constructor: you obtain instances of this
  // class by pointing to existing data only
  Struct();
  Struct(const Struct &);
  Struct &operator=(const Struct &);

  uint8_t data_[1];
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_STRUCT_H_

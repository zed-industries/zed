// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef ANONYMOUS_TOKENS_CPP_TESTING_ANONYMOUS_TOKENS_TESTDATA_UTILS_H_
#define ANONYMOUS_TOKENS_CPP_TESTING_ANONYMOUS_TOKENS_TESTDATA_UTILS_H_

#include <string>

#include "third_party/anonymous_tokens/testdata_utils_impl.h"

namespace anonymous_tokens {

inline std::string GetTestdataPath() {
  return GetTestdataPathImpl();
}

}  // namespace anonymous_tokens

#endif  // ANONYMOUS_TOKENS_CPP_TESTING_ANONYMOUS_TOKENS_TESTDATA_UTILS_H_

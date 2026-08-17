// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_DEPS_RANDOM_BASE_H_
#define MEDIAPIPE_DEPS_RANDOM_BASE_H_

#include <cstdint>

class RandomBase {
 public:
  // constructors.  Don't do too much.
  RandomBase() {}
  virtual ~RandomBase();

  virtual float RandFloat() { return 0; }
  virtual int UnbiasedUniform(int n) { return 0; }
  virtual uint64_t UnbiasedUniform64(uint64_t n) { return 0; }
};

#endif  // MEDIAPIPE_DEPS_RANDOM_BASE_H_

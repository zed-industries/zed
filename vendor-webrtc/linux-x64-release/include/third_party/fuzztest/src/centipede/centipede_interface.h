// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_CENTIPEDE_INTERFACE_H_
#define THIRD_PARTY_CENTIPEDE_CENTIPEDE_INTERFACE_H_

#include "./centipede/centipede_callbacks.h"
#include "./centipede/environment.h"

namespace fuzztest::internal {

// Usage:
//   class MyCentipedeCallbacks: public CentipedeCallbacks { ... }
//   int main(int argc, char **argv) {
//     InitGoogle(argv[0], &argc, &argv, /*remove_flags=*/true);
//     fuzztest::internal::Environment env;  // reads FLAGS.
//     fuzztest::internal::DefaultCallbacksFactory<MyCentipedeCallbacks>
//     callbacks_factory; return fuzztest::internal::CentipedeMain(env,
//     callbacks_factory);
//   }
int CentipedeMain(const Environment &env,
                  CentipedeCallbacksFactory &callbacks_factory);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_CENTIPEDE_INTERFACE_H_

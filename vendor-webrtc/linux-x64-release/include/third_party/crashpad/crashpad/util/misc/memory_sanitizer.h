// Copyright 2019 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_UTIL_MISC_MEMORY_SANITIZER_H_
#define CRASHPAD_UTIL_MISC_MEMORY_SANITIZER_H_

#include "base/compiler_specific.h"
#include "build/build_config.h"

#if !defined(MEMORY_SANITIZER)
#if HAS_FEATURE(memory_sanitizer)
#define MEMORY_SANITIZER 1
#endif  // HAS_FEATURE(memory_sanitizer)
#endif  // !defined(MEMORY_SANITIZER)

#endif  // CRASHPAD_UTIL_MISC_MEMORY_SANITIZER_H_

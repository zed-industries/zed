/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef RUSTC_DEMANGLE_H_
#define RUSTC_DEMANGLE_H_

#include <cstddef>

// This is just an empty stub for the rustc-demangle-capi rust crate.
// It is used to build libunwindstack in the perfetto standalone build.

static inline char* rustc_demangle(const char*, char*, size_t*, int*) {
  return nullptr;
}

#endif  // RUSTC_DEMANGLE_H_

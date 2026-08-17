/* Copyright 2023 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/
// Forked from a library written by Austin Appelby and Jyrki Alakuijala.
// Original copyright message below.
// Copyright 2009 Google Inc.
// Author: aappleby@google.com (Austin Appelby)
//         jyrki@google.com (Jyrki Alakuijala)
//
// MurmurHash is a fast multiplication and shifting based algorithm,
// based on Austin Appleby's MurmurHash 2.0 algorithm.

#ifndef UTIL_HASH_MURMUR_H_
#define UTIL_HASH_MURMUR_H_

#include <stddef.h>
#include <stdlib.h>  // for size_t.

#include <cstdint>

namespace mediapipe::tasks::text::language_detector::custom_ops::hash {

// Hash function for a byte array. Has a seed which allows this hash function to
// be used in algorithms that need a family of parameterized hash functions.
// e.g. Minhash.
unsigned long long MurmurHash64WithSeed(const char* buf, size_t len,  // NOLINT
                                        uint64_t seed);
}  // namespace mediapipe::tasks::text::language_detector::custom_ops::hash

#endif  // UTIL_HASH_MURMUR_H_

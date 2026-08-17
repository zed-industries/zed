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

#ifndef MEDIAPIPE_UTIL_TFLITE_CPU_OP_RESOLVER_H_
#define MEDIAPIPE_UTIL_TFLITE_CPU_OP_RESOLVER_H_

#include "tensorflow/lite/kernels/register.h"

namespace mediapipe {

// This function registers the CPU implementations for following custom ops:
// "Convolution2DTransposeBias"
// "MaxPoolArgmax"
// "MaxUnpooling"
extern "C" void MediaPipe_RegisterTfLiteOpResolver(tflite::MutableOpResolver*);

// This resolver is used for the custom ops introduced by
// `MediaPipe_RegisterTfLiteOpResolver` (see above).
class CpuOpResolver
    : public tflite::ops::builtin::BuiltinOpResolverWithoutDefaultDelegates {
 public:
  CpuOpResolver() { MediaPipe_RegisterTfLiteOpResolver(this); }
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TFLITE_CPU_OP_RESOLVER_H_

// Copyright 2020 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_UTIL_FILTERING_LOW_PASS_FILTER_H_
#define MEDIAPIPE_UTIL_FILTERING_LOW_PASS_FILTER_H_

#include <memory>

namespace mediapipe {

class LowPassFilter {
 public:
  explicit LowPassFilter(float alpha);

  float Apply(float value);

  float ApplyWithAlpha(float value, float alpha);

  bool HasLastRawValue();

  float LastRawValue();

  float LastValue();

 private:
  void SetAlpha(float alpha);

  float raw_value_;
  float alpha_;
  float stored_value_;
  bool initialized_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_FILTERING_LOW_PASS_FILTER_H_

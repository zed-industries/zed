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

#ifndef FUZZTEST_COMMON_LOGGING_H_
#define FUZZTEST_COMMON_LOGGING_H_

// TODO(b/315519925): Temporary leftover from switching to now-available
//  OSS Abseil VLOG and friends. Explicitly include these wherever necessary and
//  remove from here.
#include "absl/log/check.h"  // IWYU pragma: keep
#include "absl/log/log.h"    // IWYU pragma: keep

// Easy variable value logging: LOG(INFO) << VV(foo) << VV(bar);
#define VV(x) #x ": " << (x) << " "

#endif  // FUZZTEST_COMMON_LOGGING_H_

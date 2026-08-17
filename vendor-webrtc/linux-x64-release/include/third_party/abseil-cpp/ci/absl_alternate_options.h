// Copyright 2019 The Abseil Authors.
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

// Alternate options.h file, used in continuous integration testing to exercise
// option settings not used by default.

#ifndef ABSL_CI_ABSL_ALTERNATE_OPTIONS_H_
#define ABSL_CI_ABSL_ALTERNATE_OPTIONS_H_

#define ABSL_OPTION_USE_STD_ANY 0
#define ABSL_OPTION_USE_STD_OPTIONAL 0
#define ABSL_OPTION_USE_STD_STRING_VIEW 0
#define ABSL_OPTION_USE_STD_VARIANT 0
#define ABSL_OPTION_USE_STD_ORDERING 0
#define ABSL_OPTION_USE_INLINE_NAMESPACE 1
#define ABSL_OPTION_INLINE_NAMESPACE_NAME ns
#define ABSL_OPTION_HARDENED 1

#endif  // ABSL_CI_ABSL_ALTERNATE_OPTIONS_H_

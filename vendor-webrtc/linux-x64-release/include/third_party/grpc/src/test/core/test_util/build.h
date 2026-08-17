// Copyright 2021 gRPC authors.
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

#ifndef GRPC_TEST_CORE_TEST_UTIL_BUILD_H
#define GRPC_TEST_CORE_TEST_UTIL_BUILD_H

// Returns whether this is built using our Valgrind config
bool BuiltUnderValgrind();

// Returns whether this is built under ThreadSanitizer
bool BuiltUnderTsan();

// Returns whether this is built under AddressSanitizer
bool BuiltUnderAsan();

// Returns whether this is built under MemorySanitizer
bool BuiltUnderMsan();

// Returns whether this is built under UndefinedBehaviorSanitizer
bool BuiltUnderUbsan();

// Force a leak check if built under ASAN. If there are leaks, crash.
void AsanAssertNoLeaks();

#endif  // GRPC_TEST_CORE_TEST_UTIL_BUILD_H

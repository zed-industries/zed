// Copyright 2024 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_CRYPTO_TEST_TEST_DATA_H
#define OPENSSL_HEADER_CRYPTO_TEST_TEST_DATA_H

#include <string>

// GetTestData returns the test data for |path|, or aborts on error. |path|
// must be a slash-separated path, relative to the BoringSSL source tree. By
// default, this is implemented by reading from the filesystem, relative to
// the BORINGSSL_TEST_DATA_ROOT environment variable, or the current working
// directory if unset.
//
// Callers with more complex needs can build with
// BORINGSSL_CUSTOM_GET_TEST_DATA and then link in an alternate implementation
// of this function.
//
// Callers running from Bazel can define BORINGSSL_USE_BAZEL_RUNFILES to use
// the Bazel runfiles library.
std::string GetTestData(const char *path);

#endif  // OPENSSL_HEADER_CRYPTO_TEST_TEST_DATA_H

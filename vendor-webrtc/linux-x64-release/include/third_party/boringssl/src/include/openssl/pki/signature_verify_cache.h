// Copyright 2022 The Chromium Authors
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

#if !defined(BSSL_PKI_SIGNATURE_VERIFY_CACHE_H_) && defined(__cplusplus)
#define BSSL_PKI_SIGNATURE_VERIFY_CACHE_H_

#include <openssl/base.h>   // IWYU pragma: export
#include <string>

BSSL_NAMESPACE_BEGIN

class OPENSSL_EXPORT SignatureVerifyCache {
 public:
  enum class Value {
    kValid,    // Cached as a valid signature result.
    kInvalid,  // Cached as an invalid signature result.
    kUnknown,  // Cache has no information.
  };

  virtual ~SignatureVerifyCache() = default;

  // This interface uses a const std::string reference instead of
  // std::string_view because any implementation that may reasonably want to use
  // std::unordered_map or similar can run into problems with std::hash before
  // C++20. (https://en.cppreference.com/w/cpp/container/unordered_map/find)

  // |Store| is called to store the result of a verification for |key| as kValid
  // or kInvalid after a signature check.
  virtual void Store(const std::string &key, Value value) = 0;

  // |Check| is called to fetch a cached value for a verification for |key|. If
  // the result is kValid, or kInvalid, signature checking is skipped and the
  // corresponding cached result is used.  If the result is kUnknown signature
  // checking is performed and the corresponding result saved using |Store|.
  virtual Value Check(const std::string &key) = 0;
};

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_SIGNATURE_VERIFY_CACHE_H_ && __cplusplus

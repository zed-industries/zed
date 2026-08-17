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

#ifndef BSSL_PKI_MOCK_SIGNATURE_VERIFY_CACHE_H_
#define BSSL_PKI_MOCK_SIGNATURE_VERIFY_CACHE_H_

#include <stddef.h>

#include <string>
#include <string_view>
#include <unordered_map>

#include <openssl/pki/signature_verify_cache.h>

BSSL_NAMESPACE_BEGIN

// MockSignatureVerifyCache is an implementation of SignatureVerifyCache.  It is
// intended only for testing of cache functionality.

class MockSignatureVerifyCache : public SignatureVerifyCache {
 public:
  MockSignatureVerifyCache();

  ~MockSignatureVerifyCache() override;

  void Store(const std::string &key,
             SignatureVerifyCache::Value value) override;

  SignatureVerifyCache::Value Check(const std::string &key) override;

  size_t CacheHits() { return hits_; }

  size_t CacheMisses() { return misses_; }

  size_t CacheStores() { return stores_; }

 private:
  std::unordered_map<std::string, SignatureVerifyCache::Value> cache_;
  size_t hits_ = 0;
  size_t misses_ = 0;
  size_t stores_ = 0;
};

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_MOCK_PATH_BUILDER_DELEGATE_H_

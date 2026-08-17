// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_INTEGRITY_METADATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_INTEGRITY_METADATA_H_

#include "services/network/public/mojom/integrity_algorithm.mojom-blink.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hasher.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

using IntegrityAlgorithm = network::mojom::blink::IntegrityAlgorithm;

struct PLATFORM_EXPORT IntegrityMetadata {
  IntegrityMetadata() = default;
  IntegrityMetadata(String digest, IntegrityAlgorithm);

  bool operator==(const IntegrityMetadata& other) const {
    return this->digest == other.digest && this->algorithm == other.algorithm;
  }

  String digest;
  IntegrityAlgorithm algorithm;
};

// Contains the result of SRI's "Parse Metadata" algorithm:
//
// https://wicg.github.io/signature-based-sri/#abstract-opdef-parse-metadata
struct PLATFORM_EXPORT IntegrityMetadataSet {
  IntegrityMetadataSet() = default;
  bool empty() const { return hashes.empty() && public_keys.empty(); }
  WTF::Vector<IntegrityMetadata> hashes;
  WTF::Vector<IntegrityMetadata> public_keys;

  void Insert(IntegrityMetadata pair);

  bool operator==(const IntegrityMetadataSet& other) const {
    return this->hashes == other.hashes &&
           this->public_keys == other.public_keys;
  }
};

enum class ResourceIntegrityDisposition : uint8_t {
  kNotChecked = 0,
  kNetworkError,
  kFailedUnencodedDigest,
  kFailedIntegrityMetadata,
  kPassed
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_INTEGRITY_METADATA_H_

// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_UNENCODED_DIGEST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_UNENCODED_DIGEST_H_

#include <optional>
#include <vector>

#include "third_party/blink/renderer/platform/loader/fetch/integrity_metadata.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace WTF {
class SegmentedBuffer;
}  // namespace WTF

namespace blink {

class HTTPHeaderMap;

// Represent's a resource's `Unencoded-Digest` response headers.
//
// https://datatracker.ietf.org/doc/draft-pardue-http-identity-digest/
class PLATFORM_EXPORT UnencodedDigest {
 public:
  ~UnencodedDigest() = default;

  // Parses an HTTPHeaderMap, returning an `UnencodedDigest` if a valid
  // `Unencoded-Digest` header is present, and `std::nullopt` otherwise.
  static std::optional<UnencodedDigest> Create(const HTTPHeaderMap&);

  const WTF::Vector<IntegrityMetadata>& digests() const {
    return integrity_metadata_.hashes;
  }

  // Validates |data| against all parsed digests, returning `true` if all match,
  // and `false` otherwise.
  bool DoesMatch(WTF::SegmentedBuffer* data);

 private:
  UnencodedDigest() = default;
  explicit UnencodedDigest(IntegrityMetadataSet);

  IntegrityMetadataSet integrity_metadata_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_UNENCODED_DIGEST_H_

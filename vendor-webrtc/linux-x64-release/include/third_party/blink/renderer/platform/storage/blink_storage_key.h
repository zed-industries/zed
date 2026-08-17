// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_STORAGE_BLINK_STORAGE_KEY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_STORAGE_BLINK_STORAGE_KEY_H_

#include <iosfwd>
#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/unguessable_token.h"
#include "third_party/blink/public/common/storage_key/storage_key.h"
#include "third_party/blink/public/mojom/storage_key/ancestor_chain_bit.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/network/blink_schemeful_site.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// This class represents the key by which DOM Storage keys its
// CachedStorageAreas.
// It is typemapped to blink.mojom.StorageKey, and should stay in sync with
// blink::StorageKey (third_party/blink/public/common/storage_key/storage_key.h)
class PLATFORM_EXPORT BlinkStorageKey {

 public:
  // [Block 1 - Constructors] - Keep in sync with StorageKey.

  // (1A) Construct with a unique, opaque, origin and top_level_site.
  // This should be used only in tests or where memory must be initialized
  // before the context of some frame is known.
  BlinkStorageKey();

  // (1B) Construct a first-party (origin and top_level_site match) key.
  // This should be used only in contexts verified to be first-party or where
  // a third-party context is impossible, otherwise use Create().
  static BlinkStorageKey CreateFirstParty(
      scoped_refptr<const SecurityOrigin> origin);

  // (1C) Construct for an ephemeral browsing context with a nonce.
  // This is a common entry point when constructing a context, and callsites
  // generally must branch and call Create() if a nonce isn't set.
  static BlinkStorageKey CreateWithNonce(
      scoped_refptr<const SecurityOrigin> origin,
      const base::UnguessableToken& nonce);

  // (1D) Construct for a specific first or third party context.
  // This is a common entry point when constructing a context, and callsites
  // generally must branch and call CreateWithNonce() if a nonce is set.
  static BlinkStorageKey Create(
      scoped_refptr<const SecurityOrigin> origin,
      const BlinkSchemefulSite& top_level_site,
      mojom::blink::AncestorChainBit ancestor_chain_bit);

  // (1E) Construct for the provided isolation_info.
  // Only in StorageKey, but could be added if needed.

  // (1F) Construct a first-party storage key for tests.
  static BlinkStorageKey CreateFromStringForTesting(const WTF::String& origin);

  // (1G) Copy, move, and destruct.
  BlinkStorageKey(const BlinkStorageKey& other) = default;
  BlinkStorageKey& operator=(const BlinkStorageKey& other) = default;
  BlinkStorageKey(BlinkStorageKey&& other) = default;
  BlinkStorageKey& operator=(BlinkStorageKey&& other) = default;
  ~BlinkStorageKey() = default;

  // [Block 2 - Side Loaders] - Keep in sync with StorageKey.

  // (2A) Return a copy updated as though origin was used in construction.
  // Note that if a nonce is set this may update the top_level_site* and if
  // a nonce isn't set this may update the ancestor_chain_bit*.
  BlinkStorageKey WithOrigin(scoped_refptr<const SecurityOrigin> origin) const;

  // (2B) Return a copy updated as though storage partitioning was enabled.
  // Returns a copy of what this storage key would have been if
  // `kThirdPartyStoragePartitioning` were enabled. This is a convenience
  // function for callsites that benefit from future functionality.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  BlinkStorageKey CopyWithForceEnabledThirdPartyStoragePartitioning() const {
    BlinkStorageKey storage_key = *this;
    storage_key.top_level_site_ =
        storage_key.top_level_site_if_third_party_enabled_;
    storage_key.ancestor_chain_bit_ =
        storage_key.ancestor_chain_bit_if_third_party_enabled_;
    DCHECK(storage_key.IsValid());
    return storage_key;
  }

  // [Block 3 - Serialization] - Keep in sync with StorageKey.

  // (3A) Conversion from StorageKey to BlinkStorageKey.
  // NOLINTNEXTLINE(google-explicit-constructor)
  BlinkStorageKey(const StorageKey& storage_key);

  // (3B) Conversion from BlinkStorageKey to StorageKey.
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator StorageKey() const;

  // (3C) Conversion from Mojom values into `out`.
  // Note that if false is returned the combinations of values would not
  // construct a well-formed StorageKey and `out` was not touched.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  static bool FromWire(
      scoped_refptr<const SecurityOrigin> origin,
      const BlinkSchemefulSite& top_level_site,
      const BlinkSchemefulSite& top_level_site_if_third_party_enabled,
      const std::optional<base::UnguessableToken>& nonce,
      mojom::blink::AncestorChainBit ancestor_chain_bit,
      mojom::blink::AncestorChainBit ancestor_chain_bit_if_third_party_enabled,
      BlinkStorageKey& out);

  // (3D) Deserialization from string.
  // Only in StorageKey.

  // (3E) Serialization to string.
  // Only in StorageKey.

  // [Block 4 - Accessors] - Keep in sync with StorageKey.

  const scoped_refptr<const SecurityOrigin>& GetSecurityOrigin() const {
    return origin_;
  }

  const BlinkSchemefulSite& GetTopLevelSite() const { return top_level_site_; }

  // Returns true if unpartitioned storage access is forbidden for the current
  // storage key.
  bool ForbidsUnpartitionedStorageAccess() const { return nonce_.has_value(); }

  const std::optional<base::UnguessableToken>& GetNonce() const {
    return nonce_;
  }

  mojom::blink::AncestorChainBit GetAncestorChainBit() const {
    return ancestor_chain_bit_;
  }

  // `IsFirstPartyContext` returns true if the StorageKey is for a context that
  // is "first-party", i.e. the StorageKey's top-level site and origin have
  // the same scheme and domain, and all intervening frames in the frame tree
  // are first-party.
  //
  // `IsThirdPartyContext` returns true if the StorageKey is for a context that
  // is "third-party", i.e. the StorageKey's top-level site and origin have
  // different schemes and/or domains, or an intervening frame in the frame
  // tree is third-party. StorageKeys created using a nonce instead of a
  // top-level site will also be considered third-party.
  bool IsFirstPartyContext() const {
    return ancestor_chain_bit_ == mojom::blink::AncestorChainBit::kSameSite;
  }
  bool IsThirdPartyContext() const { return !IsFirstPartyContext(); }

  // [Block 5 - Shared Utility] - Keep in sync with StorageKey.

  // (5A) Serialize to string for use in debugging only.
  String ToDebugString() const;

  // (5B) Check exact match for testing only.
  // Checks if every single member in this key matches those in `other`.
  // Since the *_if_third_party_enabled_ fields aren't used normally
  // this function is only useful for testing purposes.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  bool ExactMatchForTesting(const blink::BlinkStorageKey& other) const;

  // [Block 6 - Other Utility] - These don't exist in StorageKey.

  // Remove this comment if any are added.

 private:
  // [Block 7 - Private Methods] - Keep in sync with StorageKey.

  // (7A) Internal constructor for custom values.
  // Note: Other than the opaque and copy/move constructors, this should be the
  // only non-static method for initializing a storage key to keep consistency.
  BlinkStorageKey(scoped_refptr<const SecurityOrigin> origin,
                  const BlinkSchemefulSite& top_level_site,
                  const base::UnguessableToken* nonce,
                  mojom::blink::AncestorChainBit ancestor_chain_bit);

  // (7B) Operators.
  // Note that not all must be friends, but all are to consolidate the header.
  friend bool operator==(const BlinkStorageKey& lhs,
                         const BlinkStorageKey& rhs) {
    DCHECK(lhs.origin_);
    DCHECK(rhs.origin_);

    return lhs.origin_->IsSameOriginWith(rhs.origin_.get()) &&
           lhs.nonce_ == rhs.nonce_ &&
           lhs.top_level_site_ == rhs.top_level_site_ &&
           lhs.ancestor_chain_bit_ == rhs.ancestor_chain_bit_;
  }
  // If there were a need for `operator<=>()` it would go here.
  PLATFORM_EXPORT
  friend std::ostream& operator<<(std::ostream& ostream,
                                  const BlinkStorageKey& sk);

  // (7C) Check validity of current storage key members.
  // This should be used when constructing, side-loading, and deserializing
  // a key to ensure correctness. This does not imply that the key is
  // serializable as keys with opaque origins will still return true.
  bool IsValid() const;

  // [Block 8 - Private Members] - Keep in sync with StorageKey.

  // The current site in the given context. BlinkStorageKey is generally
  // passed in contexts which used to pass SecurityOrigin before partitioning.
  scoped_refptr<const SecurityOrigin> origin_;

  // The "top-level site"/"top-level frame"/"main frame" of the context
  // this BlinkStorageKey was created for (for storage partitioning purposes).
  // For extensions or related enterprise policies this may not represent the
  // top-level site. For contexts with a `nonce_` or contexts without storage
  // partitioning enabled, this will be the eTLD+1 of `origin_`.
  BlinkSchemefulSite top_level_site_;

  // Stores the value `top_level_site_` would have had if
  // `kThirdPartyStoragePartitioning` were enabled. This isn't used in
  // serialization or comparison.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  BlinkSchemefulSite top_level_site_if_third_party_enabled_ = top_level_site_;

  // Optional, forcing partitioned storage and used by anonymous iframes:
  // https://github.com/camillelamy/explainers/blob/master/anonymous_iframes.md
  std::optional<base::UnguessableToken> nonce_;

  // kSameSite if the entire ancestor chain is same-site with the current frame.
  // kCrossSite otherwise. Used by service workers.
  mojom::blink::AncestorChainBit ancestor_chain_bit_{
      mojom::blink::AncestorChainBit::kCrossSite};

  // Stores the value `ancestor_chain_bit_` would have had if
  // `kThirdPartyStoragePartitioning` were enabled. This isn't used in
  // serialization or comparison.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  mojom::blink::AncestorChainBit ancestor_chain_bit_if_third_party_enabled_ =
      ancestor_chain_bit_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_STORAGE_BLINK_STORAGE_KEY_H_

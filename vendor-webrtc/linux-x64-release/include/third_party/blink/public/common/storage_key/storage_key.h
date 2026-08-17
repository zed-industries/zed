// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_STORAGE_KEY_STORAGE_KEY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_STORAGE_KEY_STORAGE_KEY_H_

#include <iosfwd>
#include <optional>
#include <string>
#include <string_view>
#include <tuple>

#include "base/unguessable_token.h"
#include "net/base/isolation_info.h"
#include "net/base/schemeful_site.h"
#include "net/cookies/cookie_partition_key.h"
#include "net/cookies/site_for_cookies.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/storage_key/ancestor_chain_bit.mojom.h"
#include "url/origin.h"

namespace blink {

// A class used by Storage APIs as a key for storage. An entity with a given
// storage key may not access data keyed with any other storage key.
//
// When third party storage partitioning is disabled, a StorageKey is equivalent
// to an origin, which is how storage has historically been partitioned.
//
// When third party storage partitioning is enabled, a storage key additionally
// contains a top-level site and an ancestor chain bit (see below). This
// achieves partitioning of an origin by the top-level site that the frame is
// embedded in. For example, https://chat.example.net embedded in
// https://social-example.org is a distinct key from https://chat.example.net
// embedded in https://news-example.org.
//
// A key is a third-party key if its origin is not in its top-level site (or if
// its ancestor chain bit is `kCrossSite`; see below); otherwise the key is a
// first-party key and the ancestor chain bit is `kSameSite`.
//
// A corner-case is a first-party origin embedded in a third-party origin, such
// as https://a.com embedded in https://b.com in https://a.com. The inner
// `a.com` frame can be controlled by `b.com`, and is thus considered
// third-party. The ancestor chain bit tracks this status.
//
// Storage keys can also optionally have a nonce. Keys with different nonces are
// considered distinct, and distinct from a key with no nonce. This is used to
// implement iframe credentialless and other forms of storage partitioning.
// Keys with a nonce disregard the top level site and ancestor chain bit. For
// consistency we set them to the origin's site and `kCrossSite` respectively.
//
// Storage keys might have an opaque top level site (for example, if an
// iframe is embedded in a data url). These storage keys always have a
// `kCrossSite` ancestor chain bit as there is no need to distinguish their
// partitions based on frame ancestry.
//
// Storage keys might have a top level site and origin that don't match. These
// storage keys always have a `kCrossSite` ancestor chain bit.
//
// Storage keys might have an opaque origin (for example, data urls). These
// storage keys always have a `kCrossSite` ancestor chain bit as there is no
// need to distinguish their partitions based on frame ancestry. These storage
// keys cannot be serialized.
//
// For more details on the overall design, see
// https://docs.google.com/document/d/1xd6MXcUhfnZqIe5dt2CTyCn6gEZ7nOezAEWS0W9hwbQ/edit.
//
// This class is typemapped to blink.mojom.StorageKey, and should stay in sync
// with BlinkStorageKey
// (third_party/blink/renderer/platform/storage/blink_storage_key.h)
class BLINK_COMMON_EXPORT StorageKey {
 public:
  // [Block 1 - Constructors] - Keep in sync with BlinkStorageKey.

  // (1A) Construct with a unique, opaque, origin and top_level_site.
  // This should be used only in tests or where memory must be initialized
  // before the context of some frame is known.
  StorageKey() = default;

  // (1B) Construct a first-party (origin and top_level_site match) key.
  // This should be used only in contexts verified to be first-party or where
  // a third-party context is impossible, otherwise use Create().
  static StorageKey CreateFirstParty(const url::Origin& origin);

  // (1C) Construct for an ephemeral browsing context with a nonce.
  // This is a common entry point when constructing a context, and callsites
  // generally must branch and call Create() if a nonce isn't set.
  static StorageKey CreateWithNonce(const url::Origin& origin,
                                    const base::UnguessableToken& nonce);

  // (1D) Construct for a specific first or third party context.
  // This is a common entry point when constructing a context, and callsites
  // generally must branch and call CreateWithNonce() if a nonce is set.
  // TODO(crbug.com/1199077): The default argument here is so tests don't need
  // to be aware of it. Find a solution that removes this default arg.
  static StorageKey Create(const url::Origin& origin,
                           const net::SchemefulSite& top_level_site,
                           blink::mojom::AncestorChainBit ancestor_chain_bit,
                           bool third_party_partitioning_allowed =
                               IsThirdPartyStoragePartitioningEnabled());

  // (1E) Construct for the provided isolation_info.
  // TODO(crbug.com/1346450): This does not account for extension URLs.
  static StorageKey CreateFromOriginAndIsolationInfo(
      const url::Origin& origin,
      const net::IsolationInfo& isolation_info);

  // (1F) Construct a first-party storage key for tests.
  static StorageKey CreateFromStringForTesting(const std::string& origin);

  // (1G) Copy, move, and destruct.
  StorageKey(const StorageKey& other) = default;
  StorageKey& operator=(const StorageKey& other) = default;
  StorageKey(StorageKey&& other) noexcept = default;
  StorageKey& operator=(StorageKey&& other) noexcept = default;
  ~StorageKey() = default;

  // [Block 2 - Side Loaders] - Keep in sync with BlinkStorageKey.

  // (2A) Return a copy updated as though origin was used in construction.
  // Note that if a nonce is set this may update the top_level_site* and if
  // a nonce isn't set this may update the ancestor_chain_bit*.
  StorageKey WithOrigin(const url::Origin& origin) const;

  // (2B) Return a copy updated as though storage partitioning was enabled.
  // Returns a copy of what this storage key would have been if
  // `kThirdPartyStoragePartitioning` were enabled. This is a convenience
  // function for callsites that benefit from future functionality.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  StorageKey CopyWithForceEnabledThirdPartyStoragePartitioning() const {
    StorageKey storage_key = *this;
    storage_key.top_level_site_ =
        storage_key.top_level_site_if_third_party_enabled_;
    storage_key.ancestor_chain_bit_ =
        storage_key.ancestor_chain_bit_if_third_party_enabled_;
    DCHECK(storage_key.IsValid());
    return storage_key;
  }

  // [Block 3 - Serialization] - Keep in sync with StorageKey.

  // (3A) Conversion from StorageKey to BlinkStorageKey.
  // Only in BlinkStorageKey.

  // (3B) Conversion from BlinkStorageKey to StorageKey.
  // Only in BlinkStorageKey.

  // (3C) Conversion from Mojom values into `out`.
  // Note that if false is returned the combinations of values would not
  // construct a well-formed StorageKey and `out` was not touched.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  static bool FromWire(
      const url::Origin& origin,
      const net::SchemefulSite& top_level_site,
      const net::SchemefulSite& top_level_site_if_third_party_enabled,
      const std::optional<base::UnguessableToken>& nonce,
      blink::mojom::AncestorChainBit ancestor_chain_bit,
      blink::mojom::AncestorChainBit ancestor_chain_bit_if_third_party_enabled,
      StorageKey& out);

  // (3D) Deserialization from string.
  // Note that if the deserialization wouldn't create a well-formed StorageKey
  // then nullopt is returned. This function must never DCHECK.
  static std::optional<StorageKey> Deserialize(std::string_view in);

  // A variant of deserialization for localStorage code only.
  // You almost always want to use Deserialize() instead.
  static std::optional<StorageKey> DeserializeForLocalStorage(
      std::string_view in);

  // (3E) Serialization to string; origin must not be opaque.
  // Note that this function will DCHECK if the origin is opaque.
  std::string Serialize() const;

  // A variant of serialization for localStorage code only.
  // You almost always want to use Serialize() instead.
  // Note that this function will DCHECK if the origin is opaque.
  std::string SerializeForLocalStorage() const;

  // [Block 4 - Accessors] - Keep in sync with BlinkStorageKey.

  const url::Origin& origin() const { return origin_; }

  const net::SchemefulSite& top_level_site() const { return top_level_site_; }

  // Returns true if unpartitioned storage access is forbidden for the current
  // storage key.
  bool ForbidsUnpartitionedStorageAccess() const { return nonce_.has_value(); }

  const std::optional<base::UnguessableToken>& nonce() const { return nonce_; }

  blink::mojom::AncestorChainBit ancestor_chain_bit() const {
    return ancestor_chain_bit_;
  }

  // [Block 5 - Shared Utility] - Keep in sync with BlinkStorageKey.

  // (5A) Serialize to string for use in debugging only.
  std::string GetDebugString() const;

  // (5B) Check exact match for testing only.
  // Checks if every single member in this key matches those in `other`.
  // Since the *_if_third_party_enabled_ fields aren't used normally
  // this function is only useful for testing purposes.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  bool ExactMatchForTesting(const StorageKey& other) const;

  // [Block 6 - Other Utility] - These don't exist in BlinkStorageKey.

  // Returns true if ThirdPartyStoragePartitioning feature flag is enabled.
  static bool IsThirdPartyStoragePartitioningEnabled();

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
    return ancestor_chain_bit_ == blink::mojom::AncestorChainBit::kSameSite;
  }
  bool IsThirdPartyContext() const { return !IsFirstPartyContext(); }

  // Provides a concise string representation suitable for memory dumps.
  // Limits the length to `max_length` chars and strips special characters.
  std::string GetMemoryDumpString(size_t max_length) const;

  // Return the "site for cookies" for the StorageKey's frame (or worker).
  //
  // While the SiteForCookie object returned matches the current default
  // behavior it's important to note that it may not exactly match a
  // SiteForCookies created for the same frame context and could cause
  // behavioral difference for users using the
  // LegacySameSiteCookieBehaviorEnabledForDomainList enterprise policy. The
  // impact is expected to be minimal however.
  //
  // (The difference is due to StorageKey not tracking the same state as
  // SiteForCookies, see see net::SiteForCookies::schemefully_same_ for more
  // info.)
  const net::SiteForCookies ToNetSiteForCookies() const;

  // Return an instance of net::IsolationInfo. This is used for forms of storage
  // like workers which have network access to ensure they only have access to
  // network state in their partition.
  //
  // The IsolationInfo that this creates will not be exactly the same as the
  // IsolationInfo of the context that created the worker. This is because
  // StorageKey only stores the top-frame *site* whereas IsolationInfo normally
  // uses top-frame *origin*. So we may lose the subdomain of the original
  // context. Although this is imperfect, it is better than using first-party
  // IsolationInfo for partitioned workers.
  //
  // For first-party contexts, the storage origin is used for the top-frame
  // origin in the resulting IsolationInfo. This matches legacy behavior before
  // storage partitioning, where the storage origin is always used as the
  // top-frame origin.
  const net::IsolationInfo ToPartialNetIsolationInfo() const;

  // Returns true if the registration key string is partitioned by top-level
  // site but storage partitioning is currently disabled, otherwise returns
  // false. Also returns false if the key string contains a serialized nonce.
  // Used in
  // components/services/storage/service_worker/service_worker_database.cc
  static bool ShouldSkipKeyDueToPartitioning(const std::string& reg_key_string);

  // Cast a storage key to a cookie partition key. If cookie partitioning is not
  // enabled, then it will always return nullopt.
  const std::optional<net::CookiePartitionKey> ToCookiePartitionKey() const;

  // Checks whether this StorageKey matches a given origin for the purposes of
  // clearing site data. This method should only be used in trusted contexts,
  // such as extensions browsingData API or settings UIs, as opposed to the
  // untrusted ones, such as the Clear-Site-Data header (where the entire
  // storage key should be matched exactly).
  // For first-party contexts, this matches on the `origin`; for third-party,
  // this matches on the `top_level_site`. This is done to prevent clearing
  // first-party data for a.example.com when only b.example.com needs to be
  // cleared. The 3P partitioned data for the entire example.com will be cleared
  // in contrast to that.
  bool MatchesOriginForTrustedStorageDeletion(const url::Origin& origin) const;

  // Like MatchesOriginForTrustedStorageDeletion, but for registrable domains.
  bool MatchesRegistrableDomainForTrustedStorageDeletion(
      std::string_view domain) const;

 private:
  // [Block 7 - Private Methods] - Keep in sync with BlinkStorageKey.

  // (7A) Internal constructor for custom values.
  // Note: Other than the opaque and copy/move constructors, this should be the
  // only non-static method for initializing a storage key to keep consistency.
  StorageKey(const url::Origin& origin,
             const net::SchemefulSite& top_level_site,
             const base::UnguessableToken* nonce,
             blink::mojom::AncestorChainBit ancestor_chain_bit,
             bool third_party_partitioning_allowed);

  // (7B) Operators.
  // Note that not all must be friends, but all are to consolidate the header.
  friend bool operator==(const StorageKey& lhs, const StorageKey& rhs) {
    return std::tie(lhs.origin_, lhs.top_level_site_, lhs.nonce_,
                    lhs.ancestor_chain_bit_) ==
           std::tie(rhs.origin_, rhs.top_level_site_, rhs.nonce_,
                    rhs.ancestor_chain_bit_);
  }
  friend auto operator<=>(const StorageKey& lhs, const StorageKey& rhs) {
    return std::tie(lhs.origin_, lhs.top_level_site_, lhs.nonce_,
                    lhs.ancestor_chain_bit_) <=>
           std::tie(rhs.origin_, rhs.top_level_site_, rhs.nonce_,
                    rhs.ancestor_chain_bit_);
  }
  BLINK_COMMON_EXPORT
  friend std::ostream& operator<<(std::ostream& ostream, const StorageKey& sk);

  // (7C) Check validity of current storage key members.
  // This should be used when constructing, side-loading, and deserializing
  // a key to ensure correctness. This does not imply that the key is
  // serializable as keys with opaque origins will still return true.
  bool IsValid() const;

  // [Block 8 - Private Members] - Keep in sync with BlinkStorageKey.

  // The current site in the given context. StorageKey is generally
  // passed in contexts which used to pass Origin before partitioning.
  url::Origin origin_;

  // The "top-level site"/"top-level frame"/"main frame" of the context
  // this StorageKey was created for (for storage partitioning purposes).
  // For extensions or related enterprise policies this may not represent the
  // top-level site. For contexts with a `nonce_` or contexts without storage
  // partitioning enabled, this will be the eTLD+1 of `origin_`.
  net::SchemefulSite top_level_site_;

  // Stores the value `top_level_site_` would have had if
  // `kThirdPartyStoragePartitioning` were enabled. This isn't used in
  // serialization or comparison.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  net::SchemefulSite top_level_site_if_third_party_enabled_ = top_level_site_;

  // Optional, forcing partitioned storage and used by anonymous iframes:
  // https://github.com/camillelamy/explainers/blob/master/anonymous_iframes.md
  std::optional<base::UnguessableToken> nonce_;

  // kSameSite if the entire ancestor chain is same-site with the current frame.
  // kCrossSite otherwise. Used by service workers.
  blink::mojom::AncestorChainBit ancestor_chain_bit_{
      blink::mojom::AncestorChainBit::kCrossSite};

  // Stores the value `ancestor_chain_bit_` would have had if
  // `kThirdPartyStoragePartitioning` were enabled. This isn't used in
  // serialization or comparison.
  // TODO(crbug.com/1159586): Remove when no longer needed.
  blink::mojom::AncestorChainBit ancestor_chain_bit_if_third_party_enabled_ =
      ancestor_chain_bit_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_STORAGE_KEY_STORAGE_KEY_H_

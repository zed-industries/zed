// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_ALTERNATE_SIGNED_EXCHANGE_RESOURCE_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_ALTERNATE_SIGNED_EXCHANGE_RESOURCE_INFO_H_

#include <optional>

#include "services/network/public/mojom/fetch_api.mojom-forward.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/kurl_hash.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

enum class ResourceType : uint8_t;

// AlternateSignedExchangeResourceInfo keeps the alternate signed exchange
// resource information which is extracted from "alternate" link headers in the
// outer response and "allowed-alt-sxg" link headers in the inner response while
// handling a signed exchange.
// Example:
//  - In outer response:
//    link: <https://distributor.example/publisher.example/image_jpeg.sxg>;
//          rel="alternate";
//          type="application/signed-exchange;v=b3";
//          variants-04="accept;image/jpeg;image/webp";
//          variant-key-04="image/jpeg";
//          anchor="https://publisher.example/image";
//  - In inner response:
//    link: <https://publisher.example/image>;
//          rel="allowed-alt-sxg";
//          variants-04="accept;image/jpeg;image/webp";
//          variant-key-04="image/jpeg";
//          header-integrity="sha256-MEUCID..."
//  - The |alternative_resources_| will be like this:
//    {
//      "https://publisher.example/image": [{
//        anchor_url: "https://publisher.example/image",
//        alternative_url:
//            "https://distributor.example/publisher.example/image_jpeg.sxg",
//        header_integrity: "sha256-MEUCID...",
//        variants: "accept;image/jpeg;image/webp",
//        variant_key: "image/jpeg"
//      }]
//    }
// Note: When a valid "allowed-alt-sxg" link header exists in the inner response
// but there is no matching "alternate" link header in the outer response, this
// class keep the information with an invalid |alternative_url|.
class CORE_EXPORT AlternateSignedExchangeResourceInfo {
 public:
  class CORE_EXPORT Entry {
   public:
    Entry(const KURL& anchor_url,
          const KURL& alternative_url,
          const String& header_integrity,
          const String& variants,
          const String& variant_key)
        : anchor_url_(anchor_url),
          alternative_url_(alternative_url),
          header_integrity_(header_integrity),
          variants_(variants),
          variant_key_(variant_key) {}
    Entry(const Entry&) = delete;
    Entry& operator=(const Entry&) = delete;
    const KURL& anchor_url() const { return anchor_url_; }
    const KURL& alternative_url() const { return alternative_url_; }
    const String& header_integrity() const { return header_integrity_; }
    const String& variants() const { return variants_; }
    const String& variant_key() const { return variant_key_; }

   private:
    const KURL anchor_url_;
    const KURL alternative_url_;
    const String header_integrity_;
    const String variants_;
    const String variant_key_;
  };

  using EntryMap =
      HashMap<KURL /* anchor_url */, Vector<std::unique_ptr<Entry>>>;

  static std::unique_ptr<AlternateSignedExchangeResourceInfo> CreateIfValid(
      const String& outer_link_header,
      const String& inner_link_header);

  explicit AlternateSignedExchangeResourceInfo(EntryMap alternative_resources);
  AlternateSignedExchangeResourceInfo(
      const AlternateSignedExchangeResourceInfo&) = delete;
  AlternateSignedExchangeResourceInfo& operator=(
      const AlternateSignedExchangeResourceInfo&) = delete;
  ~AlternateSignedExchangeResourceInfo() = default;

  // Returns the best matching alternate resource. If the first entry which
  // |anchor_url| is |url| has non-null |variants| value, this method use the
  // preference order of the result of "Cache Behaviour" [1] to find the best
  // matching entry. Otherwise returns the first entry which |anchor_url| is
  // |url|.
  // [1]
  // https://httpwg.org/http-extensions/draft-ietf-httpbis-variants.html#cache
  Entry* FindMatchingEntry(const KURL& url,
                           std::optional<ResourceType> resource_type,
                           const Vector<String>& languages) const;
  Entry* FindMatchingEntry(
      const KURL& url,
      network::mojom::RequestDestination request_destination,
      const Vector<String>& languages) const;

 private:
  friend class AlternateSignedExchangeResourceInfoTest;

  Entry* FindMatchingEntry(const KURL& url,
                           const char* accept_header,
                           const Vector<String>& languages) const;

  const EntryMap alternative_resources_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_ALTERNATE_SIGNED_EXCHANGE_RESOURCE_INFO_H_

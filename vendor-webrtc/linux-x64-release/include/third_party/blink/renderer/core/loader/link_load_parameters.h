// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LINK_LOAD_PARAMETERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LINK_LOAD_PARAMETERS_H_

#include <optional>

#include "base/unguessable_token.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/cross_origin_attribute.h"
#include "third_party/blink/renderer/core/html/link_rel_attribute.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class LinkHeader;

struct CORE_EXPORT LinkLoadParameters {
  enum class Reason { kDefault, kMediaChange };

  LinkLoadParameters(const LinkRelAttribute&,
                     const CrossOriginAttributeValue&,
                     const String& type,
                     const String& as,
                     const String& media,
                     const String& nonce,
                     const String& integrity,
                     const String& fetch_priority_hint,
                     network::mojom::ReferrerPolicy,
                     const KURL& href,
                     const String& image_srcset,
                     const String& image_sizes,
                     const String& blocking,
                     Reason reason = Reason::kDefault);
  LinkLoadParameters(const LinkHeader&, const KURL& base_url);

  LinkRelAttribute rel;
  CrossOriginAttributeValue cross_origin;
  String type;
  String as;
  String media;
  String nonce;
  String integrity;
  String fetch_priority_hint;
  network::mojom::ReferrerPolicy referrer_policy;
  KURL href;
  String image_srcset;
  String image_sizes;
  String blocking;
  // `recursive_prefetch_token` is set for preloads that were promoted to
  // prefetches because the Link preload header was received on a prefetch
  // response, recursively. The `base::UnguessableToken` value corresponds to
  // the initial top-level document prefetch and is used to ensure that the
  // prefetched resources get stored in the correct HTTP cache partition (which
  // is required for them to actually be used if the top-level document gets
  // navigated to).
  std::optional<base::UnguessableToken> recursive_prefetch_token;
  Reason reason;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LINK_LOAD_PARAMETERS_H_

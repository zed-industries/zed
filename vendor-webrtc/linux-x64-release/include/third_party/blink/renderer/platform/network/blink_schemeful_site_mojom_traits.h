// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_BLINK_SCHEMEFUL_SITE_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_BLINK_SCHEMEFUL_SITE_MOJOM_TRAITS_H_

#include "mojo/public/cpp/bindings/struct_traits.h"

#include "services/network/public/mojom/schemeful_site.mojom-blink.h"
#include "third_party/blink/renderer/platform/network/blink_schemeful_site.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {
class SecurityOrigin;
}  // namespace blink

namespace mojo {

template <>
struct PLATFORM_EXPORT StructTraits<network::mojom::SchemefulSiteDataView,
                                    blink::BlinkSchemefulSite> {
  static const scoped_refptr<const blink::SecurityOrigin>& site_as_origin(
      const blink::BlinkSchemefulSite& input) {
    return input.site_as_origin_;
  }

  static bool Read(network::mojom::SchemefulSiteDataView data,
                   blink::BlinkSchemefulSite* out);
};
}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_BLINK_SCHEMEFUL_SITE_MOJOM_TRAITS_H_

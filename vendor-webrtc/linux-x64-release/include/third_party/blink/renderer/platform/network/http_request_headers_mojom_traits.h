// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_REQUEST_HEADERS_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_REQUEST_HEADERS_MOJOM_TRAITS_H_

#include "services/network/public/mojom/http_request_headers.mojom-blink.h"
#include "third_party/blink/renderer/platform/network/http_header_map.h"

namespace mojo {

template <>
struct StructTraits<network::mojom::HttpRequestHeadersDataView,
                    blink::HTTPHeaderMap> {
  static WTF::Vector<network::mojom::blink::HttpRequestHeaderKeyValuePairPtr>
  headers(const blink::HTTPHeaderMap& map);

  static bool Read(network::mojom::HttpRequestHeadersDataView data,
                   blink::HTTPHeaderMap* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_REQUEST_HEADERS_MOJOM_TRAITS_H_

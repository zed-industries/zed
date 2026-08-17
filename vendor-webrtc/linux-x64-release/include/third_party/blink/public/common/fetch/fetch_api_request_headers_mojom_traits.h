// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FETCH_FETCH_API_REQUEST_HEADERS_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FETCH_FETCH_API_REQUEST_HEADERS_MOJOM_TRAITS_H_

#include <string>

#include "base/containers/flat_map.h"
#include "base/strings/string_util.h"
#include "third_party/blink/public/common/fetch/fetch_api_request_headers_map.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-shared.h"

namespace mojo {

template <>
struct StructTraits<blink::mojom::FetchAPIRequestHeadersDataView,
                    blink::FetchAPIRequestHeadersMap> {
  static base::flat_map<std::string, std::string> headers(
      const blink::FetchAPIRequestHeadersMap& in_headers) {
    return {in_headers.begin(), in_headers.end()};
  }

  static bool Read(blink::mojom::FetchAPIRequestHeadersDataView in,
                   blink::FetchAPIRequestHeadersMap* out) {
    return in.ReadHeaders(out);
  }
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FETCH_FETCH_API_REQUEST_HEADERS_MOJOM_TRAITS_H_

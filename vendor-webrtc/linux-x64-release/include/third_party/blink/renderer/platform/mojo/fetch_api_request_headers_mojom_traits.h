// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_FETCH_API_REQUEST_HEADERS_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_FETCH_API_REQUEST_HEADERS_MOJOM_TRAITS_H_

#include "mojo/public/cpp/bindings/map_traits_wtf_hash_map.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-shared.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/case_folding_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace mojo {

template <>
struct StructTraits<blink::mojom::FetchAPIRequestHeadersDataView,
                    WTF::HashMap<WTF::String,
                                 WTF::String,
                                 WTF::CaseFoldingHashTraits<WTF::String>>> {
  using MapType = WTF::HashMap<WTF::String,
                               WTF::String,
                               WTF::CaseFoldingHashTraits<WTF::String>>;
  static WTF::HashMap<WTF::String, WTF::String> headers(const MapType& input) {
    WTF::HashMap<WTF::String, WTF::String> map;
    for (const auto& tuple : input)
      map.insert(tuple.key, tuple.value);
    return map;
  }

  static bool Read(blink::mojom::FetchAPIRequestHeadersDataView in,
                   MapType* out) {
    WTF::HashMap<WTF::String, WTF::String> in_headers;
    if (!in.ReadHeaders(&in_headers))
      return false;
    for (const auto& tuple : in_headers)
      out->insert(tuple.key, tuple.value);
    return true;
  }
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MOJO_FETCH_API_REQUEST_HEADERS_MOJOM_TRAITS_H_

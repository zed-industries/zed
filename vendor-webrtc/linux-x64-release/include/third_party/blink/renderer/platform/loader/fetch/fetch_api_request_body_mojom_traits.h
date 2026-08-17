// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_API_REQUEST_BODY_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_API_REQUEST_BODY_MOJOM_TRAITS_H_

#include "services/network/public/cpp/data_element.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/network/encoded_form_data.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace mojo {

template <>
struct PLATFORM_EXPORT StructTraits<blink::mojom::FetchAPIRequestBodyDataView,
                                    blink::ResourceRequestBody> {
  static bool IsNull(const blink::ResourceRequestBody& body) {
    return body.IsEmpty();
  }
  static void SetToNull(blink::ResourceRequestBody* out) {
    *out = blink::ResourceRequestBody();
  }
  static WTF::Vector<network::DataElement> elements(
      blink::ResourceRequestBody& mutable_body);
  static int64_t identifier(const blink::ResourceRequestBody& body) {
    return body.FormBody() ? body.FormBody()->Identifier() : 0;
  }
  static bool contains_sensitive_info(const blink::ResourceRequestBody& body) {
    return body.FormBody() ? body.FormBody()->ContainsPasswordData() : false;
  }

  static bool Read(blink::mojom::FetchAPIRequestBodyDataView in,
                   blink::ResourceRequestBody* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_FETCH_API_REQUEST_BODY_MOJOM_TRAITS_H_

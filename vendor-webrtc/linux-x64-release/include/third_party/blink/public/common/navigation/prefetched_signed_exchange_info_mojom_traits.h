// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_PREFETCHED_SIGNED_EXCHANGE_INFO_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_PREFETCHED_SIGNED_EXCHANGE_INFO_MOJOM_TRAITS_H_

#include <string>

#include "base/containers/span.h"
#include "mojo/public/cpp/base/byte_string_mojom_traits.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "net/base/hash_value.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/navigation/prefetched_signed_exchange_info.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::SHA256HashValueDataView, net::SHA256HashValue> {
  static const std::string data(const net::SHA256HashValue& value) {
    return std::string(base::as_string_view(value));
  }

  static bool Read(blink::mojom::SHA256HashValueDataView input,
                   net::SHA256HashValue* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_PREFETCHED_SIGNED_EXCHANGE_INFO_MOJOM_TRAITS_H_

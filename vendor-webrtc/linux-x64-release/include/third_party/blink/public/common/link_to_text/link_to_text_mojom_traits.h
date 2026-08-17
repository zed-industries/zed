// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LINK_TO_TEXT_LINK_TO_TEXT_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LINK_TO_TEXT_LINK_TO_TEXT_MOJOM_TRAITS_H_

#include "components/shared_highlighting/core/common/shared_highlighting_metrics.h"
#include "mojo/public/cpp/bindings/enum_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/link_to_text/link_to_text.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::LinkGenerationError,
               shared_highlighting::LinkGenerationError> {
  static blink::mojom::LinkGenerationError ToMojom(
      shared_highlighting::LinkGenerationError input);
  static bool FromMojom(blink::mojom::LinkGenerationError input,
                        shared_highlighting::LinkGenerationError* output);
};

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::LinkGenerationReadyStatus,
               shared_highlighting::LinkGenerationReadyStatus> {
  static blink::mojom::LinkGenerationReadyStatus ToMojom(
      shared_highlighting::LinkGenerationReadyStatus input);
  static bool FromMojom(blink::mojom::LinkGenerationReadyStatus input,
                        shared_highlighting::LinkGenerationReadyStatus* output);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LINK_TO_TEXT_LINK_TO_TEXT_MOJOM_TRAITS_H_

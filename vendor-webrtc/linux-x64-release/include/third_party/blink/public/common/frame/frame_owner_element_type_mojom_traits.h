// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_OWNER_ELEMENT_TYPE_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_OWNER_ELEMENT_TYPE_MOJOM_TRAITS_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/frame/frame_owner_element_type.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-shared.h"

namespace mojo {
template <>
struct BLINK_COMMON_EXPORT EnumTraits<blink::mojom::FrameOwnerElementType,
                                      blink::FrameOwnerElementType> {
  static blink::mojom::FrameOwnerElementType ToMojom(
      blink::FrameOwnerElementType input);
  static bool FromMojom(blink::mojom::FrameOwnerElementType input,
                        blink::FrameOwnerElementType* output);
};
}  // namespace mojo
#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_OWNER_ELEMENT_TYPE_MOJOM_TRAITS_H_

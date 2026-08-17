// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_DRAG_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_DRAG_MOJOM_TRAITS_H_

#include "mojo/public/cpp/bindings/enum_traits.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/page/drag_operation.h"
#include "third_party/blink/public/mojom/page/widget.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AllowedDragOperationsDataView,
                 blink::DragOperationsMask> {
  static bool allow_copy(const blink::DragOperationsMask& op_mask) {
    return op_mask & blink::kDragOperationCopy;
  }
  static bool allow_link(const blink::DragOperationsMask& op_mask) {
    return op_mask & blink::kDragOperationLink;
  }
  static bool allow_move(const blink::DragOperationsMask& op_mask) {
    return op_mask & blink::kDragOperationMove;
  }
  static bool Read(blink::mojom::AllowedDragOperationsDataView data,
                   blink::DragOperationsMask* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_DRAG_MOJOM_TRAITS_H_

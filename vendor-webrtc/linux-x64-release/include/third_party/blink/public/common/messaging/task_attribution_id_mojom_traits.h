// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_TASK_ATTRIBUTION_ID_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_TASK_ATTRIBUTION_ID_MOJOM_TRAITS_H_

#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/public/mojom/messaging/task_attribution_id.mojom.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::TaskAttributionId::DataView,
                 blink::scheduler::TaskAttributionId> {
  static uint32_t value(const blink::scheduler::TaskAttributionId& input) {
    return input.value();
  }

  static bool Read(blink::mojom::TaskAttributionId::DataView data,
                   blink::scheduler::TaskAttributionId* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_TASK_ATTRIBUTION_ID_MOJOM_TRAITS_H_

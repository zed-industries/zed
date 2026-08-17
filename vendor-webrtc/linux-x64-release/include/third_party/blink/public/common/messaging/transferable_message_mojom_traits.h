// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_TRANSFERABLE_MESSAGE_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_TRANSFERABLE_MESSAGE_MOJOM_TRAITS_H_

#include "skia/public/mojom/bitmap_skbitmap_mojom_traits.h"
#include "third_party/blink/public/common/messaging/accelerated_static_bitmap_image_mojom_traits.h"
#include "third_party/blink/public/common/messaging/cloneable_message_mojom_traits.h"
#include "third_party/blink/public/common/messaging/message_port_descriptor_mojom_traits.h"
#include "third_party/blink/public/common/messaging/task_attribution_id_mojom_traits.h"
#include "third_party/blink/public/common/messaging/transferable_message.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/public/mojom/messaging/transferable_message.mojom.h"
#include "third_party/skia/include/core/SkBitmap.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::TransferableMessage::DataView,
                 blink::TransferableMessage> {
  static blink::CloneableMessage& message(blink::TransferableMessage& input) {
    return input;
  }

  static std::vector<blink::MessagePortDescriptor> ports(
      blink::TransferableMessage& input) {
    return blink::MessagePortChannel::ReleaseHandles(input.ports);
  }

  static std::vector<blink::MessagePortDescriptor> stream_channels(
      blink::TransferableMessage& input) {
    return blink::MessagePortChannel::ReleaseHandles(input.stream_channels);
  }

  static std::vector<blink::mojom::SerializedArrayBufferContentsPtr>
  array_buffer_contents_array(blink::TransferableMessage& input) {
    return std::move(input.array_buffer_contents_array);
  }

  static std::vector<blink::mojom::SerializedStaticBitmapImagePtr>
  image_bitmap_contents_array(blink::TransferableMessage& input) {
    return std::move(input.image_bitmap_contents_array);
  }

  static const blink::mojom::UserActivationSnapshotPtr& user_activation(
      blink::TransferableMessage& input) {
    return input.user_activation;
  }

  static blink::mojom::DelegatedCapability delegated_capability(
      blink::TransferableMessage& input) {
    return input.delegated_capability;
  }

  static std::optional<blink::scheduler::TaskAttributionId> parent_task_id(
      blink::TransferableMessage& input) {
    return input.parent_task_id
               ? std::make_optional(blink::scheduler::TaskAttributionId(
                     input.parent_task_id.value()))
               : std::nullopt;
  }

  static bool Read(blink::mojom::TransferableMessage::DataView data,
                   blink::TransferableMessage* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_TRANSFERABLE_MESSAGE_MOJOM_TRAITS_H_

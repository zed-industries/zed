// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MESSAGING_BLINK_TRANSFERABLE_MESSAGE_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MESSAGING_BLINK_TRANSFERABLE_MESSAGE_MOJOM_TRAITS_H_

#include "skia/public/mojom/bitmap_skbitmap_mojom_traits.h"
#include "third_party/blink/public/common/messaging/accelerated_static_bitmap_image_mojom_traits.h"
#include "third_party/blink/public/common/messaging/message_port_channel.h"
#include "third_party/blink/public/common/messaging/message_port_descriptor.h"
#include "third_party/blink/public/common/messaging/message_port_descriptor_mojom_traits.h"
#include "third_party/blink/public/common/messaging/task_attribution_id_mojom_traits.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/public/mojom/blob/blob.mojom-blink.h"
#include "third_party/blink/public/mojom/messaging/transferable_message.mojom-shared.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/messaging/blink_cloneable_message_mojom_traits.h"
#include "third_party/blink/renderer/core/messaging/blink_transferable_message.h"

namespace mojo {

template <>
struct CORE_EXPORT StructTraits<blink::mojom::TransferableMessageDataView,
                                blink::BlinkTransferableMessage> {
  static blink::BlinkCloneableMessage& message(
      blink::BlinkTransferableMessage& input) {
    return input;
  }

  static Vector<blink::MessagePortDescriptor> ports(
      blink::BlinkTransferableMessage& input) {
    Vector<blink::MessagePortDescriptor> result;
    result.ReserveInitialCapacity(input.ports.size());
    for (const auto& port : input.ports)
      result.push_back(port.ReleaseHandle());
    return result;
  }

  static Vector<blink::MessagePortDescriptor> stream_channels(
      blink::BlinkTransferableMessage& input) {
    Vector<blink::MessagePortDescriptor> result;
    auto& streams = input.message->GetStreams();
    result.ReserveInitialCapacity(streams.size());
    for (const auto& stream : streams)
      result.push_back(stream.channel.ReleaseHandle());
    return result;
  }

  static const blink::SerializedScriptValue::ArrayBufferContentsArray&
  array_buffer_contents_array(const blink::BlinkCloneableMessage& input) {
    return input.message->GetArrayBufferContentsArray();
  }

  static Vector<blink::mojom::blink::SerializedStaticBitmapImagePtr>
  image_bitmap_contents_array(const blink::BlinkCloneableMessage& input);

  static const blink::mojom::blink::UserActivationSnapshotPtr& user_activation(
      const blink::BlinkTransferableMessage& input) {
    return input.user_activation;
  }

  static blink::mojom::blink::DelegatedCapability delegated_capability(
      const blink::BlinkTransferableMessage& input) {
    return input.delegated_capability;
  }

  static std::optional<blink::scheduler::TaskAttributionId> parent_task_id(
      blink::BlinkTransferableMessage& input) {
    return input.parent_task_id
               ? std::make_optional(blink::scheduler::TaskAttributionId(
                     input.parent_task_id.value()))
               : std::nullopt;
  }

  static bool Read(blink::mojom::TransferableMessageDataView,
                   blink::BlinkTransferableMessage* out);
};

template <>
class CORE_EXPORT
    StructTraits<blink::mojom::SerializedArrayBufferContentsDataView,
                 blink::ArrayBufferContents> {
 public:
  static mojo_base::BigBuffer contents(
      const blink::ArrayBufferContents& array_buffer_contents) {
    return mojo_base::BigBuffer(array_buffer_contents.ByteSpan());
  }
  static bool is_resizable_by_user_javascript(
      const blink::ArrayBufferContents& array_buffer_contents) {
    return array_buffer_contents.IsResizableByUserJavaScript();
  }
  static size_t max_byte_length(
      const blink::ArrayBufferContents& array_buffer_contents) {
    return array_buffer_contents.MaxDataLength();
  }

  static bool Read(blink::mojom::SerializedArrayBufferContentsDataView,
                   blink::ArrayBufferContents* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MESSAGING_BLINK_TRANSFERABLE_MESSAGE_MOJOM_TRAITS_H_

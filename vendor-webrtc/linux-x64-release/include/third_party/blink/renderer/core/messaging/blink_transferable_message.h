// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MESSAGING_BLINK_TRANSFERABLE_MESSAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MESSAGING_BLINK_TRANSFERABLE_MESSAGE_H_

#include <optional>

#include "third_party/blink/public/common/messaging/accelerated_image_info.h"
#include "third_party/blink/public/common/messaging/message_port_channel.h"
#include "third_party/blink/public/common/messaging/transferable_message.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/public/mojom/messaging/delegated_capability.mojom-blink.h"
#include "third_party/blink/public/mojom/messaging/static_bitmap_image.mojom-blink.h"
#include "third_party/blink/public/mojom/messaging/user_activation_snapshot.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/messaging/blink_cloneable_message.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace blink {

// This struct represents messages as they are posted over a message port. This
// type can be serialized as a blink::mojom::TransferableMessage struct.
// This is the renderer-side equivalent of blink::TransferableMessage, where
// this struct uses blink types, while the other struct uses std:: types.
struct CORE_EXPORT BlinkTransferableMessage : BlinkCloneableMessage {
  static BlinkTransferableMessage FromTransferableMessage(TransferableMessage);

  BlinkTransferableMessage();
  BlinkTransferableMessage(BlinkTransferableMessage&&);
  BlinkTransferableMessage& operator=(BlinkTransferableMessage&&);
  ~BlinkTransferableMessage();

  Vector<MessagePortChannel> ports;

  mojom::blink::UserActivationSnapshotPtr user_activation;

  mojom::blink::DelegatedCapability delegated_capability =
      mojom::blink::DelegatedCapability::kNone;

  std::optional<scheduler::TaskAttributionId> parent_task_id;
};

CORE_EXPORT scoped_refptr<blink::StaticBitmapImage> ToStaticBitmapImage(
    const SkBitmap& sk_bitmap);

CORE_EXPORT scoped_refptr<blink::StaticBitmapImage> WrapAcceleratedBitmapImage(
    AcceleratedImageInfo image);

}  // namespace blink

namespace WTF {

template <>
struct CrossThreadCopier<blink::BlinkTransferableMessage> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = blink::BlinkTransferableMessage;
  static Type Copy(Type pointer) {
    return pointer;  // This is in fact a move.
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MESSAGING_BLINK_TRANSFERABLE_MESSAGE_H_

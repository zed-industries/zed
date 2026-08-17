// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_TRANSFERABLE_MESSAGE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_TRANSFERABLE_MESSAGE_H_

#include <vector>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/messaging/cloneable_message.h"
#include "third_party/blink/public/common/messaging/message_port_channel.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/public/mojom/array_buffer/array_buffer_contents.mojom.h"
#include "third_party/blink/public/mojom/blob/blob.mojom.h"
#include "third_party/blink/public/mojom/messaging/delegated_capability.mojom-shared.h"
#include "third_party/blink/public/mojom/messaging/static_bitmap_image.mojom.h"
#include "third_party/blink/public/mojom/messaging/task_attribution_id.mojom.h"
#include "third_party/blink/public/mojom/messaging/user_activation_snapshot.mojom.h"
#include "third_party/skia/include/core/SkBitmap.h"

namespace blink {

// This struct represents messages as they are posted over a message port. This
// type can be serialized as a blink::mojom::TransferableMessage struct.
struct BLINK_COMMON_EXPORT TransferableMessage : public CloneableMessage {
  TransferableMessage();
  TransferableMessage(TransferableMessage&&);
  TransferableMessage& operator=(TransferableMessage&&);
  ~TransferableMessage();

  // Any ports being transferred as part of this message.
  std::vector<MessagePortChannel> ports;
  // Channels used by transferred WHATWG streams (eg. ReadableStream).
  std::vector<MessagePortChannel> stream_channels;
  // The contents of any ArrayBuffers being transferred as part of this message.
  std::vector<mojom::SerializedArrayBufferContentsPtr>
      array_buffer_contents_array;
  // The contents of any ImageBitmaps being transferred as part of this message.
  std::vector<mojom::SerializedStaticBitmapImagePtr>
      image_bitmap_contents_array;

  // The state of user activation.
  mojom::UserActivationSnapshotPtr user_activation;

  // What capability, if any, is delegated to the destination frame.
  mojom::DelegatedCapability delegated_capability =
      mojom::DelegatedCapability::kNone;

  // The transferable message's parent task ID.
  std::optional<scheduler::TaskAttributionId> parent_task_id;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_TRANSFERABLE_MESSAGE_H_

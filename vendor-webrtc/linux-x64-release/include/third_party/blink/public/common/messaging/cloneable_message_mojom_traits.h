// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_CLONEABLE_MESSAGE_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_CLONEABLE_MESSAGE_MOJOM_TRAITS_H_

#include <optional>

#include "mojo/public/cpp/base/big_buffer.h"
#include "mojo/public/cpp/base/unguessable_token_mojom_traits.h"
#include "third_party/blink/public/common/messaging/cloneable_message.h"
#include "third_party/blink/public/mojom/messaging/cloneable_message.mojom.h"
#include "url/mojom/origin_mojom_traits.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::CloneableMessage::DataView,
                 blink::CloneableMessage> {
  static mojo_base::BigBufferView encoded_message(
      blink::CloneableMessage& input);

  static std::vector<blink::mojom::SerializedBlobPtr>& blobs(
      blink::CloneableMessage& input) {
    return input.blobs;
  }

  static const std::optional<url::Origin>& sender_origin(
      const blink::CloneableMessage& input) {
    return input.sender_origin;
  }

  static uint64_t stack_trace_id(const blink::CloneableMessage& input) {
    return input.stack_trace_id;
  }

  static int64_t stack_trace_debugger_id_first(
      const blink::CloneableMessage& input) {
    return input.stack_trace_debugger_id_first;
  }

  static int64_t stack_trace_debugger_id_second(
      const blink::CloneableMessage& input) {
    return input.stack_trace_debugger_id_second;
  }

  static bool stack_trace_should_pause(const blink::CloneableMessage& input) {
    return input.stack_trace_should_pause;
  }

  static const base::UnguessableToken& sender_agent_cluster_id(
      const blink::CloneableMessage& input) {
    return input.sender_agent_cluster_id;
  }

  static bool locked_to_sender_agent_cluster(
      const blink::CloneableMessage& input) {
    return input.locked_to_sender_agent_cluster;
  }

  static bool Read(blink::mojom::CloneableMessage::DataView data,
                   blink::CloneableMessage* out);

  static std::vector<
      mojo::PendingRemote<blink::mojom::FileSystemAccessTransferToken>>&
  file_system_access_tokens(blink::CloneableMessage& input) {
    return input.file_system_access_tokens;
  }
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_CLONEABLE_MESSAGE_MOJOM_TRAITS_H_

// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_CLONEABLE_MESSAGE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_CLONEABLE_MESSAGE_H_

#include <optional>
#include <vector>

#include "base/containers/span.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/unguessable_token.h"
#include "mojo/public/cpp/bindings/struct_ptr.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/blob/serialized_blob.mojom.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_transfer_token.mojom.h"
#include "url/origin.h"

namespace blink {

// This struct represents messages as they are posted over a broadcast channel.
// This type can be serialized as a blink::mojom::CloneableMessage struct.
struct BLINK_COMMON_EXPORT CloneableMessage {
  CloneableMessage();
  CloneableMessage(CloneableMessage&&);
  CloneableMessage& operator=(CloneableMessage&&);
  ~CloneableMessage();

  // Returns a shallow clone of this message. |encoded_message| in the clone
  // will point to the same memory as |encoded_message| in this.
  CloneableMessage ShallowClone() const;

  // To reduce copies when serializing |encoded_message| does not have to point
  // to |owned_encoded_message|. The serialization code completely ignores the
  // |owned_encoded_message| and just serializes whatever |encoded_message|
  // points to. When deserializing |owned_encoded_message| is set to the data
  // and |encoded_message| is set to point to |owned_encoded_message|.
  //
  // TODO(crbug.com/372506014): investigate dangling span, e.g. in
  // `blink_web_tests` on `linux-rel`.
  // TODO(367764863) Rewrite to base::raw_span
  RAW_PTR_EXCLUSION base::span<const uint8_t> encoded_message;
  std::vector<uint8_t> owned_encoded_message;

  // Copies data into owned_encoded_message if it's not already there.
  void EnsureDataIsOwned();

  // Blob handles for any blobs being sent in this message.
  std::vector<mojom::SerializedBlobPtr> blobs;

  // Populated by message senders to prevent remote origins from accessing
  // |file_system_access_tokens|. Tokens from a remote origin must fail by
  // dispatching a 'messageerror' event instead of a 'message' event. If
  // somehow a compromised process bypasses this origin check, the
  // FileSystemAccessManager in the browser process performs another origin
  // check that will prevent the use of remote origin FileSystemAccessHandles.
  //
  // The message sender may leave |sender_origin| null. When |sender_origin| is
  // null, the message receiver must handle the message like it is from an
  // unknown remote origin by dispatching the 'messageerror' event when
  // |file_system_access_tokens| is not an empty array.
  std::optional<url::Origin> sender_origin;

  // Stack trace captured by sender.
  // For more details see v8_inspector::V8StackTraceId.
  uint64_t stack_trace_id = 0;
  int64_t stack_trace_debugger_id_first = 0;
  int64_t stack_trace_debugger_id_second = 0;
  bool stack_trace_should_pause = false;

  // The sender's agent cluster ID.
  // See
  // https://html.spec.whatwg.org/multipage/webappapis.html#integration-with-the-javascript-agent-cluster-formalism
  base::UnguessableToken sender_agent_cluster_id;
  // If true, this message is locked to the sender agent cluster ID.
  bool locked_to_sender_agent_cluster = false;

  // Tokens required to clone FileSystemFileHandles and/or
  // FileSystemDirectoryHandles.
  std::vector<mojo::PendingRemote<mojom::FileSystemAccessTransferToken>>
      file_system_access_tokens;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_CLONEABLE_MESSAGE_H_

// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_MESSAGE_PORT_DESCRIPTOR_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_MESSAGE_PORT_DESCRIPTOR_MOJOM_TRAITS_H_

#include "third_party/blink/public/common/messaging/message_port_descriptor.h"
#include "third_party/blink/public/mojom/messaging/message_port_descriptor.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::MessagePortDescriptorDataView,
                 blink::MessagePortDescriptor> {
  // For converting a blink::mojom::MessagePortDescriptor to a
  // blink::MessagePortDescriptor.
  static bool Read(blink::mojom::MessagePortDescriptorDataView data,
                   blink::MessagePortDescriptor* output);

  // For converting a blink::MessagePortDescriptor to a
  // blink::mojom::MessagePortDescriptor.
  static mojo::ScopedMessagePipeHandle pipe_handle(
      blink::MessagePortDescriptor& input);
  static base::UnguessableToken id(blink::MessagePortDescriptor& input);
  static uint64_t sequence_number(blink::MessagePortDescriptor& input);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_MESSAGE_PORT_DESCRIPTOR_MOJOM_TRAITS_H_

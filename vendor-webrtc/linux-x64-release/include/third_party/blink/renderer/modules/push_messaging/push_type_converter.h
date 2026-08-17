// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_TYPE_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_TYPE_CONVERTER_H_

#include "mojo/public/cpp/bindings/type_converter.h"
#include "third_party/blink/public/mojom/push_messaging/push_messaging.mojom-blink.h"

namespace blink {

class PushSubscriptionOptions;

}  // namespace blink

namespace mojo {

// Converts a {blink::PushSubscriptionOptions} object into a
// {blink::mojom::blink::PushSubscriptionOptions} object. Since the buffer of
// the former may be bigger than the capacity of the latter, a caller of this
// function has to guarantee that the ByteLength of the input buffer fits into
// {wtf_size_t} range.
template <>
struct TypeConverter<blink::mojom::blink::PushSubscriptionOptionsPtr,
                     blink::PushSubscriptionOptions*> {
  static blink::mojom::blink::PushSubscriptionOptionsPtr Convert(
      const blink::PushSubscriptionOptions* input);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_TYPE_CONVERTER_H_

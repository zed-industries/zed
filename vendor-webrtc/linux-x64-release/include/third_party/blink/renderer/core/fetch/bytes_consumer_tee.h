// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BYTES_CONSUMER_TEE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BYTES_CONSUMER_TEE_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class ExecutionContext;
class BytesConsumer;

// Creates two BytesConsumer both of which represent the data sequence that
// would be read from |src| and store them to |*dest1| and |*dest2|.
// |src| must not have a client when called.
CORE_EXPORT void BytesConsumerTee(ExecutionContext*,
                                  BytesConsumer* src,
                                  BytesConsumer** dest1,
                                  BytesConsumer** dest2);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BYTES_CONSUMER_TEE_H_

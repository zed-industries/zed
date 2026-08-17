// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_RECEIVE_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_RECEIVE_STREAM_H_

#include <stdint.h>

#include "mojo/public/cpp/system/data_pipe.h"
#include "third_party/blink/renderer/core/streams/readable_stream.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webtransport/incoming_stream.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class ScriptState;
class WebTransport;

// Implementation of ReceiveStream from the standard:
// https://wicg.github.io/web-transport/#receive-stream.

class MODULES_EXPORT ReceiveStream final : public ReadableStream {
 public:
  // ReceiveStream doesn't have a JavaScript constructor. It is only
  // constructed from C++.
  explicit ReceiveStream(ScriptState*,
                         WebTransport*,
                         uint32_t stream_id,
                         mojo::ScopedDataPipeConsumerHandle);

  void Init(ExceptionState& exception_state) {
    incoming_stream_->InitWithExistingReadableStream(this, exception_state);
  }

  IncomingStream* GetIncomingStream() { return incoming_stream_.Get(); }

  void Trace(Visitor*) const override;

 private:
  const Member<IncomingStream> incoming_stream_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_RECEIVE_STREAM_H_

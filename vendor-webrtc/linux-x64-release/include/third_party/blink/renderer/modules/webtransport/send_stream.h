// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_SEND_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_SEND_STREAM_H_

#include <stdint.h>

#include "mojo/public/cpp/system/data_pipe.h"
#include "third_party/blink/renderer/core/streams/writable_stream.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webtransport/outgoing_stream.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class ExceptionState;
class WebTransport;
class ScriptState;

class MODULES_EXPORT SendStream final : public WritableStream {
 public:
  // SendStream doesn't have a JavaScript constructor. It is only constructed
  // from C++.
  explicit SendStream(ScriptState*,
                      WebTransport*,
                      uint32_t stream_id,
                      mojo::ScopedDataPipeProducerHandle);
  ~SendStream() override;

  void Init(ExceptionState& exception_state) {
    outgoing_stream_->InitWithExistingWritableStream(this, exception_state);
  }

  OutgoingStream* GetOutgoingStream() { return outgoing_stream_.Get(); }

  void Trace(Visitor*) const override;

 private:
  const Member<OutgoingStream> outgoing_stream_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_SEND_STREAM_H_

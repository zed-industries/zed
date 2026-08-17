// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_READABLE_STREAM_BYTES_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_READABLE_STREAM_BYTES_CONSUMER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/streams/readable_stream.h"
#include "third_party/blink/renderer/core/streams/readable_stream_default_reader.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ScriptState;

// This class is a BytesConsumer pulling bytes from a ReadableStream.
// The stream will be immediately locked by the consumer and will never be
// released. The stream must not be locked before this object is created.
class CORE_EXPORT ReadableStreamBytesConsumer final : public BytesConsumer {
 public:
  ReadableStreamBytesConsumer(ScriptState*, ReadableStream*);
  ReadableStreamBytesConsumer(const ReadableStreamBytesConsumer&) = delete;
  ReadableStreamBytesConsumer& operator=(const ReadableStreamBytesConsumer&) =
      delete;
  ~ReadableStreamBytesConsumer() override;

  Result BeginRead(base::span<const char>& buffer) override;
  Result EndRead(size_t read_size) override;
  void SetClient(BytesConsumer::Client*) override;
  void ClearClient() override;

  void Cancel() override;
  PublicState GetPublicState() const override;
  Error GetError() const override;
  String DebugName() const override { return "ReadableStreamBytesConsumer"; }

  void Trace(Visitor*) const override;

 private:
  class BytesConsumerReadRequest;

  void OnRead(DOMUint8Array*);
  void OnReadDone();
  void OnRejected();
  void SetErrored();
  void Notify();

  Member<ReadableStreamDefaultReader> reader_;
  Member<ScriptState> script_state_;
  Member<BytesConsumer::Client> client_;
  Member<DOMUint8Array> pending_buffer_;
  size_t pending_offset_ = 0;
  PublicState state_ = PublicState::kReadableOrWaiting;
  bool is_reading_ = false;
  bool is_inside_read_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_READABLE_STREAM_BYTES_CONSUMER_H_

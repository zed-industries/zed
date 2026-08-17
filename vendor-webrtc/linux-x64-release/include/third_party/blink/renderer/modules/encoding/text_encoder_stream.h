// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ENCODING_TEXT_ENCODER_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ENCODING_TEXT_ENCODER_STREAM_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/streams/transform_stream.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ReadableStream;
class ScriptState;
class WritableStream;

// Implements the TextEncoderStream interface as specified at
// https://encoding.spec.whatwg.org/#interface-textencoderstream.
// Converts a stream of text data in the form of string chunks to a stream of
// binary data in the form of UInt8Array chunks. After construction
// functionality is delegated to the owned TransformStream.
class TextEncoderStream final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static TextEncoderStream* Create(ScriptState*, ExceptionState&);

  TextEncoderStream(ScriptState*, ExceptionState&);

  TextEncoderStream(const TextEncoderStream&) = delete;
  TextEncoderStream& operator=(const TextEncoderStream&) = delete;

  ~TextEncoderStream() override;

  // From text_encoder_stream.idl
  String encoding() const;
  ReadableStream* readable() const;
  WritableStream* writable() const;

  void Trace(Visitor* visitor) const override;

 private:
  class Transformer;

  const Member<TransformStream> transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ENCODING_TEXT_ENCODER_STREAM_H_

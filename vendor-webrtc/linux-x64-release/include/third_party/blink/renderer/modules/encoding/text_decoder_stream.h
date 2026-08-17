// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ENCODING_TEXT_DECODER_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ENCODING_TEXT_DECODER_STREAM_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/streams/transform_stream.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ReadableStream;
class ScriptState;
class TextDecoderOptions;
class WritableStream;

// Implements the TextDecoderStream interface as specified at
// https://encoding.spec.whatwg.org/#interface-textdecoderstream.
// Converts a stream of binary data in the form of BufferSource chunks to a
// stream of text data in the form of string chunks. After construction
// functionality is delegated to the owner TransformStream.
class TextDecoderStream final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static TextDecoderStream* Create(ScriptState*,
                                   const String& label,
                                   const TextDecoderOptions*,
                                   ExceptionState&);

  TextDecoderStream(ScriptState*,
                    const WTF::TextEncoding&,
                    const TextDecoderOptions*,
                    ExceptionState&);

  TextDecoderStream(const TextDecoderStream&) = delete;
  TextDecoderStream& operator=(const TextDecoderStream&) = delete;

  ~TextDecoderStream() override;

  // From text_decoder_stream.idl
  String encoding() const;
  bool fatal() const { return fatal_; }
  bool ignoreBOM() const { return ignore_bom_; }
  ReadableStream* readable() const;
  WritableStream* writable() const;

  void Trace(Visitor* visitor) const override;

 private:
  class Transformer;

  const Member<TransformStream> transform_;
  const WTF::TextEncoding encoding_;
  const bool fatal_;
  const bool ignore_bom_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ENCODING_TEXT_DECODER_STREAM_H_

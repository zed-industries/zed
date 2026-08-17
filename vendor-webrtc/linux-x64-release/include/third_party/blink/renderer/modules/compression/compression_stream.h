// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_COMPRESSION_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_COMPRESSION_STREAM_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/streams/transform_stream.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class CompressionStream final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CompressionStream* Create(ScriptState*,
                                   const AtomicString&,
                                   ExceptionState&);
  CompressionStream(ScriptState*, const AtomicString&, ExceptionState&);

  CompressionStream(const CompressionStream&) = delete;
  CompressionStream& operator=(const CompressionStream&) = delete;

  ReadableStream* readable() const;
  WritableStream* writable() const;

  void Trace(Visitor*) const override;

 private:
  Member<TransformStream> transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_COMPRESSION_STREAM_H_

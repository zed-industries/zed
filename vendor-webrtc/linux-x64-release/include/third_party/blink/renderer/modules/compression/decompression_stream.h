// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_DECOMPRESSION_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_DECOMPRESSION_STREAM_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/streams/transform_stream.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class DecompressionStream final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DecompressionStream* Create(ScriptState*,
                                     const AtomicString&,
                                     ExceptionState&);
  DecompressionStream(ScriptState*, const AtomicString&, ExceptionState&);

  DecompressionStream(const DecompressionStream&) = delete;
  DecompressionStream& operator=(const DecompressionStream&) = delete;

  ReadableStream* readable() const;
  WritableStream* writable() const;

  void Trace(Visitor* visitor) const override;

 private:
  Member<TransformStream> transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPRESSION_DECOMPRESSION_STREAM_H_

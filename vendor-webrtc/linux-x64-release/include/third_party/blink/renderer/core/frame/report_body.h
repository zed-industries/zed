// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORT_BODY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORT_BODY_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_object_builder.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class CORE_EXPORT ReportBody : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~ReportBody() override = default;

  ScriptObject toJSON(ScriptState* script_state) const;

  // This function is public for use in Report::toJSON
  virtual void BuildJSONValue(V8ObjectBuilder& builder) const = 0;

  // Provides a hash-like value for identifying reports with same content.
  // Collision of match id is possible.
  virtual unsigned MatchId() const { return 0; }

  // Returns true if this report body would contain an extension URL as the
  // report source.
  virtual bool IsExtensionSource() const { return false; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REPORT_BODY_H_

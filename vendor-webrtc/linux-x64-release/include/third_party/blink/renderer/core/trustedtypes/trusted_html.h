// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_HTML_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_HTML_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {
class ScriptValue;
class ExceptionState;

class CORE_EXPORT TrustedHTML final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit TrustedHTML(String html);

  // TrustedHTML.idl
  const String& toString() const;
  const String& toJSON() const { return toString(); }
  static TrustedHTML* fromLiteral(ScriptState* script_state,
                                  const ScriptValue& templateString,
                                  ExceptionState& exception_state);

 private:
  const String html_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_HTML_H_

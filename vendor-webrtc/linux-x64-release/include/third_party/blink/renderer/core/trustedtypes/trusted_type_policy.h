// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_TYPE_POLICY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_TYPE_POLICY_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_trusted_type_policy_options.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class TrustedHTML;
class TrustedScript;
class TrustedScriptURL;

class CORE_EXPORT TrustedTypePolicy final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TrustedTypePolicy(const String& policy_name, TrustedTypePolicyOptions*);

  TrustedHTML* createHTML(v8::Isolate*,
                          const String&,
                          const HeapVector<ScriptValue>&,
                          ExceptionState&);
  TrustedScript* createScript(v8::Isolate*,
                              const String&,
                              const HeapVector<ScriptValue>&,
                              ExceptionState&);
  TrustedScriptURL* createScriptURL(v8::Isolate*,
                                    const String&,
                                    const HeapVector<ScriptValue>&,
                                    ExceptionState&);

  bool HasCreateHTML();
  bool HasCreateScript();
  bool HasCreateScriptURL();

  String name() const;

  void Trace(Visitor*) const override;

 private:
  String name_;
  Member<TrustedTypePolicyOptions> policy_options_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TRUSTEDTYPES_TRUSTED_TYPE_POLICY_H_

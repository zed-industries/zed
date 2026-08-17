// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_IMPORT_META_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_IMPORT_META_H_

#include "third_party/blink/renderer/bindings/core/v8/script_function.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class Modulator;
class ScriptState;

// Represents import.meta data structure, which is the return value of
// https://html.spec.whatwg.org/C/#hostgetimportmetaproperties
class CORE_EXPORT ModuleImportMeta final {
  STACK_ALLOCATED();

 public:
  explicit ModuleImportMeta(const String& url) : url_(url) {}

  const String& Url() const { return url_; }

  // This will return a fresh function each time, so generally this should only
  // be called once.
  const v8::Local<v8::Function> MakeResolveV8Function(Modulator*) const;

 private:
  class Resolve final : public ScriptFunction {
   public:
    explicit Resolve(Modulator* modulator, String url)
        : modulator_(modulator), url_(url) {}

    ScriptValue Call(ScriptState*, ScriptValue) override;
    int Length() const override { return 1; }
    void Trace(Visitor*) const override;

   private:
    const Member<Modulator> modulator_;
    const String url_;
  };

  const String url_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_IMPORT_META_H_

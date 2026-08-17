// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_RECORD_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_RECORD_RESOLVER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ModuleRecord;
class ModuleScript;

// The ModuleRecordResolver interface is used from V8 module bindings
// when it need the ModuleRecord's descendants.
//
// When a module writes import 'x', the module is called the referrer, 'x' is
// the specifier, and the module identified by 'x' is the descendant.
// ModuleRecordResolver, given a referrer and specifier, can look up the
// descendant.
class CORE_EXPORT ModuleRecordResolver
    : public GarbageCollected<ModuleRecordResolver> {
 public:
  virtual ~ModuleRecordResolver() = default;
  virtual void Trace(Visitor* visitor) const {}

  // Notifies the ModuleRecordResolver that a ModuleScript exists.
  // This hook gives a chance for the resolver impl to populate module record
  // identifier -> ModuleScript mapping entry.
  virtual void RegisterModuleScript(const ModuleScript*) = 0;

  // Notifies the ModuleRecordResolver to clear its ModuleScript mapping.
  virtual void UnregisterModuleScript(const ModuleScript*) = 0;

  virtual const ModuleScript* GetModuleScriptFromModuleRecord(
      v8::Local<v8::Module>) const = 0;

  // Implements "Runtime Semantics: HostResolveImportedModule"
  // https://tc39.github.io/ecma262/#sec-hostresolveimportedmodule
  // This returns a null ModuleRecord when an exception is thrown.
  virtual v8::Local<v8::Module> Resolve(const ModuleRequest& module_request,
                                        v8::Local<v8::Module> referrer,
                                        ExceptionState&) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_RECORD_RESOLVER_H_

// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_V8_CONTEXT_SNAPSHOT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_V8_CONTEXT_SNAPSHOT_IMPL_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

class DOMWrapperWorld;
class Document;
class ScriptState;

class MODULES_EXPORT V8ContextSnapshotImpl {
  STATIC_ONLY(V8ContextSnapshotImpl);

 public:
  static void Init();

  static v8::Local<v8::Context> CreateContext(
      v8::Isolate* isolate,
      const DOMWrapperWorld& world,
      v8::ExtensionConfiguration* extension_config,
      v8::Local<v8::Object> global_proxy,
      Document* document);

  static void InstallContextIndependentProps(ScriptState* script_state);

  static void InstallInterfaceTemplates(v8::Isolate* isolate);

  static v8::StartupData TakeSnapshot(v8::Isolate* isolate);

  static const intptr_t* GetReferenceTable();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_V8_CONTEXT_SNAPSHOT_IMPL_H_

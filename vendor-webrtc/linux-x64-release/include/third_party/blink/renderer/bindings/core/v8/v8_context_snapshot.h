// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CONTEXT_SNAPSHOT_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CONTEXT_SNAPSHOT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

class DOMWrapperWorld;
class Document;
class ScriptState;

// The V8 context snapshot is taken by //tools/v8_context_snapshot at build
// time, and it makes it faster to create a new v8::Context and global object.
//
// The actual implementation is provided by V8ContextSnapshotImpl.
class CORE_EXPORT V8ContextSnapshot {
  STATIC_ONLY(V8ContextSnapshot);

 public:
  static v8::Local<v8::Context> CreateContextFromSnapshot(
      v8::Isolate* isolate,
      const DOMWrapperWorld& world,
      v8::ExtensionConfiguration* extension_config,
      v8::Local<v8::Object> global_proxy,
      Document* document);

  static void InstallContextIndependentProps(ScriptState* script_state);

  static void EnsureInterfaceTemplates(v8::Isolate* isolate);

  static v8::StartupData TakeSnapshot(v8::Isolate* isolate);

  static const intptr_t* GetReferenceTable();

  using CreateContextFromSnapshotFuncType =
      v8::Local<v8::Context> (*)(v8::Isolate*,
                                 const DOMWrapperWorld&,
                                 v8::ExtensionConfiguration*,
                                 v8::Local<v8::Object>,
                                 Document*);
  static void SetCreateContextFromSnapshotFunc(
      CreateContextFromSnapshotFuncType func);
  using InstallContextIndependentPropsFuncType = void (*)(ScriptState*);
  static void SetInstallContextIndependentPropsFunc(
      InstallContextIndependentPropsFuncType func);
  using EnsureInterfaceTemplatesFuncType = void (*)(v8::Isolate*);
  static void SetEnsureInterfaceTemplatesFunc(
      EnsureInterfaceTemplatesFuncType func);
  using TakeSnapshotFuncType = v8::StartupData (*)(v8::Isolate*);
  static void SetTakeSnapshotFunc(TakeSnapshotFuncType func);
  using GetReferenceTableFuncType = const intptr_t* (*)();
  static void SetGetReferenceTableFunc(GetReferenceTableFuncType func);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CONTEXT_SNAPSHOT_H_

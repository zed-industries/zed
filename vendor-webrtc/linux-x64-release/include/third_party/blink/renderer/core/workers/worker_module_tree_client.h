// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_MODULE_TREE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_MODULE_TREE_CLIENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ModuleScript;
class ScriptState;

// A ModuleTreeClient that lives on the worker context's thread.
class CORE_EXPORT WorkerModuleTreeClient final : public ModuleTreeClient {
 public:
  explicit WorkerModuleTreeClient(ScriptState*);

  // Implements ModuleTreeClient.
  void NotifyModuleTreeLoadFinished(ModuleScript*) final;

  void Trace(Visitor*) const override;

 private:
  Member<ScriptState> script_state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_MODULE_TREE_CLIENT_H_

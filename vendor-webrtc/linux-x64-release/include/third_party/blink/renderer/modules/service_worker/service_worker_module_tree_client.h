// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_MODULE_TREE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_MODULE_TREE_CLIENT_H_

#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ModuleScript;
class ScriptState;

// This is an implementation of ModuleTreeClient for service workers that lives
// on the worker context's thread.
class ServiceWorkerModuleTreeClient final : public ModuleTreeClient {
 public:
  explicit ServiceWorkerModuleTreeClient(ScriptState*);

  // Implements ModuleTreeClient.
  void NotifyModuleTreeLoadFinished(ModuleScript*) final;

  void Trace(Visitor*) const override;

 private:
  Member<ScriptState> script_state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_MODULE_TREE_CLIENT_H_

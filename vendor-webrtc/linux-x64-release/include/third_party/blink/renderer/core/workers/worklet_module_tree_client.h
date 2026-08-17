// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_MODULE_TREE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_MODULE_TREE_CLIENT_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/core/workers/worklet_pending_tasks.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ModuleScript;
class ScriptState;

// A ModuleTreeClient that lives on the worklet context's thread.
class WorkletModuleTreeClient final : public ModuleTreeClient {
 public:
  WorkletModuleTreeClient(
      ScriptState*,
      scoped_refptr<base::SingleThreadTaskRunner> outside_settings_task_runner,
      WorkletPendingTasks*);

  // Implements ModuleTreeClient.
  void NotifyModuleTreeLoadFinished(ModuleScript*) final;

  void Trace(Visitor*) const override;

 private:
  Member<ScriptState> script_state_;
  scoped_refptr<base::SingleThreadTaskRunner> outside_settings_task_runner_;
  CrossThreadPersistent<WorkletPendingTasks> pending_tasks_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_MODULE_TREE_CLIENT_H_

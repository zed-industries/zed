// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_GLOBAL_SCOPE_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_GLOBAL_SCOPE_PROXY_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class FetchClientSettingsObjectSnapshot;
class WorkletPendingTasks;
class WorkerResourceTimingNotifier;

// Abstracts communication from (Main/Threaded)Worklet on the main thread to
// (Main/Threaded)WorkletGlobalScope so that Worklet class doesn't have to care
// about the thread WorkletGlobalScope runs on.
class CORE_EXPORT WorkletGlobalScopeProxy : public GarbageCollectedMixin {
 public:
  virtual ~WorkletGlobalScopeProxy() = default;

  // Runs the "fetch and invoke a worklet script" algorithm:
  // https://drafts.css-houdini.org/worklets/#fetch-and-invoke-a-worklet-script
  virtual void FetchAndInvokeScript(
      const KURL& module_url_record,
      network::mojom::CredentialsMode,
      const FetchClientSettingsObjectSnapshot& outside_settings_object,
      WorkerResourceTimingNotifier& outside_resource_timing_notifier,
      scoped_refptr<base::SingleThreadTaskRunner> outside_settings_task_runner,
      WorkletPendingTasks*) = 0;

  // Notifies that the Worklet object is destroyed. This should be called in the
  // destructor of the Worklet object. This may call
  // ThreadedMessagingProxy::ParentObjectDestroyed() and cause deletion of
  // |this|. See comments on ParentObjectDestroyed() for details.
  virtual void WorkletObjectDestroyed() = 0;

  // Terminates the worklet global scope from the main thread.
  virtual void TerminateWorkletGlobalScope() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_GLOBAL_SCOPE_PROXY_H_

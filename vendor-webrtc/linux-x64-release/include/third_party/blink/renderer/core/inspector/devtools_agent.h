// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEVTOOLS_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEVTOOLS_AGENT_H_

#include <memory>

#include "base/task/single_thread_task_runner.h"
#include "base/unguessable_token.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/devtools/devtools_agent.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace blink {

class CoreProbeSink;
class DevToolsSession;
class ExecutionContext;
class InspectedFrames;
class InspectorTaskRunner;
class WorkerThread;
struct WorkerDevToolsParams;

// All public methods of this class are expected to be called on the same thread
// that created the instance. That might be the main thread or a worker thread.
// If used on a worker via BindReceiverForWorker() this class will delegate
// internally to the IO thread to avoid blocking the worker thread. See
// DevToolsAgent::IOAgent for more details.
class CORE_EXPORT DevToolsAgent : public GarbageCollected<DevToolsAgent>,
                                  public mojom::blink::DevToolsAgent {
 public:
  class Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() = default;
    virtual void AttachSession(DevToolsSession*, bool restore) = 0;
    virtual void DetachSession(DevToolsSession*) = 0;
    virtual void InspectElement(const gfx::Point&) = 0;
    virtual void DebuggerTaskStarted() = 0;
    virtual void DebuggerTaskFinished() = 0;
  };

  static std::unique_ptr<WorkerDevToolsParams> WorkerThreadCreated(
      ExecutionContext* parent_context,
      WorkerThread*,
      const KURL&,
      const String& global_scope_name,
      const std::optional<const DedicatedWorkerToken>& token);
  static void WorkerThreadTerminated(ExecutionContext* parent_context,
                                     WorkerThread*);

  DevToolsAgent(Client*,
                InspectedFrames*,
                CoreProbeSink*,
                scoped_refptr<InspectorTaskRunner> inspector_task_runner,
                scoped_refptr<base::SingleThreadTaskRunner> io_task_runner);
  ~DevToolsAgent() override;

  void Dispose();
  void FlushProtocolNotifications();
  void DebuggerPaused();
  void DebuggerResumed();
  void BringDevToolsWindowToFocus();
  // For workers, we use the IO thread similar to DevToolsSession::IOSession to
  // ensure that we can always interrupt a worker that is stuck in JS. We don't
  // use an associated channel for workers, meaning we don't have the ordering
  // constraints related to navigation that the non-worker agents have.
  void BindReceiverForWorker(
      mojo::PendingRemote<mojom::blink::DevToolsAgentHost>,
      mojo::PendingReceiver<mojom::blink::DevToolsAgent>,
      scoped_refptr<base::SingleThreadTaskRunner>);
  // Used for non-worker agents. These do not use the IO thread like we do for
  // workers, and they use associated mojo interfaces.
  void BindReceiver(
      mojo::PendingAssociatedRemote<mojom::blink::DevToolsAgentHost>,
      mojo::PendingAssociatedReceiver<mojom::blink::DevToolsAgent>,
      scoped_refptr<base::SingleThreadTaskRunner>);
  virtual void Trace(Visitor*) const;

 private:
  friend class DevToolsSession;
  class IOAgent;

  // mojom::blink::DevToolsAgent implementation.
  void AttachDevToolsSession(
      mojo::PendingAssociatedRemote<mojom::blink::DevToolsSessionHost>,
      mojo::PendingAssociatedReceiver<mojom::blink::DevToolsSession>
          main_session,
      mojo::PendingReceiver<mojom::blink::DevToolsSession> io_session,
      mojom::blink::DevToolsSessionStatePtr reattach_session_state,
      const WTF::String& script_to_evaluate_on_load,
      bool client_expects_binary_responses,
      bool client_is_trusted,
      const WTF::String& session_id,
      bool session_waits_for_debugger) override;
  void InspectElement(const gfx::Point& point) override;
  void ReportChildTargets(bool report,
                          bool wait_for_debugger,
                          base::OnceClosure callback) override;

  void ReportChildTargetsPostCallbackToIO(bool report,
                                          bool wait_for_debugger,
                                          CrossThreadOnceClosure callback);

  struct WorkerData {
    KURL url;
    mojo::PendingRemote<mojom::blink::DevToolsAgent> agent_remote;
    mojo::PendingReceiver<mojom::blink::DevToolsAgentHost> host_receiver;
    base::UnguessableToken devtools_worker_token;
    bool waiting_for_debugger;
    String name;
    mojom::blink::DevToolsExecutionContextType context_type;
  };
  void ReportChildTarget(std::unique_ptr<WorkerData>);

  void CleanupConnection();

  void AttachDevToolsSessionImpl(
      mojo::PendingAssociatedRemote<mojom::blink::DevToolsSessionHost>,
      mojo::PendingAssociatedReceiver<mojom::blink::DevToolsSession>
          main_session,
      mojo::PendingReceiver<mojom::blink::DevToolsSession> io_session,
      mojom::blink::DevToolsSessionStatePtr reattach_session_state,
      const WTF::String& script_to_evaluate_on_load,
      bool client_expects_binary_responses,
      bool client_is_trusted,
      const WTF::String& session_id,
      bool session_waits_for_debugger);
  void DetachDevToolsSession(DevToolsSession* session);
  void InspectElementImpl(const gfx::Point& point);
  void ReportChildTargetsImpl(bool report,
                              bool wait_for_debugger,
                              base::OnceClosure callback);
  Member<Client> const client_;
  // DevToolsAgent is not tied to ExecutionContext
  HeapMojoAssociatedReceiver<mojom::blink::DevToolsAgent, DevToolsAgent>
      associated_receiver_{this, nullptr};
  // DevToolsAgent is not tied to ExecutionContext
  HeapMojoRemote<mojom::blink::DevToolsAgentHost> host_remote_{nullptr};
  // DevToolsAgent is not tied to ExecutionContext
  HeapMojoAssociatedRemote<mojom::blink::DevToolsAgentHost>
      associated_host_remote_{nullptr};
  Member<InspectedFrames> inspected_frames_;
  Member<CoreProbeSink> probe_sink_;
  HeapHashSet<Member<DevToolsSession>> sessions_;
  scoped_refptr<InspectorTaskRunner> inspector_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> io_task_runner_;
  HashMap<WorkerThread*, std::unique_ptr<WorkerData>>
      unreported_child_worker_threads_;
  IOAgent* io_agent_{nullptr};
  bool report_child_workers_ = false;
  bool pause_child_workers_on_start_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_DEVTOOLS_AGENT_H_

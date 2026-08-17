/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_DEFERRED_TASK_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_DEFERRED_TASK_HANDLER_H_

#include <atomic>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"
#include "third_party/blink/renderer/platform/wtf/threading_primitives.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class BaseAudioContext;
class OfflineAudioContext;
class AudioHandler;
class AudioNodeOutput;
class AudioSummingJunction;

// DeferredTaskHandler manages the major part of pre- and post- rendering tasks,
// and provides a lock mechanism against the audio rendering graph. A
// DeferredTaskHandler object is created when an BaseAudioContext object is
// created.
//
// DeferredTaskHandler outlives the BaseAudioContext only if all of the
// following conditions match:
// - An audio rendering thread is running,
// - It is requested to stop,
// - The audio rendering thread calls requestToDeleteHandlersOnMainThread(),
// - It posts a task of deleteHandlersOnMainThread(), and
// - GC happens and it collects the BaseAudioContext before the task execution.
//
class MODULES_EXPORT DeferredTaskHandler final
    : public ThreadSafeRefCounted<DeferredTaskHandler> {
 public:
  static scoped_refptr<DeferredTaskHandler> Create(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  ~DeferredTaskHandler();

  void HandleDeferredTasks();
  void ContextWillBeDestroyed();

  // BaseAudioContext can pull node(s) at the end of each render quantum even
  // when they are not connected to any downstream nodes.  These two methods are
  // called by the nodes who want to add/remove themselves into/from the
  // automatic pull lists.
  void AddAutomaticPullNode(scoped_refptr<AudioHandler>);
  void RemoveAutomaticPullNode(AudioHandler*);

  // Return true if there is one or more nodes in the automatic pull node list.
  bool HasAutomaticPullNodes();

  // Called right before handlePostRenderTasks() to handle nodes which need to
  // be pulled even when they are not connected to anything.
  void ProcessAutomaticPullNodes(uint32_t frames_to_process);

  // Keep track of AudioNode's that have their channel count mode changed. We
  // process the changes in the post rendering phase.
  void AddChangedChannelCountMode(AudioHandler*);
  void RemoveChangedChannelCountMode(AudioHandler*);

  // Keep track of AudioNode's that have their channel interpretation
  // changed. We process the changes in the post rendering phase.
  void AddChangedChannelInterpretation(AudioHandler*);
  void RemoveChangedChannelInterpretation(AudioHandler*);

  // Only accessed when the graph lock is held.
  void MarkSummingJunctionDirty(AudioSummingJunction*);
  // Only accessed when the graph lock is held. Must be called on the main
  // thread.
  void RemoveMarkedSummingJunction(AudioSummingJunction*);

  void MarkAudioNodeOutputDirty(AudioNodeOutput*);
  void RemoveMarkedAudioNodeOutput(AudioNodeOutput*);

  // Break connections between nodes.  This is done on the audio thread with the
  // graph lock.
  void BreakConnections();

  void AddRenderingOrphanHandler(scoped_refptr<AudioHandler>);
  void RequestToDeleteHandlersOnMainThread();
  void ClearHandlersToBeDeleted();

  // Clear the context from the rendering and deletable orphan handlers.
  void ClearContextFromOrphanHandlers();

  bool AcceptsTailProcessing() const { return accepts_tail_processing_; }
  void StopAcceptingTailProcessing() { accepts_tail_processing_ = false; }

  // If `node` requires tail processing, add it to the list of tail
  // nodes so the tail is processed.
  void AddTailProcessingHandler(scoped_refptr<AudioHandler>);

  // Remove `node` from the list of tail nodes (because the tail processing is
  // complete).  Set `disable_outputs` to true if the outputs of the handler
  // should also be disabled.  This should be true if the tail is done.  But if
  // we're reconnected or re-enabled, then `disable_outputs` should be false.
  void RemoveTailProcessingHandler(AudioHandler*, bool disable_outputs);

  // Remove all tail processing nodes.  Should be called only when the
  // context is done.
  void FinishTailProcessing();

  // For handlers that have finished processing their tail and require disabling
  // the ouputs of the handler, we do that here.
  void DisableOutputsForTailProcessing();

  //
  // Thread Safety and Graph Locking:
  //
  void SetAudioThreadToCurrentThread();

  // It is okay to use a relaxed (no-barrier) load here. Because the data
  // referenced by m_audioThread is not actually being used, thus we do not
  // need a barrier between the load of m_audioThread and of that data.
  bool IsAudioThread() const {
    return CurrentThread() == audio_thread_.load(std::memory_order_relaxed);
  }

  void lock() EXCLUSIVE_LOCK_FUNCTION(context_graph_mutex_);
  bool TryLock() EXCLUSIVE_TRYLOCK_FUNCTION(true, context_graph_mutex_);
  void unlock() UNLOCK_FUNCTION(context_graph_mutex_);

  // This locks the audio render thread for OfflineAudioContext rendering.
  // MUST NOT be used in the real-time audio context.
  void OfflineLock() EXCLUSIVE_LOCK_FUNCTION(context_graph_mutex_);

  // In DCHECK builds, fails if this thread does not own the context's lock.
  void AssertGraphOwner() const ASSERT_EXCLUSIVE_LOCK(context_graph_mutex_) {
    context_graph_mutex_.AssertAcquired();
  }

  class MODULES_EXPORT GraphAutoLocker {
    STACK_ALLOCATED();

   public:
    explicit GraphAutoLocker(DeferredTaskHandler& handler) : handler_(handler) {
      handler_.lock();
    }
    explicit GraphAutoLocker(const BaseAudioContext*);

    ~GraphAutoLocker() { handler_.unlock(); }

   private:
    DeferredTaskHandler& handler_;
  };

  // This is for locking offline render thread (which is considered as the
  // audio thread) with unlocking on self-destruction at the end of the scope.
  // Also note that it uses lock() rather than tryLock() because the timing
  // MUST be accurate on offline rendering.
  class MODULES_EXPORT OfflineGraphAutoLocker {
    STACK_ALLOCATED();

   public:
    explicit OfflineGraphAutoLocker(OfflineAudioContext*);

    ~OfflineGraphAutoLocker() { handler_.unlock(); }

   private:
    DeferredTaskHandler& handler_;
  };

  HashSet<scoped_refptr<AudioHandler>>* GetActiveSourceHandlers() {
    return &active_source_handlers_;
  }

  Vector<scoped_refptr<AudioHandler>>* GetFinishedSourceHandlers() {
    return &finished_source_handlers_;
  }

  // The number of frames to render each time.  After construction, this value
  // is only read (never modified), and may be read by both the audio thread and
  // the main thread.
  unsigned int RenderQuantumFrames() const { return render_quantum_frames_; }

 private:
  explicit DeferredTaskHandler(scoped_refptr<base::SingleThreadTaskRunner>);
  void UpdateAutomaticPullNodes();
  void UpdateChangedChannelCountMode();
  void UpdateChangedChannelInterpretation();
  void HandleDirtyAudioSummingJunctions();
  void HandleDirtyAudioNodeOutputs();
  void DeleteHandlersOnMainThread();

  // Check tail processing handlers and remove any handler if the tail
  // has been processed.
  void UpdateTailProcessingHandlers();

  // For the sake of thread safety, we maintain a seperate Vector of
  // AudioHandlers for "automatic-pull nodes":
  // `rendering_automatic_pull_handlers_`. This storage will be copied from
  // `automatic_pull_handlers` by `UpdateAutomaticPullNodes()` at the beginning
  // or end of the render quantum.
  HashSet<scoped_refptr<AudioHandler>> automatic_pull_handlers_;
  Vector<scoped_refptr<AudioHandler>> rendering_automatic_pull_handlers_
      GUARDED_BY(automatic_pull_handlers_lock_);

  // Keeps track if the `automatic_pull_handlers` storage is touched.
  bool automatic_pull_handlers_need_updating_ = false;

  // Number of frames to use when rendering the graph.  This is the frames to
  // process for each node.
  unsigned int render_quantum_frames_ = 128;

  // Collection of nodes where the channel count mode has changed. We want the
  // channel count mode to change in the pre- or post-rendering phase so as
  // not to disturb the running audio thread.
  HashSet<AudioHandler*> deferred_count_mode_change_;

  HashSet<AudioHandler*> deferred_channel_interpretation_change_;

  // These two HashSet must be accessed only when the graph lock is held.
  // These raw pointers are safe because their destructors unregister them.
  HashSet<AudioSummingJunction*> dirty_summing_junctions_;
  HashSet<AudioNodeOutput*> dirty_audio_node_outputs_;

  Vector<scoped_refptr<AudioHandler>> rendering_orphan_handlers_;
  Vector<scoped_refptr<AudioHandler>> deletable_orphan_handlers_;

  // Nodes that are processing its tail.
  Vector<scoped_refptr<AudioHandler>> tail_processing_handlers_;

  // Tail processing nodes that are now finished and want the output to be
  // disabled.  This is updated in the audio thread (with the graph lock).  The
  // main thread will disable the outputs.
  Vector<scoped_refptr<AudioHandler>> finished_tail_processing_handlers_;

  // Once the associated context closes, new tail processing handlers are not
  // accepted.
  bool accepts_tail_processing_ = true;

  // When source nodes are started, we place the handlers here to keep track of
  // these active sources.  We must call AudioHandler::makeConnection() when we
  // add an AudioNode to this, and must call AudioHandler::breakConnection()
  // when we remove an AudioNode from this.
  //
  // This can be accessed from either the main thread or the audio thread, so it
  // must be protected by the graph lock.
  HashSet<scoped_refptr<AudioHandler>> active_source_handlers_;

  // When source nodes are finished, the handler is placed here to make a note
  // of it.  At a render quantum boundary, these are used to break the
  // connection and elements here are removed from `active_source_handlers_`.
  //
  // This must be accessed only from the audio thread.
  Vector<scoped_refptr<AudioHandler>> finished_source_handlers_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // Graph locking.
  RecursiveMutex context_graph_mutex_;

  // Protects `rendering_automatic_pull_handlers_` when updating, processing,
  // and clearing. (See crbug.com/1061018)
  mutable base::Lock automatic_pull_handlers_lock_;

  std::atomic<base::PlatformThreadId> audio_thread_;

  base::WeakPtrFactory<DeferredTaskHandler> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_DEFERRED_TASK_HANDLER_H_

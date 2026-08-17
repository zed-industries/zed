// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CACHE_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CACHE_CONSUMER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class CachedMetadata;
class ClassicScript;
class ScriptCacheConsumerClient;

// A V8 code cache consumer for a script's code cache, which consumes the code
// cache off-thread and notifies a given ScriptCacheConsumerClient once it has
// completed.
//
// ScriptCacheConsumer works on unchecked CachedMetadata speculatively, before
// the source is available. If CachedMetadataHandler::Check() fails later,
// the CachedMetadata on the CachedMetadataHandler will be cleared, and the
// result of this ScriptCacheConsumer will be dropped on TakeV8ConsumeTask() due
// to a CachedMetadata mismatch.
//
// The state of the ScriptCacheConsumer state is associated with a single
// CachedMetadata, but it is independent of the state of the corresponding
// ScriptResource or its CachedMetadataHandler. Thus, it's fine that one
// ClassicPendingScript for the ScriptResource can get a non-null
// ConsumeCodeCacheTask from TakeV8ConsumeTask(), while others with the same
// ScriptResource don't use ScriptCacheConsumer. It's fine even if a
// ClassicPendingScript is notified finished (with or without
// ScriptCacheConsumer), and executes a script for a ScriptResource while
// another ClassicPendingScript for the same ScriptResource with the same
// CachedMetadata is still waiting for ScriptCacheConsumer completion.
class CORE_EXPORT ScriptCacheConsumer final
    : public GarbageCollected<ScriptCacheConsumer> {
 public:
  // Construct a cache consumer for the given CachedMetadata, corresponding to
  // the given URL and resource ID.
  //
  // The CachedMetadata should contain a V8 code cache. An off-thread cache
  // consumption task is posted immediately when this object is constructed.
  ScriptCacheConsumer(v8::Isolate* isolate,
                      scoped_refptr<CachedMetadata> cached_metadata,
                      const String& script_url_string,
                      uint64_t script_resource_identifier);

  // Construct a cache consumer for the given CachedMetadata, the completed
  // consume task, corresponding to the given URL and resource ID.
  ScriptCacheConsumer(v8::Isolate* isolate,
                      scoped_refptr<CachedMetadata> cached_metadata,
                      std::unique_ptr<v8::ScriptCompiler::ConsumeCodeCacheTask>
                          completed_consume_task,
                      const String& script_url_string,
                      uint64_t script_resource_identifier);

  // Notify this cache consumer that the corresponding resource has completed.
  //
  // If the resource calls this, it expects this consumer to call
  // ScriptCacheConsumerClient::NotifyCacheConsumeFinished once the consumption
  // task completes. There are two possibilities:
  //
  //   1) If the task has already completed, it calls NotifyCacheConsumeFinished
  //      immediately and synchronously.
  //   2) Otherwise, it updates the state and returns, Then, when the off-thread
  //      cache consume task completes, it will post a task that calls
  //      NotifyCacheConsumeFinished using the given task runner.
  void NotifyClientWaiting(
      ScriptCacheConsumerClient* client,
      ClassicScript* classic_script,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  // Take ownership of the consume task, clearing it from the consumer so that
  // no one can attempt to use it twice.
  //
  // Pass in the CachedMetadata which should match the one the consume task was
  // created with -- if there is a mismatch (e.g. because the metadata was later
  // cleared), drop the consume task and return nullptr.
  //
  // Also, may return nullptr if no consume task was ever created.
  v8::ScriptCompiler::ConsumeCodeCacheTask* TakeV8ConsumeTask(
      CachedMetadata* cached_metadata) {
    CHECK_EQ(state_, kCalledFinishCallback);
    if (cached_metadata != cached_metadata_) {
      consume_task_.reset();
      return nullptr;
    }
    return consume_task_.release();
  }

  void Trace(Visitor* visitor) const;

 private:
  // Valid state transitions:
  //
  //                           kRunning
  //                              |
  //        RunTaskOffThread()    |     NotifyClientWaiting()
  //                     .--------'---------.
  //                     v                  v
  //             kConsumeFinished      kClientReady
  //                     |                  |
  //                     '--------.---------'
  //     NotifyClientWaiting()    |     RunTaskOffThread()
  //                              v
  //                      kFinishedAndReady
  //                              |
  //                              | RunTaskOffThread(), RunMergeTaskOffThread(),
  //                              | or NotifyClientWaiting()
  //                              v
  //                    kMergeDoneOrNotNeeded
  //                              |
  //                              | CallFinishCallback()
  //                              v
  //                    kCalledFinishCallback
  //
  // These states are represented as a bit field.
  enum State {
    kRunning = 0,
    kConsumeFinished = 0b10,
    kClientReady = 0b01,
    kFinishedAndReady = 0b11,
    kMergeDoneOrNotNeededBit = 0b100,
    kMergeDoneOrNotNeeded = kFinishedAndReady | kMergeDoneOrNotNeededBit,
    kCalledFinishCallbackBit = 0b1000,
    kCalledFinishCallback = kMergeDoneOrNotNeeded | kCalledFinishCallbackBit,
  };
  static_assert(
      (kConsumeFinished & kClientReady) == 0,
      "kConsumeFinished and kClientReady have to be independent bits");
  static_assert(
      (kConsumeFinished | kClientReady) == kFinishedAndReady,
      "kFinishedAndReady has to mean kConsumeFinished and kClientReady");

  // Advance the state by setting a single bit. Returns the new state.
  State AdvanceState(State new_state_bit);

  // Helper methods for running the consume task off-thread and posting back
  // from it.
  void RunTaskOffThread();
  void RunMergeTaskOffThread();
  void PostFinishCallbackTask();
  void CallFinishCallback();

  // This class is only created for the main thread. The main thread isolate
  // will be always outlive this object so it should be valid to use on any
  // thread.
  v8::Isolate* isolate_;

  // The cached metadata storing the code cache. This is held by the consumer
  // to keep the cached data alive even if it is cleared on the script resource.
  scoped_refptr<CachedMetadata> cached_metadata_;

  // The V8 task which consumes the code cache in the cached metadata. This is
  // expected to be run on a worker thread, and then to be passed back into V8
  // to be used.
  std::unique_ptr<v8::ScriptCompiler::ConsumeCodeCacheTask> consume_task_;

  // The client which should be notified when the consume task completes.
  WeakMember<ScriptCacheConsumerClient> finish_callback_client_;

  // The task runner on which the finish callback should be run.
  scoped_refptr<base::SingleThreadTaskRunner> finish_callback_task_runner_;

  // Keep the script URL string for event tracing.
  const String script_url_string_;

  // Keep the script resource dentifier for event tracing.
  const uint64_t script_resource_identifier_;

  const char* finish_trace_name_ = nullptr;

  // The state of this ScriptCacheConsumer, advanced atomically when this
  // consume task completes, and when the resource completes.
  std::atomic<State> state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CACHE_CONSUMER_H_

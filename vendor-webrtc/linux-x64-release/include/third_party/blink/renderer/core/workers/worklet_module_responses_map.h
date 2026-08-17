// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_MODULE_RESPONSES_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_MODULE_RESPONSES_MAP_H_

#include <optional>

#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_creation_params.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_fetcher.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/kurl_hash.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// WorkletModuleResponsesMap implements the module responses map concept and the
// "fetch a worklet script" algorithm:
// https://drafts.css-houdini.org/worklets/#module-responses-map
// https://drafts.css-houdini.org/worklets/#fetch-a-worklet-script
//
// This acts as a cache for creation params (including source code) of module
// scripts. The creation params are added using SetEntryParams(), and retrieved
// using GetEntry(). If a module script for a given URL has already been
// fetched, GetEntry() returns the cached creation params. Otherwise, GetEntry()
// returns false and defers to WorkletModuleScriptFetcher to perform the fetch.
// Once the module script is fetched, its creation params are cached and
// GetEntry() returns it. This class is created on the main thread and shared
// across worklet threads. All access to this class should be mutex-guarded,
// and any data passed in or read out is copied to ensure that this object's
// internal state can be safely destructed from the main thread.
class CORE_EXPORT WorkletModuleResponsesMap final
    : public GarbageCollected<WorkletModuleResponsesMap> {
 public:
  WorkletModuleResponsesMap() = default;

  // If |url| is already fetched, synchronously calls Client::OnFetched().
  // Otherwise, it's called on the completion of the fetch. See also the
  // class-level comment.
  // Called on worklet threads.
  bool GetEntry(const KURL&,
                ModuleType,
                ModuleScriptFetcher::Client*,
                scoped_refptr<base::SingleThreadTaskRunner> client_task_runner)
      LOCKS_EXCLUDED(lock_);

  // Called on worklet threads.
  void SetEntryParams(const KURL&,
                      ModuleType,
                      const std::optional<ModuleScriptCreationParams>&)
      LOCKS_EXCLUDED(lock_);

  // Called when the associated document is destroyed and clears the map.
  // Following GetEntry() calls synchronously call Client::OnFailed().
  // Called on main thread.
  void Dispose() LOCKS_EXCLUDED(lock_);

  void Trace(Visitor*) const {}

 private:
  class Entry final {
    USING_FAST_MALLOC(Entry);

   public:
    enum class State { kFetching, kFetched, kFailed };
    Entry() = default;
    ~Entry() = default;

    State GetState() const { return state_; }
    ModuleScriptCreationParams GetParams() const {
      return params_->IsolatedCopy();
    }
    void AddClient(
        ModuleScriptFetcher::Client* client,
        scoped_refptr<base::SingleThreadTaskRunner> client_task_runner);
    void SetParams(const std::optional<ModuleScriptCreationParams>& params);

   private:
    State state_ = State::kFetching;
    std::optional<ModuleScriptCreationParams> params_;
    HashMap<CrossThreadPersistent<ModuleScriptFetcher::Client>,
            scoped_refptr<base::SingleThreadTaskRunner>>
        clients_;
  };

  // |is_available_| is written to false by the main thread on disposal, and
  // read by any thread.
  bool is_available_ GUARDED_BY(lock_) = true;

  // TODO(nhiroki): Keep the insertion order of top-level modules to replay
  // addModule() calls for a newly created global scope.
  // See https://drafts.css-houdini.org/worklets/#creating-a-workletglobalscope
  // Can be read/written by any thread.
  using Key = std::pair<KURL, ModuleType>;
  HashMap<Key, std::unique_ptr<Entry>> entries_ GUARDED_BY(lock_);

  base::Lock lock_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKLET_MODULE_RESPONSES_MAP_H_

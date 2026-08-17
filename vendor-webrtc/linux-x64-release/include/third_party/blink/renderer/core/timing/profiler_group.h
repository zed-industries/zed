// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PROFILER_GROUP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PROFILER_GROUP_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/security_context.h"
#include "third_party/blink/renderer/platform/bindings/v8_per_isolate_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-profiler.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class LocalDOMWindow;
class Profiler;
class ProfilerInitOptions;
class ProfilerTrace;
class ScriptState;

// A ProfilerGroup represents a set of profilers sharing an underlying
// v8::CpuProfiler attached to a common isolate.
class CORE_EXPORT ProfilerGroup : public V8PerIsolateData::UserData {
 public:
  // Determines whether or not the given frame can profile. Logs an exception
  // in the given ExceptionState (if non-null) if profiling is not permitted,
  // and returns false.
  static bool CanProfile(LocalDOMWindow*,
                         ExceptionState* = nullptr,
                         ReportOptions = ReportOptions::kDoNotReport);

  // Initializes logging for the given LocalDOMWindow if CanProfile returns
  // true.
  static void InitializeIfEnabled(LocalDOMWindow*);

  static ProfilerGroup* From(v8::Isolate*);

  static base::TimeDelta GetBaseSampleInterval();

  ProfilerGroup(v8::Isolate* isolate);
  ProfilerGroup(const ProfilerGroup&) = delete;
  ProfilerGroup& operator=(const ProfilerGroup&) = delete;
  ~ProfilerGroup() override;

  Profiler* CreateProfiler(ScriptState* script_state,
                           const ProfilerInitOptions&,
                           base::TimeTicks time_origin,
                           ExceptionState&);

  // Tracks a profiling-enabled document's lifecycle, ensuring that the
  // profiler is ready during its lifetime.
  void OnProfilingContextAdded(ExecutionContext* context);

  void DispatchSampleBufferFullEvent(String profiler_id);
  void WillBeDestroyed() override;
  void Trace(Visitor*) const override;

 private:
  friend class Profiler;
  class ProfilingContextObserver;

  void OnProfilingContextDestroyed(ProfilingContextObserver*);

  void InitV8Profiler();
  void TeardownV8Profiler();

  void StopProfiler(ScriptState*,
                    Profiler*,
                    ScriptPromiseResolver<ProfilerTrace>*);

  // Cancels a profiler, discarding its associated trace.
  void CancelProfiler(Profiler*);
  // Asynchronously cancels a profiler. Invoked on Profiler destruction.
  void CancelProfilerAsync(ScriptState*, Profiler*);
  // Internal implementation of cancel.
  void CancelProfilerImpl(String profiler_id);

  // Clean context independent resources for leaked profilers
  void StopDetachedProfiler(String profiler_id);
  void StopDetachedProfilers();

  // Generates an unused string identifier to use for a new profiling session.
  String NextProfilerId();

  v8::Isolate* const isolate_;
  v8::CpuProfiler* cpu_profiler_;
  int next_profiler_id_;
  int num_active_profilers_;
  HeapHashSet<WeakMember<Profiler>> profilers_;

  // Store the ids of leaked collected profilers that needs to be stopped
  Vector<String> detached_profiler_ids_;

  // A set of observers, one for each ExecutionContext that has profiling
  // enabled.
  HeapHashSet<Member<ProfilingContextObserver>> context_observers_;
};

class DiscardedSamplesDelegate : public v8::DiscardedSamplesDelegate {
 public:
  explicit DiscardedSamplesDelegate(ProfilerGroup* profiler_group,
                                    String profiler_id)
      : profiler_group_(profiler_group), profiler_id_(profiler_id) {}
  void Notify() override;

 private:
  // It is important to keep a weak reference to the profiler group
  // because Notify may be invoked after profiling stops and ProfilerGroup dies.
  WeakPersistent<ProfilerGroup> profiler_group_;
  const String profiler_id_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PROFILER_GROUP_H_

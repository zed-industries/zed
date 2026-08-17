// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TRACE_LOG_H_
#define BASE_TRACE_EVENT_TRACE_LOG_H_

#include <stddef.h>
#include <stdint.h>

#include <map>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "base/base_export.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/no_destructor.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/platform_thread.h"
#include "base/time/time_override.h"
#include "base/trace_event/builtin_categories.h"
#include "base/trace_event/trace_config.h"
#include "base/trace_event/trace_event_impl.h"
#include "build/build_config.h"
#include "third_party/perfetto/include/perfetto/tracing/core/trace_config.h"

namespace perfetto {
namespace trace_processor {
class TraceProcessorStorage;
}  // namespace trace_processor
}  // namespace perfetto

namespace base {
class RefCountedString;

namespace trace_event {

class JsonStringOutputWriter;

class BASE_EXPORT TraceLog : public perfetto::TrackEventSessionObserver {
 public:

  static TraceLog* GetInstance();

  TraceLog(const TraceLog&) = delete;
  TraceLog& operator=(const TraceLog&) = delete;

  // Retrieves a copy (for thread-safety) of the current TraceConfig.
  TraceConfig GetCurrentTraceConfig() const;

  // See TraceConfig comments for details on how to control which categories
  // will be traced.
  void SetEnabled(const TraceConfig& trace_config);

  // Enable tracing using a customized Perfetto trace config. This allows, for
  // example, enabling additional data sources and enabling protobuf output
  // instead of the legacy JSON trace format.
  void SetEnabled(const TraceConfig& trace_config,
                  const perfetto::TraceConfig& perfetto_config);

  // Disables tracing for all categories.
  void SetDisabled();

  // Returns true if TraceLog is enabled (i.e. there's an active tracing
  // session).
  bool IsEnabled() {
    // We don't rely on TrackEvent::IsEnabled() because it can be true before
    // TraceLog has processed its TrackEventSessionObserver callbacks.
    // For example, the code
    // if (TrackEvent::IsEnabled()) {
    //   auto config = TraceLog::GetCurrentTrackEventDataSourceConfig();
    //   ...
    // }
    // can fail in a situation when TrackEvent::IsEnabled() is already true, but
    // TraceLog::OnSetup() hasn't been called yet, so we don't know the config.
    // Instead, we make sure that both OnSetup() and OnStart() have been called
    // by tracking the number of active sessions that TraceLog has seen.
    AutoLock lock(track_event_lock_);
    return active_track_event_sessions_ > 0;
  }

  // Enabled state listeners give a callback when tracing is enabled or
  // disabled. This can be used to tie into other library's tracing systems
  // on-demand.
  class BASE_EXPORT EnabledStateObserver {
   public:
    virtual ~EnabledStateObserver() = default;

    // Called just after the tracing system becomes enabled, outside of the
    // |lock_|. TraceLog::IsEnabled() is true at this point.
    virtual void OnTraceLogEnabled() = 0;

    // Called just after the tracing system disables, outside of the |lock_|.
    // TraceLog::IsEnabled() is false at this point.
    virtual void OnTraceLogDisabled() = 0;
  };
  // Adds an observer. Cannot be called from within the observer callback.
  void AddEnabledStateObserver(EnabledStateObserver* listener);
  // Removes an observer. Cannot be called from within the observer callback.
  void RemoveEnabledStateObserver(EnabledStateObserver* listener);
  // Adds an observer that is owned by TraceLog. This is useful for agents that
  // implement tracing feature that needs to stay alive as long as TraceLog
  // does.
  void AddOwnedEnabledStateObserver(
      std::unique_ptr<EnabledStateObserver> listener);
  bool HasEnabledStateObserver(EnabledStateObserver* listener) const;

  // Asynchronous enabled state listeners. When tracing is enabled or disabled,
  // for each observer, a task for invoking its appropriate callback is posted
  // to the `SequencedTaskRunner` from which AddAsyncEnabledStateObserver() was
  // called. This allows the observer to be safely destroyed, provided that it
  // happens on the same `SequencedTaskRunner` that invoked
  // AddAsyncEnabledStateObserver().
  class BASE_EXPORT AsyncEnabledStateObserver {
   public:
    virtual ~AsyncEnabledStateObserver() = default;

    // Posted just after the tracing system becomes enabled, outside |lock_|.
    // TraceLog::IsEnabled() is true at this point.
    virtual void OnTraceLogEnabled() = 0;

    // Posted just after the tracing system becomes disabled, outside |lock_|.
    // TraceLog::IsEnabled() is false at this point.
    virtual void OnTraceLogDisabled() = 0;
  };
  // TODO(oysteine): This API originally needed to use WeakPtrs as the observer
  // list was copied under the global trace lock, but iterated over outside of
  // that lock so that observers could add tracing. The list is now protected by
  // its own lock, so this can be changed to a raw ptr.
  void AddAsyncEnabledStateObserver(
      WeakPtr<AsyncEnabledStateObserver> listener);
  void RemoveAsyncEnabledStateObserver(AsyncEnabledStateObserver* listener);
  bool HasAsyncEnabledStateObserver(AsyncEnabledStateObserver* listener) const;

  void SetArgumentFilterPredicate(
      const ArgumentFilterPredicate& argument_filter_predicate);
  ArgumentFilterPredicate GetArgumentFilterPredicate() const;

  void SetMetadataFilterPredicate(
      const MetadataFilterPredicate& metadata_filter_predicate);
  MetadataFilterPredicate GetMetadataFilterPredicate() const;

  // Flush all collected events to the given output callback. The callback will
  // be called one or more times either synchronously or asynchronously from
  // the current thread with IPC-bite-size chunks. The string format is
  // undefined. Use TraceResultBuffer to convert one or more trace strings to
  // JSON. The callback can be null if the caller doesn't want any data.
  // Due to the implementation of thread-local buffers, flush can't be
  // done when tracing is enabled. If called when tracing is enabled, the
  // callback will be called directly with (empty_string, false) to indicate
  // the end of this unsuccessful flush. Flush does the serialization
  // on the same thread if the caller doesn't set use_worker_thread explicitly.
  using OutputCallback =
      base::RepeatingCallback<void(const scoped_refptr<base::RefCountedString>&,
                                   bool has_more_events)>;
  void Flush(const OutputCallback& cb, bool use_worker_thread = false);

  // Cancels tracing and discards collected data.
  void CancelTracing(const OutputCallback& cb);

  // Called by TRACE_EVENT* macros, don't call this directly.
  // The name parameter is a category group for example:
  // TRACE_EVENT0("renderer,webkit", "WebViewImpl::HandleInputEvent")
  static const unsigned char* GetCategoryGroupEnabled(const char* name);
  static const char* GetCategoryGroupName(
      const unsigned char* category_group_enabled);

  ProcessId process_id() const { return process_id_; }

  // Exposed for unittesting:
  // Allows clearing up our singleton instance.
  static void ResetForTesting();

  void SetProcessID(ProcessId process_id);

  size_t GetObserverCountForTest() const;

  struct TrackEventSession {
    uint32_t internal_instance_index;
    perfetto::DataSourceConfig config;
    perfetto::BackendType backend_type = perfetto::kUnspecifiedBackend;
  };
  std::vector<TrackEventSession> GetTrackEventSessions() const;

  // DEPRECATED. In the presence of multiple simultaneous sessions, this method
  // returns only the first session's config. When no tracing sessions are
  // active, returns an empty config for compatibility with legacy code.
  // TODO(khokhlov): Remove this method and migrate all its uses to
  // GetTrackEventSessions().
  perfetto::DataSourceConfig GetCurrentTrackEventDataSourceConfig() const;
  void InitializePerfettoIfNeeded();
  bool IsPerfettoInitializedByTraceLog() const;
  void SetEnabledImpl(const TraceConfig& trace_config,
                      const perfetto::TraceConfig& perfetto_config);

  // perfetto::TrackEventSessionObserver implementation.
  void OnSetup(const perfetto::DataSourceBase::SetupArgs&) override;
  void OnStart(const perfetto::DataSourceBase::StartArgs&) override;
  void OnStop(const perfetto::DataSourceBase::StopArgs&) override;

 private:
  friend class base::NoDestructor<TraceLog>;

  struct RegisteredAsyncObserver;

  explicit TraceLog();
  ~TraceLog() override;

  void SetDisabledWhileLocked() EXCLUSIVE_LOCKS_REQUIRED(lock_);

  void FlushInternal(const OutputCallback& cb,
                     bool use_worker_thread,
                     bool discard_events);

  void OnTraceData(const char* data, size_t size, bool has_more);

  // This lock protects TraceLog member accesses (except for members protected
  // by thread_info_lock_) from arbitrary threads.
  mutable Lock lock_;

  // The lock protects observers access.
  mutable Lock observers_lock_;
  bool dispatching_to_observers_ = false;
  std::vector<raw_ptr<EnabledStateObserver, VectorExperimental>>
      enabled_state_observers_ GUARDED_BY(observers_lock_);
  std::map<AsyncEnabledStateObserver*, RegisteredAsyncObserver> async_observers_
      GUARDED_BY(observers_lock_);
  // Manages ownership of the owned observers. The owned observers will also be
  // added to |enabled_state_observers_|.
  std::vector<std::unique_ptr<EnabledStateObserver>>
      owned_enabled_state_observer_copy_ GUARDED_BY(observers_lock_);

  ProcessId process_id_;

  // Set when asynchronous Flush is in progress.
  ArgumentFilterPredicate argument_filter_predicate_;
  MetadataFilterPredicate metadata_filter_predicate_;

  std::unique_ptr<perfetto::TracingSession> tracing_session_;
  perfetto::TraceConfig perfetto_config_;
  std::vector<TrackEventSession> track_event_sessions_
      GUARDED_BY(track_event_lock_);
  int active_track_event_sessions_ = 0;
  mutable Lock track_event_lock_;
#if BUILDFLAG(USE_PERFETTO_TRACE_PROCESSOR)
  std::unique_ptr<perfetto::trace_processor::TraceProcessorStorage>
      trace_processor_;
  std::unique_ptr<JsonStringOutputWriter> json_output_writer_;
  OutputCallback proto_output_callback_;
#endif  // BUILDFLAG(USE_PERFETTO_TRACE_PROCESSOR)
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_TRACE_LOG_H_

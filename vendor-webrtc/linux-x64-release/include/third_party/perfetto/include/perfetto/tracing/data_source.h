/*
 * Copyright (C) 2019 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_TRACING_DATA_SOURCE_H_
#define INCLUDE_PERFETTO_TRACING_DATA_SOURCE_H_

// This header contains the key class (DataSource) that a producer app should
// override in order to create a custom data source that gets tracing Start/Stop
// notifications and emits tracing data.

#include <assert.h>
#include <stddef.h>
#include <stdint.h>

#include <array>
#include <atomic>
#include <functional>
#include <memory>
#include <mutex>

#include "perfetto/protozero/message_handle.h"
#include "perfetto/tracing/buffer_exhausted_policy.h"
#include "perfetto/tracing/core/flush_flags.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "perfetto/tracing/internal/basic_types.h"
#include "perfetto/tracing/internal/data_source_internal.h"
#include "perfetto/tracing/internal/data_source_type.h"
#include "perfetto/tracing/internal/tracing_muxer.h"
#include "perfetto/tracing/locked_handle.h"
#include "perfetto/tracing/trace_writer_base.h"

#include "protos/perfetto/trace/trace_packet.pbzero.h"

// DEPRECATED: Instead of using this macro, prefer specifying symbol linkage
// attributes explicitly using the `_WITH_ATTRS` macro variants (e.g.,
// PERFETTO_DECLARE_DATA_SOURCE_STATIC_MEMBERS_WITH_ATTRS). This avoids
// potential macro definition collisions between two libraries using Perfetto.
//
// PERFETTO_COMPONENT_EXPORT is used to mark symbols in Perfetto's headers
// (typically templates) that are defined by the user outside of Perfetto and
// should be made visible outside the current module. (e.g., in Chrome's
// component build).
#if !defined(PERFETTO_COMPONENT_EXPORT)
#if PERFETTO_BUILDFLAG(PERFETTO_COMPILER_MSVC)
// Workaround for C4003: not enough arguments for function-like macro invocation
// 'PERFETTO_INTERNAL_DECLARE_TRACK_EVENT_DATA_SOURCE'
#define PERFETTO_COMPONENT_EXPORT __declspec()
#else
#define PERFETTO_COMPONENT_EXPORT
#endif
#endif

namespace perfetto {
namespace internal {
class TracingMuxerImpl;
class TrackEventCategoryRegistry;
template <typename, const internal::TrackEventCategoryRegistry*>
class TrackEventDataSource;
}  // namespace internal

namespace shlib {
class TrackEvent;
}  // namespace shlib

namespace test {
class DataSourceInternalForTest;
}  // namespace test

// Base class with the virtual methods to get start/stop notifications.
// Embedders are supposed to derive the templated version below, not this one.
class PERFETTO_EXPORT_COMPONENT DataSourceBase {
 public:
  virtual ~DataSourceBase();

  // TODO(primiano): change the const& args below to be pointers instead. It
  // makes it more awkward to handle output arguments and require mutable(s).
  // This requires synchronizing a breaking API change for existing embedders.

  // OnSetup() is invoked when tracing is configured. In most cases this happens
  // just before starting the trace. In the case of deferred start (see
  // deferred_start in trace_config.proto) start might happen later.
  //
  // Can be called from any thread.
  class SetupArgs {
   public:
    // This is valid only within the scope of the OnSetup() call and must not
    // be retained.
    const DataSourceConfig* config = nullptr;

    // Backend type.
    BackendType backend_type = kUnspecifiedBackend;

    // The index of this data source instance (0..kMaxDataSourceInstances - 1).
    uint32_t internal_instance_index = 0;
  };
  virtual void OnSetup(const SetupArgs&);

  class StartArgs {
   public:
    // The index of this data source instance (0..kMaxDataSourceInstances - 1).
    uint32_t internal_instance_index = 0;
  };
  // Invoked after tracing is actually started.
  //
  // Can be called from any thread.
  virtual void OnStart(const StartArgs&);

  class PERFETTO_EXPORT_COMPONENT StopArgs {
   public:
    virtual ~StopArgs();

    // HandleAsynchronously() can optionally be called to defer the tracing
    // session stop and write tracing data just before stopping.
    // This function returns a closure that must be invoked after the last
    // trace events have been emitted. The returned closure can be called from
    // any thread. The caller also needs to explicitly call TraceContext.Flush()
    // from the last Trace() lambda invocation because no other implicit flushes
    // will happen after the stop signal.
    // When this function is called, the tracing service will defer the stop of
    // the tracing session until the returned closure is invoked.
    // However, the caller cannot hang onto this closure for too long. The
    // tracing service will forcefully stop the tracing session without waiting
    // for pending producers after TraceConfig.data_source_stop_timeout_ms
    // (default: 5s, can be overridden by Consumers when starting a trace).
    // If the closure is called after this timeout an error will be logged and
    // the trace data emitted will not be present in the trace. No other
    // functional side effects (e.g. crashes or corruptions) will happen. In
    // other words, it is fine to accidentally hold onto this closure for too
    // long but, if that happens, some tracing data will be lost.
    virtual std::function<void()> HandleStopAsynchronously() const = 0;

    // The index of this data source instance (0..kMaxDataSourceInstances - 1).
    uint32_t internal_instance_index = 0;
  };
  // Invoked before tracing is stopped.
  //
  // Can be called from any thread. Blocking this for too long it's not a good
  // idea and can cause deadlocks. Use HandleAsynchronously() to postpone
  // disabling the data source instance.
  virtual void OnStop(const StopArgs&);

  class ClearIncrementalStateArgs {
   public:
    // The index of this data source instance (0..kMaxDataSourceInstances - 1).
    uint32_t internal_instance_index = 0;
  };
  // Invoked before marking the thread local per-instance incremental state
  // outdated.
  //
  // Can be called from any thread.
  virtual void WillClearIncrementalState(const ClearIncrementalStateArgs&);

  class FlushArgs {
   public:
    virtual ~FlushArgs();

    // HandleFlushAsynchronously() can be called to postpone acknowledging the
    // flush request. This function returns a closure that must be invoked after
    // the flush request has been processed. The returned closure can be called
    // from any thread.
    virtual std::function<void()> HandleFlushAsynchronously() const = 0;

    // The index of this data source instance (0..kMaxDataSourceInstances - 1).
    uint32_t internal_instance_index = 0;

    // The reason and initiator of the flush. See flush_flags.h .
    FlushFlags flush_flags;
  };
  // Called when the tracing service requests a Flush. Users can override this
  // to tell other threads to flush their TraceContext for this data source
  // (the library cannot execute code on all the threads on its own).
  //
  // Can be called from any thread. Blocking this for too long it's not a good
  // idea and can cause deadlocks. Use HandleAsynchronously() to postpone
  // sending the flush acknowledgement to the service.
  virtual void OnFlush(const FlushArgs&);

  // Determines whether a startup session can be adopted by a service-initiated
  // tracing session (i.e. whether their configs are compatible).
  virtual bool CanAdoptStartupSession(const DataSourceConfig& startup_config,
                                      const DataSourceConfig& service_config);
};

struct DefaultDataSourceTraits {
  // |IncrementalStateType| can optionally be used store custom per-sequence
  // incremental data (e.g., interning tables).
  using IncrementalStateType = void;
  // |TlsStateType| can optionally be used to store custom per-sequence
  // session data, which is not reset when incremental state is cleared
  // (e.g. configuration options).
  using TlsStateType = void;

  // Allows overriding what type of thread-local state configuration the data
  // source uses. By default every data source gets independent thread-local
  // state, which means every instance uses separate trace writers and
  // incremental state even on the same thread. Some data sources (most notably
  // the track event data source) want to share trace writers and incremental
  // state on the same thread.
  static internal::DataSourceThreadLocalState* GetDataSourceTLS(
      internal::DataSourceStaticState* static_state,
      internal::TracingTLS* root_tls) {
    auto* ds_tls = &root_tls->data_sources_tls[static_state->index];
    // ds_tls->static_state can be:
    // * nullptr
    // * equal to static_state
    // * equal to the static state of a different data source, in tests (when
    //   ResetForTesting() has been used)
    // In any case, there's no need to do anything, the caller will reinitialize
    // static_state.
    return ds_tls;
  }
};

// Holds the type for a DataSource. Accessed by the static Trace() method
// fastpaths. This allows redefinitions under a component where a component
// specific export macro is used.
// Due to C2086 (redefinition) error on MSVC/clang-cl, internal::DataSourceType
// can't be a static data member. To avoid explicit specialization after
// instantiation error, type() needs to be in a template helper class that's
// instantiated independently from DataSource. See b/280777748.
template <typename DerivedDataSource,
          typename DataSourceTraits = DefaultDataSourceTraits>
struct DataSourceHelper {
  static internal::DataSourceType& type() {
    static perfetto::internal::DataSourceType type_;
    return type_;
  }
};

// Templated base class meant to be derived by embedders to create a custom data
// source. DerivedDataSource must be the type of the derived class itself, e.g.:
// class MyDataSource : public DataSource<MyDataSource> {...}.
//
// |DataSourceTraits| allows customizing the behavior of the data source. See
// |DefaultDataSourceTraits|.
template <typename DerivedDataSource,
          typename DataSourceTraits = DefaultDataSourceTraits>
class DataSource : public DataSourceBase {
  struct DefaultTracePointTraits;
  using Helper = DataSourceHelper<DerivedDataSource, DataSourceTraits>;

 public:
  // The BufferExhaustedPolicy to use for TraceWriters of this DataSource.
  // Override this in your DataSource class to change the default, which is to
  // drop data on shared memory overruns.
  constexpr static BufferExhaustedPolicy kBufferExhaustedPolicy =
      BufferExhaustedPolicy::kDrop;

  // When this flag is false, we cannot have multiple instances of this data
  // source. When a data source is already active and if we attempt
  // to start another instance of that data source (via another tracing
  // session), it will fail to start the second instance of data source.
  static constexpr bool kSupportsMultipleInstances = true;

  // When this flag is true, DataSource callbacks (OnSetup, OnStart, etc.) are
  // called under the lock (the same that is used in GetDataSourceLocked
  // function). This is not recommended because it can lead to deadlocks, but
  // it was the default behavior for a long time and some embedders rely on it
  // to protect concurrent access to the DataSource members. So we keep the
  // "true" value as the default.
  static constexpr bool kRequiresCallbacksUnderLock = true;

  // Argument passed to the lambda function passed to Trace() (below).
  class TraceContext {
   public:
    using TracePacketHandle =
        ::protozero::MessageHandle<::perfetto::protos::pbzero::TracePacket>;

    TraceContext(TraceContext&&) noexcept = default;
    ~TraceContext() {
      // If the data source is being intercepted, flush the trace writer after
      // each trace point to make sure the interceptor sees the data right away.
      if (PERFETTO_UNLIKELY(tls_inst_->is_intercepted))
        Flush();
    }

    // Adds an empty trace packet to the trace to ensure that the service can
    // safely read the last event from the trace buffer.
    // See PERFETTO_INTERNAL_ADD_EMPTY_EVENT macros for context.
    void AddEmptyTracePacket() {
      // If nothing was written since the last empty packet, there's nothing to
      // scrape, so adding more empty packets serves no purpose.
      if (tls_inst_->trace_writer->written() ==
          tls_inst_->last_empty_packet_position) {
        return;
      }
      tls_inst_->trace_writer->NewTracePacket();
      tls_inst_->last_empty_packet_position =
          tls_inst_->trace_writer->written();
    }

    TracePacketHandle NewTracePacket() {
      return tls_inst_->trace_writer->NewTracePacket();
    }

    // Forces a commit of the thread-local tracing data written so far to the
    // service. This is almost never required (tracing data is periodically
    // committed as trace pages are filled up) and has a non-negligible
    // performance hit (requires an IPC + refresh of the current thread-local
    // chunk). The only case when this should be used is when handling OnStop()
    // asynchronously, to ensure sure that the data is committed before the
    // Stop timeout expires.
    // The TracePacketHandle obtained by the last NewTracePacket() call must be
    // finalized before calling Flush() (either implicitly by going out of scope
    // or by explicitly calling Finalize()).
    // |cb| is an optional callback. When non-null it will request the
    // service to ACK the flush and will be invoked on an internal thread after
    // the service has  acknowledged it. The callback might be NEVER INVOKED if
    // the service crashes or the IPC connection is dropped.
    void Flush(std::function<void()> cb = {}) {
      tls_inst_->trace_writer->Flush(cb);
    }

    // Returns the number of bytes written on the current thread by the current
    // data-source since its creation.
    // This can be useful for splitting protos that might grow very large.
    uint64_t written() { return tls_inst_->trace_writer->written(); }

    // Returns a RAII handle to access the data source instance, guaranteeing
    // that it won't be deleted on another thread (because of trace stopping)
    // while accessing it from within the Trace() lambda.
    // The returned handle can be invalid (nullptr) if tracing is stopped
    // immediately before calling this. The caller is supposed to check for its
    // validity before using it. After checking, the handle is guaranteed to
    // remain valid until the handle goes out of scope.
    LockedHandle<DerivedDataSource> GetDataSourceLocked() const {
      auto* internal_state =
          Helper::type().static_state()->TryGet(instance_index_);
      if (!internal_state)
        return LockedHandle<DerivedDataSource>();
      std::unique_lock<std::recursive_mutex> lock(internal_state->lock);
      return LockedHandle<DerivedDataSource>(
          std::move(lock),
          static_cast<DerivedDataSource*>(internal_state->data_source.get()));
    }

    // Post-condition: returned ptr will be non-null.
    typename DataSourceTraits::TlsStateType* GetCustomTlsState() {
      PERFETTO_DCHECK(tls_inst_->data_source_custom_tls);
      return reinterpret_cast<typename DataSourceTraits::TlsStateType*>(
          tls_inst_->data_source_custom_tls.get());
    }

    typename DataSourceTraits::IncrementalStateType* GetIncrementalState() {
      return static_cast<typename DataSourceTraits::IncrementalStateType*>(
          Helper::type().GetIncrementalState(tls_inst_, instance_index_));
    }

   private:
    friend class DataSource;
    template <typename, const internal::TrackEventCategoryRegistry*>
    friend class internal::TrackEventDataSource;
    TraceContext(internal::DataSourceInstanceThreadLocalState* tls_inst,
                 uint32_t instance_index)
        : tls_inst_(tls_inst), instance_index_(instance_index) {}
    TraceContext(const TraceContext&) = delete;
    TraceContext& operator=(const TraceContext&) = delete;

    internal::DataSourceInstanceThreadLocalState* const tls_inst_;
    uint32_t const instance_index_;
  };

  // The main tracing method. Tracing code should call this passing a lambda as
  // argument, with the following signature: void(TraceContext).
  // The lambda will be called synchronously (i.e., always before Trace()
  // returns) only if tracing is enabled and the data source has been enabled in
  // the tracing config.
  // The lambda can be called more than once per Trace() call, in the case of
  // concurrent tracing sessions (or even if the data source is instantiated
  // twice within the same trace config).
  template <typename Lambda>
  static void Trace(Lambda tracing_fn) {
    CallIfEnabled<DefaultTracePointTraits>([&tracing_fn](uint32_t instances) {
      TraceWithInstances<DefaultTracePointTraits>(instances,
                                                  std::move(tracing_fn));
    });
  }

  // An efficient trace point guard for checking if this data source is active.
  // |callback| is a function which will only be called if there are active
  // instances. It is given an instance state parameter, which should be passed
  // to TraceWithInstances() to actually record trace data.
  template <typename Traits = DefaultTracePointTraits, typename Callback>
  static void CallIfEnabled(Callback callback,
                            typename Traits::TracePointData trace_point_data =
                                {}) PERFETTO_ALWAYS_INLINE {
    // |instances| is a per-class bitmap that tells:
    // 1. If the data source is enabled at all.
    // 2. The index of the slot within
    //    internal::DataSourceStaticState::instances that holds the instance
    //    state. In turn this allows to map the data source to the tracing
    //    session and buffers.
    // memory_order_relaxed is okay because:
    // - |instances| is re-read with an acquire barrier below if this succeeds.
    // - The code between this point and the acquire-load is based on static
    //    storage which has indefinite lifetime.
    uint32_t instances = Traits::GetActiveInstances(trace_point_data)
                             ->load(std::memory_order_relaxed);

    // This is the tracing fast-path. Bail out immediately if tracing is not
    // enabled (or tracing is enabled but not for this data source).
    if (PERFETTO_LIKELY(!instances))
      return;
    callback(instances);
  }

  // The "lower half" of a trace point which actually performs tracing after
  // this data source has been determined to be active.
  // |instances| must be the instance state value retrieved through
  // CallIfEnabled().
  // |tracing_fn| will be called to record trace data as in Trace().
  //
  // |trace_point_data| is an optional parameter given to |Traits::
  // GetActiveInstances| to make it possible to use custom storage for
  // the data source enabled state. This is, for example, used by TrackEvent to
  // implement per-tracing category enabled states.
  template <typename Traits = DefaultTracePointTraits, typename Lambda>
  static void TraceWithInstances(
      uint32_t cached_instances,
      Lambda tracing_fn,
      typename Traits::TracePointData trace_point_data = {}) {
    PERFETTO_DCHECK(cached_instances);

    if (!Helper::type().template TracePrologue<DataSourceTraits, Traits>(
            &tls_state_, &cached_instances, trace_point_data)) {
      return;
    }

    for (internal::DataSourceType::InstancesIterator it =
             Helper::type().template BeginIteration<Traits>(
                 cached_instances, tls_state_, trace_point_data);
         it.instance; Helper::type().template NextIteration<Traits>(
             &it, tls_state_, trace_point_data)) {
      tracing_fn(TraceContext(it.instance, it.i));
    }

    Helper::type().TraceEpilogue(tls_state_);
  }

  // Registers the data source on all tracing backends, including ones that
  // connect after the registration. Doing so enables the data source to receive
  // Setup/Start/Stop notifications and makes the Trace() method work when
  // tracing is enabled and the data source is selected.
  // This must be called after Tracing::Initialize().
  // Can return false to signal failure if attemping to register more than
  // kMaxDataSources (32) data sources types or if tracing hasn't been
  // initialized.
  // The optional |constructor_args| will be passed to the data source when it
  // is constructed.
  template <class... Args>
  static bool Register(const DataSourceDescriptor& descriptor,
                       const Args&... constructor_args) {
    // Silences -Wunused-variable warning in case the trace method is not used
    // by the translation unit that declares the data source.
    (void)tls_state_;

    auto factory = [constructor_args...]() {
      return std::unique_ptr<DataSourceBase>(
          new DerivedDataSource(constructor_args...));
    };
    constexpr bool no_flush =
        std::is_same_v<decltype(&DerivedDataSource::OnFlush),
                       decltype(&DataSourceBase::OnFlush)>;
    internal::DataSourceParams params{
        DerivedDataSource::kSupportsMultipleInstances,
        DerivedDataSource::kRequiresCallbacksUnderLock};
    return Helper::type().Register(
        descriptor, factory, params, DerivedDataSource::kBufferExhaustedPolicy,
        no_flush,
        GetCreateTlsFn(
            static_cast<typename DataSourceTraits::TlsStateType*>(nullptr)),
        GetCreateIncrementalStateFn(
            static_cast<typename DataSourceTraits::IncrementalStateType*>(
                nullptr)),
        nullptr);
  }

  // Updates the data source descriptor.
  static void UpdateDescriptor(const DataSourceDescriptor& descriptor) {
    Helper::type().UpdateDescriptor(descriptor);
  }

 private:
  friend ::perfetto::test::DataSourceInternalForTest;
  friend ::perfetto::shlib::TrackEvent;
  // Traits for customizing the behavior of a specific trace point.
  struct DefaultTracePointTraits {
    // By default, every call to DataSource::Trace() will record trace events
    // for every active instance of that data source. A single trace point can,
    // however, use a custom set of enable flags for more fine grained control
    // of when that trace point is active.
    //
    // DANGER: when doing this, the data source must use the appropriate memory
    // fences when changing the state of the bitmap.
    //
    // |TraceWithInstances| may be optionally given an additional parameter for
    // looking up the enable flags. That parameter is passed as |TracePointData|
    // to |GetActiveInstances|. This is, for example, used by TrackEvent to
    // implement per-category enabled states.
    struct TracePointData {};
    static constexpr std::atomic<uint32_t>* GetActiveInstances(TracePointData) {
      return Helper::type().valid_instances();
    }
  };

  template <typename T>
  static internal::DataSourceInstanceThreadLocalState::ObjectWithDeleter
  CreateIncrementalState(internal::DataSourceInstanceThreadLocalState*,
                         uint32_t,
                         void*) {
    return internal::DataSourceInstanceThreadLocalState::ObjectWithDeleter(
        reinterpret_cast<void*>(new T()),
        [](void* p) { delete reinterpret_cast<T*>(p); });
  }

  // The second parameter here is used to specialize the case where there is no
  // incremental state type.
  template <typename T>
  static internal::DataSourceType::CreateIncrementalStateFn
  GetCreateIncrementalStateFn(const T*) {
    return &CreateIncrementalState<T>;
  }

  static internal::DataSourceType::CreateIncrementalStateFn
  GetCreateIncrementalStateFn(const void*) {
    return nullptr;
  }

  template <typename T>
  static internal::DataSourceInstanceThreadLocalState::ObjectWithDeleter
  CreateDataSourceCustomTls(
      internal::DataSourceInstanceThreadLocalState* tls_inst,
      uint32_t instance_index,
      void*) {
    return internal::DataSourceInstanceThreadLocalState::ObjectWithDeleter(
        reinterpret_cast<void*>(new T(TraceContext(tls_inst, instance_index))),
        [](void* p) { delete reinterpret_cast<T*>(p); });
  }

  // The second parameter here is used to specialize the case where there is no
  // tls state type.
  template <typename T>
  static internal::DataSourceType::CreateCustomTlsFn GetCreateTlsFn(const T*) {
    return &CreateDataSourceCustomTls<T>;
  }

  static internal::DataSourceType::CreateCustomTlsFn GetCreateTlsFn(
      const void*) {
    return nullptr;
  }

  // This TLS object is a cached raw pointer and has deliberately no destructor.
  // The Platform implementation is supposed to create and manage the lifetime
  // of the Platform::ThreadLocalObject and take care of destroying it.
  // This is because non-POD thread_local variables have subtleties (global
  // destructors) that we need to defer to the embedder. In chromium's platform
  // implementation, for instance, the tls slot is implemented using
  // chromium's base::ThreadLocalStorage.
  static thread_local internal::DataSourceThreadLocalState* tls_state_;
};

// static
template <typename T, typename D>
thread_local internal::DataSourceThreadLocalState* DataSource<T, D>::tls_state_;

}  // namespace perfetto

// If placed at the end of a macro declaration, eats the semicolon at the end of
// the macro invocation (e.g., "MACRO(...);") to avoid warnings about extra
// semicolons.
#define PERFETTO_INTERNAL_SWALLOW_SEMICOLON() \
  [[maybe_unused]] extern int perfetto_internal_unused

// This macro must be used once for each data source next to the data source's
// declaration.
#define PERFETTO_DECLARE_DATA_SOURCE_STATIC_MEMBERS(...)  \
  PERFETTO_DECLARE_DATA_SOURCE_STATIC_MEMBERS_WITH_ATTRS( \
      PERFETTO_COMPONENT_EXPORT, __VA_ARGS__)

// Similar to `PERFETTO_DECLARE_DATA_SOURCE_STATIC_MEMBERS` but it also takes
// custom attributes, which are useful when DataSource is defined in a component
// where a component specific export macro is used.
#define PERFETTO_DECLARE_DATA_SOURCE_STATIC_MEMBERS_WITH_ATTRS(attrs, ...) \
  template <>                                                              \
  attrs perfetto::internal::DataSourceType&                                \
  perfetto::DataSourceHelper<__VA_ARGS__>::type()

// This macro must be used once for each data source in one source file to
// allocate static storage for the data source's static state.
#define PERFETTO_DEFINE_DATA_SOURCE_STATIC_MEMBERS(...)  \
  PERFETTO_DEFINE_DATA_SOURCE_STATIC_MEMBERS_WITH_ATTRS( \
      PERFETTO_COMPONENT_EXPORT, __VA_ARGS__)

// Similar to `PERFETTO_DEFINE_DATA_SOURCE_STATIC_MEMBERS` but it also takes
// custom attributes, which are useful when DataSource is defined in a component
// where a component specific export macro is used.
#define PERFETTO_DEFINE_DATA_SOURCE_STATIC_MEMBERS_WITH_ATTRS(attrs, ...) \
  template <>                                                             \
  perfetto::internal::DataSourceType&                                     \
  perfetto::DataSourceHelper<__VA_ARGS__>::type() {                       \
    static perfetto::internal::DataSourceType type_;                      \
    return type_;                                                         \
  }                                                                       \
  PERFETTO_INTERNAL_SWALLOW_SEMICOLON()

#endif  // INCLUDE_PERFETTO_TRACING_DATA_SOURCE_H_

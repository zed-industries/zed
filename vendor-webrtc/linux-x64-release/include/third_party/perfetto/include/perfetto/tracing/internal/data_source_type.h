#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_DATA_SOURCE_TYPE_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_DATA_SOURCE_TYPE_H_

#include "perfetto/base/build_config.h"
#include "perfetto/base/export.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "perfetto/tracing/internal/data_source_internal.h"
#include "perfetto/tracing/internal/tracing_muxer.h"

namespace perfetto {
namespace internal {

// Represents a data source type (not an instance).
//
// All the static state of a DataSource<T> lives here (including
// DataSourceStaticState).
//
// The C shared library API wrapper cannot use DataSource<T>, because it needs
// to create new data source types at runtime, so it uses this directly.
//
// The main reason why this intermediate class exist is to decouple the
// DataSourceStaticState from the specific DataSource<T>. The C API cannot
// dynamically create template instances and it needs a way to decouple those at
// runtime.
class PERFETTO_EXPORT_COMPONENT DataSourceType {
 public:
  // Function pointer type used to create custom per instance thread local
  // state.
  using CreateCustomTlsFn =
      DataSourceInstanceThreadLocalState::ObjectWithDeleter (*)(
          DataSourceInstanceThreadLocalState* tls_inst,
          uint32_t instance_index,
          void* user_arg);
  // Function pointer type used to create custom per instance thread local
  // incremental state (which might be cleared periodically by the tracing
  // service).
  using CreateIncrementalStateFn =
      DataSourceInstanceThreadLocalState::ObjectWithDeleter (*)(
          DataSourceInstanceThreadLocalState* tls_inst,
          uint32_t instance_index,
          void* user_arg);

  // Registers the data source type with the central tracing muxer.
  // * `descriptor` is the data source protobuf descriptor.
  // * `factory` is a std::function used to create instances of the data source
  //   type.
  // * `buffer_exhausted_policy` specifies what to do when the shared memory
  //   buffer runs out of chunks.
  // * `create_custom_tls_fn` and `create_incremental_state_fn` are function
  //   pointers called to create custom state. They will receive `user_arg` as
  //   an extra param.
  bool Register(const DataSourceDescriptor& descriptor,
                TracingMuxer::DataSourceFactory factory,
                internal::DataSourceParams params,
                BufferExhaustedPolicy buffer_exhausted_policy,
                bool no_flush,
                CreateCustomTlsFn create_custom_tls_fn,
                CreateIncrementalStateFn create_incremental_state_fn,
                void* user_arg) {
    buffer_exhausted_policy_ = buffer_exhausted_policy;
    create_custom_tls_fn_ = create_custom_tls_fn;
    create_incremental_state_fn_ = create_incremental_state_fn;
    user_arg_ = user_arg;
    auto* tracing_impl = TracingMuxer::Get();
    return tracing_impl->RegisterDataSource(descriptor, factory, params,
                                            no_flush, &state_);
  }

  // Updates the data source type descriptor.
  void UpdateDescriptor(const DataSourceDescriptor& descriptor) {
    auto* tracing_impl = TracingMuxer::Get();
    tracing_impl->UpdateDataSourceDescriptor(descriptor, &state_);
  }

  // The beginning of a trace point.
  //
  // `tls_state` must point to a thread local variable that caches a pointer to
  // an internal per data source type thread local state.
  //
  // `instances` must point to a copy of the current active instances for the
  // data source type.
  //
  // `DataSourceTraits` can be used to customize the thread local storage used
  // for the data source type.
  //
  // `TracePointTraits` and `trace_point_data` are customization point for
  // getting the active instances bitmap.
  //
  // If this returns false, the trace point must be skipped.
  template <typename DataSourceTraits, typename TracePointTraits>
  bool TracePrologue(
      DataSourceThreadLocalState** tls_state,
      uint32_t* instances,
      typename TracePointTraits::TracePointData trace_point_data) {
    // See tracing_muxer.h for the structure of the TLS.
    if (PERFETTO_UNLIKELY(!*tls_state)) {
      *tls_state = GetOrCreateDataSourceTLS<DataSourceTraits>();
      // If the TLS hasn't been obtained yet, it's possible that this thread
      // hasn't observed the initialization of global state like the muxer yet.
      // To ensure that the thread "sees" the effects of such initialization,
      // we have to reload |instances| with an acquire fence, ensuring that any
      // initialization performed before instances was updated is visible
      // in this thread.
      *instances &= TracePointTraits::GetActiveInstances(trace_point_data)
                        ->load(std::memory_order_acquire);
      if (!*instances)
        return false;
    }
    auto* tracing_impl = TracingMuxer::Get();

    // Avoid re-entering the trace point recursively.
    if (PERFETTO_UNLIKELY((*tls_state)->root_tls->is_in_trace_point))
      return false;

    (*tls_state)->root_tls->is_in_trace_point = true;

    // TracingTLS::generation is a global monotonic counter that is incremented
    // every time a tracing session is stopped. We use that as a signal to force
    // a slow-path garbage collection of all the trace writers for the current
    // thread and to destroy the ones that belong to tracing sessions that have
    // ended. This is to avoid having too many TraceWriter instances alive, each
    // holding onto one chunk of the shared memory buffer.
    // Rationale why memory_order_relaxed should be fine:
    // - The TraceWriter object that we use is always constructed and destructed
    //   on the current thread. There is no risk of accessing a half-initialized
    //   TraceWriter (which would be really bad).
    // - In the worst case, in the case of a race on the generation check, we
    //   might end up using a TraceWriter for the same data source that belongs
    //   to a stopped session. This is not really wrong, as we don't give any
    //   guarantee on the global atomicity of the stop. In the worst case the
    //   service will reject the data commit if this arrives too late.

    if (PERFETTO_UNLIKELY(
            (*tls_state)->root_tls->generation !=
            tracing_impl->generation(std::memory_order_relaxed))) {
      // Will update root_tls->generation.
      tracing_impl->DestroyStoppedTraceWritersForCurrentThread();
    }

    return true;
  }

  // Must be called at the ending of a trace point that was not skipped.
  void TraceEpilogue(DataSourceThreadLocalState* tls_state) {
    tls_state->root_tls->is_in_trace_point = false;
  }

  struct InstancesIterator {
    // A bitmap of the currenly active instances.
    uint32_t cached_instances;
    // The current instance index.
    uint32_t i;
    // The current instance. If this is `nullptr`, the iteration is over.
    DataSourceInstanceThreadLocalState* instance;
  };

  // Returns an iterator to the active instances of this data source type.
  //
  // `cached_instances` is a copy of the bitmap of the active instances for this
  // data source type (usually just a copy of ValidInstances(), but can be
  // customized).
  //
  // `tls_state` is the thread local pointer obtained from TracePrologue.
  //
  // `TracePointTraits` and `trace_point_data` are customization point for
  // getting the active instances bitmap.
  template <typename TracePointTraits>
  InstancesIterator BeginIteration(
      uint32_t cached_instances,
      DataSourceThreadLocalState* tls_state,
      typename TracePointTraits::TracePointData trace_point_data) {
    InstancesIterator it{};
    it.cached_instances = cached_instances;
    FirstActiveInstance<TracePointTraits>(&it, tls_state, trace_point_data);
    return it;
  }

  // Advances `*iterator` to point to the next active instance of this data
  // source type.
  //
  // `tls_state` is the thread local pointer obtained from TracePrologue.
  //
  // `TracePointTraits` and `trace_point_data` are customization point for
  // getting the active instances bitmap.
  template <typename TracePointTraits>
  void NextIteration(
      InstancesIterator* iterator,
      DataSourceThreadLocalState* tls_state,
      typename TracePointTraits::TracePointData trace_point_data) {
    iterator->i++;
    FirstActiveInstance<TracePointTraits>(iterator, tls_state,
                                          trace_point_data);
  }

  void* GetIncrementalState(
      internal::DataSourceInstanceThreadLocalState* tls_inst,
      uint32_t instance_index) {
    // Recreate incremental state data if it has been reset by the service.
    if (tls_inst->incremental_state_generation !=
        static_state()
            ->GetUnsafe(instance_index)
            ->incremental_state_generation.load(std::memory_order_relaxed)) {
      tls_inst->incremental_state.reset();
      CreateIncrementalState(tls_inst, instance_index);
    }
    return tls_inst->incremental_state.get();
  }

  std::atomic<uint32_t>* valid_instances() { return &state_.valid_instances; }

  DataSourceStaticState* static_state() { return &state_; }

 private:
  void CreateIncrementalState(
      internal::DataSourceInstanceThreadLocalState* tls_inst,
      uint32_t instance_index) {
    PERFETTO_DCHECK(create_incremental_state_fn_ != nullptr);
    tls_inst->incremental_state =
        create_incremental_state_fn_(tls_inst, instance_index, user_arg_);
    tls_inst->incremental_state_generation =
        static_state()
            ->GetUnsafe(instance_index)
            ->incremental_state_generation.load(std::memory_order_relaxed);
  }

  void PopulateTlsInst(DataSourceInstanceThreadLocalState* tls_inst,
                       DataSourceState* instance_state,
                       uint32_t instance_index);

  // Advances `*iterator` to the first active instance whose index is greater or
  // equal than `iterator->i`.
  template <typename TracePointTraits>
  void FirstActiveInstance(
      InstancesIterator* iterator,
      DataSourceThreadLocalState* tls_state,
      typename TracePointTraits::TracePointData trace_point_data) {
    iterator->instance = nullptr;
    for (; iterator->i < kMaxDataSourceInstances; iterator->i++) {
      DataSourceState* instance_state =
          state_.TryGetCached(iterator->cached_instances, iterator->i);
      if (!instance_state)
        continue;
      // Even if we passed the check above, the DataSourceInstance might be
      // still destroyed concurrently while this code runs. The code below is
      // designed to deal with such race, as follows:
      // - We don't access the user-defined data source instance state. The only
      //   bits of state we use are |backend_id| and |buffer_id|.
      // - Beyond those two integers, we access only the TraceWriter here. The
      //   TraceWriter is always safe because it lives on the TLS.
      // - |instance_state| is backed by static storage, so the pointer is
      //   always valid, even after the data source instance is destroyed.
      // - In the case of a race-on-destruction, we'll still see the latest
      //   backend_id and buffer_id and in the worst case keep trying writing
      //   into the tracing shared memory buffer after stopped. But this isn't
      //   really any worse than the case of the stop IPC being delayed by the
      //   kernel scheduler. The tracing service is robust against data commit
      //   attemps made after tracing is stopped.
      // There is a theoretical race that would case the wrong behavior w.r.t
      // writing data in the wrong buffer, but it's so rare that we ignore it:
      // if the data source is stopped and started kMaxDataSourceInstances
      // times (so that the same id is recycled) while we are in this function,
      // we might end up reusing the old data source's backend_id and buffer_id
      // for the new one, because we don't see the generation change past this
      // point. But stopping and starting tracing (even once) takes so much
      // handshaking to make this extremely unrealistic.

      auto& tls_inst = tls_state->per_instance[iterator->i];
      if (PERFETTO_UNLIKELY(!tls_inst.trace_writer)) {
        // Here we need an acquire barrier, which matches the release-store made
        // by TracingMuxerImpl::SetupDataSource(), to ensure that the backend_id
        // and buffer_id are consistent.
        iterator->cached_instances &=
            TracePointTraits::GetActiveInstances(trace_point_data)
                ->load(std::memory_order_acquire);
        instance_state =
            state_.TryGetCached(iterator->cached_instances, iterator->i);
        if (!instance_state || !instance_state->trace_lambda_enabled.load(
                                   std::memory_order_relaxed))
          continue;
        PopulateTlsInst(&tls_inst, instance_state, iterator->i);
      }
      iterator->instance = &tls_inst;
      break;
    }
  }

  // Note that the returned object is one per-thread per-data-source-type, NOT
  // per data-source *instance*.
  template <typename DataSourceTraits>
  DataSourceThreadLocalState* GetOrCreateDataSourceTLS() {
    auto* tracing_impl = TracingMuxer::Get();
    TracingTLS* root_tls = tracing_impl->GetOrCreateTracingTLS();
    DataSourceThreadLocalState* ds_tls =
        DataSourceTraits::GetDataSourceTLS(&state_, root_tls);
    // We keep re-initializing as the initialization is idempotent and not worth
    // the code for extra checks. Also, ds_tls->static_state might point to
    // another data source if ResetForTesting() has been used.
    ds_tls->static_state = &state_;
    assert(!ds_tls->root_tls || ds_tls->root_tls == root_tls);
    ds_tls->root_tls = root_tls;
    return ds_tls;
  }

  DataSourceStaticState state_;
  BufferExhaustedPolicy buffer_exhausted_policy_{};
  CreateCustomTlsFn create_custom_tls_fn_ = nullptr;
  CreateIncrementalStateFn create_incremental_state_fn_ = nullptr;
  // User defined pointer that carries extra content for the fn_ callbacks
  // above. Only used in the C shared library.
  void* user_arg_ = nullptr;
};

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_DATA_SOURCE_TYPE_H_

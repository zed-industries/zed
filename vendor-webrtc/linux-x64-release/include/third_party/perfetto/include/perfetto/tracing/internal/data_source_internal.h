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

#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_DATA_SOURCE_INTERNAL_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_DATA_SOURCE_INTERNAL_H_

#include <stddef.h>
#include <stdint.h>

#include <array>
#include <atomic>
#include <memory>
#include <mutex>

// No perfetto headers (other than tracing/api and protozero) should be here.
#include "perfetto/tracing/core/data_source_config.h"
#include "perfetto/tracing/internal/basic_types.h"
#include "perfetto/tracing/trace_writer_base.h"

namespace perfetto {

class DataSourceBase;
class InterceptorBase;
class TraceWriterBase;

namespace internal {

class TracingTLS;

// This maintains the internal state of a data source instance that is used only
// to implement the tracing mechanics and is not exposed to the API client.
// There is one of these object per DataSource instance (up to
// kMaxDataSourceInstances).
struct DataSourceState {
  // This boolean flag determines whether the DataSource::Trace() method should
  // do something or be a no-op. This flag doesn't give the full guarantee
  // that tracing data will be visible in the trace, it just makes it so that
  // the client attemps writing trace data and interacting with the service.
  // For instance, when a tracing session ends the service will reject data
  // commits that arrive too late even if the producer hasn't received the stop
  // IPC message.
  // This flag is set right before calling OnStart() and cleared right before
  // calling OnStop(), unless using HandleStopAsynchronously() (see comments
  // in data_source.h).
  // Keep this flag as the first field. This allows the compiler to directly
  // dereference the DataSourceState* pointer in the trace fast-path without
  // doing extra pointr arithmetic.
  std::atomic<bool> trace_lambda_enabled{false};

  // The overall TracingMuxerImpl instance id, which gets incremented by
  // ResetForTesting.
  uint32_t muxer_id_for_testing = 0;

  // The central buffer id that all TraceWriter(s) created by this data source
  // must target.
  BufferId buffer_id = 0;

  // The index within TracingMuxerImpl.backends_. Practically it allows to
  // lookup the Producer object, and hence the IPC channel, for this data
  // source.
  TracingBackendId backend_id = 0;

  // Each backend may connect to the tracing service multiple times if a
  // disconnection occurs. This counter is used to uniquely identify each
  // connection so that trace writers don't get reused across connections.
  uint32_t backend_connection_id = 0;

  // The instance id as assigned by the tracing service. Note that because a
  // process can be connected to >1 services, this ID is not globally unique but
  // is only unique within the scope of its backend.
  // Only the tuple (backend_id, data_source_instance_id) is globally unique.
  uint64_t data_source_instance_id = 0;

  // Set to a non-0 target buffer reservation ID iff startup tracing is
  // currently enabled for this data source.
  std::atomic<uint16_t> startup_target_buffer_reservation{0};

  // If the data source was originally started for startup tracing, this is set
  // to the startup session's ID.
  uint64_t startup_session_id = 0;

  // The trace config used by this instance. This is used to de-duplicate
  // instances for data sources with identical names (e.g., track event).
  // We store it as a pointer to be able to free memory after the datasource
  // is stopped.
  std::unique_ptr<DataSourceConfig> config;

  // If this data source is being intercepted (see Interceptor), this field
  // contains the non-zero id of a registered interceptor which should receive
  // trace packets for this session. Note: interceptor id 1 refers to the first
  // element of TracingMuxerImpl::interceptors_ with successive numbers using
  // the following slots.
  uint32_t interceptor_id = 0;

  // This is set to true when the datasource is in the process of async stop.
  // The flag is checked by the tracing muxer to avoid calling OnStop for the
  // second time.
  bool async_stop_in_progress = false;

  // Whether this data source instance should call NotifyDataSourceStopped()
  // when it's stopped.
  bool will_notify_on_stop = false;

  // Incremented whenever incremental state should be reset for this instance of
  // this data source.
  std::atomic<uint32_t> incremental_state_generation{0};

  // This lock is not held to implement Trace() and it's used only if the trace
  // code wants to access its own data source state.
  // This is to prevent that accessing the data source on an arbitrary embedder
  // thread races with the internal IPC thread destroying the data source
  // because of a end-of-tracing notification from the service.
  // This lock is also used to protect access to a possible interceptor for this
  // data source session.
  std::recursive_mutex lock;
  std::unique_ptr<DataSourceBase> data_source;
  std::unique_ptr<InterceptorBase> interceptor;
};

// This is to allow lazy-initialization and avoid static initializers and
// at-exit destructors. All the entries are initialized via placement-new when
// DataSource::Register() is called, see TracingMuxerImpl::RegisterDataSource().
struct DataSourceStateStorage {
  alignas(DataSourceState) char storage[sizeof(DataSourceState)]{};
};

// Per-DataSource-type global state.
struct DataSourceStaticState {
  // System-wide unique id of the data source.
  uint64_t id = 0;

  // Unique index of the data source, assigned at registration time.
  uint32_t index = kMaxDataSources;

  // A bitmap that tells about the validity of each |instances| entry. When the
  // i-th bit of the bitmap it's set, instances[i] is valid.
  std::atomic<uint32_t> valid_instances{};
  std::array<DataSourceStateStorage, kMaxDataSourceInstances> instances{};

  // The caller must be sure that `n` was a valid instance at some point (either
  // through a previous read of `valid_instances` or because the instance lock
  // is held).
  DataSourceState* GetUnsafe(size_t n) {
    return reinterpret_cast<DataSourceState*>(&instances[n]);
  }

  // Can be used with a cached |valid_instances| bitmap.
  DataSourceState* TryGetCached(uint32_t cached_bitmap, size_t n) {
    return cached_bitmap & (1 << n) ? GetUnsafe(n) : nullptr;
  }

  DataSourceState* TryGet(size_t n) {
    return TryGetCached(valid_instances.load(std::memory_order_acquire), n);
  }

  void CompilerAsserts() {
    static_assert(sizeof(valid_instances.load()) * 8 >= kMaxDataSourceInstances,
                  "kMaxDataSourceInstances too high");
  }

  void ResetForTesting() {
    id = 0;
    index = kMaxDataSources;
    valid_instances.store(0, std::memory_order_release);
    instances = {};
  }
};

// Per-DataSource-instance thread-local state.
struct DataSourceInstanceThreadLocalState {
  void Reset() { *this = DataSourceInstanceThreadLocalState{}; }

  std::unique_ptr<TraceWriterBase> trace_writer;
  using ObjectWithDeleter = std::unique_ptr<void, void (*)(void*)>;
  ObjectWithDeleter incremental_state = {nullptr, [](void*) {}};
  ObjectWithDeleter data_source_custom_tls = {nullptr, [](void*) {}};
  uint32_t incremental_state_generation = 0;
  uint32_t muxer_id_for_testing = 0;
  TracingBackendId backend_id = 0;
  uint32_t backend_connection_id = 0;
  BufferId buffer_id = 0;
  uint64_t data_source_instance_id = 0;
  bool is_intercepted = false;
  uint64_t last_empty_packet_position = 0;
  uint16_t startup_target_buffer_reservation = 0;
};

// Per-DataSource-type thread-local state.
struct DataSourceThreadLocalState {
  DataSourceStaticState* static_state = nullptr;

  // Pointer to the parent tls object that holds us. Used to retrieve the
  // generation, which is per-global-TLS and not per data-source.
  TracingTLS* root_tls = nullptr;

  // One entry per each data source instance.
  std::array<DataSourceInstanceThreadLocalState, kMaxDataSourceInstances>
      per_instance{};
};

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_DATA_SOURCE_INTERNAL_H_

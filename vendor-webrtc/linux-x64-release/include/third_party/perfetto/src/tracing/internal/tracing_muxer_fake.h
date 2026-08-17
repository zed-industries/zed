/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_TRACING_INTERNAL_TRACING_MUXER_FAKE_H_
#define SRC_TRACING_INTERNAL_TRACING_MUXER_FAKE_H_

#include "perfetto/base/compiler.h"
#include "perfetto/tracing/internal/tracing_muxer.h"

namespace perfetto {
namespace internal {

// An always-fail implementation of TracingMuxer. Before tracing has been
// initialized, all muxer operations will route here and fail with a helpful
// error message. This is to avoid introducing null checks in
// performance-critical parts of the codebase.
class TracingMuxerFake : public TracingMuxer {
  class FakePlatform : public Platform {
   public:
    ~FakePlatform() override;
    ThreadLocalObject* GetOrCreateThreadLocalObject() override;
    std::unique_ptr<base::TaskRunner> CreateTaskRunner(
        const CreateTaskRunnerArgs&) override;
    std::string GetCurrentProcessName() override;

    static FakePlatform instance;
  };

 public:
  TracingMuxerFake() : TracingMuxer(&FakePlatform::instance) {}
  ~TracingMuxerFake() override;

  static constexpr TracingMuxerFake* Get() {
#if PERFETTO_HAS_NO_DESTROY()
    return &instance;
#else
    return nullptr;
#endif
  }

  // TracingMuxer implementation.
  bool RegisterDataSource(const DataSourceDescriptor&,
                          DataSourceFactory,
                          DataSourceParams,
                          bool,
                          DataSourceStaticState*) override;
  void UpdateDataSourceDescriptor(const DataSourceDescriptor&,
                                  const DataSourceStaticState*) override;
  std::unique_ptr<TraceWriterBase> CreateTraceWriter(
      DataSourceStaticState*,
      uint32_t data_source_instance_index,
      DataSourceState*,
      BufferExhaustedPolicy buffer_exhausted_policy) override;
  void DestroyStoppedTraceWritersForCurrentThread() override;
  void RegisterInterceptor(const InterceptorDescriptor&,
                           InterceptorFactory,
                           InterceptorBase::TLSFactory,
                           InterceptorBase::TracePacketCallback) override;
  void ActivateTriggers(const std::vector<std::string>&, uint32_t) override;

 private:
  static TracingMuxerFake instance;
};

}  // namespace internal
}  // namespace perfetto

#endif  // SRC_TRACING_INTERNAL_TRACING_MUXER_FAKE_H_

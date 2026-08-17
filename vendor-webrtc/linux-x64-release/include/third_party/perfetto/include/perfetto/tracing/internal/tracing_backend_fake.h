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

#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_TRACING_BACKEND_FAKE_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_TRACING_BACKEND_FAKE_H_

#include "perfetto/base/export.h"
#include "perfetto/tracing/tracing_backend.h"

namespace perfetto {
namespace internal {

// A built-in implementation of TracingBackend that fails any attempt to create
// a tracing session.
class PERFETTO_EXPORT_COMPONENT TracingBackendFake : public TracingBackend {
 public:
  static TracingBackend* GetInstance();

  // TracingBackend implementation.
  std::unique_ptr<ProducerEndpoint> ConnectProducer(
      const ConnectProducerArgs&) override;
  std::unique_ptr<ConsumerEndpoint> ConnectConsumer(
      const ConnectConsumerArgs&) override;

 private:
  TracingBackendFake();
};

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_TRACING_BACKEND_FAKE_H_

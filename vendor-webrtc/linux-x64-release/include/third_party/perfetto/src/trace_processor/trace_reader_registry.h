/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_TRACE_READER_REGISTRY_H_
#define SRC_TRACE_PROCESSOR_TRACE_READER_REGISTRY_H_

#include <functional>
#include <memory>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/status_or.h"
#include "src/trace_processor/util/trace_type.h"

namespace perfetto {
namespace trace_processor {

class ChunkedTraceReader;
class TraceProcessorContext;

// Maps `TraceType` values to `ChunkedTraceReader` subclasses.
// This class is used to create `ChunkedTraceReader` instances for a given
// `TraceType`.
class TraceReaderRegistry {
 public:
  explicit TraceReaderRegistry(TraceProcessorContext* context)
      : context_(context) {}

  // Registers a mapping from `TraceType` value to `ChunkedTraceReader`
  // subclass. Only one such mapping can be registered per `TraceType` value.
  template <typename Reader>
  void RegisterTraceReader(TraceType trace_type) {
    RegisterFactory(trace_type, [](TraceProcessorContext* ctxt) {
      return std::make_unique<Reader>(ctxt);
    });
  }

  // Creates a new `ChunkedTraceReader` instance for the given `type`. Returns
  // an error if no mapping has been previously registered.
  base::StatusOr<std::unique_ptr<ChunkedTraceReader>> CreateTraceReader(
      TraceType type);

 private:
  using Factory = std::function<std::unique_ptr<ChunkedTraceReader>(
      TraceProcessorContext*)>;
  void RegisterFactory(TraceType trace_type, Factory factory);

  TraceProcessorContext* const context_;
  base::FlatHashMap<TraceType,
                    std::function<std::unique_ptr<ChunkedTraceReader>(
                        TraceProcessorContext*)>>
      factories_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_TRACE_READER_REGISTRY_H_

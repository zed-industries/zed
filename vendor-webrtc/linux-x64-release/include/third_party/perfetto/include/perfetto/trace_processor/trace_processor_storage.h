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

#ifndef INCLUDE_PERFETTO_TRACE_PROCESSOR_TRACE_PROCESSOR_STORAGE_H_
#define INCLUDE_PERFETTO_TRACE_PROCESSOR_TRACE_PROCESSOR_STORAGE_H_

#include <cstdint>

#include <memory>

#include "perfetto/base/export.h"
#include "perfetto/base/status.h"
#include "perfetto/trace_processor/basic_types.h"
#include "perfetto/trace_processor/status.h"
#include "perfetto/trace_processor/trace_blob_view.h"

namespace perfetto::trace_processor {

// Coordinates the loading of traces from an arbitrary source.
class PERFETTO_EXPORT_COMPONENT TraceProcessorStorage {
 public:
  // Creates a new instance of TraceProcessorStorage.
  static std::unique_ptr<TraceProcessorStorage> CreateInstance(const Config&);

  virtual ~TraceProcessorStorage();

  // The entry point to push trace data into the processor. The trace format
  // will be automatically discovered on the first push call. It is possible
  // to make queries between two pushes.
  // Returns the Ok status if parsing has been succeeding so far, and Error
  // status if some unrecoverable error happened. If this happens, the
  // TraceProcessor will ignore the following Parse() requests, drop data on the
  // floor and return errors forever.
  virtual base::Status Parse(TraceBlobView) = 0;

  // Shorthand for Parse(TraceBlobView(TraceBlob(TakeOwnership(buf, size))).
  // For compatibility with older API clients.
  base::Status Parse(std::unique_ptr<uint8_t[]> buf, size_t size);

  // Forces all data in the trace to be pushed to tables without buffering data
  // in sorting queues. This is useful if queries need to be performed to
  // compute post-processing data (e.g. deobfuscation, symbolization etc) which
  // will be appended to the trace in a future call to Parse.
  virtual void Flush() = 0;

  // Calls Flush and finishes all of the actions required for parsing the trace.
  // Calling this function multiple times is undefined behaviour.
  virtual base::Status NotifyEndOfFile() = 0;
};

}  // namespace perfetto::trace_processor

#endif  // INCLUDE_PERFETTO_TRACE_PROCESSOR_TRACE_PROCESSOR_STORAGE_H_

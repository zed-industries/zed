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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_ATTRS_SECTION_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_ATTRS_SECTION_READER_H_

#include <cstddef>
#include <utility>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/perf/perf_file.h"
#include "src/trace_processor/importers/perf/reader.h"

namespace perfetto::trace_processor::perf_importer {

// Helper to read the attrs section of a perf file. Provides an iterator like
// interface over the perf_event_attr entries.
class AttrsSectionReader {
 public:
  // Creates a new iterator.
  // `attrs_section` data contained in the attrs section of the perf file.
  static base::StatusOr<AttrsSectionReader> Create(
      const PerfFile::Header& header,
      TraceBlobView attrs_section);

  // Returns true while there are available entries to read via `ReadNext`.
  bool CanReadNext() const { return num_attr_ != 0; }

  // Reads the next entry. Can onlybe called if `HasMore` returns true.
  base::Status ReadNext(PerfFile::AttrsEntry& entry);

 private:
  AttrsSectionReader(TraceBlobView section, size_t num_attr, size_t attr_size)
      : reader_(std::move(section)),
        num_attr_(num_attr),
        attr_size_(attr_size) {}

  Reader reader_;
  size_t num_attr_;
  const size_t attr_size_;
};

}  // namespace perfetto::trace_processor::perf_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_ATTRS_SECTION_READER_H_

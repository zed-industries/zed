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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FUCHSIA_FUCHSIA_RECORD_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FUCHSIA_FUCHSIA_RECORD_H_

#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/containers/string_pool.h"

#include <vector>

namespace perfetto {
namespace trace_processor {

struct FuchsiaThreadInfo {
  uint64_t pid{0};
  uint64_t tid{0};
};

// Data from a trace provider that is necessary for interpreting a binary
// record. Namely, the record itself and the entries of the string table and the
// thread table that are referenced by the record. This enables understanding
// the binary record after arbitrary reordering.
class FuchsiaRecord {
 public:
  explicit FuchsiaRecord(TraceBlobView record_view)
      : record_view_(std::move(record_view)) {}

  struct StringTableEntry {
    uint32_t index;
    StringPool::Id string_id;
  };

  struct ThreadTableEntry {
    uint32_t index;
    FuchsiaThreadInfo info;
  };

  void InsertString(uint32_t, StringPool::Id);
  StringPool::Id GetString(uint32_t);

  void InsertThread(uint32_t, FuchsiaThreadInfo);
  FuchsiaThreadInfo GetThread(uint32_t);

  void set_ticks_per_second(uint64_t ticks_per_second) {
    ticks_per_second_ = ticks_per_second;
  }

  uint64_t get_ticks_per_second() { return ticks_per_second_; }

  TraceBlobView* record_view() { return &record_view_; }

 private:
  TraceBlobView record_view_;

  std::vector<StringTableEntry> string_entries_;
  std::vector<ThreadTableEntry> thread_entries_;

  uint64_t ticks_per_second_ = 1000000000;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FUCHSIA_FUCHSIA_RECORD_H_

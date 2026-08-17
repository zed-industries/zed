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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_ARCHIVE_ENTRY_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_ARCHIVE_ENTRY_H_

#include <string>

#include "src/trace_processor/util/trace_type.h"

namespace perfetto::trace_processor {

// Helper class to determine a proper tokenization. This class can be used as
// a key of a std::map to automatically sort files before sending them in proper
// order for tokenization.
struct ArchiveEntry {
  // File name. Used to break ties.
  std::string name;
  // Position. Used to break ties.
  size_t index;
  // Trace type. This is the main attribute traces are ordered by. Proto
  // traces are always parsed first as they might contains clock sync
  // data needed to correctly parse other traces.
  TraceType trace_type;
  // Comparator used to determine the order in which files in the ZIP will be
  // read.
  bool operator<(const ArchiveEntry& rhs) const;
};
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_ARCHIVE_ENTRY_H_

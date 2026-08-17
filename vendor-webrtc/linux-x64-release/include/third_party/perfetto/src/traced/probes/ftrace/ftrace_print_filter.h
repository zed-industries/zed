/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACED_PROBES_FTRACE_FTRACE_PRINT_FILTER_H_
#define SRC_TRACED_PROBES_FTRACE_FTRACE_PRINT_FILTER_H_

#include <optional>
#include <string>
#include <vector>

#include "src/traced/probes/ftrace/proto_translation_table.h"

namespace perfetto {

struct Event;

namespace protos {
namespace gen {
class FtraceConfig_PrintFilter;
}  // namespace gen
}  // namespace protos

class FtracePrintFilter {
 public:
  // Builds a filter from a proto config.
  explicit FtracePrintFilter(const protos::gen::FtraceConfig::PrintFilter&);

  // Returns true if a string is allowed by this filter, false otherwise.
  // The string begins at `start` and terminates after `size` bytes, or at the
  // first '\0' byte, whichever comes first.
  bool IsAllowed(const char* start, size_t size) const;

 private:
  struct Rule {
    enum class Type {
      kPrefixMatch,
      kAtraceMessage,
    };
    Type type;
    std::string before_pid_part;
    std::string prefix;
    bool allow;
  };

  static bool RuleMatches(const Rule&, const char* start, size_t size);

  std::vector<Rule> rules_;
};

class FtracePrintFilterConfig {
 public:
  static std::optional<FtracePrintFilterConfig> Create(
      const protos::gen::FtraceConfig_PrintFilter&,
      ProtoTranslationTable* table);

  uint32_t event_id() const { return event_id_; }

  // Returns true if the "ftrace/print" event (encoded from `start` to `end`)
  // should be allowed.
  //
  // If the event should be allowed, or **if there was a problem parsing it**
  // returns true. If the event should be disallowed (i.e. ignored), returns
  // false.
  bool IsEventInteresting(const uint8_t* start, const uint8_t* end) const;

 private:
  explicit FtracePrintFilterConfig(FtracePrintFilter filter);
  FtracePrintFilter filter_;
  uint32_t event_id_;
  uint16_t event_size_;
  uint16_t buf_field_offset_;
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_FTRACE_FTRACE_PRINT_FILTER_H_

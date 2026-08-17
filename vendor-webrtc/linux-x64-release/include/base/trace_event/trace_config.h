// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TRACE_CONFIG_H_
#define BASE_TRACE_EVENT_TRACE_CONFIG_H_

#include <stdint.h>

#include <memory>
#include <set>
#include <string>
#include <string_view>
#include <unordered_set>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/gtest_prod_util.h"
#include "base/trace_event/memory_dump_request_args.h"
#include "base/trace_event/trace_config_category_filter.h"
#include "base/values.h"

namespace base::trace_event {

class ConvertableToTraceFormat;

// Options determines how the trace buffer stores data.
// A Java counterpart will be generated for this enum.
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base
enum TraceRecordMode {
  // Record until the trace buffer is full.
  RECORD_UNTIL_FULL,

  // Record until the user ends the trace. The trace buffer is a fixed size
  // and we use it as a ring buffer during recording.
  RECORD_CONTINUOUSLY,

  // Record until the trace buffer is full, but with a huge buffer size.
  RECORD_AS_MUCH_AS_POSSIBLE,

  // Echo to console. Events are discarded.
  ECHO_TO_CONSOLE,
};

class BASE_EXPORT TraceConfig {
 public:
  using StringList = std::vector<std::string>;

  // Specifies the memory dump config for tracing.
  // Used only when "memory-infra" category is enabled.
  struct BASE_EXPORT MemoryDumpConfig {
    MemoryDumpConfig();
    MemoryDumpConfig(const MemoryDumpConfig& other);
    ~MemoryDumpConfig();

    // Specifies the triggers in the memory dump config.
    struct Trigger {
      friend bool operator==(const Trigger&, const Trigger&) = default;

      uint32_t min_time_between_dumps_ms;
      MemoryDumpLevelOfDetail level_of_detail;
      MemoryDumpType trigger_type;
    };

    // Specifies the configuration options for the heap profiler.
    struct HeapProfiler {
      // Default value for |breakdown_threshold_bytes|.
      enum { kDefaultBreakdownThresholdBytes = 1024 };

      HeapProfiler();

      // Reset the options to default.
      void Clear();

      friend bool operator==(const HeapProfiler&,
                             const HeapProfiler&) = default;

      uint32_t breakdown_threshold_bytes;
    };

    friend bool operator==(const MemoryDumpConfig&,
                           const MemoryDumpConfig&) = default;

    // Reset the values in the config.
    void Clear();

    void Merge(const MemoryDumpConfig& config);

    // Set of memory dump modes allowed for the tracing session. The explicitly
    // triggered dumps will be successful only if the dump mode is allowed in
    // the config.
    std::set<MemoryDumpLevelOfDetail> allowed_dump_modes;

    std::vector<Trigger> triggers;
    HeapProfiler heap_profiler_options;
  };

  class BASE_EXPORT ProcessFilterConfig {
   public:
    ProcessFilterConfig();
    explicit ProcessFilterConfig(
        const std::unordered_set<base::ProcessId>& included_process_ids);
    ProcessFilterConfig(const ProcessFilterConfig&);
    ~ProcessFilterConfig();

    bool empty() const { return included_process_ids_.empty(); }

    void Clear();
    void Merge(const ProcessFilterConfig&);

    void InitializeFromConfigDict(const Value::Dict&);
    void ToDict(Value::Dict& dict) const;

    bool IsEnabled(base::ProcessId) const;
    const std::unordered_set<base::ProcessId>& included_process_ids() const {
      return included_process_ids_;
    }

    friend bool operator==(const ProcessFilterConfig&,
                           const ProcessFilterConfig&) = default;

   private:
    std::unordered_set<base::ProcessId> included_process_ids_;
  };

  class BASE_EXPORT EventFilterConfig {
   public:
    explicit EventFilterConfig(const std::string& predicate_name);
    EventFilterConfig(const EventFilterConfig& tc);

    ~EventFilterConfig();

    EventFilterConfig& operator=(const EventFilterConfig& rhs);

    bool IsEquivalentTo(const EventFilterConfig& other) const;

    void InitializeFromConfigDict(const Value::Dict& event_filter);

    void SetCategoryFilter(const TraceConfigCategoryFilter& category_filter);

    void ToDict(Value::Dict& filter_dict) const;

    bool GetArgAsSet(const char* key, std::unordered_set<std::string>*) const;

    bool IsCategoryGroupEnabled(std::string_view category_group_name) const;

    const std::string& predicate_name() const LIFETIME_BOUND {
      return predicate_name_;
    }
    const Value::Dict& filter_args() const LIFETIME_BOUND { return args_; }
    const TraceConfigCategoryFilter& category_filter() const LIFETIME_BOUND {
      return category_filter_;
    }

   private:
    std::string predicate_name_;
    TraceConfigCategoryFilter category_filter_;
    Value::Dict args_;
  };
  typedef std::vector<EventFilterConfig> EventFilters;

  static std::string TraceRecordModeToStr(TraceRecordMode record_mode);

  TraceConfig();

  // Create TraceConfig object from category filter and trace options strings.
  //
  // |category_filter_string| is a comma-delimited list of category wildcards.
  // A category can have an optional '-' prefix to make it an excluded category.
  // All the same rules apply above, so for example, having both included and
  // excluded categories in the same list would not be supported.
  //
  // |trace_options_string| is a comma-delimited list of trace options.
  // Possible options are: "record-until-full", "record-continuously",
  // "record-as-much-as-possible", "trace-to-console", "enable-systrace" and
  // "enable-argument-filter".
  // The first 4 options are trace recoding modes and hence
  // mutually exclusive. If more than one trace recording modes appear in the
  // options_string, the last one takes precedence. If none of the trace
  // recording mode is specified, recording mode is RECORD_UNTIL_FULL.
  //
  // The trace option will first be reset to the default option
  // (record_mode set to RECORD_UNTIL_FULL, enable_systrace and
  // enable_argument_filter set to false) before options parsed from
  // |trace_options_string| are applied on it. If |trace_options_string| is
  // invalid, the final state of trace options is undefined.
  //
  // Example: TraceConfig("test_MyTest*", "record-until-full");
  // Example: TraceConfig("test_MyTest*,test_OtherStuff",
  //                      "record-continuously");
  // Example: TraceConfig("-excluded_category1,-excluded_category2",
  //                      "record-until-full, trace-to-console");
  //          would set ECHO_TO_CONSOLE as the recording mode.
  // Example: TraceConfig("-*,webkit", "");
  //          would disable everything but webkit; and use default options.
  // Example: TraceConfig("-webkit", "");
  //          would enable everything but webkit; and use default options.
  TraceConfig(std::string_view category_filter_string,
              std::string_view trace_options_string);

  TraceConfig(std::string_view category_filter_string,
              TraceRecordMode record_mode);

  // Create TraceConfig object from the trace config string.
  //
  // |config_string| is a dictionary formatted as a JSON string, containing both
  // category filters and trace options.
  //
  // Example:
  //   {
  //     "record_mode": "record-continuously",
  //     "enable_systrace": true,
  //     "enable_argument_filter": true,
  //     "included_categories": ["included",
  //                             "inc_pattern*",
  //                             "disabled-by-default-memory-infra"],
  //     "excluded_categories": ["excluded", "exc_pattern*"],
  //     "memory_dump_config": {
  //       "triggers": [
  //         {
  //           "mode": "detailed",
  //           "periodic_interval_ms": 2000
  //         }
  //       ]
  //     }
  //   }
  //
  // Note: memory_dump_config can be specified only if
  // disabled-by-default-memory-infra category is enabled.
  explicit TraceConfig(std::string_view config_string);

  // Functionally identical to the above, but takes a parsed dictionary as input
  // instead of its JSON serialization.
  explicit TraceConfig(const Value::Dict& config);

  TraceConfig(const TraceConfig& tc);

  ~TraceConfig();

  TraceConfig& operator=(const TraceConfig& rhs);

  bool IsEquivalentTo(const TraceConfig& other) const;

  TraceRecordMode GetTraceRecordMode() const { return record_mode_; }
  size_t GetTraceBufferSizeInEvents() const {
    return trace_buffer_size_in_events_;
  }
  size_t GetTraceBufferSizeInKb() const { return trace_buffer_size_in_kb_; }
  bool IsSystraceEnabled() const { return enable_systrace_; }
  bool IsArgumentFilterEnabled() const { return enable_argument_filter_; }

  void SetTraceRecordMode(TraceRecordMode mode) { record_mode_ = mode; }
  void SetTraceBufferSizeInEvents(size_t size) {
    trace_buffer_size_in_events_ = size;
  }
  void SetTraceBufferSizeInKb(size_t size) { trace_buffer_size_in_kb_ = size; }
  void EnableSystrace() { enable_systrace_ = true; }
  void EnableSystraceEvent(const std::string& systrace_event);
  void EnableArgumentFilter() { enable_argument_filter_ = true; }
  void EnableHistogram(const std::string& histogram_name);

  // Writes the string representation of the TraceConfig. The string is JSON
  // formatted.
  std::string ToString() const;

  // Returns a copy of the TraceConfig wrapped in a ConvertableToTraceFormat
  std::unique_ptr<ConvertableToTraceFormat> AsConvertableToTraceFormat() const;

  // Write the string representation of the CategoryFilter part.
  std::string ToCategoryFilterString() const;

  // Write the string representation of the trace options part (record mode,
  // systrace, argument filtering). Does not include category filters, event
  // filters, or memory dump configs.
  std::string ToTraceOptionsString() const;

  // Write the serialized perfetto::TrackEventConfig corresponding to this
  // TraceConfig.
  std::string ToPerfettoTrackEventConfigRaw(
      bool privacy_filtering_enabled) const;

  // Returns true if at least one category in the list is enabled by this
  // trace config. This is used to determine if the category filters are
  // enabled in the TRACE_* macros.
  bool IsCategoryGroupEnabled(std::string_view category_group_name) const;

  // Merges config with the current TraceConfig
  void Merge(const TraceConfig& config);

  void Clear();

  // Clears and resets the memory dump config.
  void ResetMemoryDumpConfig(const MemoryDumpConfig& memory_dump_config);

  const TraceConfigCategoryFilter& category_filter() const LIFETIME_BOUND {
    return category_filter_;
  }

  const MemoryDumpConfig& memory_dump_config() const LIFETIME_BOUND {
    return memory_dump_config_;
  }

  const ProcessFilterConfig& process_filter_config() const LIFETIME_BOUND {
    return process_filter_config_;
  }
  void SetProcessFilterConfig(const ProcessFilterConfig&);

  const EventFilters& event_filters() const LIFETIME_BOUND {
    return event_filters_;
  }
  void SetEventFilters(const EventFilters& filter_configs) {
    event_filters_ = filter_configs;
  }

  // Returns true if event names should not contain package names.
  bool IsEventPackageNameFilterEnabled() const {
    return enable_event_package_name_filter_;
  }

  // If `enabled` is true, event names will not contain package names.
  void SetEventPackageNameFilterEnabled(bool enabled) {
    enable_event_package_name_filter_ = enabled;
  }

  const std::unordered_set<std::string>& systrace_events() const {
    return systrace_events_;
  }

  const std::unordered_set<std::string>& histogram_names() const {
    return histogram_names_;
  }

 private:
  FRIEND_TEST_ALL_PREFIXES(TraceConfigTest, TraceConfigFromValidLegacyFormat);
  FRIEND_TEST_ALL_PREFIXES(TraceConfigTest,
                           TraceConfigFromInvalidLegacyStrings);
  FRIEND_TEST_ALL_PREFIXES(TraceConfigTest, SystraceEventsSerialization);

  // The default trace config, used when none is provided.
  // Allows all non-disabled-by-default categories through, except if they end
  // in the suffix 'Debug' or 'Test'.
  void InitializeDefault();

  // Initialize from a config dictionary.
  void InitializeFromConfigDict(const Value::Dict& dict);

  // Initialize from a config string.
  void InitializeFromConfigString(std::string_view config_string);

  // Initialize from category filter and trace options strings
  void InitializeFromStrings(std::string_view category_filter_string,
                             std::string_view trace_options_string);

  void SetMemoryDumpConfigFromConfigDict(const Value::Dict& memory_dump_config);
  void SetDefaultMemoryDumpConfig();

  void SetHistogramNamesFromConfigList(const Value::List& histogram_names);
  void SetEventFiltersFromConfigList(const Value::List& event_filters);
  Value ToValue() const;

  TraceRecordMode record_mode_;
  size_t trace_buffer_size_in_events_ = 0;  // 0 specifies default size
  size_t trace_buffer_size_in_kb_ = 0;      // 0 specifies default size
  bool enable_systrace_ : 1;
  bool enable_argument_filter_ : 1;

  TraceConfigCategoryFilter category_filter_;

  MemoryDumpConfig memory_dump_config_;
  ProcessFilterConfig process_filter_config_;

  EventFilters event_filters_;
  bool enable_event_package_name_filter_ : 1;
  std::unordered_set<std::string> histogram_names_;
  std::unordered_set<std::string> systrace_events_;
};

}  // namespace base::trace_event

#endif  // BASE_TRACE_EVENT_TRACE_CONFIG_H_

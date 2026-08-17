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

#ifndef SRC_TRACE_PROCESSOR_UTIL_PROFILE_BUILDER_H_
#define SRC_TRACE_PROCESSOR_UTIL_PROFILE_BUILDER_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/protozero/packed_repeated_fields.h"
#include "perfetto/protozero/scattered_heap_buffer.h"
#include "protos/perfetto/trace_processor/stack.pbzero.h"
#include "protos/third_party/pprof/profile.pbzero.h"
#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/profiler_tables_py.h"
#include "src/trace_processor/util/annotated_callsites.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

// Builds a |perftools.profiles.Profile| proto.
class GProfileBuilder {
 public:
  struct ValueType {
    std::string type;
    std::string unit;
  };

  // |sample_types| A description of the values stored with each sample.
  // |annotated| Whether to annotate callstack frames.
  //
  // Important: Annotations might interfere with certain aggregations, as we
  // will could have a frame that is annotated with different annotations. That
  // will lead to multiple functions being generated (sane name, line etc, but
  // different annotation). Since there is no field in a Profile proto to track
  // these annotations we extend the function name (my_func [annotation]), so
  // from pprof perspective we now have different functions. So in flame graphs
  // for example you will have one separate slice for each of these same
  // functions with different annotations.
  GProfileBuilder(const TraceProcessorContext* context,
                  const std::vector<ValueType>& sample_types);
  ~GProfileBuilder();

  // Returns false if the operation fails (e.g callsite_id was not found)
  bool AddSample(const protos::pbzero::Stack_Decoder& stack,
                 const std::vector<int64_t>& values);

  // Finalizes the profile and returns the serialized proto. Can be called
  // multiple times but after the first invocation `AddSample` calls will have
  // no effect.
  std::string Build();

 private:
  static constexpr int64_t kEmptyStringIndex = 0;
  static constexpr uint64_t kNullFunctionId = 0;

  // Strings are stored in the `Profile` in a table and referenced by their
  // index. This helper class takes care of all the book keeping.
  // `TraceProcessor` uses its own `StringPool` for strings. This helper
  // provides convenient ways of dealing with `StringPool::Id` values instead of
  // actual string. This class ensures that two equal strings will have the same
  // index, so you can compare them instead of the actual strings.
  class StringTable {
   public:
    // |result| This is the `Profile` proto we are building. Strings will be
    // added to it as necessary. |string_pool| `StringPool` to quey for strings
    // passed as `StringPool:Id`
    StringTable(protozero::HeapBuffered<
                    third_party::perftools::profiles::pbzero::Profile>* result,
                const StringPool* string_pool);

    // Adds the given string to the table, if not currently present, and returns
    // the index to it. Might write data to the infligt `Profile` so it should
    // not be called while in the middle of writing a message to the proto.
    int64_t InternString(base::StringView str);
    // Adds a string stored in the `TraceProcessor` `StringPool` to the table,
    // if not currently present, and returns the index to it. Might write data
    // to the inflight `Profile` so it should not be called while in the middle
    // of writing a message to the proto.
    int64_t InternString(StringPool::Id id);

    int64_t GetAnnotatedString(StringPool::Id str,
                               CallsiteAnnotation annotation);
    int64_t GetAnnotatedString(base::StringView str,
                               CallsiteAnnotation annotation);

   private:
    // Unconditionally writes the given string to the table and returns its
    // index.
    int64_t WriteString(base::StringView str);

    const StringPool& string_pool_;
    protozero::HeapBuffered<third_party::perftools::profiles::pbzero::Profile>&
        result_;

    std::unordered_map<StringPool::Id, int64_t> seen_string_pool_ids_;
    // Maps strings (hashes thereof) to indexes in the table.
    std::unordered_map<uint64_t, int64_t> seen_strings_;
    // Index where the next string will be written to
    int64_t next_index_{0};
  };

  struct AnnotatedFrameId {
    struct Hash {
      size_t operator()(const AnnotatedFrameId& id) const {
        return static_cast<size_t>(perfetto::base::Hasher::Combine(
            id.frame_id.value, static_cast<int>(id.annotation)));
      }
    };

    FrameId frame_id;
    CallsiteAnnotation annotation;

    bool operator==(const AnnotatedFrameId& other) const {
      return frame_id == other.frame_id && annotation == other.annotation;
    }
  };

  struct Line {
    uint64_t function_id;
    int64_t line;
    bool operator==(const Line& other) const {
      return function_id == other.function_id && line == other.line;
    }
  };

  // Location, MappingKey, Mapping, Function, and Line are helper structs to
  // deduplicate entities. We do not write these directly to the proto Profile
  // but instead stage them and write them out during `Finalize`. Samples on the
  // other hand are directly written to the proto.

  struct Location {
    struct Hash {
      size_t operator()(const Location& loc) const {
        perfetto::base::Hasher hasher;
        hasher.UpdateAll(loc.mapping_id, loc.rel_pc, loc.lines.size());
        for (const auto& line : loc.lines) {
          hasher.UpdateAll(line.function_id, line.line);
        }
        return static_cast<size_t>(hasher.digest());
      }
    };

    uint64_t mapping_id;
    uint64_t rel_pc;
    std::vector<Line> lines;

    bool operator==(const Location& other) const {
      return mapping_id == other.mapping_id && rel_pc == other.rel_pc &&
             lines == other.lines;
    }
  };

  // Mappings are tricky. We could have samples for different processes and
  // given address space layout randomization the same mapping could be located
  // at different addresses. MappingKey has the set of properties that uniquely
  // identify mapping in order to deduplicate rows in the stack_profile_mapping
  // table.
  struct MappingKey {
    struct Hash {
      size_t operator()(const MappingKey& mapping) const {
        perfetto::base::Hasher hasher;
        hasher.UpdateAll(mapping.size, mapping.file_offset,
                         mapping.build_id_or_filename);
        return static_cast<size_t>(hasher.digest());
      }
    };

    explicit MappingKey(
        const tables::StackProfileMappingTable::ConstRowReference& mapping,
        StringTable& string_table);

    bool operator==(const MappingKey& other) const {
      return size == other.size && file_offset == other.file_offset &&
             build_id_or_filename == other.build_id_or_filename;
    }

    uint64_t size;
    uint64_t file_offset;
    int64_t build_id_or_filename;
  };

  // Keeps track of what debug information is available for a mapping.
  // TODO(carlscab): We could be a bit more "clever" here. Currently if there is
  // debug info for at least one frame we flag the mapping as having debug info.
  // We could use some heuristic instead, e.g. if x% for frames have the info
  // etc.
  struct DebugInfo {
    bool has_functions{false};
    bool has_filenames{false};
    bool has_line_numbers{false};
    bool has_inline_frames{false};
  };

  struct Mapping {
    explicit Mapping(
        const tables::StackProfileMappingTable::ConstRowReference& mapping,
        const StringPool& string_pool,
        StringTable& string_table);

    // Heuristic to determine if this maps to the main binary. Bigger scores
    // mean higher likelihood.
    int64_t ComputeMainBinaryScore() const;

    const uint64_t memory_start;
    const uint64_t memory_limit;
    const uint64_t file_offset;
    const int64_t filename;
    const int64_t build_id;

    const std::string filename_str;

    DebugInfo debug_info;
  };

  struct Function {
    struct Hash {
      size_t operator()(const Function& func) const {
        return static_cast<size_t>(perfetto::base::Hasher::Combine(
            func.name, func.system_name, func.filename));
      }
    };

    int64_t name;
    int64_t system_name;
    int64_t filename;

    bool operator==(const Function& other) const {
      return name == other.name && system_name == other.system_name &&
             filename == other.filename;
    }
  };

  // Aggregates samples with the same location_ids (i.e. stack) by computing the
  // sum of their values. This helps keep the generated profiles small as it
  // potentially removes a lot of duplication from having multiple samples with
  // the same stack.
  class SampleAggregator {
   public:
    bool AddSample(const protozero::PackedVarInt& location_ids,
                   const std::vector<int64_t>& values);

    void WriteTo(third_party::perftools::profiles::pbzero::Profile& profile);

   private:
    // Key holds the serialized value of the Sample::location_id proto field
    // (packed varint).
    using SerializedLocationId = std::vector<uint8_t>;
    struct Hasher {
      size_t operator()(const SerializedLocationId& data) const {
        base::Hasher hasher;
        hasher.Update(reinterpret_cast<const char*>(data.data()), data.size());
        return static_cast<size_t>(hasher.digest());
      }
    };
    base::FlatHashMap<SerializedLocationId, std::vector<int64_t>, Hasher>
        samples_;
  };

  const protozero::PackedVarInt& GetLocationIdsForCallsite(
      const CallsiteId& callsite_id,
      bool annotated);

  std::vector<Line> GetLinesForJitFrame(
      const tables::StackProfileFrameTable::ConstRowReference& frame,
      CallsiteAnnotation annotation,
      uint64_t mapping_id);

  std::vector<Line> GetLinesForSymbolSetId(
      std::optional<uint32_t> symbol_set_id,
      CallsiteAnnotation annotation,
      uint64_t mapping_id);

  std::vector<Line> GetLines(
      const tables::StackProfileFrameTable::ConstRowReference& frame,
      CallsiteAnnotation annotation,
      uint64_t mapping_id);

  int64_t GetNameForFrame(
      const tables::StackProfileFrameTable::ConstRowReference& frame,
      CallsiteAnnotation annotation);

  int64_t GetSystemNameForFrame(
      const tables::StackProfileFrameTable::ConstRowReference& frame);

  uint64_t WriteLocationIfNeeded(FrameId frame_id,
                                 CallsiteAnnotation annotation);
  uint64_t WriteFakeLocationIfNeeded(const std::string& name);

  uint64_t WriteFunctionIfNeeded(base::StringView name,
                                 StringPool::Id filename,
                                 CallsiteAnnotation annotation,
                                 uint64_t mapping_id);

  uint64_t WriteFunctionIfNeeded(
      const tables::SymbolTable::ConstRowReference& symbol,
      CallsiteAnnotation annotation,
      uint64_t mapping_id);

  uint64_t WriteFunctionIfNeeded(
      const tables::StackProfileFrameTable::ConstRowReference& frame,
      CallsiteAnnotation annotation,
      uint64_t mapping_id);

  uint64_t WriteFakeFunctionIfNeeded(int64_t name_id);

  uint64_t WriteMappingIfNeeded(
      const tables::StackProfileMappingTable::ConstRowReference& mapping);
  void WriteMappings();
  void WriteMapping(uint64_t mapping_id);
  void WriteFunctions();
  void WriteLocations();

  void WriteSampleTypes(const std::vector<ValueType>& sample_types);

  void Finalize();

  Mapping& GetMapping(uint64_t mapping_id) {
    return mappings_[static_cast<size_t>(mapping_id - 1)];
  }

  // Goes over the list of staged mappings and tries to determine which is the
  // most likely main binary.
  std::optional<uint64_t> GuessMainBinary() const;

  // Profile proto being serialized.
  protozero::HeapBuffered<third_party::perftools::profiles::pbzero::Profile>
      result_;

  const TraceProcessorContext& context_;
  StringTable string_table_;

  bool finalized_{false};
  AnnotatedCallsites annotations_;

  // Caches a (possibly annotated) CallsiteId (callstack) to the list of
  // locations emitted to the profile.
  struct MaybeAnnotatedCallsiteId {
    struct Hash {
      size_t operator()(const MaybeAnnotatedCallsiteId& id) const {
        return static_cast<size_t>(
            perfetto::base::Hasher::Combine(id.callsite_id.value, id.annotate));
      }
    };

    CallsiteId callsite_id;
    bool annotate;

    bool operator==(const MaybeAnnotatedCallsiteId& other) const {
      return callsite_id == other.callsite_id && annotate == other.annotate;
    }
  };
  std::unordered_map<MaybeAnnotatedCallsiteId,
                     protozero::PackedVarInt,
                     MaybeAnnotatedCallsiteId::Hash>
      cached_location_ids_;

  // Helpers to map TraceProcessor rows to already written Profile entities
  // (their ids).
  std::unordered_map<AnnotatedFrameId, uint64_t, AnnotatedFrameId::Hash>
      seen_locations_;
  std::unordered_map<AnnotatedFrameId, uint64_t, AnnotatedFrameId::Hash>
      seen_functions_;
  std::unordered_map<MappingId, uint64_t> seen_mappings_;
  std::unordered_map<int64_t, uint64_t> seen_fake_locations_;

  // Helpers to deduplicate entries. Map entity to its id. These also serve as a
  // staging area until written out to the profile proto during `Finalize`. Ids
  // are consecutive integers starting at 1. (Ids with value 0 are not allowed).
  // Ids are not unique across entities (i.e. there can be a mapping_id = 1 and
  // a function_id = 1)
  std::unordered_map<Location, uint64_t, Location::Hash> locations_;
  std::unordered_map<MappingKey, uint64_t, MappingKey::Hash> mapping_keys_;
  std::unordered_map<Function, uint64_t, Function::Hash> functions_;
  // Staging area for Mappings. mapping_id - 1 = index in the vector.
  std::vector<Mapping> mappings_;
  SampleAggregator samples_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_UTIL_PROFILE_BUILDER_H_

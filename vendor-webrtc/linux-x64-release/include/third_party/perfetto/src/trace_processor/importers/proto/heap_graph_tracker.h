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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_HEAP_GRAPH_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_HEAP_GRAPH_TRACKER_H_

#include <map>
#include <optional>
#include <set>
#include <utility>
#include <vector>

#include "perfetto/base/flat_set.h"
#include "perfetto/ext/base/string_view.h"

#include "protos/perfetto/trace/profiling/heap_graph.pbzero.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/profiler_tables_py.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;

struct NormalizedType {
  base::StringView name;
  bool is_static_class;
  size_t number_of_arrays;
};

struct PathFromRoot {
  static constexpr size_t kRoot = 0;
  struct Node {
    uint32_t depth = 0;
    // Invariant: parent_id < id of this node.
    size_t parent_id = 0;
    int64_t size = 0;
    int64_t count = 0;
    StringId class_name_id = {};
    std::map<StringId, size_t> children;
  };
  std::vector<Node> nodes{Node{}};
  std::set<tables::HeapGraphObjectTable::Id> visited;
};

std::optional<base::StringView> GetStaticClassTypeName(base::StringView type);
size_t NumberOfArrays(base::StringView type);
NormalizedType GetNormalizedType(base::StringView type);
base::StringView NormalizeTypeName(base::StringView type);
std::string DenormalizeTypeName(NormalizedType normalized,
                                base::StringView deobfuscated_type_name);

class HeapGraphTracker : public Destructible {
 public:
  struct SourceObject {
    // All ids in this are in the trace iid space, not in the trace processor
    // id space.
    uint64_t object_id = 0;
    uint64_t self_size = 0;
    uint64_t type_id = 0;
    protos::pbzero::HeapGraphObject::HeapType heap_type =
        protos::pbzero::HeapGraphObject::HEAP_TYPE_UNKNOWN;

    std::vector<uint64_t> field_name_ids;
    std::vector<uint64_t> referred_objects;

    // If this object is an instance of `libcore.util.NativeAllocationRegistry`,
    // this is the value of its `size` field.
    std::optional<int64_t> native_allocation_registry_size;
  };

  struct SourceRoot {
    protos::pbzero::HeapGraphRoot::Type root_type;
    std::vector<uint64_t> object_ids;
  };

  explicit HeapGraphTracker(TraceStorage* storage);

  static HeapGraphTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->heap_graph_tracker) {
      context->heap_graph_tracker.reset(
          new HeapGraphTracker(context->storage.get()));
    }
    return static_cast<HeapGraphTracker*>(context->heap_graph_tracker.get());
  }

  void AddRoot(uint32_t seq_id, UniquePid upid, int64_t ts, SourceRoot root);
  void AddObject(uint32_t seq_id, UniquePid upid, int64_t ts, SourceObject obj);
  void AddInternedType(uint32_t seq_id,
                       uint64_t intern_id,
                       StringId strid,
                       std::optional<uint64_t> location_id,
                       uint64_t object_size,
                       std::vector<uint64_t> field_name_ids,
                       uint64_t superclass_id,
                       uint64_t classloader_id,
                       bool no_fields,
                       protos::pbzero::HeapGraphType::Kind kind);
  void AddInternedFieldName(uint32_t seq_id,
                            uint64_t intern_id,
                            base::StringView str);
  void AddInternedLocationName(uint32_t seq_id,
                               uint64_t intern_id,
                               StringId str);
  void FinalizeProfile(uint32_t seq);
  void FinalizeAllProfiles();
  void SetPacketIndex(uint32_t seq_id, uint64_t index);

  ~HeapGraphTracker() override;

  const std::vector<tables::HeapGraphClassTable::RowNumber>* RowsForType(
      std::optional<StringId> package_name,
      StringId type_name) const {
    auto it = class_to_rows_.find(std::make_pair(package_name, type_name));
    if (it == class_to_rows_.end())
      return nullptr;
    return &it->second;
  }

  const std::vector<tables::HeapGraphReferenceTable::RowNumber>* RowsForField(
      StringId field_name) const {
    return field_to_rows_.Find(field_name);
  }

  std::unique_ptr<tables::ExperimentalFlamegraphTable> BuildFlamegraph(
      const int64_t current_ts,
      const UniquePid current_upid);

  uint64_t GetLastObjectId(uint32_t seq_id) {
    return GetOrCreateSequence(seq_id).last_object_id;
  }

  perfetto::protos::pbzero::HeapGraphObject::HeapType GetLastObjectHeapType(
      uint32_t seq_id) {
    return GetOrCreateSequence(seq_id).last_heap_type;
  }

 private:
  struct InternedField {
    StringId name;
    StringId type_name;
  };
  struct InternedType {
    StringId name;
    std::optional<uint64_t> location_id;
    uint64_t object_size;
    std::vector<uint64_t> field_name_ids;
    uint64_t superclass_id;
    bool no_fields;
    uint64_t classloader_id;
    protos::pbzero::HeapGraphType::Kind kind;
  };
  struct SequenceState {
    UniquePid current_upid = 0;
    int64_t current_ts = 0;
    uint64_t last_object_id = 0;
    protos::pbzero::HeapGraphObject::HeapType last_heap_type =
        protos::pbzero::HeapGraphObject::HEAP_TYPE_UNKNOWN;
    std::vector<SourceRoot> current_roots;

    // Note: the below maps are a mix of std::map and base::FlatHashMap because
    // of the incremental evolution of this code (i.e. when the code was written
    // FlatHashMap did not exist and pieces were migrated as they were found to
    // be performance problems).
    //
    // In the future, likely all of these should be base::FlatHashMap. This
    // was not done when the first use of base::FlatHashMap happened because
    // there are some subtle cases where base::FlatHashMap *regresses* perf and
    // there was not time for investigation.

    std::map<uint64_t, InternedType> interned_types;
    std::map<uint64_t, StringId> interned_location_names;
    base::FlatHashMap<uint64_t, tables::HeapGraphObjectTable::RowNumber>
        object_id_to_db_row;
    base::FlatHashMap<uint64_t, tables::HeapGraphClassTable::RowNumber>
        type_id_to_db_row;
    std::map<uint64_t, std::vector<tables::HeapGraphReferenceTable::RowNumber>>
        references_for_field_name_id;
    base::FlatHashMap<uint64_t, InternedField> interned_fields;
    std::map<tables::HeapGraphClassTable::Id,
             std::vector<tables::HeapGraphObjectTable::RowNumber>>
        deferred_reference_objects_for_type_;
    std::optional<uint64_t> prev_index;
    // For most objects, we need not store the size in the object's message
    // itself, because all instances of the type have the same type. In this
    // case, we defer setting self_size in the table until we process the class
    // message in FinalizeProfile.
    std::map<tables::HeapGraphClassTable::Id,
             std::vector<tables::HeapGraphObjectTable::RowNumber>>
        deferred_size_objects_for_type_;
    // Contains the value of the "size" field for each
    // "libcore.util.NativeAllocationRegistry" object.
    std::map<tables::HeapGraphObjectTable::Id, int64_t> nar_size_by_obj_id;
    bool truncated = false;
  };

  SequenceState& GetOrCreateSequence(uint32_t seq_id);
  tables::HeapGraphObjectTable::RowReference GetOrInsertObject(
      SequenceState* sequence_state,
      uint64_t object_id);
  tables::HeapGraphClassTable::RowReference GetOrInsertType(
      SequenceState* sequence_state,
      uint64_t type_id);
  bool SetPidAndTimestamp(SequenceState* seq, UniquePid upid, int64_t ts);
  void PopulateSuperClasses(const SequenceState& seq);
  InternedType* GetSuperClass(SequenceState* sequence_state,
                              const InternedType* current_type);
  bool IsTruncated(UniquePid upid, int64_t ts);
  StringId InternRootTypeString(protos::pbzero::HeapGraphRoot::Type);
  StringId InternTypeKindString(protos::pbzero::HeapGraphType::Kind);

  // Returns the object pointed to by `field` in `obj`.
  std::optional<tables::HeapGraphObjectTable::Id> GetReferenceByFieldName(
      tables::HeapGraphObjectTable::Id obj,
      StringId field);

  // Populates HeapGraphObject::native_size by walking the graph for
  // `seq`.
  //
  // This should be called only once (it is not idempotent) per seq, after the
  // all the other tables have been fully populated.
  void PopulateNativeSize(const SequenceState& seq);

  void GetChildren(tables::HeapGraphObjectTable::RowReference,
                   std::vector<tables::HeapGraphObjectTable::Id>&);
  void MarkRoot(tables::HeapGraphObjectTable::RowReference, StringId type);
  size_t RankRoot(StringId type);
  void UpdateShortestPaths(tables::HeapGraphObjectTable::RowReference row_ref);
  void FindPathFromRoot(tables::HeapGraphObjectTable::RowReference,
                        PathFromRoot* path);

  TraceStorage* const storage_;
  std::map<uint32_t, SequenceState> sequence_state_;

  std::map<std::pair<std::optional<StringId>, StringId>,
           std::vector<tables::HeapGraphClassTable::RowNumber>>
      class_to_rows_;
  base::FlatHashMap<StringId,
                    std::vector<tables::HeapGraphReferenceTable::RowNumber>>
      field_to_rows_;

  std::map<std::pair<std::optional<StringId>, StringId>, StringId>
      deobfuscation_mapping_;
  std::map<std::pair<UniquePid, int64_t>,
           std::set<tables::HeapGraphObjectTable::RowNumber>>
      roots_;
  std::set<std::pair<UniquePid, int64_t>> truncated_graphs_;

  StringId cleaner_thunk_str_id_;
  StringId referent_str_id_;
  StringId cleaner_thunk_this0_str_id_;
  StringId native_size_str_id_;
  StringId cleaner_next_str_id_;

  std::array<StringId, 15> root_type_string_ids_ = {};
  static_assert(protos::pbzero::HeapGraphRoot_Type_MIN == 0);
  static_assert(protos::pbzero::HeapGraphRoot_Type_MAX + 1 ==
                std::tuple_size<decltype(root_type_string_ids_)>{});

  std::array<StringId, 12> type_kind_string_ids_ = {};
  static_assert(protos::pbzero::HeapGraphType_Kind_MIN == 0);
  static_assert(protos::pbzero::HeapGraphType_Kind_MAX + 1 ==
                std::tuple_size<decltype(type_kind_string_ids_)>{});
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_HEAP_GRAPH_TRACKER_H_

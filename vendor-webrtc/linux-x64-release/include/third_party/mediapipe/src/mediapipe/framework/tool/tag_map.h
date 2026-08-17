// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_TAG_MAP_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_TAG_MAP_H_

#include <map>
#include <string>
#include <vector>

#include "absl/base/macros.h"
#include "absl/container/btree_map.h"
#include "absl/strings/string_view.h"
#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/port/canonical_errors.h"
#include "mediapipe/framework/port/core_proto_inc.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/framework/port/statusor.h"
#include "mediapipe/framework/tool/validate_name.h"

namespace mediapipe {
namespace tool {

// Holds the information needed for tag/index retrieval for stream and
// side packet lists.
class TagMap {
 public:
  // Struct to hold the initial id and the number of indexes per tag.
  struct TagData {
    TagData() : id(-1), count(0) {}
    TagData(CollectionItemId first_id, int the_count)
        : id(first_id), count(the_count) {}

    // The initial id for this tag.
    CollectionItemId id;
    // The number of entries with this tag.
    int count;
  };

  // Create a TagMap from a repeated string proto field of TAG:<index>:name.
  // This is the most common usage:
  // MP_ASSIGN_OR_RETURN(std::shared_ptr<TagMap> tag_map,
  //                  tool::TagMap::Create(node.input_streams()));
  static absl::StatusOr<std::shared_ptr<TagMap>> Create(
      const proto_ns::RepeatedPtrField<ProtoString>& tag_index_names) {
    std::shared_ptr<TagMap> output(new TagMap());
    MP_RETURN_IF_ERROR(output->Initialize(tag_index_names));
    return std::move(output);
  }

  // Create a TagMap from a TagAndNameInfo.
  // TODO: Migrate callers and delete this method.
  ABSL_DEPRECATED(
      "Use mediapipe::tool::TagMap::Create(tag_index_names) instead.")
  static absl::StatusOr<std::shared_ptr<TagMap>> Create(
      const TagAndNameInfo& info) {
    std::shared_ptr<TagMap> output(new TagMap());
    MP_RETURN_IF_ERROR(output->Initialize(info));
    return std::move(output);
  }

  // Returns a reference to the mapping from tag to tag data.
  const absl::btree_map<std::string, TagData>& Mapping() const {
    return mapping_;
  }

  // Returns the vector of names (indexed by CollectionItemId).
  const std::vector<std::string>& Names() const { return names_; }

  // Returns true if "this" and "other" use equivalent tags and indexes
  // (disregards stream/side packet names).
  bool SameAs(const TagMap& other) const;

  // Returns canonicalized strings describing the TagMap.
  proto_ns::RepeatedPtrField<ProtoString> CanonicalEntries() const;
  // Returns a string description for debug purposes.
  std::string DebugString() const;
  // Returns a shorter description for debug purposes (doesn't include
  // stream/side packet names).
  std::string ShortDebugString() const;

  // The following functions are directly utilized by collection.h see
  // that file for comments.
  bool HasTag(absl::string_view tag) const;
  int NumEntries() const { return num_entries_; }
  int NumEntries(absl::string_view tag) const;
  CollectionItemId GetId(absl::string_view tag, int index) const;
  std::set<std::string> GetTags() const;
  std::pair<std::string, int> TagAndIndexFromId(CollectionItemId id) const;
  CollectionItemId BeginId() const { return CollectionItemId(0); }
  CollectionItemId EndId() const { return CollectionItemId(num_entries_); }
  CollectionItemId BeginId(absl::string_view tag) const;
  CollectionItemId EndId(absl::string_view tag) const;

 private:
  // Use static factory function TagMap::Create().
  TagMap() {}

  // Initialize the TagMap.  Due to only having a factory function for
  // creation, there is no way for a user to have an uninitialized TagMap.
  absl::Status Initialize(
      const proto_ns::RepeatedPtrField<ProtoString>& tag_index_names);

  // Initialize from a TagAndNameInfo.
  ABSL_DEPRECATED("Use Initialize(tag_index_names) instead.")
  absl::Status Initialize(const TagAndNameInfo& info);

  // Initialize names_ using a map from tag to the names for that tag.
  void InitializeNames(
      const std::map<std::string, std::vector<std::string>>& tag_to_names);

  // The total number of entries under all tags.
  int num_entries_;
  // Mapping from tag to tag data.
  absl::btree_map<std::string, TagData> mapping_;
  // The names of the data (indexed by CollectionItemId).
  std::vector<std::string> names_;
};

// Equal TagData structs define equal id ranges.
inline bool operator==(const TagMap::TagData& d1, const TagMap::TagData& d2) {
  return d1.id == d2.id && d1.count == d2.count;
}

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_TAG_MAP_H_

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
//
// Defines functions for validating and parsing tags and stream names
// (and side packet names).

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_VALIDATE_NAME_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_VALIDATE_NAME_H_

#include <string>
#include <vector>

#include "absl/base/macros.h"
#include "absl/strings/string_view.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {
namespace internal {
inline constexpr int kMaxCollectionItemId = 10000;
}  // namespace internal

namespace tool {

struct ABSL_DEPRECATED(
    "Prefer using mediapipe::tool::TagMap instead, since this structure does "
    "not "
    "consider the TAG:INDEX:name notation.") TagAndNameInfo {
  // The tag names.  If this is empty then the collection will use
  // indexes.
  std::vector<std::string> tags;
  // Names of the streams or side packets.  If tags are used then
  // tags.size() and names.size() must match.
  std::vector<std::string> names;
};

// Create a TagAndNameInfo from a list of strings in the form TAG:name.
// The TAG (including colon) is optional, but the entire list must either
// always include tags or never include tags.
ABSL_DEPRECATED(
    "Prefer using mediapipe::tool::TagMap instead, since this method does not "
    "support the TAG:INDEX:name notation. You can use Create() to create the "
    "tag map, and then Names(), Mapping(), and other methods to access the "
    "tag, index and name information.")
absl::Status GetTagAndNameInfo(
    const proto_ns::RepeatedPtrField<ProtoString>& tags_and_names,
    TagAndNameInfo* info);

// Create the proto field names in the form TAG:name based on a
// TagAndNameInfo.
ABSL_DEPRECATED(
    "Prefer using mediapipe::tool::TagMap instead, since this method does not "
    "support the TAG:INDEX:name notation. You can use CanonicalEntries() to "
    "translate a tag map to a RepeatedPtrField of tag and names.")
absl::Status SetFromTagAndNameInfo(
    const TagAndNameInfo& info,
    proto_ns::RepeatedPtrField<ProtoString>* tags_and_names);

// The string is a valid name for an input stream, output stream,
// side packet, and input collection.  Names use only lower case letters,
// numbers, and underscores.
//
// The reason for this restriction is threefold.
//     (1) To enforce a consistent style in graph configs.
//     (2) To distinguish between "arguments" to calculators and
//         trainer/calculator names.
//     (3) Because input side packet names end up in model directory names,
//         where lower case naming is the norm.
absl::Status ValidateName(const std::string& name);
// The string is a valid tag name.  Tags use only upper case letters,
// numbers, and underscores.
absl::Status ValidateTag(const std::string& tag);

// Parse a "Tag and Name" string into a tag and a name.
// The format is an optional tag and colon, followed by a name.
// Example 1: "VIDEO:frames2" -> tag: "VIDEO", name: "frames2"
// Example 2: "video_frames_1" -> tag: "", name: "video_frames_1"
absl::Status ParseTagAndName(absl::string_view tag_and_name, std::string* tag,
                             std::string* name);

// Parse a generic TAG:index:name string.  The format is a tag, then an
// index, then a name.  The tag and index are optional.  If the index
// is included, then the tag must be included.  If no tag is used then
// index is set to -1 (and should be assigned by argument position).
// Examples:
//   "VIDEO:frames2"  -> tag: "VIDEO", index: 0,  name: "frames2"
//   "VIDEO:1:frames" -> tag: "VIDEO", index: 1,  name: "frames"
//   "raw_frames"     -> tag: "",      index: -1, name: "raw_frames"
absl::Status ParseTagIndexName(const std::string& tag_and_name,
                               std::string* tag, int* index, std::string* name);

// Parse a generic TAG:index string.  The format is a tag, then an index
// with both being optional.  If the tag is missing it is assumed to be
// "" and if the index is missing then it is assumed to be 0.  If the
// index is provided then a colon (':') must be used.
// Examples:
//   "VIDEO"   -> tag: "VIDEO", index: 0
//   "VIDEO:1" -> tag: "VIDEO", index: 1
//   ":2"      -> tag: "",      index: 2
//   ""        -> tag: "",      index: 0
absl::Status ParseTagIndex(const std::string& tag_and_index, std::string* tag,
                           int* index);

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_VALIDATE_NAME_H_

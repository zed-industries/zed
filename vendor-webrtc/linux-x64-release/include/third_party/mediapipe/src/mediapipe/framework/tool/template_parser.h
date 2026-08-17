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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_TEMPLATE_PARSER_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_TEMPLATE_PARSER_H_

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "absl/base/macros.h"
#include "mediapipe/framework/port/advanced_proto_inc.h"

namespace mediapipe {
namespace tool {

class TemplateParser {
 public:
  // A data structure for reporting source code locations.
  class ParseInfoTree;

  // For parsing mediapipe templates, use this class.
  class Parser {
   public:
    Parser();
    ~Parser();

    // Like TextFormat::Parse().
    bool Parse(proto_ns::io::ZeroCopyInputStream* input,
               proto_ns::Message* output);
    // Like TextFormat::ParseFromString().
    bool ParseFromString(const std::string& input, proto_ns::Message* output);
    // Like TextFormat::Merge().
    bool Merge(proto_ns::io::ZeroCopyInputStream* input,
               proto_ns::Message* output);
    // Like TextFormat::MergeFromString().
    bool MergeFromString(const std::string& input, proto_ns::Message* output);

    // Set where to report parse errors.  If NULL (the default), errors will
    // be printed to stderr.
    void RecordErrorsTo(proto_ns::io::ErrorCollector* error_collector) {
      error_collector_ = error_collector;
    }

    // Set how parser finds extensions.  If NULL (the default), the
    // parser will use the standard Reflection object associated with
    // the message being parsed.
    void SetFinder(proto_ns::TextFormat::Finder* finder) { finder_ = finder; }

    // Sets where location information about the parse will be written. If NULL
    // (the default), then no location will be written.
    void WriteLocationsTo(ParseInfoTree* tree) { parse_info_tree_ = tree; }

    // Normally parsing fails if, after parsing, output->IsInitialized()
    // returns false.  Call AllowPartialMessage(true) to skip this check.
    void AllowPartialMessage(bool allow) { allow_partial_ = allow; }

    // Allow field names to be matched case-insensitively.
    // This is not advisable if there are fields that only differ in case, or
    // if you want to enforce writing in the canonical form.
    // This is 'false' by default.
    void AllowCaseInsensitiveField(bool allow) {
      allow_case_insensitive_field_ = allow;
    }

    // Like TextFormat::ParseFieldValueFromString
    bool ParseFieldValueFromString(const std::string& input,
                                   const proto_ns::FieldDescriptor* field,
                                   proto_ns::Message* output);

    void AllowFieldNumber(bool allow) { allow_field_number_ = allow; }

   private:
    // A specialization of ParserImpl for parsing mediapipe template rules.
    class MediaPipeParserImpl;
    // The internal class used to parse proto text.
    class ParserImpl;

    // Like TextFormat::Merge().  The provided implementation is used
    // to do the parsing.
    bool MergeUsingImpl(proto_ns::io::ZeroCopyInputStream* input,
                        proto_ns::Message* output, ParserImpl* parser_impl);

    proto_ns::io::ErrorCollector* error_collector_;
    proto_ns::TextFormat::Finder* finder_;
    ParseInfoTree* parse_info_tree_;
    bool allow_partial_;
    bool allow_case_insensitive_field_;
    bool allow_unknown_field_;
    bool allow_unknown_enum_;
    bool allow_field_number_;
    bool allow_relaxed_whitespace_;
    bool allow_singular_overwrites_;
  };
};

// Data structure which is populated with the locations of each field
// value parsed from the text.
// Forked from
class TemplateParser::ParseInfoTree {
 public:
  typedef proto_ns::TextFormat::ParseLocation ParseLocation;
  typedef proto_ns::FieldDescriptor FieldDescriptor;
  ParseInfoTree();
  ParseInfoTree(const ParseInfoTree&) = delete;
  ParseInfoTree& operator=(const ParseInfoTree&) = delete;
  ~ParseInfoTree();

  // Returns the parse location for index-th value of the field in the parsed
  // text. If none exists, returns a location with line = -1. Index should be
  // -1 for not-repeated fields.
  ParseLocation GetLocation(const FieldDescriptor* field, int index) const;

  // Returns the parse info tree for the given field, which must be a message
  // type. The nested information tree is owned by the root tree and will be
  // deleted when it is deleted.
  ParseInfoTree* GetTreeForNested(const FieldDescriptor* field,
                                  int index) const;

  // Records the starting location of a single value for a field.
  void RecordLocation(const FieldDescriptor* field, ParseLocation location);

  // Create and records a nested tree for a nested message field.
  ParseInfoTree* CreateNested(const FieldDescriptor* field);

  // Return the proto path for the last index for a field.
  std::string GetLastPath(const FieldDescriptor* field);

  // Return the proto path for the current message.
  std::string GetPath();

 private:
  // Defines the map from the index-th field descriptor to its parse location.
  typedef std::map<const FieldDescriptor*, std::vector<ParseLocation>>
      LocationMap;

  // Defines the map from the index-th field descriptor to the nested parse
  // info tree.
  typedef std::map<const FieldDescriptor*,
                   std::vector<std::unique_ptr<ParseInfoTree>>>
      NestedMap;

  LocationMap locations_;
  NestedMap nested_;
  std::string path_;
};

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_TEMPLATE_PARSER_H_

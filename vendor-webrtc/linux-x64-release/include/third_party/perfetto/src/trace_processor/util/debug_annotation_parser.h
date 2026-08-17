/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_UTIL_DEBUG_ANNOTATION_PARSER_H_
#define SRC_TRACE_PROCESSOR_UTIL_DEBUG_ANNOTATION_PARSER_H_

#include <string>

#include "perfetto/base/status.h"
#include "perfetto/protozero/field.h"
#include "src/trace_processor/util/proto_to_args_parser.h"

#include "protos/perfetto/trace/track_event/debug_annotation.pbzero.h"

namespace perfetto::trace_processor::util {

// |DebugAnnotationParser| is responsible for parsing DebugAnnotation protos
// and turning it into key-value arg pairs.
// |DebugAnnotationParser| is a logical extension of |ProtoToArgsParser|:
// it uses |ProtoToArgsParser::Delegate| for writing the results and uses
// |ProtoToArgsParser| to parse arbitrary protos nested inside DebugAnnotation.
class DebugAnnotationParser {
 public:
  explicit DebugAnnotationParser(ProtoToArgsParser& proto_to_args_parser);

  base::Status Parse(protozero::ConstBytes data,
                     ProtoToArgsParser::Delegate& delegate);

 private:
  struct ParseResult {
    base::Status status;
    // True if parsing of the annotation added at least one entry to the
    // |delegate|.
    bool added_entry;
  };

  static base::Status ParseDebugAnnotationName(
      protos::pbzero::DebugAnnotation::Decoder& annotation,
      ProtoToArgsParser::Delegate& delegate,
      std::string& result);
  ParseResult ParseDebugAnnotationValue(
      protos::pbzero::DebugAnnotation::Decoder& annotation,
      ProtoToArgsParser::Delegate& delegate,
      const ProtoToArgsParser::Key& context_name);
  ParseResult ParseNestedValueArgs(protozero::ConstBytes nested_value,
                                   const ProtoToArgsParser::Key& context_name,
                                   ProtoToArgsParser::Delegate& delegate);

  ProtoToArgsParser& proto_to_args_parser_;
};

}  // namespace perfetto::trace_processor::util

#endif  // SRC_TRACE_PROCESSOR_UTIL_DEBUG_ANNOTATION_PARSER_H_

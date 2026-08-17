/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_TOOLS_FTRACE_PROTO_GEN_FTRACE_PROTO_GEN_H_
#define SRC_TOOLS_FTRACE_PROTO_GEN_FTRACE_PROTO_GEN_H_

#include <google/protobuf/descriptor.h>
#include <set>
#include <string>
#include <vector>

#include "src/tools/ftrace_proto_gen/proto_gen_utils.h"
#include "src/traced/probes/ftrace/format_parser/format_parser.h"

namespace perfetto {

std::vector<Proto::Field> ToProtoFields(const FtraceEvent& format);

std::string EventNameToProtoName(const std::string& group,
                                 const std::string& name);
std::string EventNameToProtoFieldName(const std::string& group,
                                      const std::string& name);

std::vector<FtraceEventName> ReadAllowList(const std::string& filename);
void GenerateFtraceEventProto(const std::vector<FtraceEventName>& raw_eventlist,
                              const std::set<std::string>& groups,
                              std::ostream* fout);
std::string SingleEventInfo(perfetto::Proto proto,
                            const std::string& group,
                            uint32_t proto_field_id);
void GenerateEventInfo(const std::vector<std::string>& events_info,
                       std::ostream* fout);
std::string ProtoHeader();

}  // namespace perfetto

#endif  // SRC_TOOLS_FTRACE_PROTO_GEN_FTRACE_PROTO_GEN_H_

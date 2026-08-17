/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_UTIL_PROFILER_UTIL_H_
#define SRC_TRACE_PROCESSOR_UTIL_PROFILER_UTIL_H_

#include <optional>
#include <string>

#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/storage/trace_storage.h"

#include "protos/perfetto/trace/profiling/deobfuscation.pbzero.h"

namespace perfetto::trace_processor {

std::string FullyQualifiedDeobfuscatedName(
    protos::pbzero::ObfuscatedClass::Decoder& cls,
    protos::pbzero::ObfuscatedMember::Decoder& member);

std::optional<std::string> PackageFromLocation(TraceStorage* storage,
                                               base::StringView location);

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_UTIL_PROFILER_UTIL_H_

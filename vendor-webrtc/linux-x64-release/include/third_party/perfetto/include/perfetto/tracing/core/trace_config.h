/*
 * Copyright (C) 2017 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACING_CORE_TRACE_CONFIG_H_
#define INCLUDE_PERFETTO_TRACING_CORE_TRACE_CONFIG_H_

// Creates the aliases in the ::perfetto namespace, doing things like:
// using ::perfetto::Foo = ::perfetto::protos::gen::Foo.
// See comments in forward_decls.h for the historical reasons of this
// indirection layer.
#include "perfetto/tracing/core/forward_decls.h"

#include "protos/perfetto/config/trace_config.gen.h"

namespace perfetto {

inline TraceConfig::TriggerConfig::TriggerMode GetTriggerMode(
    const TraceConfig& cfg) {
  auto mode = cfg.trigger_config().trigger_mode();
  if (cfg.trigger_config().use_clone_snapshot_if_available())
    mode = TraceConfig::TriggerConfig::CLONE_SNAPSHOT;
  return mode;
}

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_CORE_TRACE_CONFIG_H_

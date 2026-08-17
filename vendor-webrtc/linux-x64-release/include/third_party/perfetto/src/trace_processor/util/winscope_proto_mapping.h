/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_UTIL_WINSCOPE_PROTO_MAPPING_H_
#define SRC_TRACE_PROCESSOR_UTIL_WINSCOPE_PROTO_MAPPING_H_

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "src/trace_processor/db/table.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/android_tables_py.h"
#include "src/trace_processor/tables/winscope_tables_py.h"

namespace perfetto::trace_processor::util::winscope_proto_mapping {

inline base::StatusOr<const char* const> GetProtoName(
    const std::string& table_name) {
  if (table_name == tables::SurfaceFlingerLayerTable::Name()) {
    return ".perfetto.protos.LayerProto";
  }
  if (table_name == tables::SurfaceFlingerLayersSnapshotTable::Name()) {
    return ".perfetto.protos.LayersSnapshotProto";
  }
  if (table_name == tables::SurfaceFlingerTransactionsTable::Name()) {
    return ".perfetto.protos.TransactionTraceEntry";
  }
  if (table_name == tables::WindowManagerShellTransitionProtosTable::Name()) {
    return ".perfetto.protos.ShellTransition";
  }
  if (table_name == tables::InputMethodClientsTable::Name()) {
    return ".perfetto.protos.InputMethodClientsTraceProto";
  }
  if (table_name == tables::InputMethodManagerServiceTable::Name()) {
    return ".perfetto.protos.InputMethodManagerServiceTraceProto";
  }
  if (table_name == tables::InputMethodServiceTable::Name()) {
    return ".perfetto.protos.InputMethodServiceTraceProto";
  }
  if (table_name == tables::ViewCaptureTable::Name()) {
    return ".perfetto.protos.ViewCapture";
  }
  if (table_name == tables::ViewCaptureViewTable::Name()) {
    return ".perfetto.protos.ViewCapture.View";
  }
  if (table_name == tables::WindowManagerTable::Name()) {
    return ".perfetto.protos.WindowManagerTraceEntry";
  }
  if (table_name == tables::AndroidKeyEventsTable::Name()) {
    return ".perfetto.protos.AndroidKeyEvent";
  }
  if (table_name == tables::AndroidMotionEventsTable::Name()) {
    return ".perfetto.protos.AndroidMotionEvent";
  }
  if (table_name == tables::AndroidInputEventDispatchTable::Name()) {
    return ".perfetto.protos.AndroidWindowInputDispatchEvent";
  }
  return base::ErrStatus("%s table does not have proto descriptor.",
                         table_name.c_str());
}

inline std::optional<const std::vector<uint32_t>> GetAllowedFields(
    const std::string& table_name) {
  if (table_name == tables::SurfaceFlingerLayersSnapshotTable::Name()) {
    return std::vector<uint32_t>({1, 2, 4, 5, 6, 7, 8});
  }
  if (table_name == tables::ViewCaptureTable::Name()) {
    return std::vector<uint32_t>({1, 2});
  }
  return std::nullopt;
}

inline std::optional<const std::string> GetGroupIdColName(
    const std::string& table_name) {
  if (table_name == tables::WindowManagerShellTransitionProtosTable::Name()) {
    return "transition_id";
  }
  return std::nullopt;
}

inline std::optional<const Table*> GetInternedDataTable(
    const std::string& table_name,
    TraceStorage* storage) {
  if (table_name == tables::ViewCaptureTable::Name() ||
      table_name == tables::ViewCaptureViewTable::Name()) {
    return storage->mutable_viewcapture_interned_data_table();
  }
  return std::nullopt;
}

}  // namespace perfetto::trace_processor::util::winscope_proto_mapping

#endif  // SRC_TRACE_PROCESSOR_UTIL_WINSCOPE_PROTO_MAPPING_H_

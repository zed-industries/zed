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

#ifndef SRC_TRACED_PROBES_ANDROID_GAME_INTERVENTION_LIST_ANDROID_GAME_INTERVENTION_LIST_DATA_SOURCE_H_
#define SRC_TRACED_PROBES_ANDROID_GAME_INTERVENTION_LIST_ANDROID_GAME_INTERVENTION_LIST_DATA_SOURCE_H_

#include <memory>
#include <string>
#include <vector>

#include "perfetto/base/task_runner.h"
#include "perfetto/ext/base/pipe.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/tracing/core/basic_types.h"

#include "src/traced/probes/probes_data_source.h"

namespace perfetto {

// forward declaration of protos to be loaded
namespace protos {
namespace pbzero {
class AndroidGameInterventionList;
class AndroidGameInterventionList_GamePackageInfo;
}  // namespace pbzero
}  // namespace protos

class TraceWriter;

class AndroidGameInterventionListDataSource : public ProbesDataSource {
 public:
  static const ProbesDataSource::Descriptor descriptor;

  AndroidGameInterventionListDataSource(
      const DataSourceConfig& ds_config,
      TracingSessionID session_id,
      std::unique_ptr<TraceWriter> trace_writer);

  ~AndroidGameInterventionListDataSource() override;

  // ProbesDataSource implementation.
  void Start() override;
  void Flush(FlushRequestID, std::function<void()> callback) override;

  bool ParseAndroidGameInterventionListStream(
      protos::pbzero::AndroidGameInterventionList*
          android_game_intervention_list,
      const base::ScopedFstream& fs,
      const std::vector<std::string>& package_name_filter);

 private:
  bool ParseAndroidGameInterventionListLine(
      char* line,
      const std::vector<std::string>& package_name_filter,
      protos::pbzero::AndroidGameInterventionList* packet);

  std::vector<std::string> package_name_filter_;
  std::unique_ptr<TraceWriter> trace_writer_;
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_ANDROID_GAME_INTERVENTION_LIST_ANDROID_GAME_INTERVENTION_LIST_DATA_SOURCE_H_

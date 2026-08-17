/* Copyright 2022 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef MEDIAPIPE_TASKS_CC_CORE_TASK_API_FACTORY_H_
#define MEDIAPIPE_TASKS_CC_CORE_TASK_API_FACTORY_H_

#include <memory>
#include <optional>
#include <type_traits>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/match.h"
#include "absl/strings/str_cat.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/executor.h"
#include "mediapipe/framework/port/requires.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/tasks/cc/common.h"
#include "mediapipe/tasks/cc/core/base_task_api.h"
#include "mediapipe/tasks/cc/core/proto/base_options.pb.h"
#include "mediapipe/tasks/cc/core/proto/external_file.pb.h"
#include "mediapipe/tasks/cc/core/proto/inference_subgraph.pb.h"
#include "mediapipe/tasks/cc/core/task_runner.h"
#include "tensorflow/lite/core/api/op_resolver.h"

namespace mediapipe {
namespace tasks {
namespace core {

template <typename T>
using EnableIfBaseTaskApiSubclass =
    typename std::enable_if<std::is_base_of<BaseTaskApi, T>::value>::type*;

// Template creator for all subclasses of BaseTaskApi.
class TaskApiFactory {
 public:
  TaskApiFactory() = delete;

  template <typename T, typename Options,
            EnableIfBaseTaskApiSubclass<T> = nullptr>
  static absl::StatusOr<std::unique_ptr<T>> Create(
      CalculatorGraphConfig graph_config,
      std::unique_ptr<tflite::OpResolver> resolver,
      PacketsCallback packets_callback = nullptr,
      std::shared_ptr<Executor> default_executor = nullptr,
      std::optional<PacketMap> input_side_packets = std::nullopt,
      std::optional<ErrorFn> error_fn = std::nullopt) {
    bool found_task_subgraph = false;
    // This for-loop ensures there's only one subgraph besides
    // FlowLimiterCalculator.
    for (const auto& node : graph_config.node()) {
      if (node.calculator() == "FlowLimiterCalculator") {
        continue;
      }
      if (found_task_subgraph) {
        return CreateStatusWithPayload(
            absl::StatusCode::kInvalidArgument,
            "Task graph config should only contain one task subgraph node.",
            MediaPipeTasksStatus::kInvalidTaskGraphConfigError);
      } else {
        MP_RETURN_IF_ERROR(CheckHasValidOptions<Options>(node));
        found_task_subgraph = true;
      }
    }
    MP_ASSIGN_OR_RETURN(
        auto runner,
#if !MEDIAPIPE_DISABLE_GPU
        core::TaskRunner::Create(std::move(graph_config), std::move(resolver),
                                 std::move(packets_callback),
                                 std::move(default_executor),
                                 std::move(input_side_packets),
                                 /*resources=*/nullptr, std::move(error_fn)));
#else
        core::TaskRunner::Create(
            std::move(graph_config), std::move(resolver),
            std::move(packets_callback), std::move(default_executor),
            std::move(input_side_packets), std::move(error_fn)));
#endif
    return std::make_unique<T>(std::move(runner));
  }

  template <typename Options>
  static absl::Status CheckHasValidOptions(
      const CalculatorGraphConfig::Node& node) {
    if constexpr (mediapipe::Requires<Options>(
                      [](auto&& o) -> decltype(o.ext) {})) {
      if (node.options().HasExtension(Options::ext)) {
        return absl::OkStatus();
      }
    } else {
#ifndef MEDIAPIPE_PROTO_LITE
      for (const auto& option : node.node_options()) {
        if (absl::StrContains(option.type_url(),
                              Options::descriptor()->full_name())) {
          return absl::OkStatus();
        }
      }
#else   // MEDIAPIPE_PROTO_LITE
      // Skip the check for proto lite, as Options::descriptor() is unavailable.
      return absl::OkStatus();
#endif  // MEDIAPIPE_PROTO_LITE
    }
    return CreateStatusWithPayload(
        absl::StatusCode::kInvalidArgument,
        absl::StrCat(node.calculator(),
                     " is missing the required task options field."),
        MediaPipeTasksStatus::kInvalidTaskGraphConfigError);
  }
};

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_TASK_API_FACTORY_H_

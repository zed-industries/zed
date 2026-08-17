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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_TEMPLATE_EXPANDER_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_TEMPLATE_EXPANDER_H_

#include <vector>

#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/tool/calculator_graph_template.pb.h"

namespace mediapipe {

namespace tool {

// The interpreter for mediapipe template rules.
class TemplateExpander {
 public:
  // Creates an interpreter to expand templates.
  TemplateExpander();

  // Applies the rules specified in a CalculatorGraphTemplate to a
  // CalculatorGraphConfig.  Each rule references a nested field-value or
  // message and defines zero or more replacement values for it.
  absl::Status ExpandTemplates(const TemplateDict& args,
                               const CalculatorGraphTemplate& templ,
                               CalculatorGraphConfig* output);

 private:
  // List of errors found in template parameters.
  std::vector<absl::Status> errors_;
};

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_TEMPLATE_EXPANDER_H_

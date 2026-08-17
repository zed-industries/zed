/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_INTENT_REPR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_INTENT_REPR_H_

#include <string>

#include "absl/status/statusor.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl

namespace tflite::task::text::clu {

// Takes care of adding and parsing Domain prefixes
class IntentRepr {
 public:
  const std::string& Domain() const { return domain_; }
  const std::string& Name() const { return name_; }
  std::string FullName() const;
  static absl::StatusOr<IntentRepr> CreateFromFullName(const absl::string_view);
  static IntentRepr Create(absl::string_view name, absl::string_view domain,
                           const bool share_across_domains);

 private:
  std::string domain_;
  std::string name_;
};

}  // namespace tflite::task::text::clu

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_INTENT_REPR_H_

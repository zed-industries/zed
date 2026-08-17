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
#ifndef TENSORFLOW_MODELS_SEQ_FLOW_LITE_TFLITE_OPS_DENYLIST_H_
#define TENSORFLOW_MODELS_SEQ_FLOW_LITE_TFLITE_OPS_DENYLIST_H_

#include <string>
#include <vector>

#include "absl/container/flat_hash_set.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "flatbuffers/flexbuffers.h"  // flatbuffer
#include "tensorflow/lite/context.h"

namespace seq_flow_lite {
namespace ops {
namespace custom {
namespace denylist {

/*
 * A framework for writing ops that generate prediction vectors using a
 * denylist.
 *
 * Input is defined by the specific implementation.
 *
 * Attributes:
 *   denylist:           string[n]
 *     Terms in the denylist.
 *   denylist_category:  int[n]
 *     Category for each term in the denylist.  Each category must be in
 *     [0, categories).
 *   categories:          int[]
 *     Total number of categories.
 *   negative_categories: int[]
 *     Total number of negative categories.
 *
 * Output:
 *   tensor[0]: Category indicators for each message, float[..., categories]
 *
 */

class DenylistOp {
 public:
  explicit DenylistOp(const flexbuffers::Map& custom_options)
      : categories_(custom_options["categories"].AsInt32()),
        negative_categories_(custom_options["negative_categories"].AsInt32()) {
    if (categories_ <= 0) {
      AddError(absl::StrFormat("categories (%d) <= 0", categories_));
    }

    if (negative_categories_ <= 0) {
      AddError(absl::StrFormat("negative_categories (%d) <= 0",
                               negative_categories_));
    }

    if (negative_categories_ >= categories_) {
      AddError(absl::StrFormat("negative_categories (%d) >= categories (%d)",
                               negative_categories_, categories_));
    }
  }

  virtual ~DenylistOp() {}

  int categories() const { return categories_; }
  int negative_categories() const { return negative_categories_; }

  virtual TfLiteStatus InitializeInput(TfLiteContext* context,
                                       TfLiteNode* node) = 0;
  virtual TfLiteStatus GetCategories(
      TfLiteContext* context, int i,
      absl::flat_hash_set<int>& categories) const = 0;
  virtual void FinalizeInput() = 0;

  // Returns the input shape.  TfLiteIntArray is owned by the object.
  virtual TfLiteIntArray* GetInputShape(TfLiteContext* context,
                                        TfLiteNode* node) = 0;

  TfLiteStatus CheckErrors(TfLiteContext* context) {
    if (!errors_.empty()) {
      for (const std::string& error : errors_) {
        context->ReportError(context, "%s", error.c_str());
      }
      return kTfLiteError;
    }
    return kTfLiteOk;
  }

 protected:
  void AddError(absl::string_view error) { errors_.emplace_back(error); }

 private:
  int categories_;
  int negative_categories_;
  std::vector<std::string> errors_;
};

// Individual ops should define an Init() function that returns a
// DenylistOp.

void Free(TfLiteContext* context, void* buffer);

TfLiteStatus Resize(TfLiteContext* context, TfLiteNode* node);

TfLiteStatus Eval(TfLiteContext* context, TfLiteNode* node);

}  // namespace denylist
}  // namespace custom
}  // namespace ops
}  // namespace seq_flow_lite

#endif  // TENSORFLOW_MODELS_SEQ_FLOW_LITE_TFLITE_OPS_DENYLIST_H_

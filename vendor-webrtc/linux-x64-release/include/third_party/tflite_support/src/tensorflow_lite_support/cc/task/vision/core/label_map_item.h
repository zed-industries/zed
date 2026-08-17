/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_CORE_LABEL_MAP_ITEM_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_CORE_LABEL_MAP_ITEM_H_

#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"  // from @com_google_absl
#include "absl/container/flat_hash_set.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/port/statusor.h"

namespace tflite {
namespace task {
namespace vision {

// Structure mapping a numerical class index output to a Knowledge Graph entity
// ID or any other string label representing this class. Optionally it is
// possible to specify an additional display name (in a given language) which is
// typically used for display purposes.
struct LabelMapItem {
  // E.g. name = "/m/02xwb"
  std::string name;
  // E.g. display_name = "Fruit"
  std::string display_name;
  // Optional list of children (e.g. subcategories) used to represent a
  // hierarchy.
  std::vector<std::string> child_name;
};

// Builds a label map from labels and (optional) display names file contents,
// both expected to contain one label per line. Those are typically obtained
// from TFLite Model Metadata TENSOR_AXIS_LABELS or TENSOR_VALUE_LABELS
// associated files.
// Returns an error e.g. if there's a mismatch between the number of labels and
// display names.
tflite::support::StatusOr<std::vector<LabelMapItem>> BuildLabelMapFromFiles(
    absl::string_view labels_file, absl::string_view display_names_file);

// A class that represents a hierarchy of labels as specified in a label map.
//
// For example, it is useful to determine if one label is a descendant of
// another label or not. This can be used to implement labels pruning based on
// hierarchy, e.g. if both "fruit" and "banana" have been inferred by a given
// classifier model prune "fruit" from the final results as "banana" is a more
// fine-grained descendant.
class LabelHierarchy {
 public:
  LabelHierarchy() = default;

  // Initializes the hierarchy of labels from a given label map vector. Returns
  // an error status in case of failure, typically if the input label map does
  // not contain any hierarchical relations between labels.
  absl::Status InitializeFromLabelMap(
      std::vector<LabelMapItem> label_map_items);

  // Returns true if `descendant_name` is a descendant of `ancestor_name` in the
  // hierarchy of labels. Invalid names, i.e. names which do not exist in the
  // label map used at initialization time, are ignored.
  bool HaveAncestorDescendantRelationship(
      const std::string& ancestor_name,
      const std::string& descendant_name) const;

 private:
  // Retrieve and return all parent names, if any, for the input label name.
  absl::flat_hash_set<std::string> GetParents(const std::string& name) const;

  // Retrieve all ancestor names, if any, for the input label name.
  void GetAncestors(const std::string& name,
                    absl::flat_hash_set<std::string>* ancestors) const;

  // Label name (key) to parent names (value) direct mapping.
  absl::flat_hash_map<std::string, absl::flat_hash_set<std::string>>
      parents_map_;
};

}  // namespace vision
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_CORE_LABEL_MAP_ITEM_H_

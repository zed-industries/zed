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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_SLOT_REPR_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_SLOT_REPR_H_

#include <string>
#include <utility>
#include <vector>

#include "absl/status/status.h"  // from @com_google_absl
#include "absl/status/statusor.h"  // from @com_google_absl
#include "absl/strings/str_cat.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/task/text/clu_lib/constants.h"

namespace tflite::task::text::clu {

// Takes care of adding IOB and Domain prefixes and parsing them later
class SlotRepr {
 public:
  // I-tag for the slot
  std::string ITag() const { return absl::StrCat(kSlotITagPrefix, FullName()); }

  // B-tag for the slot
  std::string BTag() const { return absl::StrCat(kSlotBTagPrefix, FullName()); }

  // The domain name
  const std::string& Domain() const { return domain_; }

  // The slot name
  const std::string& Name() const { return name_; }

  // The domain and slot name
  std::string FullName() const;

  bool operator==(const SlotRepr& other) const;

  // I-tag for the given slot name
  static std::string ITag(absl::string_view name) {
    return absl::StrCat(kSlotITagPrefix, name);
  }

  // B-tag for the given slot name
  static std::string BTag(absl::string_view name) {
    return absl::StrCat(kSlotBTagPrefix, name);
  }

  static bool IsI(const absl::string_view repr);

  static bool IsB(const absl::string_view repr);

  static bool IsO(const absl::string_view repr);

  // Factory function given the BIO string representation
  static absl::StatusOr<SlotRepr> CreateFromIob(const absl::string_view);

  // Factory
  static SlotRepr Create(absl::string_view name, absl::string_view domain = "",
                         const bool share_across_domains = true);

  // Splits the full_name into domain and slot name.
  static absl::StatusOr<std::tuple<absl::string_view, absl::string_view>>
  SplitDomainAndName(const absl::string_view full_name);

 private:
  std::string domain_;
  std::string name_;
};

std::ostream& operator<<(std::ostream& os, const SlotRepr& slot_repr);

// A non-proto message version of SlotMention, with some additional fields.
// TODO(amiraf, pengwang): Consider using a proto instead.
struct SlotMentionStruct {
  SlotRepr repr;
  int start;
  int exclusive_end;
  float confidence;
};

// A span over character or tokens.
struct Span {
  int start;
  int exclusive_end;

  Span(const int start_arg, const int exclusive_end_arg)
      : start(start_arg), exclusive_end(exclusive_end_arg) {}

  explicit Span(const std::pair<int, int>& start_end)
      : start(start_end.first), exclusive_end(start_end.second) {}

  inline int Middle() const { return (start + exclusive_end) / 2; }
};

// Produces chunks/spans for the slots from the IOB tags with confidence. The
// result is ordered by `start` in ascending order.
absl::StatusOr<std::vector<SlotMentionStruct>> DecodeSlotChunks(
    const absl::Span<const absl::string_view> tag_names,
    const absl::Span<const float> tag_probs,
    const absl::Span<const std::pair<int, int>> token_alignments);

// Resolve inconsistent IOB tags.
// Three cases: (1) O I-y (2) B-x I-y (3) I-x I-y. In either case, change the
// second to 'B-y'.
absl::Status ResolveInconsistentIobTagSeq(std::vector<std::string>* tag_names);

}  // namespace tflite::task::text::clu

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_TEXT_CLU_LIB_SLOT_REPR_H_

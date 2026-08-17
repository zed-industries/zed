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

#ifndef MEDIAPIPE_CALCULATORS_UTIL_ASSOCIATION_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_ASSOCIATION_CALCULATOR_H_

#include <memory>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/memory/memory.h"
#include "mediapipe/calculators/util/association_calculator.pb.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/port/canonical_errors.h"
#include "mediapipe/framework/port/rectangle.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/util/rectangle_util.h"

namespace mediapipe {

// AssocationCalculator<T> accepts multiple inputs of vectors of type T that can
// be converted to Rectangle_f. The output is a vector of type T that contains
// elements from the input vectors that don't overlap with each other. When
// two elements overlap, the element that comes in from a later input stream
// is kept in the output. This association operation is useful for multiple
// instance inference pipelines in MediaPipe.
// If an input stream is tagged with "PREV" tag, IDs of overlapping elements
// from "PREV" input stream are propagated to the output. Elements in the "PREV"
// input stream that don't overlap with other elements are not added to the
// output. This stream is designed to take detections from previous timestamp,
// e.g. output of PreviousLoopbackCalculator to provide temporal association.
// See AssociationDetectionCalculator and AssociationNormRectCalculator for
// example uses.
template <typename T>
class AssociationCalculator : public CalculatorBase {
 public:
  static absl::Status GetContract(CalculatorContract* cc) {
    // Atmost one input stream can be tagged with "PREV".
    RET_CHECK_LE(cc->Inputs().NumEntries("PREV"), 1);

    if (cc->Inputs().HasTag("PREV")) {
      RET_CHECK_GE(cc->Inputs().NumEntries(), 2);
    }

    for (CollectionItemId id = cc->Inputs().BeginId();
         id < cc->Inputs().EndId(); ++id) {
      cc->Inputs().Get(id).Set<std::vector<T>>();
    }

    cc->Outputs().Index(0).Set<std::vector<T>>();

    return absl::OkStatus();
  }

  absl::Status Open(CalculatorContext* cc) override {
    cc->SetOffset(TimestampDiff(0));

    has_prev_input_stream_ = cc->Inputs().HasTag("PREV");
    if (has_prev_input_stream_) {
      prev_input_stream_id_ = cc->Inputs().GetId("PREV", 0);
    }
    options_ = cc->Options<::mediapipe::AssociationCalculatorOptions>();
    ABSL_CHECK_GE(options_.min_similarity_threshold(), 0);

    return absl::OkStatus();
  }

  absl::Status Process(CalculatorContext* cc) override {
    auto get_non_overlapping_elements = GetNonOverlappingElements(cc);
    if (!get_non_overlapping_elements.ok()) {
      return get_non_overlapping_elements.status();
    }
    std::list<T> result = get_non_overlapping_elements.value();

    if (has_prev_input_stream_ &&
        !cc->Inputs().Get(prev_input_stream_id_).IsEmpty()) {
      // Processed all regular input streams. Now compare the result list
      // elements with those in the PREV input stream, and propagate IDs from
      // PREV input stream as appropriate.
      const std::vector<T>& prev_input_vec =
          cc->Inputs()
              .Get(prev_input_stream_id_)
              .template Get<std::vector<T>>();

      MP_RETURN_IF_ERROR(
          PropagateIdsFromPreviousToCurrent(prev_input_vec, &result));
    }

    auto output = absl::make_unique<std::vector<T>>();
    for (auto it = result.begin(); it != result.end(); ++it) {
      output->push_back(*it);
    }
    cc->Outputs().Index(0).Add(output.release(), cc->InputTimestamp());

    return absl::OkStatus();
  }

 protected:
  ::mediapipe::AssociationCalculatorOptions options_;

  bool has_prev_input_stream_;
  CollectionItemId prev_input_stream_id_;

  virtual absl::StatusOr<Rectangle_f> GetRectangle(const T& input) {
    return absl::OkStatus();
  }

  virtual std::pair<bool, int> GetId(const T& input) { return {false, -1}; }

  virtual void SetId(T* input, int id) {}

 private:
  // Get a list of non-overlapping elements from all input streams, with
  // increasing order of priority based on input stream index.
  absl::StatusOr<std::list<T>> GetNonOverlappingElements(
      CalculatorContext* cc) {
    std::list<T> result;

    // Initialize result with the first non-empty input vector.
    CollectionItemId non_empty_id = cc->Inputs().BeginId();
    for (CollectionItemId id = cc->Inputs().BeginId();
         id < cc->Inputs().EndId(); ++id) {
      if (id == prev_input_stream_id_ || cc->Inputs().Get(id).IsEmpty()) {
        continue;
      }
      const std::vector<T>& input_vec =
          cc->Inputs().Get(id).Get<std::vector<T>>();
      if (!input_vec.empty()) {
        non_empty_id = id;
        result.push_back(input_vec[0]);
        for (int j = 1; j < input_vec.size(); ++j) {
          MP_RETURN_IF_ERROR(AddElementToList(input_vec[j], &result));
        }
        break;
      }
    }

    // Compare remaining input vectors with the non-empty result vector,
    // remove lower-priority overlapping elements from the result vector and
    // had corresponding higher-priority elements as necessary.
    for (CollectionItemId id = non_empty_id + 1; id < cc->Inputs().EndId();
         ++id) {
      if (id == prev_input_stream_id_ || cc->Inputs().Get(id).IsEmpty()) {
        continue;
      }
      const std::vector<T>& input_vec =
          cc->Inputs().Get(id).Get<std::vector<T>>();

      for (int vi = 0; vi < input_vec.size(); ++vi) {
        MP_RETURN_IF_ERROR(AddElementToList(input_vec[vi], &result));
      }
    }

    return result;
  }

  absl::Status AddElementToList(T element, std::list<T>* current) {
    // Compare this element with elements of the input collection. If this
    // element has high overlap with elements of the collection, remove
    // those elements from the collection and add this element.
    MP_ASSIGN_OR_RETURN(auto cur_rect, GetRectangle(element));

    bool change_id = false;
    int new_elem_id = -1;

    for (auto uit = current->begin(); uit != current->end();) {
      MP_ASSIGN_OR_RETURN(auto prev_rect, GetRectangle(*uit));
      if (CalculateIou(cur_rect, prev_rect) >
          options_.min_similarity_threshold()) {
        std::pair<bool, int> prev_id = GetId(*uit);
        // If prev_id.first is false when some element doesn't have an ID,
        // change_id and new_elem_id will not be updated.
        if (prev_id.first) {
          change_id = prev_id.first;
          new_elem_id = prev_id.second;
        }
        uit = current->erase(uit);
      } else {
        ++uit;
      }
    }

    if (change_id) {
      SetId(&element, new_elem_id);
    }
    current->push_back(element);

    return absl::OkStatus();
  }

  // Compare elements of the current list with elements in from the collection
  // of elements from the previous input stream, and propagate IDs from the
  // previous input stream as appropriate.
  absl::Status PropagateIdsFromPreviousToCurrent(
      const std::vector<T>& prev_input_vec, std::list<T>* current) {
    for (auto vit = current->begin(); vit != current->end(); ++vit) {
      auto get_cur_rectangle = GetRectangle(*vit);
      if (!get_cur_rectangle.ok()) {
        return get_cur_rectangle.status();
      }
      const Rectangle_f& cur_rect = get_cur_rectangle.value();

      bool change_id = false;
      int id_for_vi = -1;

      for (int ui = 0; ui < prev_input_vec.size(); ++ui) {
        auto get_prev_rectangle = GetRectangle(prev_input_vec[ui]);
        if (!get_prev_rectangle.ok()) {
          return get_prev_rectangle.status();
        }
        const Rectangle_f& prev_rect = get_prev_rectangle.value();

        if (CalculateIou(cur_rect, prev_rect) >
            options_.min_similarity_threshold()) {
          std::pair<bool, int> prev_id = GetId(prev_input_vec[ui]);
          // If prev_id.first is false when some element doesn't have an ID,
          // change_id and id_for_vi will not be updated.
          if (prev_id.first) {
            change_id = prev_id.first;
            id_for_vi = prev_id.second;
          }
        }
      }

      if (change_id) {
        T element = *vit;
        SetId(&element, id_for_vi);
        *vit = element;
      }
    }
    return absl::OkStatus();
  }
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_UTIL_ASSOCIATION_CALCULATOR_H_

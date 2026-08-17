#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_SPLIT_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_SPLIT_H_

#include <initializer_list>
#include <iterator>
#include <type_traits>
#include <utility>
#include <vector>

#include "mediapipe/calculators/core/split_vector_calculator.pb.h"
#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/formats/body_rig.pb.h"
#include "mediapipe/framework/formats/classification.pb.h"
#include "mediapipe/framework/formats/detection.pb.h"
#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/formats/matrix.h"
#include "mediapipe/framework/formats/rect.pb.h"
#include "mediapipe/framework/formats/tensor.h"
#include "tensorflow/lite/c/common.h"

namespace mediapipe::api2::builder {

namespace stream_split_internal {

// Helper function that adds a node to a graph, that is capable of splitting a
// specific type (T).
template <class T>
mediapipe::api2::builder::GenericNode& AddSplitVectorNode(
    mediapipe::api2::builder::Graph& graph) {
  if constexpr (std::is_same_v<T, std::vector<TfLiteTensor>>) {
    return graph.AddNode("SplitTfLiteTensorVectorCalculator");
  } else if constexpr (std::is_same_v<T, std::vector<mediapipe::Tensor>>) {
    return graph.AddNode("SplitTensorVectorCalculator");
  } else if constexpr (std::is_same_v<T, std::vector<uint64_t>>) {
    return graph.AddNode("SplitUint64tVectorCalculator");
  } else if constexpr (std::is_same_v<
                           T, std::vector<mediapipe::NormalizedLandmark>>) {
    return graph.AddNode("SplitLandmarkVectorCalculator");
  } else if constexpr (std::is_same_v<
                           T, std::vector<mediapipe::NormalizedLandmarkList>>) {
    return graph.AddNode("SplitNormalizedLandmarkListVectorCalculator");
  } else if constexpr (std::is_same_v<T,
                                      std::vector<mediapipe::NormalizedRect>>) {
    return graph.AddNode("SplitNormalizedRectVectorCalculator");
  } else if constexpr (std::is_same_v<T, std::vector<Matrix>>) {
    return graph.AddNode("SplitMatrixVectorCalculator");
  } else if constexpr (std::is_same_v<T, std::vector<mediapipe::Detection>>) {
    return graph.AddNode("SplitDetectionVectorCalculator");
  } else if constexpr (std::is_same_v<
                           T, std::vector<mediapipe::ClassificationList>>) {
    return graph.AddNode("SplitClassificationListVectorCalculator");
  } else if constexpr (std::is_same_v<T, mediapipe::NormalizedLandmarkList>) {
    return graph.AddNode("SplitNormalizedLandmarkListCalculator");
  } else if constexpr (std::is_same_v<T, mediapipe::LandmarkList>) {
    return graph.AddNode("SplitLandmarkListCalculator");
  } else if constexpr (std::is_same_v<T, mediapipe::JointList>) {
    return graph.AddNode("SplitJointListCalculator");
  } else {
    static_assert(dependent_false<T>::value,
                  "Split node is not available for the specified type.");
  }
}

template <typename T, bool kIteratorContainsRanges = false>
struct split_result_item {
  using type = typename T::value_type;
};
template <>
struct split_result_item<mediapipe::NormalizedLandmarkList,
                         /*kIteratorContainsRanges=*/false> {
  using type = mediapipe::NormalizedLandmark;
};
template <>
struct split_result_item<mediapipe::LandmarkList,
                         /*kIteratorContainsRanges=*/false> {
  using type = mediapipe::Landmark;
};

template <typename T>
struct split_result_item<T, /*kIteratorContainsRanges=*/true> {
  using type = std::vector<typename T::value_type>;
};
template <>
struct split_result_item<mediapipe::NormalizedLandmarkList,
                         /*kIteratorContainsRanges=*/true> {
  using type = mediapipe::NormalizedLandmarkList;
};
template <>
struct split_result_item<mediapipe::LandmarkList,
                         /*kIteratorContainsRanges=*/true> {
  using type = mediapipe::LandmarkList;
};

template <typename CollectionT, typename I>
auto Split(Stream<CollectionT> items, I begin, I end,
           mediapipe::api2::builder::Graph& graph) {
  auto& splitter = AddSplitVectorNode<CollectionT>(graph);
  items.ConnectTo(splitter.In(""));

  constexpr bool kIteratorContainsRanges =
      std::is_same_v<typename std::iterator_traits<I>::value_type,
                     std::pair<int, int>>;
  using R =
      typename split_result_item<CollectionT, kIteratorContainsRanges>::type;
  auto& splitter_opts =
      splitter.template GetOptions<mediapipe::SplitVectorCalculatorOptions>();
  if constexpr (!kIteratorContainsRanges) {
    splitter_opts.set_element_only(true);
  }
  std::vector<Stream<R>> result;
  int output = 0;
  for (auto it = begin; it != end; ++it) {
    auto* range = splitter_opts.add_ranges();
    if constexpr (kIteratorContainsRanges) {
      range->set_begin(it->first);
      range->set_end(it->second);
    } else {
      range->set_begin(*it);
      range->set_end(*it + 1);
    }
    result.push_back(splitter.Out("")[output++].template Cast<R>());
  }
  return result;
}

template <typename CollectionT, typename I>
Stream<CollectionT> SplitAndCombine(Stream<CollectionT> items, I begin, I end,
                                    mediapipe::api2::builder::Graph& graph) {
  auto& splitter = AddSplitVectorNode<CollectionT>(graph);
  items.ConnectTo(splitter.In(""));

  constexpr bool kIteratorContainsRanges =
      std::is_same_v<typename std::iterator_traits<I>::value_type,
                     std::pair<int, int>>;

  auto& splitter_opts =
      splitter.template GetOptions<mediapipe::SplitVectorCalculatorOptions>();
  splitter_opts.set_combine_outputs(true);

  for (auto it = begin; it != end; ++it) {
    auto* range = splitter_opts.add_ranges();
    if constexpr (kIteratorContainsRanges) {
      range->set_begin(it->first);
      range->set_end(it->second);
    } else {
      range->set_begin(*it);
      range->set_end(*it + 1);
    }
  }
  return splitter.Out("").template Cast<CollectionT>();
}

}  // namespace stream_split_internal

// Splits stream containing a collection based on passed @indices into a vector
// of streams where each steam repesents individual item of a collection.
//
// Example:
// ```
//
//   Graph graph;
//   std::vector<int> indices = {0, 1, 2, 3};
//
//   Stream<std::vector<Detection>> detections = ...;
//   std::vector<Stream<Detection>> detections_split =
//       Split(detections, indices, graph);
//
//   Stream<NormalizedLandmarkList> landmarks = ...;
//   std::vector<Stream<NormalizedLandmark>> landmarks_split =
//       Split(landmarks, indices, graph);
//
// ```
template <typename CollectionT, typename I>
auto Split(Stream<CollectionT> items, const I& indices,
           mediapipe::api2::builder::Graph& graph) {
  return stream_split_internal::Split(items, indices.begin(), indices.end(),
                                      graph);
}
// Splits stream containing a collection based on passed @indices into a vector
// of streams where each steam repesents individual item of a collection.
//
// Example:
// ```
//
//   Graph graph;
//   std::vector<int> indices = {0, 1, 2, 3};
//
//   Stream<std::vector<Detection>> detections = ...;
//   std::vector<Stream<Detection>> detections_split =
//       Split(detections, indices, graph);
//
//   Stream<NormalizedLandmarkList> landmarks = ...;
//   std::vector<Stream<NormalizedLandmark>> landmarks_split =
//       Split(landmarks, indices, graph);
//
// ```
template <typename CollectionT>
auto Split(Stream<CollectionT> items, std::initializer_list<int> indices,
           mediapipe::api2::builder::Graph& graph) {
  return stream_split_internal::Split(items, indices.begin(), indices.end(),
                                      graph);
}

// Splits stream containing a collection into a sub ranges, each represented as
// a stream containing same collection type.
//
// Example:
// ```
//
//   Graph graph;
//   std::vector<std::pair<int, int>> ranges = {{0, 3}, {7, 10}};
//
//   Stream<std::vector<Detection>> detections = ...;
//   std::vector<Stream<std::vector<Detection>>> detections_split =
//       SplitToRanges(detections, ranges, graph);
//
//   Stream<NormalizedLandmarkList> landmarks = ...;
//   std::vector<Stream<NormalizedLandmarkList>> landmarks_split =
//       SplitToRanges(landmarks, ranges, graph);
//
// ```
template <typename CollectionT, typename RangeT>
auto SplitToRanges(Stream<CollectionT> items, const RangeT& ranges,
                   mediapipe::api2::builder::Graph& graph) {
  return stream_split_internal::Split(items, ranges.begin(), ranges.end(),
                                      graph);
}

// Splits stream containing a collection into a sub ranges, each represented as
// a stream containing same collection type.
//
// Example:
// ```
//
//   Graph graph;
//   std::vector<std::pair<int, int>> ranges = {{0, 3}, {7, 10}};
//
//   Stream<std::vector<Detection>> detections = ...;
//   std::vector<Stream<std::vector<Detection>>> detections_split =
//       SplitToRanges(detections, ranges, graph);
//
//   Stream<NormalizedLandmarkList> landmarks = ...;
//   std::vector<Stream<NormalizedLandmarkList>> landmarks_split =
//       SplitToRanges(landmarks, ranges, graph);
//
// ```
template <typename CollectionT>
auto SplitToRanges(Stream<CollectionT> items,
                   std::initializer_list<std::pair<int, int>> ranges,
                   mediapipe::api2::builder::Graph& graph) {
  return stream_split_internal::Split(items, ranges.begin(), ranges.end(),
                                      graph);
}

// Splits stream containing a collection into a sub ranges and combines them
// into a stream containing same collection type.
//
// Example:
// ```
//
//   Graph graph;
//   std::vector<std::pair<int, int>> ranges = {{0, 3}, {7, 10}};
//
//   Stream<std::vector<Detection>> detections = ...;
//   Stream<std::vector<Detection>> detections_split_and_combined =
//       SplitAndCombine(detections, ranges, graph);
//
//   Stream<NormalizedLandmarkList> landmarks = ...;
//   Stream<NormalizedLandmarkList> landmarks_split_and_combined =
//       SplitAndCombine(landmarks, ranges, graph);
//
// ```
template <typename CollectionT, typename RangeT>
Stream<CollectionT> SplitAndCombine(Stream<CollectionT> items,
                                    const RangeT& ranges,
                                    mediapipe::api2::builder::Graph& graph) {
  return stream_split_internal::SplitAndCombine(items, ranges.begin(),
                                                ranges.end(), graph);
}

// Splits stream containing a collection into a sub ranges and combines them
// into a stream containing same collection type.
//
// Example:
// ```
//
//   Graph graph;
//
//   Stream<std::vector<Detection>> detections = ...;
//   Stream<std::vector<Detection>> detections_split_and_combined =
//       SplitAndCombine(detections, {{0, 3}, {7, 10}}, graph);
//
//   Stream<NormalizedLandmarkList> landmarks = ...;
//   Stream<NormalizedLandmarkList> landmarks_split_and_combined =
//       SplitAndCombine(landmarks, {{0, 3}, {7, 10}}, graph);
//
// ```
template <typename CollectionT>
Stream<CollectionT> SplitAndCombine(
    Stream<CollectionT> items,
    std::initializer_list<std::pair<int, int>> ranges,
    mediapipe::api2::builder::Graph& graph) {
  return stream_split_internal::SplitAndCombine(items, ranges.begin(),
                                                ranges.end(), graph);
}

// Splits stream containing a collection into individual items and combines them
// into a stream containing same collection type.
//
// Example:
// ```
//
//   Graph graph;
//
//   Stream<std::vector<Detection>> detections = ...;
//   Stream<std::vector<Detection>> detections_split_and_combined =
//       SplitAndCombine(detections, {0, 7, 10}, graph);
//
//   Stream<NormalizedLandmarkList> landmarks = ...;
//   Stream<NormalizedLandmarkList> landmarks_split_and_combined =
//       SplitAndCombine(landmarks, {0, 7, 10}, graph);
//
// ```
template <typename CollectionT>
Stream<CollectionT> SplitAndCombine(Stream<CollectionT> items,
                                    std::initializer_list<int> ranges,
                                    mediapipe::api2::builder::Graph& graph) {
  return stream_split_internal::SplitAndCombine(items, ranges.begin(),
                                                ranges.end(), graph);
}

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_SPLIT_H_

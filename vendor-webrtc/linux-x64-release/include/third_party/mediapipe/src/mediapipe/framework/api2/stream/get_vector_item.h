#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_GET_VECTOR_ITEM_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_GET_VECTOR_ITEM_H_

#include <type_traits>
#include <vector>

#include "mediapipe/calculators/core/get_vector_item_calculator.h"
#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/formats/classification.pb.h"
#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/formats/rect.pb.h"
#include "tensorflow/lite/c/common.h"

namespace mediapipe::api2::builder {

namespace internal_get_vector_item {

// Helper function that adds a node to a graph, that is capable of getting item
// from a vector of type (T).
template <class T>
mediapipe::api2::builder::GenericNode& AddGetVectorItemNode(
    mediapipe::api2::builder::Graph& graph) {
  if constexpr (std::is_same_v<T, mediapipe::NormalizedLandmarkList>) {
    return graph.AddNode("GetNormalizedLandmarkListVectorItemCalculator");
  } else if constexpr (std::is_same_v<T, mediapipe::LandmarkList>) {
    return graph.AddNode("GetLandmarkListVectorItemCalculator");
  } else if constexpr (std::is_same_v<T, mediapipe::ClassificationList>) {
    return graph.AddNode("GetClassificationListVectorItemCalculator");
  } else if constexpr (std::is_same_v<T, mediapipe::NormalizedRect>) {
    return graph.AddNode("GetNormalizedRectVectorItemCalculator");
  } else if constexpr (std::is_same_v<T, mediapipe::Rect>) {
    return graph.AddNode("GetRectVectorItemCalculator");
  } else {
    static_assert(
        dependent_false<T>::value,
        "Get vector item node is not available for the specified type.");
  }
}

}  // namespace internal_get_vector_item

// Gets item from the vector.
//
// Example:
// ```
//
//   Graph graph;
//
//   Stream<std::vector<LandmarkList>> multi_landmarks = ...;
//   Stream<LandmarkList> landmarks =
//       GetItem(multi_landmarks, 0, graph);
//
// ```
template <typename T>
Stream<T> GetItem(Stream<std::vector<T>> items, Stream<int> idx,
                  mediapipe::api2::builder::Graph& graph) {
  auto& getter = internal_get_vector_item::AddGetVectorItemNode<T>(graph);
  items.ConnectTo(getter.In("VECTOR"));
  idx.ConnectTo(getter.In("INDEX"));
  return getter.Out("ITEM").template Cast<T>();
}

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_GET_VECTOR_ITEM_H_

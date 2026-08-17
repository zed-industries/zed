#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_CONCATENATE_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_CONCATENATE_H_

#include <vector>

#include "mediapipe/calculators/core/concatenate_vector_calculator.pb.h"
#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/formats/body_rig.pb.h"
#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/formats/tensor.h"

namespace mediapipe::api2::builder {

namespace internal_stream_concatenate {

// Helper function that adds a node to a graph, that is capable of concatenating
// a specific type (T).
template <class T>
GenericNode& AddConcatenateVectorNode(Graph& graph) {
  if constexpr (std::is_same_v<T, mediapipe::LandmarkList>) {
    return graph.AddNode("ConcatenateLandmarkListCalculator");
  } else if constexpr (std::is_same_v<T, mediapipe::JointList>) {
    return graph.AddNode("ConcatenateJointListCalculator");
  } else if constexpr (std::is_same_v<T, std::vector<Tensor>>) {
    return graph.AddNode("ConcatenateTensorVectorCalculator");
  } else {
    static_assert(dependent_false<T>::value,
                  "Concatenate node is not available for the specified type.");
  }
}

template <typename StreamsT,
          typename PayloadT = typename StreamsT::value_type::PayloadT>
Stream<PayloadT> Concatenate(StreamsT& streams,
                             const bool only_emit_if_all_present,
                             Graph& graph) {
  auto& concatenator = AddConcatenateVectorNode<PayloadT>(graph);
  for (int i = 0; i < streams.size(); ++i) {
    streams[i].ConnectTo(concatenator.In("")[i]);
  }

  auto& concatenator_opts =
      concatenator
          .template GetOptions<mediapipe::ConcatenateVectorCalculatorOptions>();
  concatenator_opts.set_only_emit_if_all_present(only_emit_if_all_present);

  return concatenator.Out("").template Cast<PayloadT>();
}

}  // namespace internal_stream_concatenate

template <typename StreamsT,
          typename PayloadT = typename StreamsT::value_type::PayloadT>
Stream<PayloadT> Concatenate(StreamsT& streams, Graph& graph) {
  return internal_stream_concatenate::Concatenate(
      streams, /*only_emit_if_all_present=*/false, graph);
}

template <typename StreamsT,
          typename PayloadT = typename StreamsT::value_type::PayloadT>
Stream<PayloadT> ConcatenateIfAllPresent(StreamsT& streams, Graph& graph) {
  return internal_stream_concatenate::Concatenate(
      streams, /*only_emit_if_all_present=*/true, graph);
}

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_CONCATENATE_H_

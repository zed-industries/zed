#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_LOOPBACK_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_LOOPBACK_H_

#include <functional>
#include <utility>

#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/api2/port.h"

namespace mediapipe::api2::builder {

// Returns a pair of two values:
// - A stream with loopback data. Such stream, for each new packet in @tick
//   stream, provides a packet previously calculated within the graph.
// - A function to define/set loopback data producing stream.
//   NOTE:
//     * function must be called and only once, otherwise graph validation will
//       fail.
//     * calling function after graph is destroyed results in undefined behavior
//
// The function wraps `PreviousLoopbackCalculator` into a convenience function
// and allows graph input to be processed together with some previous output.
//
// -------
//
// Example:
//
// ```
//
//   Graph graph;
//   Stream<...> tick = ...; // E.g. main input can surve as a tick.
//   auto [prev_data, set_loopback_fn] = GetLoopbackData<int>(tick, graph);
//   ...
//   Stream<int> data = ...;
//   set_loopback_fn(data);
//
// ```
template <class DataT, class TickT>
std::pair<Stream<DataT>, std::function<void(Stream<DataT>)>> GetLoopbackData(
    Stream<TickT> tick, mediapipe::api2::builder::Graph& graph) {
  auto& prev = graph.AddNode("PreviousLoopbackCalculator");
  tick.ConnectTo(prev.In("MAIN"));
  return {prev.Out("PREV_LOOP").template Cast<DataT>(),
          [prev_ptr = &prev](Stream<DataT> data) {
            // TODO: input stream info must be specified, but
            // builder api doesn't support it at the moment. As a workaround,
            // input stream info is added by GraphBuilder as a graph building
            // post processing step.
            data.ConnectTo(prev_ptr->In("LOOP"));
          }};
}

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_LOOPBACK_H_

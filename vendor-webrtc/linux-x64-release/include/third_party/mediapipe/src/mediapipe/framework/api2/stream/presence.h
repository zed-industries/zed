#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_PRESENCE_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_PRESENCE_H_

#include "mediapipe/framework/api2/builder.h"

namespace mediapipe::api2::builder {

// Updates @graph to emit a stream containing `bool` packets, where each packet
// indicates whether @stream has a packet with corresponding timestamp or not.
template <typename T>
Stream<bool> IsPresent(Stream<T> stream, Graph& graph) {
  auto& presence_node = graph.AddNode("PacketPresenceCalculator");
  stream.ConnectTo(presence_node.In("PACKET"));
  return presence_node.Out("PRESENCE").Cast<bool>();
}

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_PRESENCE_H_

#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_MERGE_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_MERGE_H_

#include "mediapipe/framework/api2/builder.h"

namespace mediapipe::api2::builder {

// Updates @graph to choose @a stream if it's available (not empty stream at
// specific timestamp) or @b stream otherwise.
template <typename T>
Stream<T> Merge(Stream<T> a, Stream<T> b, Graph& graph) {
  auto& merge_node = graph.AddNode("MergeCalculator");
  a.ConnectTo(merge_node.In("")[0]);
  b.ConnectTo(merge_node.In("")[1]);
  return merge_node.Out("").template Cast<T>();
}

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_MERGE_H_

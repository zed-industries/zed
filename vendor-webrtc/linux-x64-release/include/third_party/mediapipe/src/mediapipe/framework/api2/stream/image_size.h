#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_IMAGE_SIZE_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_IMAGE_SIZE_H_

#include <utility>

#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/formats/image.h"
#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/gpu/gpu_buffer.h"

namespace mediapipe::api2::builder {

// Updates graph to calculate image size and returns corresponding stream.
//
// @image image represented as ImageFrame/Image/GpuBuffer.
// @graph graph to update.
template <typename ImageT>
Stream<std::pair<int, int>> GetImageSize(
    Stream<ImageT> image, mediapipe::api2::builder::Graph& graph) {
  auto& img_props_node = graph.AddNode("ImagePropertiesCalculator");
  if constexpr (std::is_same_v<ImageT, ImageFrame> ||
                std::is_same_v<ImageT, mediapipe::Image>) {
    image.ConnectTo(img_props_node.In("IMAGE"));
  } else if constexpr (std::is_same_v<ImageT, GpuBuffer>) {
    image.ConnectTo(img_props_node.In("IMAGE_GPU"));
  } else {
    static_assert(dependent_false<ImageT>::value, "Type not supported.");
  }
  return img_props_node.Out("SIZE").Cast<std::pair<int, int>>();
}

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_IMAGE_SIZE_H_

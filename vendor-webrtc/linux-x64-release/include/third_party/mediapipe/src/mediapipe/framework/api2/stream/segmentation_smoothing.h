#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_SEGMENTATION_SMOOTHING_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_SEGMENTATION_SMOOTHING_H_

#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/formats/image.h"

namespace mediapipe::api2::builder {

// Updates @graph to smooth @mask by mixing @mask and @previous_mask based on an
// uncertantity probability estimate calculated per each @mask pixel multiplied
// by @combine_with_previous_ratio.
Stream<Image> SmoothSegmentationMask(Stream<Image> mask,
                                     Stream<Image> previous_mask,
                                     float combine_with_previous_ratio,
                                     Graph& graph);

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_SEGMENTATION_SMOOTHING_H_

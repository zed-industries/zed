#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_LANDMARKS_PROJECTION_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_LANDMARKS_PROJECTION_H_

#include <array>

#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/formats/landmark.pb.h"

namespace mediapipe::api2::builder {

// Updates @graph to project predicted @landmarks back to the original @image
// based on @projection_matrix
//
// @landmarks - landmarks (NormalizedLandmarkList) stream, output from the model
// @projection_matrix - matrix that stores the preprocessing information
// @graph - mediapipe graph to update.
Stream<mediapipe::NormalizedLandmarkList> ProjectLandmarks(
    Stream<mediapipe::NormalizedLandmarkList> landmarks,
    Stream<std::array<float, 16>> projection_matrix, Graph& graph);

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_LANDMARKS_PROJECTION_H_

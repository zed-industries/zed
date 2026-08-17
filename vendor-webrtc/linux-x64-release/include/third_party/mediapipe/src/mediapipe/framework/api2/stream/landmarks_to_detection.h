#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_LANDMARKS_TO_DETECTION_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_LANDMARKS_TO_DETECTION_H_

#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/formats/detection.pb.h"
#include "mediapipe/framework/formats/landmark.pb.h"

namespace mediapipe::api2::builder {

// Updates @graph to convert @landmarks to a detection.
Stream<mediapipe::Detection> ConvertLandmarksToDetection(
    Stream<mediapipe::NormalizedLandmarkList> landmarks, Graph& graph);

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_LANDMARKS_TO_DETECTION_H_

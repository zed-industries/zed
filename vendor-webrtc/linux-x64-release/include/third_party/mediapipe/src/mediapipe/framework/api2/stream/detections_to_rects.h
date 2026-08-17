#ifndef MEDIAPIPE_FRAMEWORK_API2_STREAM_DETECTIONS_TO_RECTS_H_
#define MEDIAPIPE_FRAMEWORK_API2_STREAM_DETECTIONS_TO_RECTS_H_

#include <utility>
#include <vector>

#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/formats/detection.pb.h"
#include "mediapipe/framework/formats/rect.pb.h"

namespace mediapipe::api2::builder {

// Updates @graph to convert @detection into a `NormalizedRect` according to
// passed parameters.
Stream<mediapipe::NormalizedRect> ConvertAlignmentPointsDetectionToRect(
    Stream<mediapipe::Detection> detection,
    Stream<std::pair<int, int>> image_size, int start_keypoint_index,
    int end_keypoint_index, float target_angle,
    mediapipe::api2::builder::Graph& graph);

// Updates @graph to convert first detection from @detections into a
// `NormalizedRect` according to passed parameters.
Stream<mediapipe::NormalizedRect> ConvertAlignmentPointsDetectionsToRect(
    Stream<std::vector<mediapipe::Detection>> detections,
    Stream<std::pair<int, int>> image_size, int start_keypoint_index,
    int end_keypoint_index, float target_angle,
    mediapipe::api2::builder::Graph& graph);

// Updates @graph to convert @detection into a `NormalizedRect` according to
// passed parameters.
Stream<mediapipe::NormalizedRect> ConvertDetectionToRect(
    Stream<mediapipe::Detection> detections,
    Stream<std::pair<int, int>> image_size, int start_keypoint_index,
    int end_keypoint_index, float target_angle,
    mediapipe::api2::builder::Graph& graph);

// Updates @graph to convert @detections into a stream holding vector of
// `NormalizedRect` according to passed parameters.
Stream<std::vector<mediapipe::NormalizedRect>> ConvertDetectionsToRects(
    Stream<std::vector<mediapipe::Detection>> detections,
    Stream<std::pair<int, int>> image_size, int start_keypoint_index,
    int end_keypoint_index, float target_angle,
    mediapipe::api2::builder::Graph& graph);

// Updates @graph to convert @detections into a stream holding vector of
// `NormalizedRect` according to passed parameters and using keypoints.
Stream<mediapipe::NormalizedRect> ConvertDetectionsToRectUsingKeypoints(
    Stream<std::vector<mediapipe::Detection>> detections,
    Stream<std::pair<int, int>> image_size, int start_keypoint_index,
    int end_keypoint_index, float target_angle,
    mediapipe::api2::builder::Graph& graph);

}  // namespace mediapipe::api2::builder

#endif  // MEDIAPIPE_FRAMEWORK_API2_STREAM_DETECTIONS_TO_RECTS_H_

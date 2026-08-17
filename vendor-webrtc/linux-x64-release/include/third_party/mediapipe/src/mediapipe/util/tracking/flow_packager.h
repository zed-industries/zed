// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_UTIL_TRACKING_FLOW_PACKAGER_H_
#define MEDIAPIPE_UTIL_TRACKING_FLOW_PACKAGER_H_

#include <cstdint>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "mediapipe/util/tracking/flow_packager.pb.h"
#include "mediapipe/util/tracking/motion_estimation.pb.h"
#include "mediapipe/util/tracking/region_flow.pb.h"

namespace mediapipe {

// Usage (output):
// FlowPackager flow_packager((FlowPackagerOptions()));
//
// // Input: Feature lists and optional camera motion
// vector<RegionFlowFeatureList> input_features;      // Externally supplied.
// vector<CameraMotion> input_motions;
//
// const int num_frames = input_features.size();
//
// // Can encode to either TrackingContainerFormat or use protos.
// TrackingContainerFormat container;
// TrackingContainerProto proto;
//
// for (int f = 0; f < num_frames; ++f) {
//   // Obtain tracking data.
//   TrackingData track_data;
//   flow_packager.PackFlow(input_features[f],
//                          input_motions[f],
//                          &track_data);
//
//   // Encode tracking data.
//   BinaryTrackingData binary_data;
//   flow_packager.EncodeTrackingData(track_data,
//                                    &binary_data);
//
//   // Add to either container format or proto.
//   // Container:
//   TrackingContainer* track_data_encoded = container.add_track_data();
//   flow_packager.BinaryTrackingDataToContainer(binary_data,
//                                               track_data_encoded);
//   // Proto:
//   proto.add_track_data()->CopyFrom(binary_data);
// }
//
// // Write meta and term containers.
// flow_packager.FinalizeTrackingContainerFormat(&container);
// flow_packager.FinalizeTrackingProto(&proto);
//
// // Convert to binary string to stream out.
// std::string output;
// flow_packager.TrackingContainerFormatToBinary(container, &output);
// // OR:
// proto.SerializeToString(&output);

// Usage (input):
// std::string input;
// FlowPackager flow_packager((FlowPackagerOptions()));
// TrackingContainerFormat container;
// flow_packager.TrackingContainerFormatFromBinary(input, &container);
//
// // Obtain Track data.
// vector<TrackingData> tracking_data;    // To be used by tracking.
// for (const auto& track_data_encoded : container.track_data()) {
//   tracking_data.push_back(TrackingData());
//   flow_packager.DecodeTrackingData(track_data_encoded,
//                                    &tracking_data.back());
// }
//
// // Use tracking_data with Tracker.

class CameraMotion;
class RegionFlowFeatureList;

class FlowPackager {
 public:
  explicit FlowPackager(const FlowPackagerOptions& options);
  FlowPackager(const FlowPackager&) = delete;
  FlowPackager& operator=(const FlowPackager&) = delete;

  void PackFlow(const RegionFlowFeatureList& feature_list,
                const CameraMotion* camera_motion,  // optional.
                TrackingData* tracking_data) const;

  // Converts TrackingData to condensed binary representation.
  void EncodeTrackingData(const TrackingData& tracking_data,
                          BinaryTrackingData* binary_data) const;

  void DecodeTrackingData(const BinaryTrackingData& data,
                          TrackingData* tracking_data) const;

  void BinaryTrackingDataToContainer(const BinaryTrackingData& binary_data,
                                     TrackingContainer* container) const;

  void BinaryTrackingDataFromContainer(const TrackingContainer& container,
                                       BinaryTrackingData* binary_data) const;

  void DecodeMetaData(const TrackingContainer& data, MetaData* meta_ata) const;

  // Fills in meta (first container) and termination data (last container).
  // Optionally, pass timestamps for each frame.
  void FinalizeTrackingContainerFormat(
      std::vector<uint32_t>* timestamps,  // optional, can be null.
      TrackingContainerFormat* container_fromat);
  void FinalizeTrackingContainerProto(
      std::vector<uint32_t>* timestamps,  // optional, can be null.
      TrackingContainerProto* proto);

  // Fast encode to binary representation.
  void TrackingContainerFormatToBinary(
      const TrackingContainerFormat& container_format, std::string* binary);

  // Fast decode from binary representation.
  void TrackingContainerFormatFromBinary(
      const std::string& binary, TrackingContainerFormat* container_format);

  // Checks whether tracking data can be encoded in high profile mode without
  // duplicating any features. This occurs if the horizonal distance between two
  // features is less than 64.
  bool CompatibleForEncodeWithoutDuplication(
      const TrackingData& tracking_data) const;

  // Helper function for test. Sorts according to scaled, integer based
  // lexicographical ordering.
  // Declared public for access by test.
  void SortRegionFlowFeatureList(float scale_x, float scale_y,
                                 RegionFlowFeatureList* feature_list) const;

  // Removes binary encoded container from string and parses it to container.
  // Returns header string of the parsed container. Useful for random seek.
  std::string SplitContainerFromString(absl::string_view* binary_data,
                                       TrackingContainer* container);

 private:
  // Sets meta data for a set
  void InitializeMetaData(int num_frames, const std::vector<uint32_t>& msecs,
                          const std::vector<int>& data_sizes,
                          MetaData* meta_data) const;

  // Serializes container to binary string and adds it to binary_data.
  void AddContainerToString(const TrackingContainer& container,
                            std::string* binary_data);

 private:
  FlowPackagerOptions options_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TRACKING_FLOW_PACKAGER_H_

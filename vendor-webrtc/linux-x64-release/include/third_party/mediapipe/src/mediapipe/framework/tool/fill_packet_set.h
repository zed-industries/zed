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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_FILL_PACKET_SET_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_FILL_PACKET_SET_H_

#include <map>
#include <memory>
#include <string>

#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port/statusor.h"

namespace mediapipe {
namespace tool {

// Finds the packet names from input_side_packet_types's TagMap, looks
// them up in |input_side_packets| and creates a PacketSet.  An error
// is returned if any packets fail the type check.  If
// missing_packet_count_ptr is not null, the number of missing packets
// is returned in *missing_packet_count_ptr.  Otherwise, an error is
// returned if any packets are missing.
absl::StatusOr<std::unique_ptr<PacketSet>> FillPacketSet(
    const PacketTypeSet& input_side_packet_types,
    const std::map<std::string, Packet>& input_side_packets,
    int* missing_packet_count_ptr);

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_FILL_PACKET_SET_H_

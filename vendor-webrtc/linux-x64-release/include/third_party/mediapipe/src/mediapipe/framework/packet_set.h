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

#ifndef MEDIAPIPE_FRAMEWORK_PACKET_SET_H_
#define MEDIAPIPE_FRAMEWORK_PACKET_SET_H_

#include "mediapipe/framework/collection.h"
#include "mediapipe/framework/packet.h"

namespace mediapipe {

// A PacketSet is used to hold a collection of Packets accessed either
// by index or by tag name.
typedef internal::Collection<Packet> PacketSet;

// A similar construct for output side packets.
class OutputSidePacket;
typedef internal::Collection<OutputSidePacket,
                             internal::CollectionStorage::kStorePointer>
    OutputSidePacketSet;

// Similar constructs for input and output streams.
// TODO: Remove InputStreamSet and OutputStreamSet.
class InputStream;
typedef internal::Collection<InputStream*> InputStreamSet;
class OutputStream;
typedef internal::Collection<OutputStream*> OutputStreamSet;

// Similar constructs for input and output stream shards.
// TODO: Rename to InputStreamSet and OutputStreamSet.
class InputStreamShard;
typedef internal::Collection<InputStreamShard> InputStreamShardSet;
class OutputStreamShard;
typedef internal::Collection<OutputStreamShard> OutputStreamShardSet;

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PACKET_SET_H_

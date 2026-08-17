/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_SIMULCAST_DESCRIPTION_H_
#define PC_SIMULCAST_DESCRIPTION_H_

#include <stddef.h>

#include <string>
#include <vector>

#include "absl/strings/string_view.h"

namespace webrtc {

// Describes a Simulcast Layer.
// Each simulcast layer has a rid as the identifier and a paused flag.
// See also: https://tools.ietf.org/html/draft-ietf-mmusic-rid-15 for
// an explanation about rids.
struct SimulcastLayer final {
  SimulcastLayer(absl::string_view rid, bool is_paused);

  SimulcastLayer(const SimulcastLayer& other) = default;
  SimulcastLayer& operator=(const SimulcastLayer& other) = default;
  bool operator==(const SimulcastLayer& other) const;

  std::string rid;
  bool is_paused;
};

// Describes a list of Simulcast layers.
// Simulcast layers are specified in order of preference.
// Each layer can have a list of alternatives (in order of preference).
// https://tools.ietf.org/html/draft-ietf-mmusic-sdp-simulcast-13#section-5.1
// Example Usage:
//   To populate a list that specifies the following:
//     1. Layer 1 or Layer 2
//     2. Layer 3
//     3. Layer 4 or Layer 5
//   Use the following code:
//     SimulcastLayerList list;
//     list.AddLayerWithAlternatives(
//            {SimulcastLayer("1", false), SimulcastLayer("2", false});
//     list.AddLayer("3");
//     list.AddLayerWithAlternatives(
//            {SimulcastLayer("4", false), SimulcastLayer("5", false});
class SimulcastLayerList final {
 public:
  // Type definitions required by a container.
  typedef size_t size_type;
  typedef std::vector<SimulcastLayer> value_type;
  typedef std::vector<std::vector<SimulcastLayer>>::const_iterator
      const_iterator;

  // Use to add a layer when there will be no alternatives.
  void AddLayer(const SimulcastLayer& layer);

  // Use to add a list of alternatives.
  // The alternatives should be specified in order of preference.
  void AddLayerWithAlternatives(const std::vector<SimulcastLayer>& layers);

  // Read-only access to the contents.
  // Note: This object does not allow removal of layers.
  const_iterator begin() const { return list_.begin(); }

  const_iterator end() const { return list_.end(); }

  const std::vector<SimulcastLayer>& operator[](size_t index) const;

  size_t size() const { return list_.size(); }
  bool empty() const { return list_.empty(); }

  // Provides access to all the layers in the simulcast without their
  // association into groups of alternatives.
  std::vector<SimulcastLayer> GetAllLayers() const;

 private:
  // TODO(amithi, bugs.webrtc.org/10075):
  // Validate that rids do not repeat in the list.
  std::vector<std::vector<SimulcastLayer>> list_;
};

// Describes the simulcast options of a video media section.
// This will list the send and receive layers (along with their alternatives).
// Each simulcast layer has an identifier (rid) and can optionally be paused.
// The order of the layers (as well as alternates) indicates user preference
// from first to last (most preferred to least preferred).
// https://tools.ietf.org/html/draft-ietf-mmusic-sdp-simulcast-13#section-5.1
class SimulcastDescription final {
 public:
  const SimulcastLayerList& send_layers() const { return send_layers_; }
  SimulcastLayerList& send_layers() { return send_layers_; }

  const SimulcastLayerList& receive_layers() const { return receive_layers_; }
  SimulcastLayerList& receive_layers() { return receive_layers_; }

  bool empty() const;

 private:
  // TODO(amithi, bugs.webrtc.org/10075):
  // Validate that rids do not repeat in send and receive layers.
  SimulcastLayerList send_layers_;
  SimulcastLayerList receive_layers_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::SimulcastDescription;
using ::webrtc::SimulcastLayer;
using ::webrtc::SimulcastLayerList;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_SIMULCAST_DESCRIPTION_H_

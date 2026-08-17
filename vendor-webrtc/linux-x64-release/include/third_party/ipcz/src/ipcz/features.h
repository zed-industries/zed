// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_FEATURES_H_
#define IPCZ_SRC_IPCZ_FEATURES_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <utility>

#include "ipcz/ipcz.h"
#include "third_party/abseil-cpp/absl/types/span.h"

namespace ipcz {

class Message;

// Flags for any available ipcz features that might be enabled on a node.
struct Features {
  using Bitfield = uint64_t;

  // Computes the intersection of two feature sets.
  Features Intersect(const Features& features) const;

  // Extracts a Features value from node options.
  static Features FromNodeOptions(const IpczCreateNodeOptions* options);

  bool mem_v2() const { return bit(kMemV2Bit); }

  // Serializes this object into an internal ipcz message and returns the offset
  // of the encoded data within the message, to be assigned to a field value.
  uint32_t Serialize(Message& message) const;

  // Deserializes from an internal ipcz message given the encoded offset of an
  // array of Bitfields. `offset` must already be validated as the location of a
  // valid Bitfield array header (see ipcz::Message::ValidateParameters.)
  static Features Deserialize(Message& message, uint32_t offset);

 private:
  using BitIndex = std::pair<size_t, uint64_t>;

  // Internal bit indices for different features.
  static constexpr BitIndex kMemV2Bit{0, 0};

  bool bit(BitIndex bit) const {
    return bitfield_values[bit.first] & (1ull << bit.second);
  }

  void set_bit(BitIndex bit, bool enabled) {
    const uint64_t value = 1ull << bit.second;
    auto& bitfield = bitfield_values[bit.first];
    bitfield = (bitfield & ~value) | (enabled ? value : 0);
  }

  // Features are serialized to a variable-length array of 64-bit bitfields.
  // This lends some flexibility over time, retaining wire compatibility as the
  // number of features increases.
  std::array<Bitfield, 1> bitfield_values = {};

  void SetFeatureEnabled(IpczFeature feature, bool enabled);
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_FEATURES_H_

/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_AUDIO_CODEC_PAIR_ID_H_
#define API_AUDIO_CODECS_AUDIO_CODEC_PAIR_ID_H_

#include <stdint.h>

#include <utility>

namespace webrtc {

class AudioCodecPairId final {
 public:
  // Copyable, but not default constructible.
  AudioCodecPairId() = delete;
  AudioCodecPairId(const AudioCodecPairId&) = default;
  AudioCodecPairId(AudioCodecPairId&&) = default;
  AudioCodecPairId& operator=(const AudioCodecPairId&) = default;
  AudioCodecPairId& operator=(AudioCodecPairId&&) = default;

  friend void swap(AudioCodecPairId& a, AudioCodecPairId& b) {
    using std::swap;
    swap(a.id_, b.id_);
  }

  // Creates a new ID, unequal to any previously created ID.
  static AudioCodecPairId Create();

  // IDs can be tested for equality.
  friend bool operator==(AudioCodecPairId a, AudioCodecPairId b) {
    return a.id_ == b.id_;
  }
  friend bool operator!=(AudioCodecPairId a, AudioCodecPairId b) {
    return a.id_ != b.id_;
  }

  // Comparisons. The ordering of ID values is completely arbitrary, but
  // stable, so it's useful e.g. if you want to use IDs as keys in an ordered
  // map.
  friend bool operator<(AudioCodecPairId a, AudioCodecPairId b) {
    return a.id_ < b.id_;
  }
  friend bool operator<=(AudioCodecPairId a, AudioCodecPairId b) {
    return a.id_ <= b.id_;
  }
  friend bool operator>=(AudioCodecPairId a, AudioCodecPairId b) {
    return a.id_ >= b.id_;
  }
  friend bool operator>(AudioCodecPairId a, AudioCodecPairId b) {
    return a.id_ > b.id_;
  }

  // Returns a numeric representation of the ID. The numeric values are
  // completely arbitrary, but stable, collision-free, and reasonably evenly
  // distributed, so they are e.g. useful as hash values in unordered maps.
  uint64_t NumericRepresentation() const { return id_; }

 private:
  explicit AudioCodecPairId(uint64_t id) : id_(id) {}

  uint64_t id_;
};

}  // namespace webrtc

#endif  // API_AUDIO_CODECS_AUDIO_CODEC_PAIR_ID_H_

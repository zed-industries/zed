/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains an interface for the (obsolete) StatsCollector class that
// is used by compilation units that do not wish to depend on the StatsCollector
// implementation.

#ifndef PC_LEGACY_STATS_COLLECTOR_INTERFACE_H_
#define PC_LEGACY_STATS_COLLECTOR_INTERFACE_H_

#include <stdint.h>

#include "api/legacy_stats_types.h"
#include "api/media_stream_interface.h"

namespace webrtc {

class LegacyStatsCollectorInterface {
 public:
  virtual ~LegacyStatsCollectorInterface() {}

  // Adds a local audio track that is used for getting some voice statistics.
  virtual void AddLocalAudioTrack(AudioTrackInterface* audio_track,
                                  uint32_t ssrc) = 0;

  // Removes a local audio tracks that is used for getting some voice
  // statistics.
  virtual void RemoveLocalAudioTrack(AudioTrackInterface* audio_track,
                                     uint32_t ssrc) = 0;
  virtual void GetStats(MediaStreamTrackInterface* track,
                        StatsReports* reports) = 0;
};

}  // namespace webrtc

#endif  // PC_LEGACY_STATS_COLLECTOR_INTERFACE_H_

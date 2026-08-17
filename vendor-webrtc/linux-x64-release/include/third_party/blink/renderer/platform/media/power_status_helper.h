// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_POWER_STATUS_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_POWER_STATUS_HELPER_H_

#include <optional>

#include "base/functional/callback.h"
#include "base/sequence_checker.h"
#include "base/time/time.h"
#include "media/base/video_codecs.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/device/public/mojom/battery_monitor.mojom-blink.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/size.h"

namespace media {
struct PipelineMetadata;
}

namespace blink {

// Class to monitor for power events during playback and record them to UMA/UKM.
class PLATFORM_EXPORT PowerStatusHelper {
 public:
  using CreateBatteryMonitorCB = base::RepeatingCallback<
      mojo::PendingRemote<device::mojom::blink::BatteryMonitor>()>;

  // Bits used to construct UMA buckets.
  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum /* not class */ Bits {
    // Bit layout is: [msb] xx f F RR CC [lsb]
    // R == resolution
    // C == codec
    // F == frame rate
    // f == full screen
    // x == unused
    // Remember that we can't use more than 6 bits, since we shouldn't go over
    // 100 UMA buckets.

    // Codec bits, values 0x00 to 0x03
    // Named "CodecBits" to prevent a name collision with media::VideoCodec.
    kCodecBitsH264 = (0x00) << 0,
    kCodecBitsVP9Profile0 = (0x01) << 0,
    // Ignore Profile1
    kCodecBitsVP9Profile2 = (0x02) << 0,
    // TODO(liberato): add AV1

    // Resolution bits, values 0x00 to 0x03
    kResolution360p = (0x00) << 2,
    kResolution720p = (0x01) << 2,
    kResolution1080p = (0x02) << 2,

    // Frame rate bits, values 0x00 to 0x01
    kFrameRate30 = (0x00) << 4,
    kFrameRate60 = (0x01) << 4,

    // Fullscreen bits, values 0x00 to 0x01
    kFullScreenNo = (0x00) << 5,
    kFullScreenYes = (0x01) << 5,

    // This is not a valid bit for, you know, testing.
    kNotAValidBitForTesting = (0x01) << 10,
  };

  // If |stats_cb| is not provided, then we'll record to UMA.  It's just for
  // the tests.
  explicit PowerStatusHelper(CreateBatteryMonitorCB create_battery_monitor_cb);
  PowerStatusHelper(const PowerStatusHelper&) = delete;
  PowerStatusHelper& operator=(const PowerStatusHelper&) = delete;
  ~PowerStatusHelper();

  // Notify us about changes to the player.
  void SetIsPlaying(bool is_playing);
  void SetMetadata(const media::PipelineMetadata& metadata);
  void SetIsFullscreen(bool is_fullscreen);
  void SetAverageFrameRate(std::optional<int> average_fps);

  // Handle notifications about the experiment state from the power experiment.
  // manager.  |state| indicates whether our player is eligible to record power
  // experiments readings.
  void UpdatePowerExperimentState(bool state);

 private:
  friend class PowerStatusHelperTest;
  friend class PowerStatusHelperBucketTest;

  // Return the UMA bucket for the given video configuration, or nullopt if we
  // don't want to record it.
  static std::optional<int> BucketFor(bool is_playing,
                                      bool has_video,
                                      media::VideoCodec codec,
                                      media::VideoCodecProfile profile,
                                      gfx::Size natural_size,
                                      bool is_fullscreen,
                                      std::optional<int> average_fps);

  // Recompute everything when playback state or power experiment state changes.
  void OnAnyStateChange();

  // Handle updates about the current battery status.
  void OnBatteryStatus(device::mojom::blink::BatteryStatusPtr battery_status);

  // Start monitoring if we haven't already.  Any outstanding callbacks will be
  // cancelled if monitoring was already in progress.
  void StartMonitoring();
  void StopMonitoring();

  // Register to receive a power update the next time it changes.
  void QueryNextStatus();

  CreateBatteryMonitorCB create_battery_monitor_cb_;

  // Most recent parameters we were given.
  bool is_playing_ = false;
  bool has_video_ = false;
  media::VideoCodec codec_ = media::VideoCodec::kUnknown;
  media::VideoCodecProfile profile_ =
      media::VideoCodecProfile::VIDEO_CODEC_PROFILE_UNKNOWN;
  gfx::Size natural_size_;
  bool is_fullscreen_ = false;
  // For estimating fps.  Can be unset if we don't know.
  std::optional<int> average_fps_;

  // Current UMA bucket, if any.
  std::optional<int> current_bucket_;

  // If set, our previous battery level, from 0-100.
  std::optional<float> battery_level_baseline_;
  // The time at which we last got an update from |battery_monitor_|.
  base::TimeTicks last_update_;

  // Are we currently the player that should be recording power for the power
  // experiment, according to the MediaPowerExperimentManager?
  bool experiment_state_ = false;

  mojo::Remote<device::mojom::blink::BatteryMonitor> battery_monitor_;

  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_POWER_STATUS_HELPER_H_

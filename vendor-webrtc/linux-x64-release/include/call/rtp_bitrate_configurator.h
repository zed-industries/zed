/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_RTP_BITRATE_CONFIGURATOR_H_
#define CALL_RTP_BITRATE_CONFIGURATOR_H_

#include <optional>

#include "api/transport/bitrate_settings.h"
#include "api/units/data_rate.h"

namespace webrtc {

// RtpBitrateConfigurator calculates the bitrate configuration based on received
// remote configuration combined with local overrides.
class RtpBitrateConfigurator {
 public:
  explicit RtpBitrateConfigurator(const BitrateConstraints& bitrate_config);
  ~RtpBitrateConfigurator();

  RtpBitrateConfigurator(const RtpBitrateConfigurator&) = delete;
  RtpBitrateConfigurator& operator=(const RtpBitrateConfigurator&) = delete;

  BitrateConstraints GetConfig() const;

  // The greater min and smaller max set by this and SetClientBitratePreferences
  // will be used. The latest non-negative start value from either call will be
  // used. Specifying a start bitrate (>0) will reset the current bitrate
  // estimate. This is due to how the 'x-google-start-bitrate' flag is currently
  // implemented. Passing -1 leaves the start bitrate unchanged. Behavior is not
  // guaranteed for other negative values or 0.
  // The optional return value is set with new configuration if it was updated.
  std::optional<BitrateConstraints> UpdateWithSdpParameters(
      const BitrateConstraints& bitrate_config_);

  // The greater min and smaller max set by this and SetSdpBitrateParameters
  // will be used. The latest non-negative start value form either call will be
  // used. Specifying a start bitrate will reset the current bitrate estimate.
  // Assumes 0 <= min <= start <= max holds for set parameters.
  // Update the bitrate configuration
  // The optional return value is set with new configuration if it was updated.
  std::optional<BitrateConstraints> UpdateWithClientPreferences(
      const BitrateSettings& bitrate_mask);

  // Apply a cap for relayed calls.
  std::optional<BitrateConstraints> UpdateWithRelayCap(DataRate cap);

 private:
  // Applies update to the BitrateConstraints cached in `config_`, resetting
  // with `new_start` if set.
  std::optional<BitrateConstraints> UpdateConstraints(
      const std::optional<int>& new_start);

  // Bitrate config used until valid bitrate estimates are calculated. Also
  // used to cap total bitrate used. This comes from the remote connection.
  BitrateConstraints bitrate_config_;

  // The config mask set by SetClientBitratePreferences.
  // 0 <= min <= start <= max
  BitrateSettings bitrate_config_mask_;

  // The config set by SetSdpBitrateParameters.
  // min >= 0, start != 0, max == -1 || max > 0
  BitrateConstraints base_bitrate_config_;

  // Bandwidth cap applied for relayed calls.
  DataRate max_bitrate_over_relay_ = DataRate::PlusInfinity();
};
}  // namespace webrtc

#endif  // CALL_RTP_BITRATE_CONFIGURATOR_H_

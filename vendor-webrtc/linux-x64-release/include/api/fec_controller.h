/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_FEC_CONTROLLER_H_
#define API_FEC_CONTROLLER_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

#include "api/environment/environment.h"
#include "api/video/video_frame_type.h"
#include "modules/include/module_fec_types.h"

namespace webrtc {
// TODO(yinwa): work in progress. API in class FecController should not be
// used by other users until this comment is removed.

// Callback class used for telling the user about how to configure the FEC,
// and the rates sent the last second is returned to the VCM.
class VCMProtectionCallback {
 public:
  virtual int ProtectionRequest(const FecProtectionParams* delta_params,
                                const FecProtectionParams* key_params,
                                uint32_t* sent_video_rate_bps,
                                uint32_t* sent_nack_rate_bps,
                                uint32_t* sent_fec_rate_bps) = 0;

  // 'retransmission_mode' is either a value of enum RetransmissionMode, or
  // computed with bitwise operators on values of enum RetransmissionMode.
  virtual void SetRetransmissionMode(int retransmission_mode) = 0;

 protected:
  virtual ~VCMProtectionCallback() {}
};

// FecController calculates how much of the allocated network
// capacity that can be used by an encoder and how much that
// is needed for redundant packets such as FEC and NACK. It uses an
// implementation of `VCMProtectionCallback` to set new FEC parameters and get
// the bitrate currently used for FEC and NACK.
// Usage:
// Setup by calling SetProtectionMethod and SetEncodingData.
// For each encoded image, call UpdateWithEncodedData.
// Each time the bandwidth estimate change, call UpdateFecRates. UpdateFecRates
// will return the bitrate that can be used by an encoder.
// A lock is used to protect internal states, so methods can be called on an
// arbitrary thread.
class FecController {
 public:
  virtual ~FecController() {}

  virtual void SetProtectionCallback(
      VCMProtectionCallback* protection_callback) = 0;
  virtual void SetProtectionMethod(bool enable_fec, bool enable_nack) = 0;

  // Informs loss protectoin logic of initial encoding state.
  virtual void SetEncodingData(size_t width,
                               size_t height,
                               size_t num_temporal_layers,
                               size_t max_payload_size) = 0;

  // Returns target rate for the encoder given the channel parameters.
  // Inputs:  estimated_bitrate_bps - the estimated network bitrate in bits/s.
  //          actual_framerate - encoder frame rate.
  //          fraction_lost - packet loss rate in % in the network.
  //          loss_mask_vector - packet loss mask since last time this method
  //          was called. round_trip_time_ms - round trip time in milliseconds.
  virtual uint32_t UpdateFecRates(uint32_t estimated_bitrate_bps,
                                  int actual_framerate,
                                  uint8_t fraction_lost,
                                  std::vector<bool> loss_mask_vector,
                                  int64_t round_trip_time_ms) = 0;

  // Informs of encoded output.
  virtual void UpdateWithEncodedData(
      size_t encoded_image_length,
      VideoFrameType encoded_image_frametype) = 0;

  // Returns whether this FEC Controller needs Loss Vector Mask as input.
  virtual bool UseLossVectorMask() = 0;
};

class FecControllerFactoryInterface {
 public:
  virtual ~FecControllerFactoryInterface() = default;

  virtual std::unique_ptr<FecController> CreateFecController(
      const Environment& env) = 0;
};

}  // namespace webrtc
#endif  // API_FEC_CONTROLLER_H_

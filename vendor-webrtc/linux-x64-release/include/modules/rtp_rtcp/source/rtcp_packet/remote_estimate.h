/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_REMOTE_ESTIMATE_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_REMOTE_ESTIMATE_H_

#include <cstdint>

#include "api/array_view.h"
#include "api/transport/network_types.h"
#include "api/units/time_delta.h"
#include "modules/rtp_rtcp/source/rtcp_packet/app.h"
#include "rtc_base/buffer.h"

namespace webrtc {
namespace rtcp {

class CommonHeader;
class RemoteEstimateSerializer {
 public:
  virtual bool Parse(ArrayView<const uint8_t> src,
                     NetworkStateEstimate* target) const = 0;
  virtual Buffer Serialize(const NetworkStateEstimate& src) const = 0;
  virtual ~RemoteEstimateSerializer() = default;
};

// Using a static global implementation to avoid incurring initialization
// overhead of the serializer every time RemoteEstimate is created.
const RemoteEstimateSerializer* GetRemoteEstimateSerializer();

// The RemoteEstimate packet provides network estimation results from the
// receive side. This functionality is experimental and subject to change
// without notice.
class RemoteEstimate : public App {
 public:
  RemoteEstimate();
  explicit RemoteEstimate(App&& app);
  // Note, sub type must be unique among all app messages with "goog" name.
  static constexpr uint8_t kSubType = 13;
  static constexpr uint32_t kName = NameToInt("goog");
  static TimeDelta GetTimestampPeriod();

  bool ParseData();
  void SetEstimate(NetworkStateEstimate estimate);
  NetworkStateEstimate estimate() const { return estimate_; }

 private:
  NetworkStateEstimate estimate_;
  const RemoteEstimateSerializer* const serializer_;
};

}  // namespace rtcp
}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTCP_PACKET_REMOTE_ESTIMATE_H_

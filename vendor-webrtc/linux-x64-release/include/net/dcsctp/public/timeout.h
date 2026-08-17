/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_TIMEOUT_H_
#define NET_DCSCTP_PUBLIC_TIMEOUT_H_

#include <cstdint>

#include "net/dcsctp/public/types.h"

namespace dcsctp {

// A very simple timeout that can be started and stopped. When started,
// it will be given a unique `timeout_id` which should be provided to
// `DcSctpSocket::HandleTimeout` when it expires.
class Timeout {
 public:
  virtual ~Timeout() = default;

  // Called to start time timeout, with the duration in milliseconds as
  // `duration` and with the timeout identifier as `timeout_id`, which - if
  // the timeout expires - shall be provided to `DcSctpSocket::HandleTimeout`.
  //
  // `Start` and `Stop` will always be called in pairs. In other words will
  // ´Start` never be called twice, without a call to `Stop` in between.
  virtual void Start(DurationMs duration, TimeoutID timeout_id) = 0;

  // Called to stop the running timeout.
  //
  // `Start` and `Stop` will always be called in pairs. In other words will
  // ´Start` never be called twice, without a call to `Stop` in between.
  //
  // `Stop` will always be called prior to releasing this object.
  virtual void Stop() = 0;

  // Called to restart an already running timeout, with the `duration` and
  // `timeout_id` parameters as described in `Start`. This can be overridden by
  // the implementation to restart it more efficiently.
  virtual void Restart(DurationMs duration, TimeoutID timeout_id) {
    Stop();
    Start(duration, timeout_id);
  }
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PUBLIC_TIMEOUT_H_

/*
 *  Copyright 2025 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_DTLS_DTLS_STUN_PIGGYBACK_CALLBACKS_H_
#define P2P_DTLS_DTLS_STUN_PIGGYBACK_CALLBACKS_H_

#include <optional>
#include <utility>

#include "absl/functional/any_invocable.h"
#include "absl/strings/string_view.h"
#include "api/transport/stun.h"
#include "rtc_base/checks.h"

namespace webrtc {

class DtlsStunPiggybackCallbacks {
 public:
  DtlsStunPiggybackCallbacks() : send_data_(nullptr), recv_data_(nullptr) {}

  DtlsStunPiggybackCallbacks(
      // Function invoked when sending a `request-type` (e.g.
      // STUN_BINDING_REQUEST). Returns a pair of data that will be sent:
      // - an optional DTLS_IN_STUN attribute
      // - an optional DTLS_IN_STUN_ACK attribute
      absl::AnyInvocable<std::pair<std::optional<absl::string_view>,
                                   std::optional<absl::string_view>>(
          /* request-type */ webrtc::StunMessageType)>&& send_data,

      // Function invoked when receiving a STUN_BINDING { REQUEST / RESPONSE }
      // contains the (nullable) DTLS_IN_STUN and DTLS_IN_STUN_ACK attributes.
      absl::AnyInvocable<
          void(const webrtc::StunByteStringAttribute* /* DTLS_IN_STUN */,
               const webrtc::StunByteStringAttribute* /* DTLS_IN_STUN_ACK */)>&&
          recv_data)
      : send_data_(std::move(send_data)), recv_data_(std::move(recv_data)) {
    RTC_DCHECK(
        // either all set
        (send_data_ != nullptr && recv_data_ != nullptr) ||
        // or all nullptr
        (send_data_ == nullptr && recv_data_ == nullptr));
  }

  std::pair<std::optional<absl::string_view>, std::optional<absl::string_view>>
  send_data(StunMessageType request_type) {
    RTC_DCHECK(send_data_);
    return send_data_(request_type);
  }

  void recv_data(const StunByteStringAttribute* data,
                 const StunByteStringAttribute* ack) {
    RTC_DCHECK(recv_data_);
    return recv_data_(data, ack);
  }

  bool empty() const { return send_data_ == nullptr; }
  void reset() {
    send_data_ = nullptr;
    recv_data_ = nullptr;
  }

 private:
  absl::AnyInvocable<std::pair<std::optional<absl::string_view>,
                               std::optional<absl::string_view>>(
      /* request-type */ webrtc::StunMessageType)>
      send_data_;
  absl::AnyInvocable<void(
      const webrtc::StunByteStringAttribute* /* DTLS_IN_STUN */,
      const webrtc::StunByteStringAttribute* /* DTLS_IN_STUN_ACK */)>
      recv_data_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::DtlsStunPiggybackCallbacks;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_DTLS_DTLS_STUN_PIGGYBACK_CALLBACKS_H_

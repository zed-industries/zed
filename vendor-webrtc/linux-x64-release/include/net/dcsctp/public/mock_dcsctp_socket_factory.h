/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_MOCK_DCSCTP_SOCKET_FACTORY_H_
#define NET_DCSCTP_PUBLIC_MOCK_DCSCTP_SOCKET_FACTORY_H_

#include <memory>

#include "net/dcsctp/public/dcsctp_socket_factory.h"
#include "test/gmock.h"

namespace dcsctp {

class MockDcSctpSocketFactory : public DcSctpSocketFactory {
 public:
  MOCK_METHOD(std::unique_ptr<DcSctpSocketInterface>,
              Create,
              (absl::string_view log_prefix,
               DcSctpSocketCallbacks& callbacks,
               std::unique_ptr<PacketObserver> packet_observer,
               const DcSctpOptions& options),
              (override));
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PUBLIC_MOCK_DCSCTP_SOCKET_FACTORY_H_

/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TRANSPORT_SCTP_TRANSPORT_FACTORY_INTERFACE_H_
#define API_TRANSPORT_SCTP_TRANSPORT_FACTORY_INTERFACE_H_

#include <memory>

#include "api/environment/environment.h"

// These classes are not part of the API, and are treated as opaque pointers.

namespace webrtc {

class DtlsTransportInternal;
class SctpTransportInternal;

// Factory class which can be used to allow fake SctpTransports to be injected
// for testing. An application is not intended to implement this interface nor
// 'webrtc::SctpTransportInternal' because SctpTransportInternal is not
// guaranteed to remain stable in future WebRTC versions.
class SctpTransportFactoryInterface {
 public:
  virtual ~SctpTransportFactoryInterface() = default;

  // Create an SCTP transport using `channel` for the underlying transport.
  virtual std::unique_ptr<SctpTransportInternal> CreateSctpTransport(
      const Environment& env,
      DtlsTransportInternal* channel) = 0;
};

}  // namespace webrtc

#endif  // API_TRANSPORT_SCTP_TRANSPORT_FACTORY_INTERFACE_H_

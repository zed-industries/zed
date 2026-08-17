/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_MDNS_RESPONDER_INTERFACE_H_
#define RTC_BASE_MDNS_RESPONDER_INTERFACE_H_

#include <functional>
#include <string>

#include "absl/strings/string_view.h"
#include "rtc_base/ip_address.h"

namespace webrtc {

// Defines an mDNS responder that can be used in ICE candidate gathering, where
// the local IP addresses of host candidates are replaced by mDNS hostnames.
class MdnsResponderInterface {
 public:
  using NameCreatedCallback =
      std::function<void(const webrtc::IPAddress&, absl::string_view)>;
  using NameRemovedCallback = std::function<void(bool)>;

  MdnsResponderInterface() = default;
  virtual ~MdnsResponderInterface() = default;

  // Asynchronously creates and returns a new name via `callback` for `addr` if
  // there is no name mapped to it by this responder, and initializes the
  // reference count of this name to one. Otherwise the existing name mapped to
  // `addr` is returned and its reference count is incremented by one.
  virtual void CreateNameForAddress(const IPAddress& addr,
                                    NameCreatedCallback callback) = 0;
  // Decrements the reference count of the mapped name of `addr`, if
  // there is a map created previously via CreateNameForAddress; asynchronously
  // removes the association between `addr` and its mapped name, and returns
  // true via `callback` if the decremented reference count reaches zero.
  // Otherwise no operation is done and false is returned via `callback`
  // asynchronously.
  virtual void RemoveNameForAddress(const IPAddress& addr,
                                    NameRemovedCallback callback) = 0;
};

}  // namespace webrtc

#endif  // RTC_BASE_MDNS_RESPONDER_INTERFACE_H_

/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_PORTAL_XDG_SESSION_DETAILS_H_
#define MODULES_PORTAL_XDG_SESSION_DETAILS_H_

#include <gio/gio.h>
#include <stdint.h>

#include <string>

namespace webrtc {
namespace xdg_portal {

// Details of the session associated with XDG desktop portal session. Portal API
// calls can be invoked by utilizing the information here.
struct SessionDetails {
  GDBusProxy* proxy = nullptr;
  GCancellable* cancellable = nullptr;
  std::string session_handle;
  uint32_t pipewire_stream_node_id = 0;
};

}  // namespace xdg_portal
}  // namespace webrtc

#endif  // MODULES_PORTAL_XDG_SESSION_DETAILS_H_

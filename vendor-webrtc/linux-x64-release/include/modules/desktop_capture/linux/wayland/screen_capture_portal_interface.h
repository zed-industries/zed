/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_LINUX_WAYLAND_SCREEN_CAPTURE_PORTAL_INTERFACE_H_
#define MODULES_DESKTOP_CAPTURE_LINUX_WAYLAND_SCREEN_CAPTURE_PORTAL_INTERFACE_H_

#include <gio/gio.h>

#include <string>

#include "modules/portal/portal_request_response.h"
#include "modules/portal/scoped_glib.h"
#include "modules/portal/xdg_desktop_portal_utils.h"
#include "modules/portal/xdg_session_details.h"

namespace webrtc {
namespace xdg_portal {

using SessionClosedSignalHandler = void (*)(GDBusConnection*,
                                            const char*,
                                            const char*,
                                            const char*,
                                            const char*,
                                            GVariant*,
                                            gpointer);

// A base class for XDG desktop portals that can capture desktop/screen.
// Note: downstream clients inherit from this class so it is advisable to
// provide a default implementation of any new virtual methods that may be added
// to this class.
class RTC_EXPORT ScreenCapturePortalInterface {
 public:
  virtual ~ScreenCapturePortalInterface() {}
  // Gets details about the session such as session handle.
  virtual xdg_portal::SessionDetails GetSessionDetails() { return {}; }
  // Starts the portal setup.
  virtual void Start() {}

  // Stops and cleans up the portal.
  virtual void Stop() {}

  // Notifies observers about the success/fail state of the portal
  // request/response.
  virtual void OnPortalDone(xdg_portal::RequestResponse result) {}
  // Sends a create session request to the portal.
  virtual void RequestSession(GDBusProxy* proxy) {}

  // Following methods should not be made virtual as they share a common
  // implementation between portals.

  // Requests portal session using the proxy object.
  void RequestSessionUsingProxy(GAsyncResult* result);
  // Handles the session request result.
  void OnSessionRequestResult(GDBusProxy* proxy, GAsyncResult* result);
  // Subscribes to session close signal and sets up a handler for it.
  void RegisterSessionClosedSignalHandler(
      const SessionClosedSignalHandler session_close_signal_handler,
      GVariant* parameters,
      GDBusConnection* connection,
      std::string& session_handle,
      guint& session_closed_signal_id);
  // Handles the result of session start request.
  void OnStartRequestResult(GDBusProxy* proxy, GAsyncResult* result);
};

}  // namespace xdg_portal
}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_LINUX_WAYLAND_SCREEN_CAPTURE_PORTAL_INTERFACE_H_

/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_LINUX_WAYLAND_SCREENCAST_PORTAL_H_
#define MODULES_DESKTOP_CAPTURE_LINUX_WAYLAND_SCREENCAST_PORTAL_H_

#include <gio/gio.h>

#include <string>

#include "modules/desktop_capture/desktop_capture_types.h"
#include "modules/desktop_capture/linux/wayland/screen_capture_portal_interface.h"
#include "modules/portal/pipewire_utils.h"
#include "modules/portal/portal_request_response.h"
#include "modules/portal/xdg_desktop_portal_utils.h"
#include "modules/portal/xdg_session_details.h"

namespace webrtc {

class RTC_EXPORT ScreenCastPortal
    : public xdg_portal::ScreenCapturePortalInterface {
 public:
  using ProxyRequestResponseHandler = void (*)(GObject* object,
                                               GAsyncResult* result,
                                               gpointer user_data);

  using SourcesRequestResponseSignalHandler =
      void (*)(GDBusConnection* connection,
               const char* sender_name,
               const char* object_path,
               const char* interface_name,
               const char* signal_name,
               GVariant* parameters,
               gpointer user_data);

  // Values are set based on cursor mode property in
  // xdg-desktop-portal/screencast
  // https://github.com/flatpak/xdg-desktop-portal/blob/main/data/org.freedesktop.portal.ScreenCast.xml
  enum class CursorMode : uint32_t {
    // Mouse cursor will not be included in any form
    kHidden = 0b01,
    // Mouse cursor will be part of the screen content
    kEmbedded = 0b10,
    // Mouse cursor information will be send separately in form of metadata
    kMetadata = 0b100
  };

  // Values are set based on persist mode property in
  // xdg-desktop-portal/screencast
  // https://github.com/flatpak/xdg-desktop-portal/blob/main/data/org.freedesktop.portal.ScreenCast.xml
  enum class PersistMode : uint32_t {
    // Do not allow to restore stream
    kDoNotPersist = 0b00,
    // The restore token is valid as long as the application is alive. It's
    // stored in memory and revoked when the application closes its DBus
    // connection
    kTransient = 0b01,
    // The restore token is stored in disk and is valid until the user manually
    // revokes it
    kPersistent = 0b10
  };

  // Interface that must be implemented by the ScreenCastPortal consumers.
  class PortalNotifier {
   public:
    virtual void OnScreenCastRequestResult(xdg_portal::RequestResponse result,
                                           uint32_t stream_node_id,
                                           int fd) = 0;
    virtual void OnScreenCastSessionClosed() = 0;

   protected:
    PortalNotifier() = default;
    virtual ~PortalNotifier() = default;
  };

  ScreenCastPortal(CaptureType type, PortalNotifier* notifier);
  ScreenCastPortal(CaptureType type,
                   PortalNotifier* notifier,
                   ProxyRequestResponseHandler proxy_request_response_handler,
                   SourcesRequestResponseSignalHandler
                       sources_request_response_signal_handler,
                   gpointer user_data,
                   // TODO(chromium:1291247): Remove the default option once
                   // downstream has been adjusted.
                   bool prefer_cursor_embedded = false);

  ~ScreenCastPortal();

  // Initialize ScreenCastPortal with series of DBus calls where we try to
  // obtain all the required information, like PipeWire file descriptor and
  // PipeWire stream node ID.
  //
  // The observer will return whether the communication with xdg-desktop-portal
  // was successful and only then you will be able to get all the required
  // information in order to continue working with PipeWire.
  void Start() override;
  void Stop() override;
  xdg_portal::SessionDetails GetSessionDetails() override;

  // Method to notify the reason for failure of a portal request.
  void OnPortalDone(xdg_portal::RequestResponse result) override;

  // Sends a create session request to the portal.
  void RequestSession(GDBusProxy* proxy) override;

  // Set of methods leveraged by remote desktop portal to setup a common session
  // with screen cast portal.
  void SetSessionDetails(const xdg_portal::SessionDetails& session_details);
  uint32_t pipewire_stream_node_id();
  void SourcesRequest();
  void OpenPipeWireRemote();

  // ScreenCast specific methods for stream restoration
  void SetPersistMode(ScreenCastPortal::PersistMode mode);
  void SetRestoreToken(const std::string& token);
  std::string RestoreToken() const;

 private:
  // Values are set based on source type property in
  // xdg-desktop-portal/screencast
  // https://github.com/flatpak/xdg-desktop-portal/blob/main/data/org.freedesktop.portal.ScreenCast.xml
  enum class CaptureSourceType : uint32_t {
    kScreen = 0b01,
    kWindow = 0b10,
    kAnyScreenContent = kScreen | kWindow
  };
  static CaptureSourceType ToCaptureSourceType(CaptureType type);

  PortalNotifier* notifier_;

  // A PipeWire stream ID of stream we will be connecting to
  uint32_t pw_stream_node_id_ = 0;
  // A file descriptor of PipeWire socket
  int pw_fd_ = kInvalidPipeWireFd;
  // Restore token that can be used to restore previous session
  std::string restore_token_;

  CaptureSourceType capture_source_type_ =
      ScreenCastPortal::CaptureSourceType::kScreen;

  CursorMode cursor_mode_ = CursorMode::kMetadata;

  PersistMode persist_mode_ = ScreenCastPortal::PersistMode::kDoNotPersist;

  ProxyRequestResponseHandler proxy_request_response_handler_;
  SourcesRequestResponseSignalHandler sources_request_response_signal_handler_;
  gpointer user_data_;

  GDBusConnection* connection_ = nullptr;
  GDBusProxy* proxy_ = nullptr;
  GCancellable* cancellable_ = nullptr;
  std::string portal_handle_;
  std::string session_handle_;
  std::string sources_handle_;
  std::string start_handle_;
  guint session_request_signal_id_ = 0;
  guint sources_request_signal_id_ = 0;
  guint start_request_signal_id_ = 0;
  guint session_closed_signal_id_ = 0;

  void UnsubscribeSignalHandlers();
  static void OnProxyRequested(GObject* object,
                               GAsyncResult* result,
                               gpointer user_data);
  static void OnSessionRequested(GDBusProxy* proxy,
                                 GAsyncResult* result,
                                 gpointer user_data);
  static void OnSessionRequestResponseSignal(GDBusConnection* connection,
                                             const char* sender_name,
                                             const char* object_path,
                                             const char* interface_name,
                                             const char* signal_name,
                                             GVariant* parameters,
                                             gpointer user_data);
  static void OnSessionClosedSignal(GDBusConnection* connection,
                                    const char* sender_name,
                                    const char* object_path,
                                    const char* interface_name,
                                    const char* signal_name,
                                    GVariant* parameters,
                                    gpointer user_data);
  static void OnSourcesRequested(GDBusProxy* proxy,
                                 GAsyncResult* result,
                                 gpointer user_data);
  static void OnSourcesRequestResponseSignal(GDBusConnection* connection,
                                             const char* sender_name,
                                             const char* object_path,
                                             const char* interface_name,
                                             const char* signal_name,
                                             GVariant* parameters,
                                             gpointer user_data);

  void StartRequest();
  static void OnStartRequested(GDBusProxy* proxy,
                               GAsyncResult* result,
                               gpointer user_data);
  static void OnStartRequestResponseSignal(GDBusConnection* connection,
                                           const char* sender_name,
                                           const char* object_path,
                                           const char* interface_name,
                                           const char* signal_name,
                                           GVariant* parameters,
                                           gpointer user_data);

  static void OnOpenPipeWireRemoteRequested(GDBusProxy* proxy,
                                            GAsyncResult* result,
                                            gpointer user_data);
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_LINUX_WAYLAND_SCREENCAST_PORTAL_H_

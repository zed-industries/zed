/*
 * Copyright (C) 2011 Ericsson AB. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer
 *    in the documentation and/or other materials provided with the
 *    distribution.
 * 3. Neither the name of Ericsson nor the names of its contributors
 *    may be used to endorse or promote products derived from this
 *    software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_USER_MEDIA_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_USER_MEDIA_REQUEST_H_

#include "third_party/blink/public/common/privacy_budget/identifiable_surface.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/mediastream/capture_controller.h"
#include "third_party/blink/renderer/modules/mediastream/media_constraints.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class LocalDOMWindow;
class MediaStreamConstraints;
class ScriptWrappable;
class TransferredMediaStreamTrack;
class UserMediaClient;

enum class UserMediaRequestType { kUserMedia, kDisplayMedia, kAllScreensMedia };

enum class UserMediaRequestResult {
  kOk = 0,
  kTimedOut = 1,
  kSecurityError = 2,
  kInvalidConstraints = 3,
  kOverConstrainedError = 4,
  kContextDestroyed = 5,
  kNotAllowedError = 6,
  kNotFoundError = 7,
  kAbortError = 8,
  kNotReadableError = 9,
  kNotSupportedError = 10,
  kInsecureContext = 11,
  kInvalidStateError = 12,
  kMaxValue = kInvalidStateError
};

class MODULES_EXPORT UserMediaRequest final
    : public GarbageCollected<UserMediaRequest>,
      public ExecutionContextLifecycleObserver {
 public:
  class Callbacks : public GarbageCollected<Callbacks> {
   public:
    virtual ~Callbacks() = default;

    virtual void OnSuccess(const MediaStreamVector&,
                           CaptureController* capture_controller) = 0;
    virtual void OnError(ScriptWrappable* callback_this_value,
                         const V8MediaStreamError* error,
                         CaptureController* capture_controller,
                         UserMediaRequestResult result) = 0;

    virtual void Trace(Visitor*) const {}

   protected:
    Callbacks() = default;
  };

  class V8Callbacks;

  static UserMediaRequest* Create(ExecutionContext*,
                                  UserMediaClient*,
                                  UserMediaRequestType media_type,
                                  const MediaStreamConstraints* options,
                                  Callbacks*,
                                  ExceptionState&,
                                  IdentifiableSurface surface);
  static UserMediaRequest* CreateForTesting(const MediaConstraints& audio,
                                            const MediaConstraints& video);

  UserMediaRequest(ExecutionContext*,
                   UserMediaClient*,
                   UserMediaRequestType media_type,
                   MediaConstraints audio,
                   MediaConstraints video,
                   bool should_prefer_current_tab,
                   CaptureController* capture_controller,
                   Callbacks*,
                   IdentifiableSurface surface);
  ~UserMediaRequest() override;

  LocalDOMWindow* GetWindow();

  void Start();

  void Succeed(const GCedMediaStreamDescriptorVector& streams);
  void OnMediaStreamInitialized(MediaStream* stream);
  void OnMediaStreamsInitialized(MediaStreamVector streams);
  void FailConstraint(const String& constraint_name, const String& message);
  void Fail(mojom::blink::MediaStreamRequestResult error,
            const String& message);

  UserMediaRequestType MediaRequestType() const;
  bool Audio() const;
  bool Video() const;
  MediaConstraints AudioConstraints() const;
  MediaConstraints VideoConstraints() const;
  // The MediaStreamType for the audio part of a request with audio. Returns
  // NO_SERVICE for requests where Audio() == false.
  mojom::blink::MediaStreamType AudioMediaStreamType() const;
  // The MediaStreamType for the video part of a request with video. Returns
  // NO_SERVICE for requests where Video() == false.
  mojom::blink::MediaStreamType VideoMediaStreamType() const;

  // Flag tied to whether or not the similarly named Origin Trial is
  // enabled. Will be removed at end of trial. See: http://crbug.com/789152.
  bool ShouldDisableHardwareNoiseSuppression() const;

  // errorMessage is only set if requestIsPrivilegedContext() returns |false|.
  // Caller is responsible for properly setting errors and canceling request.
  bool IsSecureContextUse(String& error_message);

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  void set_request_id(int32_t id) { request_id_ = id; }
  int32_t request_id() { return request_id_; }

  void set_has_transient_user_activation(bool value) {
    has_transient_user_activation_ = value;
  }
  bool has_transient_user_activation() const {
    return has_transient_user_activation_;
  }

  bool should_prefer_current_tab() const { return should_prefer_current_tab_; }

  void set_exclude_system_audio(bool value) { exclude_system_audio_ = value; }
  bool exclude_system_audio() const { return exclude_system_audio_; }

  void set_exclude_self_browser_surface(bool value) {
    exclude_self_browser_surface_ = value;
  }
  bool exclude_self_browser_surface() const {
    return exclude_self_browser_surface_;
  }

  void set_preferred_display_surface(
      mojom::blink::PreferredDisplaySurface value) {
    preferred_display_surface_ = value;
  }
  mojom::blink::PreferredDisplaySurface preferred_display_surface() const {
    return preferred_display_surface_;
  }

  void set_dynamic_surface_switching_requested(bool value) {
    dynamic_surface_switching_requested_ = value;
  }
  bool dynamic_surface_switching_requested() const {
    return dynamic_surface_switching_requested_;
  }

  void set_exclude_monitor_type_surfaces(bool value) {
    exclude_monitor_type_surfaces_ = value;
  }
  bool exclude_monitor_type_surfaces() const {
    return exclude_monitor_type_surfaces_;
  }

  void set_suppress_local_audio_playback(bool value) {
    suppress_local_audio_playback_ = value;
  }
  bool suppress_local_audio_playback() const {
    return suppress_local_audio_playback_;
  }

  // Mark this request as an GetOpenDevice request for initializing a
  // TransferredMediaStreamTrack from the deviced identified by session_id.
  void SetTransferData(const base::UnguessableToken& session_id,
                       const base::UnguessableToken& transfer_id,
                       TransferredMediaStreamTrack* track) {
    transferred_track_session_id_ = session_id;
    transferred_track_transfer_id_ = transfer_id;
    transferred_track_ = track;
  }
  std::optional<base::UnguessableToken> GetSessionId() const {
    return transferred_track_session_id_;
  }
  std::optional<base::UnguessableToken> GetTransferId() const {
    return transferred_track_transfer_id_;
  }
  bool IsTransferredTrackRequest() const {
    return !!transferred_track_session_id_;
  }
  void SetTransferredTrackComponent(MediaStreamComponent* component);
  // Completes the re-creation of the transferred MediaStreamTrack by
  // constructing the MediaStreamTrackImpl object.
  void FinalizeTransferredTrackInitialization(
      const GCedMediaStreamDescriptorVector& streams_descriptors);

  void Trace(Visitor*) const override;

 private:
  UserMediaRequestType media_type_;
  MediaConstraints audio_;
  MediaConstraints video_;
  const Member<CaptureController> capture_controller_;
  const bool should_prefer_current_tab_ = false;
  bool exclude_system_audio_ = false;
  bool exclude_self_browser_surface_ = false;
  mojom::blink::PreferredDisplaySurface preferred_display_surface_ =
      mojom::blink::PreferredDisplaySurface::NO_PREFERENCE;
  bool dynamic_surface_switching_requested_ = true;
  bool exclude_monitor_type_surfaces_ = false;
  bool suppress_local_audio_playback_ = false;
  const bool auto_select_all_screens_ = false;
  bool should_disable_hardware_noise_suppression_;
  bool has_transient_user_activation_ = false;
  int32_t request_id_ = -1;

  Member<UserMediaClient> client_;

  Member<Callbacks> callbacks_;
  IdentifiableSurface surface_;
  bool is_resolved_ = false;

  std::optional<base::UnguessableToken> transferred_track_session_id_;
  std::optional<base::UnguessableToken> transferred_track_transfer_id_;
  Member<TransferredMediaStreamTrack> transferred_track_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_USER_MEDIA_REQUEST_H_

// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_FRAME_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_FRAME_PROVIDER_H_

#include <optional>

#include "base/time/time.h"
#include "device/vr/public/mojom/vr_service.mojom-blink.h"
#include "gpu/command_buffer/client/client_shared_image.h"
#include "third_party/blink/renderer/modules/xr/average_timer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/disallow_new_wrapper.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class LocalDOMWindow;
class XRFrameTransport;
class XRGPUProjectionLayer;
class XRProjectionLayer;
class XRSession;
class XRSystem;
class XRWebGLLayer;
class XRWebGLLayerClient;

// This class manages requesting and dispatching frame updates, which includes
// pose information for a given XRDevice.
class XRFrameProvider final : public GarbageCollected<XRFrameProvider> {
 public:
  // Class
  class ImmersiveSessionObserver : public GarbageCollectedMixin {
   public:
    virtual void OnImmersiveSessionStart() = 0;
    virtual void OnImmersiveSessionEnd() = 0;
    virtual void OnImmersiveFrame() = 0;
  };

  explicit XRFrameProvider(XRSystem*);

  XRSession* immersive_session() const { return immersive_session_.Get(); }

  void OnSessionStarted(XRSession* session,
                        device::mojom::blink::XRSessionPtr session_ptr);

  // The FrameProvider needs to be notified before the page does that the
  // session has been ended so that requesting a new session is possible.
  // However, the non-immersive frame loop shouldn't start until after the page
  // has been notified.
  void OnSessionEnded(XRSession* session);
  void RestartNonImmersiveFrameLoop();

  void RequestFrame(XRSession* session);

  void OnNonImmersiveVSync(double high_res_now_ms);

  void SubmitWebGLLayer(XRWebGLLayerClient*, bool was_changed);
  void UpdateWebGLLayerViewports(XRWebGLLayer*);

  void SubmitWebGPULayer(XRGPUProjectionLayer*, bool was_queried);

  // Used for both WebGPU and WebGL layers.
  void UpdateLayerViewports(XRProjectionLayer*);

  void Dispose();
  void OnFocusChanged();

  device::mojom::blink::XRFrameDataProvider* GetImmersiveDataProvider() {
    return immersive_data_provider_.get();
  }

  // Adds an ImmersiveSessionObserver. Observers will be automatically removed
  // by Oilpan when they are destroyed, and their WeakMember becomes null.
  void AddImmersiveSessionObserver(ImmersiveSessionObserver*);

  bool DrawingIntoSharedBuffer() const;

  void Trace(Visitor*) const;

 private:
  enum class ScheduledFrameType {
    kInline = 0,
    kImmersive = 1,
  };

  void OnImmersiveFrameData(device::mojom::blink::XRFrameDataPtr data);
  void OnNonImmersiveFrameData(XRSession* session,
                               device::mojom::blink::XRFrameDataPtr data);

  // Posts a request to the |XRFrameDataProvider| for the given session for
  // frame data. If the given session has no provider, it will be given null
  // frame data.
  void RequestNonImmersiveFrameData(XRSession* session);

  // TODO(https://crbug.com/955819): options should be removed from those
  // methods as they'll no longer be passed on a per-frame basis.
  void ScheduleImmersiveFrame(
      device::mojom::blink::XRFrameDataRequestOptionsPtr options);

  // Schedules an animation frame to service all non-immersive requesting
  // sessions. This will be postponed if there is a currently running immmersive
  // session.
  void ScheduleNonImmersiveFrame(
      device::mojom::blink::XRFrameDataRequestOptionsPtr options);

  // Sends the frame data to the requesting sessions for calculating
  // diagnostics.
  void SendFrameData();
  void OnRenderComplete();

  void OnProviderConnectionError(XRSession* session);
  void ProcessScheduledFrame(device::mojom::blink::XRFrameDataPtr frame_data,
                             double high_res_now_ms,
                             ScheduledFrameType type);

  // Called before dispatching a frame to an inline session. This method ensures
  // that inline session frame calls can be scheduled and that they are neither
  // served nor dropped if an immersive session is started while the inline
  // session was waiting to be served.
  void OnPreDispatchInlineFrame(XRSession* session, double timestamp);

  // Updates the |first_immersive_frame_time_| and
  // |first_immersive_frame_time_delta_| members and returns the computed high
  // resolution timestamp for the received frame. The result corresponds to
  // WebXR's XRFrame's `time` attribute.
  double UpdateImmersiveFrameTime(
      LocalDOMWindow* window,
      const device::mojom::blink::XRFrameData& data);

  const Member<XRSystem> xr_;

  // Immersive session state
  Member<XRSession> immersive_session_;
  Member<XRFrameTransport> frame_transport_;
  HeapMojoRemote<device::mojom::blink::XRFrameDataProvider>
      immersive_data_provider_;
  HeapMojoRemote<device::mojom::blink::XRPresentationProvider>
      immersive_presentation_provider_;

  // Note: Oilpan automatically removes destroyed observers from
  // |immersive_observers_| and does not need an explicit removal.
  HeapHashSet<WeakMember<ImmersiveSessionObserver>> immersive_observers_;

  // Time the first immersive frame has arrived - used to align the monotonic
  // clock the devices use with the base::TimeTicks.
  std::optional<base::TimeTicks> first_immersive_frame_time_;
  // The time_delta value of the first immersive frame that has arrived.
  std::optional<base::TimeDelta> first_immersive_frame_time_delta_;

  // Non-immersive session state
  HeapHashMap<Member<XRSession>,
              Member<DisallowNewWrapper<
                  HeapMojoRemote<device::mojom::blink::XRFrameDataProvider>>>>
      non_immersive_data_providers_;
  HeapHashMap<Member<XRSession>, device::mojom::blink::XRFrameDataPtr>
      requesting_sessions_;

  // This frame ID is XR-specific and is used to track when frames arrive at the
  // XR compositor so that it knows which poses to use, when to apply bounds
  // updates, etc.
  // TODO(https://crbug.com/1477016): Investigate making this get passed along
  // the frame stack for the places it's used rather than be a member variable.
  int16_t frame_id_ = -1;
  bool pending_immersive_vsync_ = false;
  bool pending_non_immersive_vsync_ = false;

  // TODO(crbug.com/1494911): Remove |buffer_sync_token_| and
  // |camera_image_sync_token_| once the sync tokens are incorporated
  // into |buffer_shared_image_| and |camera_image_shared_image_| respectively.
  scoped_refptr<gpu::ClientSharedImage> buffer_shared_image_;
  gpu::SyncToken buffer_sync_token_;

  scoped_refptr<gpu::ClientSharedImage> camera_image_shared_image_;
  gpu::SyncToken camera_image_sync_token_;

  bool last_has_focus_ = false;

  int num_frames_ = 0;
  int dropped_frames_ = 0;
  AverageTimer frame_data_time_;
  AverageTimer submit_frame_time_;

  base::TimeTicks last_frame_statistics_sent_time_;
  base::RepeatingTimer repeating_timer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_FRAME_PROVIDER_H_

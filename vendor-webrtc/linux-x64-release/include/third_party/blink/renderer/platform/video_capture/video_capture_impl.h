// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_VIDEO_CAPTURE_VIDEO_CAPTURE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_VIDEO_CAPTURE_VIDEO_CAPTURE_IMPL_H_

#include <stdint.h>

#include <map>

#include "base/feature_list.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ref.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/unsafe_shared_memory_pool.h"
#include "base/memory/weak_ptr.h"
#include "base/thread_annotations.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "base/token.h"
#include "media/base/video_frame.h"
#include "media/capture/mojom/video_capture.mojom-blink.h"
#include "media/capture/video_capture_types.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/platform/media/video_capture.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace media {
class GpuVideoAcceleratorFactories;
}  // namespace media

namespace blink {

class BrowserInterfaceBrokerProxy;

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class VideoCaptureStartOutcome {
  kStarted = 0,
  kTimedout = 1,
  kFailed = 2,
  kMaxValue = kFailed,
};

// VideoCaptureImpl represents a capture device in renderer process. It provides
// an interface for clients to command the capture (Start, Stop, etc), and
// communicates back to these clients e.g. the capture state or incoming
// captured VideoFrames. VideoCaptureImpl is created in the main Renderer thread
// but otherwise operates on |io_task_runner_|, which is usually the IO thread.
class PLATFORM_EXPORT VideoCaptureImpl
    : public media::mojom::blink::VideoCaptureObserver {
 public:
  VideoCaptureImpl(media::VideoCaptureSessionId session_id,
                   scoped_refptr<base::SequencedTaskRunner> main_task_runner,
                   const BrowserInterfaceBrokerProxy& browser_interface_broker);
  VideoCaptureImpl(const VideoCaptureImpl&) = delete;
  VideoCaptureImpl& operator=(const VideoCaptureImpl&) = delete;
  ~VideoCaptureImpl() override;

  // Stop/resume delivering video frames to clients, based on flag |suspend|.
  void SuspendCapture(bool suspend);

  // Start capturing using the provided parameters.
  // |client_id| must be unique to this object in the render process. It is
  // used later to stop receiving video frames.
  // |state_update_cb| will be called when state changes.
  // |deliver_frame_cb| will be called when a frame is ready.
  // |sub_capture_target_version_cb| will be called when it is guaranteed that
  // all subsequent frames |deliver_frame_cb| is called for, have a crop version
  // that is equal-to-or-greater-than the given crop version.
  // |frame_dropped_cb| will be called when a frame was dropped prior to
  // delivery (i.e. |deliver_frame_cb| was not called for this frame).
  void StartCapture(int client_id,
                    const media::VideoCaptureParams& params,
                    const VideoCaptureStateUpdateCB& state_update_cb,
                    const VideoCaptureDeliverFrameCB& deliver_frame_cb,
                    const VideoCaptureSubCaptureTargetVersionCB&
                        sub_capture_target_version_cb,
                    const VideoCaptureNotifyFrameDroppedCB& frame_dropped_cb);

  // Stop capturing. |client_id| is the identifier used to call StartCapture.
  void StopCapture(int client_id);

  // Requests that the video capturer send a frame "soon" (e.g., to resolve
  // picture loss or quality issues).
  void RequestRefreshFrame();

  // Get capturing formats supported by this device.
  // |callback| will be invoked with the results.
  //
  using VideoCaptureDeviceFormatsCallback =
      base::OnceCallback<void(const Vector<media::VideoCaptureFormat>&)>;
  void GetDeviceSupportedFormats(VideoCaptureDeviceFormatsCallback callback);

  // Get capturing formats currently in use by this device.
  // |callback| will be invoked with the results.
  void GetDeviceFormatsInUse(VideoCaptureDeviceFormatsCallback callback);

  void OnLog(const String& message);

  const media::VideoCaptureSessionId& session_id() const { return session_id_; }

  void SetVideoCaptureHostForTesting(
      media::mojom::blink::VideoCaptureHost* service) {
    video_capture_host_for_testing_ = service;
  }

  // media::mojom::VideoCaptureObserver implementation.
  void OnStateChanged(
      media::mojom::blink::VideoCaptureResultPtr result) override;
  void OnNewBuffer(
      int32_t buffer_id,
      media::mojom::blink::VideoBufferHandlePtr buffer_handle) override;
  void OnBufferReady(media::mojom::blink::ReadyBufferPtr buffer) override;
  void OnBufferDestroyed(int32_t buffer_id) override;
  void OnFrameDropped(media::VideoCaptureFrameDropReason reason) override;
  void OnNewSubCaptureTargetVersion(
      uint32_t sub_capture_target_version) override;

  void ProcessFeedback(const media::VideoCaptureFeedback& feedback);

  // The returned weak pointer can only be dereferenced on the IO thread.
  base::WeakPtr<VideoCaptureImpl> GetWeakPtr();

  static constexpr base::TimeDelta kCaptureStartTimeout = base::Seconds(10);

 private:
  friend class VideoCaptureImplTest;
  friend class MockVideoCaptureImpl;

  struct BufferContext;
  struct VideoFrameInitData;

  // Contains information about a video capture client, including capture
  // parameters callbacks to the client.
  struct ClientInfo;
  using ClientInfoMap ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327)") =
      std::map<int, ClientInfo>;

  using BufferFinishedCallback = base::OnceClosure;

  bool ProcessBuffer(media::mojom::blink::ReadyBufferPtr ready_buffer);
  static bool BindVideoFrameOnMediaTaskRunner(
      media::GpuVideoAcceleratorFactories* gpu_factories,
      gfx::GpuMemoryBufferHandle gmb_handle,
      VideoFrameInitData& video_frame_init_data,
      base::OnceCallback<void()> on_gmb_not_supported);

  void OnVideoFrameReady(base::TimeTicks reference_time,
                         VideoFrameInitData video_frame_init_data);

  void OnAllClientsFinishedConsumingFrame(
      int buffer_id,
      scoped_refptr<BufferContext> buffer_context);

  void StopDevice();
  void RestartCapture();
  void StartCaptureInternal();

  void OnDeviceSupportedFormats(
      VideoCaptureDeviceFormatsCallback callback,
      const Vector<media::VideoCaptureFormat>& supported_formats);

  void OnDeviceFormatsInUse(
      VideoCaptureDeviceFormatsCallback callback,
      const Vector<media::VideoCaptureFormat>& formats_in_use);

  // Tries to remove |client_id| from |clients|, returning false if not found.
  bool RemoveClient(int client_id, ClientInfoMap* clients);

  media::mojom::blink::VideoCaptureHost* GetVideoCaptureHost();

  // Called (by an unknown thread) when all consumers are done with a VideoFrame
  // and its ref-count has gone to zero.  This helper function grabs the
  // RESOURCE_UTILIZATION value from the |metadata| and then runs the given
  // callback, to trampoline back to the IO thread with the values.
  static void DidFinishConsumingFrame(
      BufferFinishedCallback callback_to_io_thread);

  void OnStartTimedout();

  void RecordStartOutcomeUMA(media::VideoCaptureError error_code);

  // Callback for when GPU context lost is detected. The method fetches the new
  // GPU factories handle on |main_task_runner_| and sets |gpu_factories_| to
  // the new handle.
  static void OnGpuContextLost(
      base::WeakPtr<VideoCaptureImpl> video_capture_impl);

  void SetGpuFactoriesHandleOnIOTaskRunner(
      media::GpuVideoAcceleratorFactories* gpu_factories);

  // Performs RequirePremappedFrames() and sets
  // `mappable_buffers_not_supported_`.
  void OnMappableBuffersNotSupported();
  // Sets fallback mode which will make it always request
  // premapped frames from the capturer.
  void RequirePremappedFrames();

  // Generates feedback accounding for premapped frames requirement.
  media::VideoCaptureFeedback DefaultFeedback();

  // |device_id_| and |session_id_| are different concepts, but we reuse the
  // same numerical value, passed on construction.
  const base::UnguessableToken device_id_;
  const base::UnguessableToken session_id_;

  // |video_capture_host_| is an IO-thread mojo::Remote to a remote service
  // implementation and is created by binding |pending_video_capture_host_|,
  // unless a |video_capture_host_for_testing_| has been injected.
  mojo::PendingRemote<media::mojom::blink::VideoCaptureHost>
      pending_video_capture_host_;
  mojo::Remote<media::mojom::blink::VideoCaptureHost> video_capture_host_;
  raw_ptr<media::mojom::blink::VideoCaptureHost>
      video_capture_host_for_testing_;

  mojo::Receiver<media::mojom::blink::VideoCaptureObserver> observer_receiver_{
      this};

  // Buffers available for sending to the client.
  using ClientBufferMap ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327)") =
      std::map<int32_t, scoped_refptr<BufferContext>>;
  ClientBufferMap client_buffers_;

  ClientInfoMap clients_;
  ClientInfoMap clients_pending_on_restart_;

  // Video format requested by the client to this class via StartCapture().
  media::VideoCaptureParams params_;

  // First captured frame reference time sent from browser process side.
  base::TimeTicks first_frame_ref_time_;

  VideoCaptureState state_;
  bool start_outcome_reported_ = false;

  int num_first_frame_logs_ = 0;

  // Methods of |gpu_factories_| need to run on |media_task_runner_|.
  raw_ptr<media::GpuVideoAcceleratorFactories> gpu_factories_ = nullptr;
  scoped_refptr<base::SequencedTaskRunner> media_task_runner_;
  scoped_refptr<base::SequencedTaskRunner> main_task_runner_;
  bool mappable_buffers_not_supported_ = false;

  // Stores feedback from the clients, received in |ProcessFeedback()|.
  // Only accessed on the IO thread.
  media::VideoCaptureFeedback feedback_;

  bool require_premapped_frames_ = false;

  THREAD_CHECKER(io_thread_checker_);

  base::OneShotTimer startup_timeout_;

  base::WeakPtr<VideoCaptureImpl> weak_this_;
  // WeakPtrFactory pointing back to |this| object, for use with
  // media::VideoFrames constructed in OnBufferReceived() from buffers cached
  // in |client_buffers_|.
  base::WeakPtrFactory<VideoCaptureImpl> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_VIDEO_CAPTURE_VIDEO_CAPTURE_IMPL_H_

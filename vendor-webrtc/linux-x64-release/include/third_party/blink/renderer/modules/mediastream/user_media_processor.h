// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_USER_MEDIA_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_USER_MEDIA_PROCESSOR_H_

#include <memory>
#include <utility>

#include "base/functional/callback_forward.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "build/build_config.h"
#include "third_party/blink/public/common/mediastream/media_stream_request.h"
#include "third_party/blink/public/mojom/mediastream/media_devices.mojom-blink.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-blink.h"
#include "third_party/blink/public/platform/modules/mediastream/web_platform_media_stream_source.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_constraints_util_audio.h"
#include "third_party/blink/renderer/modules/mediastream/user_media_request.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_descriptor.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AudioCaptureSettings;
class LocalFrame;
class MediaStreamAudioSource;
class MediaStreamVideoSource;
class VideoCaptureSettings;
class WebMediaStreamDeviceObserver;
class WebMediaStreamSource;
class WebString;

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class CameraCaptureCapability {
  kHdAndFullHdMissing = 0,
  kHdOrFullHd = 1,
  kHdOrFullHd_360p = 2,
  kHdOrFullHd_480p = 3,
  kHdOrFullHd_360p_480p = 4,
  kMaxValue = kHdOrFullHd_360p_480p,
};

// UserMediaProcessor is responsible for processing getUserMedia() requests.
// It also keeps tracks of all sources used by streams created with
// getUserMedia().
// It communicates with the browser via MediaStreamDispatcherHost.
// Only one MediaStream at a time can be in the process of being created.
// UserMediaProcessor must be created, called and destroyed on the main
// render thread. There should be only one UserMediaProcessor per frame.
class MODULES_EXPORT UserMediaProcessor
    : public GarbageCollected<UserMediaProcessor> {
  USING_PRE_FINALIZER(UserMediaProcessor, StopAllProcessing);

 public:
  using MediaDevicesDispatcherCallback = base::RepeatingCallback<
      blink::mojom::blink::MediaDevicesDispatcherHost*()>;
  using KeepDeviceAliveForTransferCallback = base::OnceCallback<void(bool)>;

  // |frame| must outlive this instance.
  UserMediaProcessor(LocalFrame* frame,
                     MediaDevicesDispatcherCallback media_devices_dispatcher_cb,
                     scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  UserMediaProcessor(const UserMediaProcessor&) = delete;
  UserMediaProcessor& operator=(const UserMediaProcessor&) = delete;

  virtual ~UserMediaProcessor();

  // It can be assumed that the output of CurrentRequest() remains the same
  // during the execution of a task on the main thread unless ProcessRequest or
  // DeleteUserMediaRequest are invoked.
  // TODO(guidou): Remove this method. https://crbug.com/764293
  UserMediaRequest* CurrentRequest();

  // Starts processing |request| in order to create a new MediaStream. When
  // processing of |request| is complete, it notifies by invoking |callback|.
  // This method must be called only if there is no request currently being
  // processed.
  void ProcessRequest(UserMediaRequest* request, base::OnceClosure callback);

  // If |user_media_request| is the request currently being processed, stops
  // processing the request and returns true. Otherwise, performs no action and
  // returns false.
  // TODO(guidou): Make this method private and replace with a public
  // CancelRequest() method that deletes the request only if it has not been
  // generated yet. https://crbug.com/764293
  bool DeleteUserMediaRequest(UserMediaRequest* user_media_request);

  // Stops processing the current request, if any, and stops all sources
  // currently being tracked, effectively stopping all tracks associated with
  // those sources.
  void StopAllProcessing();

  bool HasActiveSources() const;

#if !BUILDFLAG(IS_ANDROID) && !BUILDFLAG(IS_IOS)
  void FocusCapturedSurface(const String& label, bool focus);
#endif

  void OnDeviceStopped(const blink::MediaStreamDevice& device);
  void OnDeviceChanged(const blink::MediaStreamDevice& old_device,
                       const blink::MediaStreamDevice& new_device);
  void OnDeviceRequestStateChange(
      const MediaStreamDevice& device,
      const mojom::blink::MediaStreamStateChange new_state);
  void OnDeviceCaptureConfigurationChange(const MediaStreamDevice& device);
  void OnDeviceCaptureHandleChange(const MediaStreamDevice& device);
  void OnZoomLevelChange(const MediaStreamDevice& device, int zoom_level);
  void set_media_stream_dispatcher_host_for_testing(
      mojo::PendingRemote<blink::mojom::blink::MediaStreamDispatcherHost>
          dispatcher_host) {
    dispatcher_host_.Bind(std::move(dispatcher_host), task_runner_);
  }

  // Ensure the MediaStreamDevice underlying a source is not closed even if
  // there are no remaining usages from this frame, as it's in the process of
  // being transferred.
  void KeepDeviceAliveForTransfer(
      base::UnguessableToken session_id,
      base::UnguessableToken transfer_id,
      KeepDeviceAliveForTransferCallback keep_alive_cb);

  virtual void Trace(Visitor*) const;

 protected:
  // These methods are virtual for test purposes. A test can override them to
  // test requesting local media streams. The function notifies WebKit that the
  // |request| have completed.
  virtual void GetUserMediaRequestSucceeded(
      GCedMediaStreamDescriptorVector* descriptors,
      UserMediaRequest* user_media_request);
  virtual void GetUserMediaRequestFailed(
      blink::mojom::blink::MediaStreamRequestResult result,
      const String& constraint_name = String());

  // Creates a MediaStreamAudioSource/MediaStreamVideoSource objects.
  // These are virtual for test purposes.
  virtual std::unique_ptr<blink::MediaStreamAudioSource> CreateAudioSource(
      const blink::MediaStreamDevice& device,
      blink::WebPlatformMediaStreamSource::ConstraintsRepeatingCallback
          source_ready);
  virtual std::unique_ptr<blink::MediaStreamVideoSource> CreateVideoSource(
      const blink::MediaStreamDevice& device,
      blink::WebPlatformMediaStreamSource::SourceStoppedCallback stop_callback);

  // Intended to be used only for testing.
  const blink::AudioCaptureSettings& AudioCaptureSettingsForTesting() const;
  const Vector<blink::AudioCaptureSettings>&
  EligibleAudioCaptureSettingsForTesting() const;
  const blink::VideoCaptureSettings& VideoCaptureSettingsForTesting() const;
  const Vector<blink::VideoCaptureSettings>&
  EligibleVideoCaptureSettingsForTesting() const;

  void SetMediaStreamDeviceObserverForTesting(
      WebMediaStreamDeviceObserver* media_stream_device_observer);

 private:
  FRIEND_TEST_ALL_PREFIXES(UserMediaClientTest,
                           PanConstraintRequestPanTiltZoomPermission);
  FRIEND_TEST_ALL_PREFIXES(UserMediaClientTest,
                           TiltConstraintRequestPanTiltZoomPermission);
  FRIEND_TEST_ALL_PREFIXES(UserMediaClientTest,
                           ZoomConstraintRequestPanTiltZoomPermission);
  FRIEND_TEST_ALL_PREFIXES(UserMediaClientTest, MultiDeviceOnStreamsGenerated);
  class RequestInfo;
  using LocalStreamSources = HeapVector<Member<MediaStreamSource>>;

  void GotOpenDevice(int32_t request_id,
                     blink::mojom::blink::MediaStreamRequestResult result,
                     blink::mojom::blink::GetOpenDeviceResponsePtr response);

  void OnStreamsGenerated(int32_t request_id,
                          blink::mojom::blink::MediaStreamRequestResult result,
                          const String& label,
                          mojom::blink::StreamDevicesSetPtr stream_devices_set,
                          bool pan_tilt_zoom_allowed);

  void GotAllVideoInputFormatsForDevice(
      UserMediaRequest* user_media_request,
      const String& label,
      const Vector<String>& device_ids,
      const Vector<media::VideoCaptureFormat>& formats);

  void OnStreamGenerationFailed(
      int32_t request_id,
      blink::mojom::blink::MediaStreamRequestResult result);

  bool IsCurrentRequestInfo(int32_t request_id) const;
  bool IsCurrentRequestInfo(UserMediaRequest* user_media_request) const;
  void DelayedGetUserMediaRequestSucceeded(
      int32_t request_id,
      GCedMediaStreamDescriptorVector* descriptors,
      UserMediaRequest* user_media_request);
  void DelayedGetUserMediaRequestFailed(
      int32_t request_id,
      UserMediaRequest* user_media_request,
      blink::mojom::blink::MediaStreamRequestResult result,
      const String& constraint_name);

  // Called when |source| has been stopped from JavaScript.
  void OnLocalSourceStopped(const blink::WebMediaStreamSource& source);

  // Creates a WebKit representation of a stream source based on
  // |device| from the MediaStreamDispatcherHost.
  MediaStreamSource* InitializeVideoSourceObject(
      const blink::MediaStreamDevice& device);

  MediaStreamSource* InitializeAudioSourceObject(
      const blink::MediaStreamDevice& device,
      bool* is_pending);

  void StartTracks(const String& label);

  blink::MediaStreamComponent* CreateVideoTrack(
      const std::optional<blink::MediaStreamDevice>& device);

  blink::MediaStreamComponent* CreateAudioTrack(
      const std::optional<blink::MediaStreamDevice>& device);

  // Callback function triggered when all native versions of the
  // underlying media sources and tracks have been created and started.
  void OnCreateNativeTracksCompleted(
      const String& label,
      RequestInfo* request,
      blink::mojom::blink::MediaStreamRequestResult result,
      const String& result_name);

  void OnStreamGeneratedForCancelledRequest(
      const blink::mojom::blink::StreamDevices& stream_devices);

  static void OnAudioSourceStartedOnAudioThread(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      UserMediaProcessor* weak_ptr,
      blink::WebPlatformMediaStreamSource* source,
      blink::mojom::blink::MediaStreamRequestResult result,
      const blink::WebString& result_name);

  void OnAudioSourceStarted(
      blink::WebPlatformMediaStreamSource* source,
      blink::mojom::blink::MediaStreamRequestResult result,
      const String& result_name);

  void OnVideoSourceStarted(
      blink::WebPlatformMediaStreamSource* source,
      blink::mojom::blink::MediaStreamRequestResult result);

  void NotifyCurrentRequestInfoOfAudioSourceStarted(
      blink::WebPlatformMediaStreamSource* source,
      blink::mojom::blink::MediaStreamRequestResult result,
      const String& result_name);

  void DeleteAllUserMediaRequests();

  // Returns the source that use a device with |device.session_id|
  // and |device.device.id|. nullptr if such source doesn't exist.
  MediaStreamSource* FindLocalSource(const MediaStreamDevice& device) const {
    return FindLocalSource(local_sources_, device);
  }
  MediaStreamSource* FindPendingLocalSource(
      const MediaStreamDevice& device) const {
    return FindLocalSource(pending_local_sources_, device);
  }
  MediaStreamSource* FindLocalSource(
      const LocalStreamSources& sources,
      const blink::MediaStreamDevice& device) const;

  MediaStreamSource* InitializeSourceObject(
      const MediaStreamDevice& device,
      std::unique_ptr<WebPlatformMediaStreamSource> platform_source);

  // Returns true if we do find and remove the |source|.
  // Otherwise returns false.
  bool RemoveLocalSource(MediaStreamSource* source);

  void StopLocalSource(MediaStreamSource* source, bool notify_dispatcher);

  blink::mojom::blink::MediaStreamDispatcherHost*
  GetMediaStreamDispatcherHost();
  blink::mojom::blink::MediaDevicesDispatcherHost* GetMediaDevicesDispatcher();

  void SetupAudioInput();
  void SelectAudioDeviceSettings(
      UserMediaRequest* user_media_request,
      Vector<blink::mojom::blink::AudioInputDeviceCapabilitiesPtr>
          audio_input_capabilities);
  void SelectAudioSettings(
      UserMediaRequest* user_media_request,
      const blink::AudioDeviceCaptureCapabilities& capabilities);

  void SetupVideoInput();
  // Exported for testing.
  static bool IsPanTiltZoomPermissionRequested(
      const MediaConstraints& constraints);
  void SelectVideoDeviceSettings(
      UserMediaRequest* user_media_request,
      Vector<blink::mojom::blink::VideoInputDeviceCapabilitiesPtr>
          video_input_capabilities);
  void FinalizeSelectVideoDeviceSettings(
      UserMediaRequest* user_media_request,
      const blink::VideoCaptureSettings& settings);
  void SelectVideoContentSettings();

  std::optional<base::UnguessableToken> DetermineExistingAudioSessionId(
      const blink::AudioCaptureSettings& settings);

  WTF::HashMap<String, base::UnguessableToken>
  DetermineExistingAudioSessionIds();

  void GenerateStreamForCurrentRequestInfo(
      WTF::HashMap<String, base::UnguessableToken>
          requested_audio_capture_session_ids = {});

  WebMediaStreamDeviceObserver* GetMediaStreamDeviceObserver();

  // Owned by the test.
  raw_ptr<WebMediaStreamDeviceObserver, DanglingUntriaged>
      media_stream_device_observer_for_testing_ = nullptr;

  LocalStreamSources local_sources_;
  LocalStreamSources pending_local_sources_;

  HeapMojoRemote<blink::mojom::blink::MediaStreamDispatcherHost>
      dispatcher_host_;

  // UserMedia requests are processed sequentially. |current_request_info_|
  // contains the request currently being processed.
  Member<RequestInfo> current_request_info_;
  MediaDevicesDispatcherCallback media_devices_dispatcher_cb_;
  base::OnceClosure request_completed_cb_;

  Member<LocalFrame> frame_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  THREAD_CHECKER(thread_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_USER_MEDIA_PROCESSOR_H_

// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_DEVICE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_DEVICE_OBSERVER_H_

#include "base/gtest_prod_util.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/common/mediastream/media_stream_request.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-blink.h"
#include "third_party/blink/public/web/modules/mediastream/web_media_stream_device_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {
class LocalFrame;
class UserMediaProcessor;

// This class implements a Mojo object that receives device stopped
// notifications and forwards them to UserMediaProcessor.
class MODULES_EXPORT MediaStreamDeviceObserver
    : public mojom::blink::MediaStreamDeviceObserver {
 public:
  explicit MediaStreamDeviceObserver(LocalFrame* frame);

  MediaStreamDeviceObserver(const MediaStreamDeviceObserver&) = delete;
  MediaStreamDeviceObserver& operator=(const MediaStreamDeviceObserver&) =
      delete;

  ~MediaStreamDeviceObserver() override;

  // Get all the media devices of video capture, e.g. webcam. This is the set
  // of devices that should be suspended when the content frame is no longer
  // being shown to the user.
  blink::MediaStreamDevices GetNonScreenCaptureDevices();

  void AddStreams(
      const String& label,
      const mojom::blink::StreamDevicesSet& stream_devices_set,
      const WebMediaStreamDeviceObserver::StreamCallbacks& stream_callbacks);
  void AddStream(const String& label, const blink::MediaStreamDevice& device);
  bool RemoveStreams(const String& label);
  void RemoveStreamDevice(const blink::MediaStreamDevice& device);

  // Get the video session_id given a label. The label identifies a stream.
  // If the label does not designate a valid video session, an empty token
  // will be returned.
  base::UnguessableToken GetVideoSessionId(const String& label);
  // Returns an audio session_id given a label.
  // If the label does not designate a valid video session, an empty token
  // will be returned.
  base::UnguessableToken GetAudioSessionId(const String& label);

 private:
  friend class MediaStreamDeviceObserverTest;
  FRIEND_TEST_ALL_PREFIXES(MediaStreamDeviceObserverTest,
                           GetNonScreenCaptureDevices);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamDeviceObserverTest, OnDeviceStopped);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamDeviceObserverTest, OnDeviceChanged);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamDeviceObserverTest,
                           OnDeviceChangedChangesDeviceAfterRebind);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamDeviceObserverTest,
                           OnDeviceRequestStateChange);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamDeviceObserverTest,
                           MultiCaptureAddAndRemoveStreams);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamDeviceObserverTest,
                           MultiCaptureChangeDevices);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamDeviceObserverTest,
                           MultiCaptureChangeDeviceRequestState);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamDeviceObserverTest,
                           MultiCaptureRemoveStreamDevice);

  // Private class for keeping track of opened devices and who have
  // opened it.
  struct Stream {
    WebMediaStreamDeviceObserver::OnDeviceStoppedCb on_device_stopped_cb;
    WebMediaStreamDeviceObserver::OnDeviceChangedCb on_device_changed_cb;
    WebMediaStreamDeviceObserver::OnDeviceRequestStateChangeCb
        on_device_request_state_change_cb;
    WebMediaStreamDeviceObserver::OnDeviceCaptureConfigurationChangeCb
        on_device_capture_configuration_change_cb;
    WebMediaStreamDeviceObserver::OnDeviceCaptureHandleChangeCb
        on_device_capture_handle_change_cb;
#if !BUILDFLAG(IS_ANDROID) && !BUILDFLAG(IS_IOS)
    WebMediaStreamDeviceObserver::OnZoomLevelChangeCb on_zoom_level_change_cb;
#endif
    MediaStreamDevices audio_devices;
    MediaStreamDevices video_devices;

    bool ContainsDevice(const MediaStreamDevice& device) const;
  };

  // mojom::MediaStreamDeviceObserver implementation.
  void OnDeviceStopped(const String& label,
                       const MediaStreamDevice& device) override;
  void OnDeviceChanged(const String& label,
                       const MediaStreamDevice& old_device,
                       const MediaStreamDevice& new_device) override;
  void OnDeviceRequestStateChange(
      const String& label,
      const MediaStreamDevice& device,
      const mojom::blink::MediaStreamStateChange new_state) override;
  void OnDeviceCaptureConfigurationChange(
      const String& label,
      const MediaStreamDevice& device) override;
  void OnDeviceCaptureHandleChange(const String& label,
                                   const MediaStreamDevice& device) override;
  void OnZoomLevelChange(const String& label,
                         const MediaStreamDevice& device,
                         int zoom_level) override;

  void BindMediaStreamDeviceObserverReceiver(
      mojo::PendingReceiver<mojom::blink::MediaStreamDeviceObserver> receiver);

  mojo::Receiver<mojom::blink::MediaStreamDeviceObserver> receiver_{this};

  // Used for DCHECKs so methods calls won't execute in the wrong thread.
  THREAD_CHECKER(thread_checker_);

  using LabelStreamMap = HashMap<String, Vector<Stream>>;
  LabelStreamMap label_stream_map_;
  base::WeakPtrFactory<MediaStreamDeviceObserver> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_DEVICE_OBSERVER_H_

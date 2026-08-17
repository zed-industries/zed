/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_AUDIO_RTP_RECEIVER_H_
#define PC_AUDIO_RTP_RECEIVER_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "api/crypto/frame_decryptor_interface.h"
#include "api/dtls_transport_interface.h"
#include "api/frame_transformer_interface.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/rtp_parameters.h"
#include "api/rtp_receiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/transport/rtp/rtp_source.h"
#include "media/base/media_channel.h"
#include "pc/audio_track.h"
#include "pc/jitter_buffer_delay.h"
#include "pc/media_stream_track_proxy.h"
#include "pc/remote_audio_source.h"
#include "pc/rtp_receiver.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class AudioRtpReceiver : public ObserverInterface,
                         public AudioSourceInterface::AudioObserver,
                         public RtpReceiverInternal {
 public:
  // The constructor supports optionally passing the voice channel to the
  // instance at construction time without having to call `SetMediaChannel()`
  // on the worker thread straight after construction.
  // However, when using that, the assumption is that right after construction,
  // a call to either `SetupUnsignaledMediaChannel` or `SetupMediaChannel`
  // will be made, which will internally start the source on the worker thread.
  AudioRtpReceiver(Thread* worker_thread,
                   std::string receiver_id,
                   std::vector<std::string> stream_ids,
                   bool is_unified_plan,
                   VoiceMediaReceiveChannelInterface* voice_channel = nullptr);
  // TODO(https://crbug.com/webrtc/9480): Remove this when streams() is removed.
  AudioRtpReceiver(
      Thread* worker_thread,
      const std::string& receiver_id,
      const std::vector<scoped_refptr<MediaStreamInterface>>& streams,
      bool is_unified_plan,
      VoiceMediaReceiveChannelInterface* media_channel = nullptr);
  virtual ~AudioRtpReceiver();

  // ObserverInterface implementation
  void OnChanged() override;

  // AudioSourceInterface::AudioObserver implementation
  void OnSetVolume(double volume) override;

  scoped_refptr<AudioTrackInterface> audio_track() const { return track_; }

  // RtpReceiverInterface implementation
  scoped_refptr<MediaStreamTrackInterface> track() const override {
    return track_;
  }
  scoped_refptr<DtlsTransportInterface> dtls_transport() const override;
  std::vector<std::string> stream_ids() const override;
  std::vector<scoped_refptr<MediaStreamInterface>> streams() const override;

  webrtc::MediaType media_type() const override {
    return webrtc::MediaType::AUDIO;
  }

  std::string id() const override { return id_; }

  RtpParameters GetParameters() const override;

  void SetFrameDecryptor(
      scoped_refptr<FrameDecryptorInterface> frame_decryptor) override;

  scoped_refptr<FrameDecryptorInterface> GetFrameDecryptor() const override;

  // RtpReceiverInternal implementation.
  void Stop() override;
  void SetupMediaChannel(uint32_t ssrc) override;
  void SetupUnsignaledMediaChannel() override;
  std::optional<uint32_t> ssrc() const override;
  void NotifyFirstPacketReceived() override;
  void set_stream_ids(std::vector<std::string> stream_ids) override;
  void set_transport(
      scoped_refptr<DtlsTransportInterface> dtls_transport) override;
  void SetStreams(
      const std::vector<scoped_refptr<MediaStreamInterface>>& streams) override;
  void SetObserver(RtpReceiverObserverInterface* observer) override;

  void SetJitterBufferMinimumDelay(
      std::optional<double> delay_seconds) override;

  void SetMediaChannel(MediaReceiveChannelInterface* media_channel) override;

  std::vector<RtpSource> GetSources() const override;
  int AttachmentId() const override { return attachment_id_; }
  void SetFrameTransformer(
      scoped_refptr<FrameTransformerInterface> frame_transformer) override;

 private:
  void RestartMediaChannel(std::optional<uint32_t> ssrc)
      RTC_RUN_ON(&signaling_thread_checker_);
  void RestartMediaChannel_w(std::optional<uint32_t> ssrc,
                             bool track_enabled,
                             MediaSourceInterface::SourceState state)
      RTC_RUN_ON(worker_thread_);
  void Reconfigure(bool track_enabled) RTC_RUN_ON(worker_thread_);
  void SetOutputVolume_w(double volume) RTC_RUN_ON(worker_thread_);

  RTC_NO_UNIQUE_ADDRESS SequenceChecker signaling_thread_checker_;
  Thread* const worker_thread_;
  const std::string id_;
  const scoped_refptr<RemoteAudioSource> source_;
  const scoped_refptr<AudioTrackProxyWithInternal<AudioTrack>> track_;
  VoiceMediaReceiveChannelInterface* media_channel_
      RTC_GUARDED_BY(worker_thread_) = nullptr;
  std::optional<uint32_t> signaled_ssrc_ RTC_GUARDED_BY(worker_thread_);
  std::vector<scoped_refptr<MediaStreamInterface>> streams_
      RTC_GUARDED_BY(&signaling_thread_checker_);
  bool cached_track_enabled_ RTC_GUARDED_BY(&signaling_thread_checker_);
  double cached_volume_ RTC_GUARDED_BY(worker_thread_) = 1.0;
  RtpReceiverObserverInterface* observer_
      RTC_GUARDED_BY(&signaling_thread_checker_) = nullptr;
  bool received_first_packet_ RTC_GUARDED_BY(&signaling_thread_checker_) =
      false;
  const int attachment_id_;
  scoped_refptr<FrameDecryptorInterface> frame_decryptor_
      RTC_GUARDED_BY(worker_thread_);
  scoped_refptr<DtlsTransportInterface> dtls_transport_
      RTC_GUARDED_BY(&signaling_thread_checker_);
  // Stores and updates the playout delay. Handles caching cases if
  // `SetJitterBufferMinimumDelay` is called before start.
  JitterBufferDelay delay_ RTC_GUARDED_BY(worker_thread_);
  scoped_refptr<FrameTransformerInterface> frame_transformer_
      RTC_GUARDED_BY(worker_thread_);
  const scoped_refptr<PendingTaskSafetyFlag> worker_thread_safety_;
};

}  // namespace webrtc

#endif  // PC_AUDIO_RTP_RECEIVER_H_

/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains classes that implement RtpSenderInterface.
// An RtpSender associates a MediaStreamTrackInterface with an underlying
// transport (provided by AudioProviderInterface/VideoProviderInterface)

#ifndef PC_RTP_SENDER_H_
#define PC_RTP_SENDER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/crypto/frame_encryptor_interface.h"
#include "api/dtls_transport_interface.h"
#include "api/dtmf_sender_interface.h"
#include "api/environment/environment.h"
#include "api/field_trials_view.h"
#include "api/frame_transformer_interface.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/rtp_sender_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "media/base/audio_source.h"
#include "media/base/media_channel.h"
#include "pc/dtmf_sender.h"
#include "pc/legacy_stats_collector_interface.h"
#include "rtc_base/checks.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

bool UnimplementedRtpParameterHasValue(const RtpParameters& parameters);

// Internal interface used by PeerConnection.
class RtpSenderInternal : public RtpSenderInterface {
 public:
  // Sets the underlying MediaEngine channel associated with this RtpSender.
  // A VoiceMediaChannel should be used for audio RtpSenders and
  // a VideoMediaChannel should be used for video RtpSenders.
  // Must call SetMediaChannel(nullptr) before the media channel is destroyed.
  virtual void SetMediaChannel(MediaSendChannelInterface* media_channel) = 0;

  // Used to set the SSRC of the sender, once a local description has been set.
  // If `ssrc` is 0, this indiates that the sender should disconnect from the
  // underlying transport (this occurs if the sender isn't seen in a local
  // description).
  virtual void SetSsrc(uint32_t ssrc) = 0;

  virtual void set_stream_ids(const std::vector<std::string>& stream_ids) = 0;
  virtual void set_init_send_encodings(
      const std::vector<RtpEncodingParameters>& init_send_encodings) = 0;
  virtual void set_transport(
      scoped_refptr<DtlsTransportInterface> dtls_transport) = 0;

  virtual void Stop() = 0;

  // `GetParameters` and `SetParameters` operate with a transactional model.
  // Allow access to get/set parameters without invalidating transaction id.
  virtual RtpParameters GetParametersInternal() const = 0;
  virtual void SetParametersInternal(const RtpParameters& parameters,
                                     SetParametersCallback,
                                     bool blocking) = 0;

  // GetParameters and SetParameters will remove deactivated simulcast layers
  // and restore them on SetParameters. This is probably a Bad Idea, but we
  // do not know who depends on this behavior
  virtual RtpParameters GetParametersInternalWithAllLayers() const = 0;
  virtual RTCError SetParametersInternalWithAllLayers(
      const RtpParameters& parameters) = 0;

  // Additional checks that are specific to the current codec settings
  virtual RTCError CheckCodecParameters(const RtpParameters& parameters) = 0;

  // Returns an ID that changes every time SetTrack() is called, but
  // otherwise remains constant. Used to generate IDs for stats.
  // The special value zero means that no track is attached.
  virtual int AttachmentId() const = 0;

  // Disables the layers identified by the specified RIDs.
  // If the specified list is empty, this is a no-op.
  virtual RTCError DisableEncodingLayers(
      const std::vector<std::string>& rid) = 0;

  virtual void SetTransceiverAsStopped() = 0;

  // Used by the owning transceiver to inform the sender on the currently
  // selected codecs.
  virtual void SetSendCodecs(std::vector<Codec> send_codecs) = 0;
  virtual std::vector<Codec> GetSendCodecs() const = 0;

  virtual void NotifyFirstPacketSent() = 0;
};

// Shared implementation for RtpSenderInternal interface.
class RtpSenderBase : public RtpSenderInternal, public ObserverInterface {
 public:
  class SetStreamsObserver {
   public:
    virtual ~SetStreamsObserver() = default;
    virtual void OnSetStreams() = 0;
  };

  // Sets the underlying MediaEngine channel associated with this RtpSender.
  // A VoiceMediaChannel should be used for audio RtpSenders and
  // a VideoMediaChannel should be used for video RtpSenders.
  // Must call SetMediaChannel(nullptr) before the media channel is destroyed.
  void SetMediaChannel(MediaSendChannelInterface* media_channel) override;

  bool SetTrack(MediaStreamTrackInterface* track) override;
  scoped_refptr<MediaStreamTrackInterface> track() const override {
    // This method is currently called from the worker thread by
    // RTCStatsCollector::PrepareTransceiverStatsInfosAndCallStats_s_w_n.
    // RTC_DCHECK_RUN_ON(signaling_thread_);
    return track_;
  }

  RtpParameters GetParameters() const override;
  RTCError SetParameters(const RtpParameters& parameters) override;
  void SetParametersAsync(const RtpParameters& parameters,
                          SetParametersCallback callback) override;

  // `GetParameters` and `SetParameters` operate with a transactional model.
  // Allow access to get/set parameters without invalidating transaction id.
  RtpParameters GetParametersInternal() const override;
  void SetParametersInternal(const RtpParameters& parameters,
                             SetParametersCallback callback = nullptr,
                             bool blocking = true) override;
  RTCError CheckSetParameters(const RtpParameters& parameters);
  RTCError CheckCodecParameters(const RtpParameters& parameters) override;
  RtpParameters GetParametersInternalWithAllLayers() const override;
  RTCError SetParametersInternalWithAllLayers(
      const RtpParameters& parameters) override;

  // Used to set the SSRC of the sender, once a local description has been set.
  // If `ssrc` is 0, this indiates that the sender should disconnect from the
  // underlying transport (this occurs if the sender isn't seen in a local
  // description).
  void SetSsrc(uint32_t ssrc) override;
  uint32_t ssrc() const override {
    // This method is currently called from the worker thread by
    // RTCStatsCollector::PrepareTransceiverStatsInfosAndCallStats_s_w_n.
    // RTC_DCHECK_RUN_ON(signaling_thread_);
    return ssrc_;
  }

  std::vector<std::string> stream_ids() const override {
    RTC_DCHECK_RUN_ON(signaling_thread_);
    return stream_ids_;
  }

  // Set stream ids, eliminating duplicates in the process.
  void set_stream_ids(const std::vector<std::string>& stream_ids) override;
  void SetStreams(const std::vector<std::string>& stream_ids) override;

  std::string id() const override { return id_; }

  void set_init_send_encodings(
      const std::vector<RtpEncodingParameters>& init_send_encodings) override {
    init_parameters_.encodings = init_send_encodings;
  }
  std::vector<RtpEncodingParameters> init_send_encodings() const override {
    RTC_DCHECK_RUN_ON(signaling_thread_);
    return init_parameters_.encodings;
  }

  void set_transport(
      scoped_refptr<DtlsTransportInterface> dtls_transport) override {
    dtls_transport_ = dtls_transport;
  }
  scoped_refptr<DtlsTransportInterface> dtls_transport() const override {
    RTC_DCHECK_RUN_ON(signaling_thread_);
    return dtls_transport_;
  }

  void SetFrameEncryptor(
      scoped_refptr<FrameEncryptorInterface> frame_encryptor) override;

  scoped_refptr<FrameEncryptorInterface> GetFrameEncryptor() const override {
    return frame_encryptor_;
  }

  void Stop() override;

  // Returns an ID that changes every time SetTrack() is called, but
  // otherwise remains constant. Used to generate IDs for stats.
  // The special value zero means that no track is attached.
  int AttachmentId() const override { return attachment_id_; }

  // Disables the layers identified by the specified RIDs.
  // If the specified list is empty, this is a no-op.
  RTCError DisableEncodingLayers(const std::vector<std::string>& rid) override;

  void SetFrameTransformer(
      scoped_refptr<FrameTransformerInterface> frame_transformer) override;

  void SetEncoderSelector(
      std::unique_ptr<VideoEncoderFactory::EncoderSelectorInterface>
          encoder_selector) override;

  void SetEncoderSelectorOnChannel();

  void SetTransceiverAsStopped() override {
    RTC_DCHECK_RUN_ON(signaling_thread_);
    is_transceiver_stopped_ = true;
  }

  void SetSendCodecs(std::vector<Codec> send_codecs) override {
    send_codecs_ = send_codecs;
  }
  std::vector<Codec> GetSendCodecs() const override { return send_codecs_; }

  void NotifyFirstPacketSent() override;
  void SetObserver(RtpSenderObserverInterface* observer) override;

 protected:
  // If `set_streams_observer` is not null, it is invoked when SetStreams()
  // is called. `set_streams_observer` is not owned by this object. If not
  // null, it must be valid at least until this sender becomes stopped.
  RtpSenderBase(const Environment& env,
                Thread* worker_thread,
                const std::string& id,
                SetStreamsObserver* set_streams_observer);
  // TODO(bugs.webrtc.org/8694): Since SSRC == 0 is technically valid, figure
  // out some other way to test if we have a valid SSRC.
  bool can_send_track() const { return track_ && ssrc_; }

  virtual std::string track_kind() const = 0;

  // Enable sending on the media channel.
  virtual void SetSend() = 0;
  // Disable sending on the media channel.
  virtual void ClearSend() = 0;

  // Template method pattern to allow subclasses to add custom behavior for
  // when tracks are attached, detached, and for adding tracks to statistics.
  virtual void AttachTrack() {}
  virtual void DetachTrack() {}
  virtual void AddTrackToStats() {}
  virtual void RemoveTrackFromStats() {}

  const Environment env_;
  Thread* const signaling_thread_;
  Thread* const worker_thread_;
  uint32_t ssrc_ = 0;
  bool stopped_ RTC_GUARDED_BY(signaling_thread_) = false;
  bool is_transceiver_stopped_ RTC_GUARDED_BY(signaling_thread_) = false;
  int attachment_id_ = 0;
  const std::string id_;

  std::vector<std::string> stream_ids_;
  RtpParameters init_parameters_;
  std::vector<Codec> send_codecs_;

  // TODO(tommi): `media_channel_` and several other member variables in this
  // class (ssrc_, stopped_, etc) are accessed from more than one thread without
  // a guard or lock. Internally there are also several Invoke()s that we could
  // remove since the upstream code may already be performing several operations
  // on the worker thread.
  MediaSendChannelInterface* media_channel_ = nullptr;
  scoped_refptr<MediaStreamTrackInterface> track_;

  scoped_refptr<DtlsTransportInterface> dtls_transport_;
  scoped_refptr<FrameEncryptorInterface> frame_encryptor_;
  // `last_transaction_id_` is used to verify that `SetParameters` is receiving
  // the parameters object that was last returned from `GetParameters`.
  // As such, it is used for internal verification and is not observable by the
  // the client. It is marked as mutable to enable `GetParameters` to be a
  // const method.
  mutable std::optional<std::string> last_transaction_id_;
  std::vector<std::string> disabled_rids_;

  SetStreamsObserver* set_streams_observer_ = nullptr;
  RtpSenderObserverInterface* observer_ = nullptr;
  bool sent_first_packet_ = false;

  scoped_refptr<FrameTransformerInterface> frame_transformer_;
  std::unique_ptr<VideoEncoderFactory::EncoderSelectorInterface>
      encoder_selector_;

  virtual RTCError GenerateKeyFrame(const std::vector<std::string>& rids) = 0;
};

// LocalAudioSinkAdapter receives data callback as a sink to the local
// AudioTrack, and passes the data to the sink of AudioSource.
class LocalAudioSinkAdapter : public AudioTrackSinkInterface,
                              public AudioSource {
 public:
  LocalAudioSinkAdapter();
  virtual ~LocalAudioSinkAdapter();

 private:
  // AudioSinkInterface implementation.
  void OnData(const void* audio_data,
              int bits_per_sample,
              int sample_rate,
              size_t number_of_channels,
              size_t number_of_frames,
              std::optional<int64_t> absolute_capture_timestamp_ms) override;

  // AudioSinkInterface implementation.
  void OnData(const void* audio_data,
              int bits_per_sample,
              int sample_rate,
              size_t number_of_channels,
              size_t number_of_frames) override {
    OnData(audio_data, bits_per_sample, sample_rate, number_of_channels,
           number_of_frames,
           /*absolute_capture_timestamp_ms=*/std::nullopt);
  }

  // AudioSinkInterface implementation.
  int NumPreferredChannels() const override { return num_preferred_channels_; }

  // webrtc::AudioSource implementation.
  void SetSink(AudioSource::Sink* sink) override;

  AudioSource::Sink* sink_;
  // Critical section protecting `sink_`.
  Mutex lock_;
  int num_preferred_channels_ = -1;
};

class AudioRtpSender : public DtmfProviderInterface, public RtpSenderBase {
 public:
  // Construct an RtpSender for audio with the given sender ID.
  // The sender is initialized with no track to send and no associated streams.
  // StatsCollector provided so that Add/RemoveLocalAudioTrack can be called
  // at the appropriate times.
  // If `set_streams_observer` is not null, it is invoked when SetStreams()
  // is called. `set_streams_observer` is not owned by this object. If not
  // null, it must be valid at least until this sender becomes stopped.
  static scoped_refptr<AudioRtpSender> Create(
      const Environment& env,
      Thread* worker_thread,
      const std::string& id,
      LegacyStatsCollectorInterface* stats,
      SetStreamsObserver* set_streams_observer);
  virtual ~AudioRtpSender();

  // DtmfSenderProvider implementation.
  bool CanInsertDtmf() override;
  bool InsertDtmf(int code, int duration) override;

  // ObserverInterface implementation.
  void OnChanged() override;

  webrtc::MediaType media_type() const override {
    return webrtc::MediaType::AUDIO;
  }
  std::string track_kind() const override {
    return MediaStreamTrackInterface::kAudioKind;
  }

  scoped_refptr<DtmfSenderInterface> GetDtmfSender() const override;
  RTCError GenerateKeyFrame(const std::vector<std::string>& rids) override;

 protected:
  AudioRtpSender(const Environment& env,
                 Thread* worker_thread,
                 const std::string& id,
                 LegacyStatsCollectorInterface* legacy_stats,
                 SetStreamsObserver* set_streams_observer);

  void SetSend() override;
  void ClearSend() override;

  // Hooks to allow custom logic when tracks are attached and detached.
  void AttachTrack() override;
  void DetachTrack() override;
  void AddTrackToStats() override;
  void RemoveTrackFromStats() override;

 private:
  VoiceMediaSendChannelInterface* voice_media_channel() {
    return media_channel_->AsVoiceSendChannel();
  }
  scoped_refptr<AudioTrackInterface> audio_track() const {
    return scoped_refptr<AudioTrackInterface>(
        static_cast<AudioTrackInterface*>(track_.get()));
  }

  LegacyStatsCollectorInterface* legacy_stats_ = nullptr;
  scoped_refptr<DtmfSender> dtmf_sender_;
  scoped_refptr<DtmfSenderInterface> dtmf_sender_proxy_;
  bool cached_track_enabled_ = false;

  // Used to pass the data callback from the `track_` to the other end of
  // webrtc::AudioSource.
  std::unique_ptr<LocalAudioSinkAdapter> sink_adapter_;
};

class VideoRtpSender : public RtpSenderBase {
 public:
  // Construct an RtpSender for video with the given sender ID.
  // The sender is initialized with no track to send and no associated streams.
  // If `set_streams_observer` is not null, it is invoked when SetStreams()
  // is called. `set_streams_observer` is not owned by this object. If not
  // null, it must be valid at least until this sender becomes stopped.
  static scoped_refptr<VideoRtpSender> Create(
      const Environment& env,
      Thread* worker_thread,
      const std::string& id,
      SetStreamsObserver* set_streams_observer);
  virtual ~VideoRtpSender();

  // ObserverInterface implementation
  void OnChanged() override;

  webrtc::MediaType media_type() const override {
    return webrtc::MediaType::VIDEO;
  }
  std::string track_kind() const override {
    return MediaStreamTrackInterface::kVideoKind;
  }

  scoped_refptr<DtmfSenderInterface> GetDtmfSender() const override;
  RTCError GenerateKeyFrame(const std::vector<std::string>& rids) override;

 protected:
  VideoRtpSender(const Environment& env,
                 Thread* worker_thread,
                 const std::string& id,
                 SetStreamsObserver* set_streams_observer);

  void SetSend() override;
  void ClearSend() override;

  // Hook to allow custom logic when tracks are attached.
  void AttachTrack() override;

 private:
  VideoMediaSendChannelInterface* video_media_channel() {
    return media_channel_->AsVideoSendChannel();
  }
  scoped_refptr<VideoTrackInterface> video_track() const {
    return scoped_refptr<VideoTrackInterface>(
        static_cast<VideoTrackInterface*>(track_.get()));
  }

  VideoTrackInterface::ContentHint cached_track_content_hint_ =
      VideoTrackInterface::ContentHint::kNone;
};

}  // namespace webrtc

#endif  // PC_RTP_SENDER_H_

// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_RECEIVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_RECEIVER_H_

#include <optional>
#include <vector>

#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_contributing_source.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_receive_parameters.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_synchronization_source.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_transceiver_direction.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_script_transform.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_encoded_audio_stream_transformer.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_encoded_video_stream_transformer.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_receiver_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_source.h"
#include "third_party/webrtc/api/media_types.h"

namespace blink {
class RTCDtlsTransport;
class RTCEncodedAudioUnderlyingSource;
class RTCEncodedAudioUnderlyingSink;
class RTCEncodedVideoUnderlyingSource;
class RTCEncodedVideoUnderlyingSink;
class RTCInsertableStreams;
class RTCPeerConnection;
class RTCRtpCapabilities;
class RTCRtpScriptTransform;
class RTCRtpTransceiver;
class RTCStatsReport;

// https://w3c.github.io/webrtc-pc/#rtcrtpreceiver-interface
class RTCRtpReceiver final : public ScriptWrappable,
                             public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class MediaKind { kAudio, kVideo };

  // If |require_encoded_insertable_streams| is true, no received frames will be
  // passed to the decoder until |createEncodedStreams()| has been called and
  // the frames have been transformed and passed back to the returned
  // WritableStream. If it's false, during construction a task will be posted to
  // |encoded_transform_shortcircuit_runner| to check if
  // |createEncodedStreams()| has been called yet and if not will tell the
  // underlying WebRTC receiver to 'short circuit' the transform, so frames will
  // flow directly to the decoder.
  RTCRtpReceiver(RTCPeerConnection*,
                 std::unique_ptr<RTCRtpReceiverPlatform>,
                 MediaStreamTrack*,
                 MediaStreamVector,
                 bool require_encoded_insertable_streams,
                 scoped_refptr<base::SequencedTaskRunner>
                     encoded_transform_shortcircuit_runner);

  static RTCRtpCapabilities* getCapabilities(ScriptState* state,
                                             const String& kind);

  MediaStreamTrack* track() const;
  RTCDtlsTransport* transport();
  RTCDtlsTransport* rtcpTransport();
  std::optional<double> playoutDelayHint() const;
  void setPlayoutDelayHint(std::optional<double>, ExceptionState&);
  std::optional<double> jitterBufferTarget() const;
  void setJitterBufferTarget(std::optional<double>, ExceptionState&);
  RTCRtpReceiveParameters* getParameters();
  HeapVector<Member<RTCRtpSynchronizationSource>> getSynchronizationSources(
      ScriptState*,
      ExceptionState&);
  HeapVector<Member<RTCRtpContributingSource>> getContributingSources(
      ScriptState*,
      ExceptionState&);
  ScriptPromise<RTCStatsReport> getStats(ScriptState*);
  RTCInsertableStreams* createEncodedStreams(ScriptState*, ExceptionState&);

  RTCRtpScriptTransform* transform() { return transform_; }
  void setTransform(RTCRtpScriptTransform*, ExceptionState&);

  RTCRtpReceiverPlatform* platform_receiver();
  MediaKind kind() const;
  MediaStreamVector streams() const;
  void set_streams(MediaStreamVector streams);
  void set_transceiver(RTCRtpTransceiver*);
  void set_transport(RTCDtlsTransport*);

  V8RTCRtpTransceiverDirection TransceiverDirection();
  std::optional<V8RTCRtpTransceiverDirection> TransceiverCurrentDirection();

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  void Trace(Visitor*) const override;

 private:
  // Insertable Streams audio support methods
  RTCInsertableStreams* CreateEncodedAudioStreams(ScriptState*);
  void RegisterEncodedAudioStreamCallback();
  void UnregisterEncodedAudioStreamCallback();
  void SetAudioUnderlyingSource(
      RTCEncodedAudioUnderlyingSource* new_underlying_source,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  void SetAudioUnderlyingSink(
      RTCEncodedAudioUnderlyingSink* new_underlying_sink);
  void OnAudioFrameFromDepacketizer(
      std::unique_ptr<webrtc::TransformableAudioFrameInterface>
          encoded_audio_frame);

  // Insertable Streams video support methods
  RTCInsertableStreams* CreateEncodedVideoStreams(ScriptState*);
  void RegisterEncodedVideoStreamCallback();
  void UnregisterEncodedVideoStreamCallback();
  void SetVideoUnderlyingSource(
      RTCEncodedVideoUnderlyingSource* new_underlying_source,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  void SetVideoUnderlyingSink(
      RTCEncodedVideoUnderlyingSink* new_underlying_sink);
  void OnVideoFrameFromDepacketizer(
      std::unique_ptr<webrtc::TransformableVideoFrameInterface>
          encoded_video_frame);

  void LogMessage(const std::string& message);

  // If createEncodedStreams has not yet been called, instead tell the webrtc
  // encoded transform to 'short circuit', skipping calling the transform.
  void MaybeShortCircuitEncodedStreams();

  Member<RTCPeerConnection> pc_;
  std::unique_ptr<RTCRtpReceiverPlatform> receiver_;
  Member<MediaStreamTrack> track_;
  Member<RTCDtlsTransport> transport_;
  MediaStreamVector streams_;

  // The current SSRCs and CSRCs. getSynchronizationSources() returns the SSRCs
  // and getContributingSources() returns the CSRCs.
  Vector<std::unique_ptr<RTCRtpSource>> web_sources_;
  Member<RTCRtpTransceiver> transceiver_;

  // Hint to the WebRTC Jitter Buffer about desired playout delay. Actual
  // observed delay may differ depending on the congestion control. |nullopt|
  // means default value must be used.
  std::optional<double> playout_delay_hint_;
  std::optional<double> jitter_buffer_target_;

  THREAD_CHECKER(thread_checker_);
  Member<RTCInsertableStreams> encoded_streams_;
  Member<RTCRtpScriptTransform> transform_;

  // Insertable Streams support for audio.
  base::Lock audio_underlying_source_lock_;
  CrossThreadPersistent<RTCEncodedAudioUnderlyingSource>
      audio_from_depacketizer_underlying_source_
          GUARDED_BY(audio_underlying_source_lock_);
  base::Lock audio_underlying_sink_lock_;
  CrossThreadPersistent<RTCEncodedAudioUnderlyingSink>
      audio_to_decoder_underlying_sink_ GUARDED_BY(audio_underlying_sink_lock_);
  const scoped_refptr<blink::RTCEncodedAudioStreamTransformer::Broker>
      encoded_audio_transformer_;

  // Insertable Streams support for video.
  base::Lock video_underlying_source_lock_;
  CrossThreadPersistent<RTCEncodedVideoUnderlyingSource>
      video_from_depacketizer_underlying_source_
          GUARDED_BY(video_underlying_source_lock_);
  base::Lock video_underlying_sink_lock_;
  CrossThreadPersistent<RTCEncodedVideoUnderlyingSink>
      video_to_decoder_underlying_sink_ GUARDED_BY(video_underlying_sink_lock_);
  const scoped_refptr<blink::RTCEncodedVideoStreamTransformer::Broker>
      encoded_video_transformer_;
  bool transform_shortcircuited_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_RECEIVER_H_

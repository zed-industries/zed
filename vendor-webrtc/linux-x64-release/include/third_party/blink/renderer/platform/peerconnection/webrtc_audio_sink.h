// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_AUDIO_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_AUDIO_SINK_H_

#include <stdint.h>

#include <atomic>
#include <memory>
#include <string>
#include <utility>

#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "media/base/audio_parameters.h"
#include "media/base/audio_push_fifo.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_audio_sink.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_level_calculator.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/media_stream_interface.h"
#include "third_party/webrtc/api/media_stream_track.h"
#include "third_party/webrtc/rtc_base/time_utils.h"
#include "third_party/webrtc/rtc_base/timestamp_aligner.h"

namespace blink {

// Provides an implementation of the WebMediaStreamAudioSink which
// re-chunks audio data into the 10ms chunks required by WebRTC and then
// delivers the audio to one or more objects implementing the
// webrtc::AudioTrackSinkInterface.
//
// The inner class, Adapter, implements the webrtc::AudioTrackInterface and
// manages one or more "WebRTC sinks" (i.e., instances of
// webrtc::AudioTrackSinkInterface) which are added/removed on the WebRTC
// signaling thread.
//
// TODO(crbug.com/787254): Switch this class away from using std::string and
// std::vector. Also merge it with WebMediaStreamAudioSink.
class PLATFORM_EXPORT WebRtcAudioSink : public WebMediaStreamAudioSink {
 public:
  WebRtcAudioSink(
      const std::string& label,
      scoped_refptr<webrtc::AudioSourceInterface> track_source,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner);
  WebRtcAudioSink(const WebRtcAudioSink&) = delete;
  WebRtcAudioSink& operator=(const WebRtcAudioSink&) = delete;

  ~WebRtcAudioSink() override;

  webrtc::AudioTrackInterface* webrtc_audio_track() const {
    return adapter_.get();
  }

  // Set the object that provides shared access to the current audio signal
  // level. This is passed via the Adapter to libjingle. This method may only
  // be called once, before the audio data flow starts, and before any calls to
  // Adapter::GetSignalLevel() might be made.
  void SetLevel(scoped_refptr<MediaStreamAudioLevelCalculator::Level> level);

  // Set the processor that applies signal processing on the data from the
  // source. This is passed via the Adapter to libjingle. This method may only
  // be called once, before the audio data flow starts, and before any calls to
  // GetAudioProcessor() might be made.
  void SetAudioProcessor(
      scoped_refptr<webrtc::AudioProcessorInterface> processor);

  // MediaStreamSink override.
  void OnEnabledChanged(bool enabled) override;

 private:
  // Private implementation of the webrtc::AudioTrackInterface whose control
  // methods are all called on the WebRTC signaling thread. This class is
  // ref-counted, per the requirements of webrtc::AudioTrackInterface.
  class Adapter : public webrtc::MediaStreamTrack<webrtc::AudioTrackInterface> {
   public:
    Adapter(const std::string& label,
            scoped_refptr<webrtc::AudioSourceInterface> source,
            scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
            scoped_refptr<base::SingleThreadTaskRunner> main_task_runner);
    Adapter(const Adapter&) = delete;
    Adapter& operator=(const Adapter&) = delete;

    base::SingleThreadTaskRunner* signaling_task_runner() const {
      return signaling_task_runner_.get();
    }

    // These setters are called before the audio data flow starts, and before
    // any methods called on the signaling thread reference these objects.
    void set_processor(
        scoped_refptr<webrtc::AudioProcessorInterface> processor) {
      audio_processor_ = std::move(processor);
    }
    void set_level(
        scoped_refptr<MediaStreamAudioLevelCalculator::Level> level) {
      level_ = std::move(level);
    }

    // Delivers a 10ms chunk of audio to all WebRTC sinks managed by this
    // Adapter and returns the maximum number of channels the sinks are
    // interested in (number of channels encoded). A return value of -1 means
    // that the preferred number of channels is unknown. This is called on the
    // audio thread.
    int DeliverPCMToWebRtcSinks(const int16_t* audio_data,
                                int sample_rate,
                                size_t number_of_channels,
                                size_t number_of_frames,
                                base::TimeTicks estimated_capture_time);

    std::string label() const { return label_; }

    // webrtc::MediaStreamTrack implementation.
    std::string kind() const override;
    bool set_enabled(bool enable) override;

    // webrtc::AudioTrackInterface implementation.
    void AddSink(webrtc::AudioTrackSinkInterface* sink) override;
    void RemoveSink(webrtc::AudioTrackSinkInterface* sink) override;
    bool GetSignalLevel(int* level) override;
    webrtc::scoped_refptr<webrtc::AudioProcessorInterface> GetAudioProcessor()
        override;
    webrtc::AudioSourceInterface* GetSource() const override;

    void UpdateTimestampAligner(base::TimeTicks capture_time);

   protected:
    ~Adapter() override;

   private:
    const std::string label_;

    const scoped_refptr<webrtc::AudioSourceInterface> source_;

    // Task runner for operations that must be done on libjingle's signaling
    // thread.
    const scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner_;

    // Task runner used for the final de-referencing of |audio_processor_| at
    // destruction time.
    const scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;

    // The audio processsor that applies audio post-processing on the source
    // audio. This is null if there is no audio processing taking place
    // upstream. This must be set before calls to GetAudioProcessor() are made.
    scoped_refptr<webrtc::AudioProcessorInterface> audio_processor_;

    // Thread-safe accessor to current audio signal level. This may be null, if
    // not applicable to the current use case. This must be set before calls to
    // GetSignalLevel() are made.
    scoped_refptr<MediaStreamAudioLevelCalculator::Level> level_;

    // Lock that protects concurrent access to the |sinks_| list.
    base::Lock lock_;

    // A vector of pointers to unowned WebRTC-internal objects which each
    // receive the audio data.
    Vector<webrtc::AudioTrackSinkInterface*> sinks_;

    // Used for getting capture timestamps referenced on the
    // webrtc::TimeMicros() clock. See the comment at the implementation of
    // UpdateTimestampAligner() for more details.
    webrtc::TimestampAligner timestamp_aligner_;
  };

  template <typename>
  friend struct WTF::CrossThreadCopier;

  // WebMediaStreamAudioSink implementation.
  void OnData(const media::AudioBus& audio_bus,
              base::TimeTicks estimated_capture_time) override;
  void OnSetFormat(const media::AudioParameters& params) override;

  // WebMediaStreamAudioSink implementation.
  int NumPreferredChannels() override { return num_preferred_channels_; }

  // Called by AudioPushFifo zero or more times during the call to OnData().
  // Delivers audio data with the required 10ms buffer size to |adapter_|.
  void DeliverRebufferedAudio(const media::AudioBus& audio_bus,
                              int frame_delay);

  // Owner of the WebRTC sinks. May outlive this WebRtcAudioSink (if references
  // are held by libjingle).
  const scoped_refptr<Adapter> adapter_;

  // The current format of the audio passing through this sink.
  media::AudioParameters params_;

  // Light-weight fifo used for re-chunking audio into the 10ms chunks required
  // by the WebRTC sinks.
  media::AudioPushFifo fifo_;

  // Buffer used for converting into the required signed 16-bit integer
  // interleaved samples.
  std::unique_ptr<int16_t[]> interleaved_data_;

  base::TimeTicks last_estimated_capture_time_;

  // The maximum number of preferred audio channels by any sink or -1 if
  // unknown.
  std::atomic<int> num_preferred_channels_;

  // In debug builds, check that WebRtcAudioSink's public methods are all being
  // called on the main render thread.
  THREAD_CHECKER(thread_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_AUDIO_SINK_H_

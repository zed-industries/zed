// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_DELIVERER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_DELIVERER_H_

#include <inttypes.h>

#include <algorithm>

#include "base/containers/contains.h"
#include "base/synchronization/lock.h"
#include "base/threading/thread_checker.h"
#include "base/trace_event/trace_event.h"
#include "media/base/audio_bus.h"
#include "media/base/audio_parameters.h"
#include "third_party/blink/public/platform/modules/webrtc/webrtc_logging.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace media {
struct AudioGlitchInfo;
}

namespace blink {

// Template containing functionality common to both MediaStreamAudioSource and
// MediaStreamAudioTrack. This is used for managing the connections between
// objects through which audio data flows, and doing so in a thread-safe manner.
//
// The Consumer parameter of the template is the type of the objects to which
// audio data is delivered: MediaStreamAudioTrack or MediaStreamAudioSink. It's
// assumed the Consumer class defines methods named OnSetFormat() and OnData()
// that have the same signature as the ones defined in this template.
// MediaStreamAudioDeliverer will always guarantee the Consumer's OnSetFormat()
// and OnData() methods are called sequentially.
template <typename Consumer>
class MediaStreamAudioDeliverer {
 public:
  MediaStreamAudioDeliverer() {}
  MediaStreamAudioDeliverer(const MediaStreamAudioDeliverer&) = delete;
  MediaStreamAudioDeliverer& operator=(const MediaStreamAudioDeliverer&) =
      delete;
  ~MediaStreamAudioDeliverer() {}

  // Returns the current audio parameters. These will be invalid before the
  // first call to OnSetFormat(). This method is thread-safe.
  media::AudioParameters GetAudioParameters() const {
    base::AutoLock auto_lock(params_lock_);
    return params_;
  }

  // Begin delivering audio to |consumer|. The caller must guarantee |consumer|
  // is not destroyed until after calling RemoveConsumer(consumer). This method
  // must be called on the main thread.
  void AddConsumer(Consumer* consumer) {
    DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
    DCHECK(consumer);
    base::AutoLock auto_lock(consumers_lock_);
    DCHECK(!base::Contains(consumers_, consumer));
    DCHECK(!base::Contains(pending_consumers_, consumer));
    pending_consumers_.push_back(consumer);
    SendLogMessage(
        String::Format("%s => (number of consumer: active=%u, pending=%u)",
                       __func__, consumers_.size(), pending_consumers_.size()));
  }

  // Stop delivering audio to |consumer|. Returns true if |consumer| was the
  // last consumer removed, false otherwise. When this method returns, no
  // further calls will be made to OnSetFormat() or OnData() on any thread.
  // This method must be called on the main thread.
  bool RemoveConsumer(Consumer* consumer) {
    DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
    base::AutoLock auto_lock(consumers_lock_);
    const bool had_consumers =
        !consumers_.empty() || !pending_consumers_.empty();
    auto it = std::ranges::find(consumers_, consumer);
    if (it != consumers_.end()) {
      consumers_.erase(it);
    } else {
      it = std::ranges::find(pending_consumers_, consumer);
      if (it != pending_consumers_.end())
        pending_consumers_.erase(it);
    }
    SendLogMessage(
        String::Format("%s => (number of consumers: active=%u, pending=%u)",
                       __func__, consumers_.size(), pending_consumers_.size()));
    return had_consumers && consumers_.empty() && pending_consumers_.empty();
  }

  // Returns the current list of connected Consumers. This is normally used to
  // send a notification to all consumers. This method must be called on the
  // main thread.
  void GetConsumerList(Vector<Consumer*>* consumer_list) const {
    DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
    base::AutoLock auto_lock(consumers_lock_);
    *consumer_list = consumers_;
    consumer_list->AppendRange(pending_consumers_.begin(),
                               pending_consumers_.end());
  }

  // Change the format of the audio passed in the next call to OnData(). This
  // method may be called on any thread but, logically, should only be called
  // between calls to OnData().
  void OnSetFormat(const media::AudioParameters& params) {
    DCHECK(params.IsValid());
    base::AutoLock auto_lock(consumers_lock_);
    {
      base::AutoLock auto_params_lock(params_lock_);
      if (params_.Equals(params))
        return;
      SendLogMessage(String::Format("%s({params=[%s]})", __func__,
                                    params.AsHumanReadableString().c_str()));
      params_ = params;
    }
    pending_consumers_.AppendRange(consumers_.begin(), consumers_.end());
    consumers_.clear();
  }

  // Deliver data to all consumers. This method may be called on any thread.
  void OnData(const media::AudioBus& audio_bus,
              base::TimeTicks reference_time,
              const media::AudioGlitchInfo& glitch_info) {
    TRACE_EVENT("audio", "MediaStreamAudioDeliverer::OnData",
                "reference_time (ms)",
                (reference_time - base::TimeTicks()).InMillisecondsF(),
                "layover_delay (ms)",
                (base::TimeTicks::Now() - reference_time).InMillisecondsF());
    base::AutoLock auto_lock(consumers_lock_);

    // Call OnSetFormat() for all pending consumers and move them to the
    // active-delivery list.
    if (!pending_consumers_.empty()) {
      const media::AudioParameters params = GetAudioParameters();
      DCHECK(params.IsValid());
      for (Consumer* consumer : pending_consumers_)
        consumer->OnSetFormat(params);
      consumers_.AppendRange(pending_consumers_.begin(),
                             pending_consumers_.end());
      pending_consumers_.clear();
      SendLogMessage(String::Format("%s => (number of active consumers=%u)",
                                    __func__, consumers_.size()));
    }

    // Deliver the audio data to each consumer.
    for (Consumer* consumer : consumers_)
      consumer->OnData(audio_bus, reference_time, glitch_info);
  }

  // Returns the maximum number of channels preferred by any consumer or -1 if
  // unknown.
  int NumPreferredChannels() const {
    base::AutoLock auto_lock(consumers_lock_);
    int num_preferred_channels = -1;
    for (Consumer* consumer : consumers_) {
      num_preferred_channels =
          std::max(num_preferred_channels, consumer->NumPreferredChannels());
    }
    return num_preferred_channels;
  }

 private:
  void SendLogMessage(const WTF::String& message) {
    WebRtcLogMessage(String::Format("MSAD::%s [this=0x%" PRIXPTR "]",
                                    message.Utf8().c_str(),
                                    reinterpret_cast<uintptr_t>(this))
                         .Utf8());
  }

  // In debug builds, check that all methods that could cause object graph or
  // data flow changes are being called on the main thread.
  THREAD_CHECKER(thread_checker_);

  // Protects concurrent access to |pending_consumers_| and |consumers_|.
  mutable base::Lock consumers_lock_;

  // Any consumers needing a call to OnSetFormat(), to be notified of the
  // changed audio format, are placed in this list. This includes consumers
  // added via AddConsumer() that need to have an initial OnSetFormat() call
  // before audio data is first delivered. Consumers are moved from this list to
  // |consumers_| on the audio thread.
  Vector<Consumer*> pending_consumers_;

  // Consumers that are up to date on the current audio format and are receiving
  // audio data are placed in this list.
  Vector<Consumer*> consumers_;

  // Protects concurrent access to |params_|.
  mutable base::Lock params_lock_;

  // Specifies the current format of the audio passing through this
  // MediaStreamAudioDeliverer.
  media::AudioParameters params_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_DELIVERER_H_

// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_MANAGER_H_

#include <list>
#include <memory>
#include <string>

#include "base/containers/flat_map.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "base/unguessable_token.h"
#include "media/audio/audio_device_description.h"
#include "media/audio/audio_sink_parameters.h"
#include "media/base/audio_latency.h"
#include "media/base/audio_parameters.h"
#include "media/base/output_device_info.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/modules/media/audio/audio_renderer_mixer_pool.h"

namespace media {
class AudioRendererSink;
}  // namespace media

namespace blink {
class AudioRendererMixer;
class AudioRendererMixerInput;

// Manages sharing of an AudioRendererMixer among AudioRendererMixerInputs based
// on their AudioParameters configuration.  Inputs with the same AudioParameters
// configuration will share a mixer while a new AudioRendererMixer will be
// lazily created if one with the exact AudioParameters does not exist. When an
// AudioRendererMixer is returned by AudioRendererMixerInput, it will be deleted
// if its only other reference is held by AudioRendererMixerManager.
//
// There should only be one instance of AudioRendererMixerManager per render
// thread.
class BLINK_MODULES_EXPORT AudioRendererMixerManager final
    : public AudioRendererMixerPool {
 public:
  // Callback which will be used to create sinks. See AudioDeviceFactory for
  // more details on the parameters.
  using CreateSinkCB =
      base::RepeatingCallback<scoped_refptr<media::AudioRendererSink>(
          const blink::LocalFrameToken& source_frame_token,
          const media::AudioSinkParameters& params)>;

  explicit AudioRendererMixerManager(CreateSinkCB create_sink_cb);
  ~AudioRendererMixerManager() final;

  AudioRendererMixerManager(const AudioRendererMixerManager&) = delete;
  AudioRendererMixerManager& operator=(const AudioRendererMixerManager&) =
      delete;

  // Creates an AudioRendererMixerInput with the proper callbacks necessary to
  // retrieve an AudioRendererMixer instance from AudioRendererMixerManager.
  //
  // `source_frame_token` refers to the RenderFrame containing the entity
  // rendering the audio.  Caller must ensure AudioRendererMixerManager outlives
  // the returned input.
  //
  // `main_frame_token` refers to the local or remote main frame at the root of
  // the tree containing the RenderFrame referenced by `source_frame_token` and
  // is used for sharing the underlying audio output device.
  //
  // `device_id` and `session_id` identify the output device to use. If
  // `device_id` is empty and `session_id` is nonzero, output device associated
  // with the opened input device designated by `session_id` is used. Otherwise,
  // `session_id` is ignored.
  scoped_refptr<AudioRendererMixerInput> CreateInput(
      const LocalFrameToken& source_frame_token,
      const FrameToken& main_frame_token,
      const base::UnguessableToken& session_id,
      std::string_view device_id,
      media::AudioLatency::Type latency);

  // media::AudioRendererMixerPool implementation. The rest of the
  // implementation is kept private (see comment below).
  AudioRendererMixer* GetMixer(
      const LocalFrameToken& source_frame_token,
      const FrameToken& main_frame_token,
      const media::AudioParameters& input_params,
      media::AudioLatency::Type latency,
      const media::OutputDeviceInfo& sink_info,
      scoped_refptr<media::AudioRendererSink> sink) final;
  void ReturnMixer(AudioRendererMixer* mixer) final;
  scoped_refptr<media::AudioRendererSink> GetSink(
      const LocalFrameToken& source_frame_token,
      const FrameToken& main_frame_token,
      std::string_view device_id) final;

 private:
  friend class AudioRendererMixerManagerTest;

  // Define a key so that only those AudioRendererMixerInputs from the same
  // RenderView, AudioParameters and output device can be mixed together.
  struct MixerKey {
    MixerKey(const LocalFrameToken& source_frame_token,
             const FrameToken& main_frame_token,
             const media::AudioParameters& params,
             media::AudioLatency::Type latency,
             std::string_view device_id);
    MixerKey(const MixerKey& other);
    ~MixerKey();
    blink::LocalFrameToken source_frame_token;
    blink::FrameToken main_frame_token;
    media::AudioParameters params;
    media::AudioLatency::Type latency;
    std::string device_id;
  };

  // Custom compare operator for the AudioRendererMixerMap.  Allows reuse of
  // mixers where only irrelevant keys mismatch.
  struct MixerKeyCompare {
    bool operator()(const MixerKey& a, const MixerKey& b) const {
      if (a.main_frame_token != b.main_frame_token) {
        return a.main_frame_token < b.main_frame_token;
      }
      if (a.params.channels() != b.params.channels()) {
        return a.params.channels() < b.params.channels();
      }
      if (a.latency != b.latency) {
        return a.latency < b.latency;
      }

      // TODO(olka) add buffer duration comparison for kLatencyExactMS when
      // adding support for it.
      DCHECK_NE(media::AudioLatency::Type::kExactMS, a.latency);

      // Ignore format(), and frames_per_buffer(), these parameters do not
      // affect mixer reuse.  All AudioRendererMixer units disable FIFO, so
      // frames_per_buffer() can be safely ignored.
      if (a.params.channel_layout() != b.params.channel_layout()) {
        return a.params.channel_layout() < b.params.channel_layout();
      }
      if (a.params.effects() != b.params.effects()) {
        return a.params.effects() < b.params.effects();
      }

      if (media::AudioDeviceDescription::IsDefaultDevice(a.device_id) &&
          media::AudioDeviceDescription::IsDefaultDevice(b.device_id)) {
        // Both device IDs represent the same default device => do not compare
        // them. We can skip the `source_frame_token` check in this case since
        // the default device is always authorized.
        return false;
      }

      // The lifetime of sinks for non-default devices are bound to the frame
      // that created them. So even if two sub-frames are authorized for the
      // same device, they can't share an output device.
      if (a.source_frame_token != b.source_frame_token) {
        return a.source_frame_token < b.source_frame_token;
      }

      return a.device_id < b.device_id;
    }
  };

  const CreateSinkCB create_sink_cb_;

  // Map of MixerKey to <AudioRendererMixer, Count>.  Count allows
  // AudioRendererMixerManager to keep track explicitly (v.s. RefCounted which
  // is implicit) of the number of outstanding AudioRendererMixers.
  struct AudioRendererMixerReference {
    std::unique_ptr<AudioRendererMixer> mixer;
    size_t ref_count;
  };

  using AudioRendererMixerMap =
      base::flat_map<MixerKey, AudioRendererMixerReference, MixerKeyCompare>;

  // Active mixers.
  AudioRendererMixerMap mixers_;

  // Mixers which encountered errors, but can't yet be destroyed since they are
  // still owned by an input. This must be a list since the same mixer key may
  // end up associated with multiple mixers with errors.
  std::list<AudioRendererMixerReference> dead_mixers_;

  base::Lock mixers_lock_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_MANAGER_H_

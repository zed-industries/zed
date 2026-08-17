// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_LISTENER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_LISTENER_HANDLER_H_

#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "ui/gfx/geometry/point3_f.h"
#include "ui/gfx/geometry/vector3d_f.h"

namespace blink {

class HRTFDatabaseLoader;
class PannerHandler;

class AudioListenerHandler final
    : public ThreadSafeRefCounted<AudioListenerHandler> {
 public:
  static scoped_refptr<AudioListenerHandler> Create(
      AudioParamHandler& position_x_handler,
      AudioParamHandler& position_y_handler,
      AudioParamHandler& position_z_handler,
      AudioParamHandler& forward_x_handler,
      AudioParamHandler& forward_y_handler,
      AudioParamHandler& forward_z_handler,
      AudioParamHandler& up_x_handler,
      AudioParamHandler& up_y_handler,
      AudioParamHandler& up_z_handler,
      unsigned int render_quantum_frames);

  ~AudioListenerHandler();

  const gfx::Point3F GetPosition() const {
    return gfx::Point3F(position_x_handler_->Value(),
                        position_y_handler_->Value(),
                        position_z_handler_->Value());
  }

  const gfx::Vector3dF GetOrientation() const {
    return gfx::Vector3dF(forward_x_handler_->Value(),
                          forward_y_handler_->Value(),
                          forward_z_handler_->Value());
  }

  const gfx::Vector3dF GetUpVector() const {
    return gfx::Vector3dF(up_x_handler_->Value(),
                          up_y_handler_->Value(),
                          up_z_handler_->Value());
  }

  const float* GetPositionXValues(uint32_t frames_to_process);
  const float* GetPositionYValues(uint32_t frames_to_process);
  const float* GetPositionZValues(uint32_t frames_to_process);
  const float* GetForwardXValues(uint32_t frames_to_process);
  const float* GetForwardYValues(uint32_t frames_to_process);
  const float* GetForwardZValues(uint32_t frames_to_process);
  const float* GetUpXValues(uint32_t frames_to_process);
  const float* GetUpYValues(uint32_t frames_to_process);
  const float* GetUpZValues(uint32_t frames_to_process);

  // True if any of AudioParams have automations.
  bool HasSampleAccurateValues() const;

  // True if any of AudioParams are set for a-rate automations (the default).
  bool IsAudioRate() const;

  // Register/remove/notify PannerHandlers: AudioListener keeps a list of
  // PannerHandlers so it can notify them when the listener state changes.
  void AddPannerHandler(PannerHandler&);
  void RemovePannerHandler(PannerHandler&);
  void MarkPannersAsDirty(unsigned);

  // Updates the internal state of the listener, including updating the dirty
  // state of all PannerNodes if necessary.
  void UpdateState();

  bool IsListenerDirty() const { return is_listener_dirty_; }

  base::Lock& Lock() { return listener_lock_; }

  void CreateAndLoadHRTFDatabaseLoader(float sample_rate);
  HRTFDatabaseLoader* HrtfDatabaseLoader();

  // TODO(crbug.com/1471284): this method can be called from both main and
  // audio thread.
  void WaitForHRTFDatabaseLoaderThreadCompletion();

 private:
  AudioListenerHandler(AudioParamHandler& position_x_handler,
                       AudioParamHandler& position_y_handler,
                       AudioParamHandler& position_z_handler,
                       AudioParamHandler& forward_x_handler,
                       AudioParamHandler& forward_y_handler,
                       AudioParamHandler& forward_z_handler,
                       AudioParamHandler& up_x_handler,
                       AudioParamHandler& up_y_handler,
                       AudioParamHandler& up_z_handler,
                       unsigned int render_quantum_frames);

  void UpdateValuesIfNeeded(uint32_t frames_to_process);

  scoped_refptr<AudioParamHandler> position_x_handler_;
  scoped_refptr<AudioParamHandler> position_y_handler_;
  scoped_refptr<AudioParamHandler> position_z_handler_;
  scoped_refptr<AudioParamHandler> forward_x_handler_;
  scoped_refptr<AudioParamHandler> forward_y_handler_;
  scoped_refptr<AudioParamHandler> forward_z_handler_;
  scoped_refptr<AudioParamHandler> up_x_handler_;
  scoped_refptr<AudioParamHandler> up_y_handler_;
  scoped_refptr<AudioParamHandler> up_z_handler_;

  AudioFloatArray position_x_values_;
  AudioFloatArray position_y_values_;
  AudioFloatArray position_z_values_;
  AudioFloatArray forward_x_values_;
  AudioFloatArray forward_y_values_;
  AudioFloatArray forward_z_values_;
  AudioFloatArray up_x_values_;
  AudioFloatArray up_y_values_;
  AudioFloatArray up_z_values_;

  // Last time that the automations were updated.
  double last_update_time_ = -1;

  // Parameters from the last render quantum.
  gfx::Point3F last_position_ GUARDED_BY(listener_lock_);
  gfx::Vector3dF last_forward_ GUARDED_BY(listener_lock_);
  gfx::Vector3dF last_up_ GUARDED_BY(listener_lock_);

  // Set at every render quantum if the listener has changed in any way
  // (position, forward, or up). This should only be read or written to from
  // the audio thread.
  bool is_listener_dirty_ = false;

  // To synchronize settings of the state of the listener during
  // `PannerHandler::Process()` and related functions.
  mutable base::Lock listener_lock_;

  // A set of PannerHandlers. This is updated only in the main thread and is
  // referred in the audio thread. These raw pointers are safe because
  // `PannerHandler::uninitialize()` unregisters it from this set.
  HashSet<PannerHandler*> panner_handlers_;

  // HRTF database loader used by PannerHandlers in the same context.
  scoped_refptr<HRTFDatabaseLoader> hrtf_database_loader_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_LISTENER_HANDLER_H_

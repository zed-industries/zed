// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_PANNER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_PANNER_HANDLER_H_

#include <memory>

#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_distance_model_type.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_panning_model_type.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/platform/audio/cone_effect.h"
#include "third_party/blink/renderer/platform/audio/distance_effect.h"
#include "third_party/blink/renderer/platform/audio/panner.h"
#include "ui/gfx/geometry/point3_f.h"
#include "ui/gfx/geometry/vector3d_f.h"

namespace blink {

class AudioBus;
class AudioListenerHandler;
class AudioParamHandler;

class PannerHandler final : public AudioHandler {
 public:
  // These enums are used to distinguish what cached values of panner are dirty.
  enum {
    kAzimuthElevationDirty = 0x1,
    kDistanceConeGainDirty = 0x2,
  };

  static scoped_refptr<PannerHandler> Create(AudioNode&,
                                             float sample_rate,
                                             AudioParamHandler& position_x,
                                             AudioParamHandler& position_y,
                                             AudioParamHandler& position_z,
                                             AudioParamHandler& orientation_x,
                                             AudioParamHandler& orientation_y,
                                             AudioParamHandler& orientation_z);

  PannerHandler(const PannerHandler&) = delete;
  PannerHandler& operator=(const PannerHandler&) = delete;

  ~PannerHandler() override;

  // AudioHandler
  void Initialize() override;
  double LatencyTime() const override {
    return panner_ ? panner_->LatencyTime() : 0;
  }
  void Process(uint32_t frames_to_process) override;
  void ProcessIfNecessary(uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) override;
  void ProcessSampleAccurateValues(AudioBus* destination,
                                   const AudioBus* source,
                                   uint32_t frames_to_process);
  bool RequiresTailProcessing() const override;
  void SetChannelCount(unsigned, ExceptionState&) override;
  void SetChannelCountMode(V8ChannelCountMode::Enum, ExceptionState&) override;
  double TailTime() const override { return panner_ ? panner_->TailTime() : 0; }
  void Uninitialize() override;

  double ConeInnerAngle() const { return cone_effect_.InnerAngle(); }
  double ConeOuterAngle() const { return cone_effect_.OuterAngle(); }
  double ConeOuterGain() const { return cone_effect_.OuterGain(); }
  V8DistanceModelType::Enum DistanceModel() const;
  double MaxDistance() { return distance_effect_.MaxDistance(); }
  V8PanningModelType::Enum PanningModel() const;
  double RefDistance() { return distance_effect_.RefDistance(); }
  double RolloffFactor() { return distance_effect_.RolloffFactor(); }

  void SetConeInnerAngle(double angles_in_degrees);
  void SetConeOuterAngle(double angles_in_degrees);
  void SetConeOuterGain(double);
  void SetDistanceModel(V8DistanceModelType::Enum);
  void SetMaxDistance(double);
  void SetOrientation(float x, float y, float z, ExceptionState&);
  void SetPanningModel(V8PanningModelType::Enum);
  void SetPosition(float x, float y, float z, ExceptionState&);
  void SetRefDistance(double);
  void SetRolloffFactor(double);

  void MarkPannerAsDirty(unsigned);

 private:
  PannerHandler(AudioNode&,
                float sample_rate,
                AudioParamHandler& position_x,
                AudioParamHandler& position_y,
                AudioParamHandler& position_z,
                AudioParamHandler& orientation_x,
                AudioParamHandler& orientation_y,
                AudioParamHandler& orientation_z);

  // Returns true on successful operation.
  bool SetPanningModel(Panner::PanningModel);
  bool SetDistanceModel(unsigned);

  gfx::Point3F GetPosition() const;
  gfx::Vector3dF Orientation() const;

  // Returns a combined gain attenuation for azimuth and elevation.
  void CalculateAzimuthElevation(double* out_azimuth,
                                 double* out_elevation,
                                 const gfx::Point3F& position,
                                 const gfx::Point3F& listener_position,
                                 const gfx::Vector3dF& listener_forward,
                                 const gfx::Vector3dF& listener_up);

  // Returns a combined gain attenuation for distance and sound cone.
  float CalculateDistanceConeGain(const gfx::Point3F& position,
                                  const gfx::Vector3dF& orientation,
                                  const gfx::Point3F& listener_position);

  // The in-place version of `CalculateAzimuthElevation` above for k-rate.
  void AzimuthElevation(double* out_azimuth, double* out_elevation);

  // Returns a combined gain attenuation for distance and sound cone in k-rate.
  float DistanceConeGain();

  bool IsAzimuthElevationDirty() const { return is_azimuth_elevation_dirty_; }
  bool IsDistanceConeGainDirty() const { return is_distance_cone_gain_dirty_; }
  void UpdateDirtyState();

  // True if any of this panner's AudioParams have automations.
  bool HasSampleAccurateValues() const;

  // True if any of the panner's AudioParams are set for a-rate automations
  // (the default).
  bool IsAudioRate() const;

  std::unique_ptr<Panner> panner_;

  Panner::PanningModel panning_model_;
  unsigned distance_model_ = DistanceEffect::kModelInverse;

  gfx::Point3F last_position_;
  gfx::Vector3dF last_orientation_;

  double cached_azimuth_ = 0;
  double cached_elevation_ = 0;
  bool is_azimuth_elevation_dirty_ = true;

  DistanceEffect distance_effect_;
  ConeEffect cone_effect_;
  float cached_distance_cone_gain_ = 1.0f;
  bool is_distance_cone_gain_dirty_ = true;

  scoped_refptr<AudioParamHandler> position_x_;
  scoped_refptr<AudioParamHandler> position_y_;
  scoped_refptr<AudioParamHandler> position_z_;
  scoped_refptr<AudioParamHandler> orientation_x_;
  scoped_refptr<AudioParamHandler> orientation_y_;
  scoped_refptr<AudioParamHandler> orientation_z_;

  scoped_refptr<AudioListenerHandler> listener_handler_;

  // To synchronize `Process()` with the setting of this panner's state. (e.g.
  // position, orientation, distance, sound cone, and the listener)
  mutable base::Lock process_lock_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_PANNER_HANDLER_H_

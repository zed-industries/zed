/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC_GAIN_CONTROL_H_
#define MODULES_AUDIO_PROCESSING_AGC_GAIN_CONTROL_H_

namespace webrtc {

// The automatic gain control (AGC) component brings the signal to an
// appropriate range. This is done by applying a digital gain directly and, in
// the analog mode, prescribing an analog gain to be applied at the audio HAL.
//
// Recommended to be enabled on the client-side.
class GainControl {
 public:
  // When an analog mode is set, this must be called prior to `ProcessStream()`
  // to pass the current analog level from the audio HAL. Must be within the
  // range provided to `set_analog_level_limits()`.
  virtual int set_stream_analog_level(int level) = 0;

  // When an analog mode is set, this should be called after `ProcessStream()`
  // to obtain the recommended new analog level for the audio HAL. It is the
  // users responsibility to apply this level.
  virtual int stream_analog_level() const = 0;

  enum Mode {
    // Adaptive mode intended for use if an analog volume control is available
    // on the capture device. It will require the user to provide coupling
    // between the OS mixer controls and AGC through the `stream_analog_level()`
    // functions.
    //
    // It consists of an analog gain prescription for the audio device and a
    // digital compression stage.
    kAdaptiveAnalog,

    // Adaptive mode intended for situations in which an analog volume control
    // is unavailable. It operates in a similar fashion to the adaptive analog
    // mode, but with scaling instead applied in the digital domain. As with
    // the analog mode, it additionally uses a digital compression stage.
    kAdaptiveDigital,

    // Fixed mode which enables only the digital compression stage also used by
    // the two adaptive modes.
    //
    // It is distinguished from the adaptive modes by considering only a
    // short time-window of the input signal. It applies a fixed gain through
    // most of the input level range, and compresses (gradually reduces gain
    // with increasing level) the input signal at higher levels. This mode is
    // preferred on embedded devices where the capture signal level is
    // predictable, so that a known gain can be applied.
    kFixedDigital
  };

  virtual int set_mode(Mode mode) = 0;
  virtual Mode mode() const = 0;

  // Sets the target peak `level` (or envelope) of the AGC in dBFs (decibels
  // from digital full-scale). The convention is to use positive values. For
  // instance, passing in a value of 3 corresponds to -3 dBFs, or a target
  // level 3 dB below full-scale. Limited to [0, 31].
  //
  // TODO(ajm): use a negative value here instead, if/when VoE will similarly
  //            update its interface.
  virtual int set_target_level_dbfs(int level) = 0;
  virtual int target_level_dbfs() const = 0;

  // Sets the maximum `gain` the digital compression stage may apply, in dB. A
  // higher number corresponds to greater compression, while a value of 0 will
  // leave the signal uncompressed. Limited to [0, 90].
  virtual int set_compression_gain_db(int gain) = 0;
  virtual int compression_gain_db() const = 0;

  // When enabled, the compression stage will hard limit the signal to the
  // target level. Otherwise, the signal will be compressed but not limited
  // above the target level.
  virtual int enable_limiter(bool enable) = 0;
  virtual bool is_limiter_enabled() const = 0;

  // Sets the `minimum` and `maximum` analog levels of the audio capture device.
  // Must be set if and only if an analog mode is used. Limited to [0, 65535].
  virtual int set_analog_level_limits(int minimum, int maximum) = 0;
  virtual int analog_level_minimum() const = 0;
  virtual int analog_level_maximum() const = 0;

  // Returns true if the AGC has detected a saturation event (period where the
  // signal reaches digital full-scale) in the current frame and the analog
  // level cannot be reduced.
  //
  // This could be used as an indicator to reduce or disable analog mic gain at
  // the audio HAL.
  virtual bool stream_is_saturated() const = 0;

 protected:
  virtual ~GainControl() {}
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC_GAIN_CONTROL_H_

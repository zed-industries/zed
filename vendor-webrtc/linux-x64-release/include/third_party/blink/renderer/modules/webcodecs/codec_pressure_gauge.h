// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_PRESSURE_GAUGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_PRESSURE_GAUGE_H_

#include <utility>

#include "base/functional/callback.h"
#include "base/sequence_checker.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/reclaimable_codec.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace blink {

// Per-renderer-process singleton which keeps track of total codec pressure,
// across many ExecutionContexts (e.g different CodecPressureManagers).
class MODULES_EXPORT CodecPressureGauge {
 public:
  using PressureCallbackId = size_t;

  // Callback to receive the global pressure exceeded flag.
  using PressureThresholdChangedCallback = base::RepeatingCallback<void(bool)>;

  // Return result comprised of a callback's ID and the status of the global
  // pressure exceeded flag at the time of the callback's registration.
  using RegistrationResult = std::pair<PressureCallbackId, bool>;

  static CodecPressureGauge& GetInstance(ReclaimableCodec::CodecType);

  // Disable copy and assign.
  CodecPressureGauge(const CodecPressureGauge&) = delete;
  CodecPressureGauge& operator=(const CodecPressureGauge&) = delete;

  // Adds a new callback, which listens for changes to global pressure levels.
  // Returns the callback's ID (for unregistering) and whether we are currently
  // exceeding global pressure levels. Callbacks can be run from any thread, and
  // should post to the right sequences accordingly.
  // Can be called on any thread.
  RegistrationResult RegisterPressureCallback(PressureThresholdChangedCallback);

  // Removes a callback from getting pressure threshold notifications.
  // Callers can use |pressure_released| to release a block of pressure, rather
  // than calling Decrement() many times.
  // Can be called on any thread.
  void UnregisterPressureCallback(PressureCallbackId, size_t pressure_released);

  // Increments/decrements global pressure. Can be called from any thread.
  void Increment();
  void Decrement();

  void set_pressure_threshold_for_testing(size_t threshold) {
    base::AutoLock locker(lock_);
    pressure_threshold_ = threshold;
  }

  size_t global_pressure_for_testing() {
    base::AutoLock locker(lock_);
    return global_pressure_;
  }

  bool is_global_pressure_exceeded_for_testing() {
    base::AutoLock locker(lock_);
    return global_pressure_exceeded_;
  }

 private:
  using PressureCallbacks =
      WTF::HashMap<PressureCallbackId, PressureThresholdChangedCallback>;

  // Used on platforms with a pressure limit shared by encoders and decoders.
  static CodecPressureGauge& SharedInstance();

  // Used on platforms with a different limit for encoders and decoders.
  static CodecPressureGauge& DecoderInstance();
  static CodecPressureGauge& EncoderInstance();

  explicit CodecPressureGauge(size_t pressure_threshold);

  void CheckForThresholdChanges_Locked();

  base::Lock lock_;
  bool global_pressure_exceeded_ GUARDED_BY(lock_) = false;

  size_t global_pressure_ GUARDED_BY(lock_) = 0u;

  size_t pressure_threshold_ GUARDED_BY(lock_);

  // Start at 1, because WTF::HashMap uses 0 to denote deleted elements.
  PressureCallbackId next_pressure_callback_id_ GUARDED_BY(lock_) = 1u;
  PressureCallbacks pressure_callbacks_ GUARDED_BY(lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_PRESSURE_GAUGE_H_

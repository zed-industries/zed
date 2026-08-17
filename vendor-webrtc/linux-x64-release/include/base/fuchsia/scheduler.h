// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_SCHEDULER_H_
#define BASE_FUCHSIA_SCHEDULER_H_

#include "base/time/time.h"

namespace base {

// Scheduling interval to use for realtime audio threads.
// TODO(crbug.com/42050308): Add scheduling period to Thread::Options and remove
// this constants.
constexpr TimeDelta kAudioSchedulingPeriod = Milliseconds(10);

// Request 30% max CPU deadline utilization for an audio thread.
// TODO(crbug.com/42050235): A different value may need to be used for WebAudio
// threads (see media::FuchsiaAudioOutputDevice). A higher capacity may need to
// be allocated in that case.
constexpr float kAudioSchedulingCapacity = 0.3;

// Scheduling interval to use for display threads.
// TODO(crbug.com/42050308): Add scheduling period to Thread::Options and remove
// this constants.
constexpr TimeDelta kDisplaySchedulingPeriod = Seconds(1) / 60;

// Request 50% max CPU deadline utilization for a display thread.
// TODO(crbug.com/40750845): Currently DISPLAY priority is not enabled for any
// thread on Fuchsia. The value below will need to be fine-tuned when it's
// enabled.
const float kDisplaySchedulingCapacity = 0.5;

}  // namespace base

#endif  // BASE_FUCHSIA_SCHEDULER_H_

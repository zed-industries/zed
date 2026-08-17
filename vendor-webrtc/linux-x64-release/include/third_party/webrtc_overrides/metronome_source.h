// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_WEBRTC_OVERRIDES_METRONOME_SOURCE_H_
#define THIRD_PARTY_BLINK_WEBRTC_OVERRIDES_METRONOME_SOURCE_H_

#include <memory>
#include <vector>

#include "base/functional/callback.h"
#include "base/functional/callback_forward.h"
#include "base/memory/ref_counted.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "base/synchronization/waitable_event.h"
#include "base/task/sequenced_task_runner.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "third_party/abseil-cpp/absl/functional/any_invocable.h"
#include "third_party/webrtc/api/metronome/metronome.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"

namespace blink {

// The MetronomeSource ticks at a constant frequency, scheduling to wake up on
// ticks where listeners have work to do, and not scheduling to wake up on ticks
// where there is no work to do.
//
// When coalescing a large number of wakeup sources onto the MetronomeSource,
// this should reduce package Idle Wake Ups with potential to improve
// performance.
//
// The public API of the class except construction is meant to run on
// `metronome_task_runner`.
//
// `webrtc_component` does not have a test binary. See
// /third_party/blink/renderer/platform/peerconnection/metronome_source_test.cc
// for testing.
class RTC_EXPORT MetronomeSource final {
 public:
  // Class abstracting requesting a callback on the next tick. This class is a
  // thread safe ref-counted object that can be shared by multiple
  // `MetronomeSource` instance.
  class TickProvider : public base::RefCountedThreadSafe<TickProvider> {
   public:
    // Requests a callback on the next tick. The callback must be run on the
    // same sequence that called this method.
    virtual void RequestCallOnNextTick(base::OnceClosure callback) = 0;

    // Estimate the current tick period. A soft lower bound value is okay here.
    virtual base::TimeDelta TickPeriod() = 0;

   protected:
    friend class base::RefCountedThreadSafe<TickProvider>;

    virtual ~TickProvider() = default;
  };

  explicit MetronomeSource(scoped_refptr<TickProvider> tick_provider);
  ~MetronomeSource();

  MetronomeSource(const MetronomeSource&) = delete;
  MetronomeSource& operator=(const MetronomeSource&) = delete;

  // Creates a webrtc::Metronome which is backed by this metronome.
  std::unique_ptr<webrtc::Metronome> CreateWebRtcMetronome();

 private:
  friend class WebRtcMetronomeAdapter;

  // Called by metronome when a callback is available for execution on the next
  // tick.
  void RequestCallOnNextTick(absl::AnyInvocable<void() &&> callback);

  // Called when a tick happens.
  void OnMetronomeTick();

  // Reschedules an invocation of OnMetronomeTick.
  void Reschedule();

  // Returns the tick provider's tick period.
  base::TimeDelta TickPeriod();

  SEQUENCE_CHECKER(metronome_sequence_checker_);
  std::vector<absl::AnyInvocable<void() &&>> callbacks_
      GUARDED_BY_CONTEXT(metronome_sequence_checker_);
  scoped_refptr<TickProvider> tick_provider_
      GUARDED_BY_CONTEXT(metronome_sequence_checker_);
  base::WeakPtrFactory<MetronomeSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_WEBRTC_OVERRIDES_METRONOME_SOURCE_H_

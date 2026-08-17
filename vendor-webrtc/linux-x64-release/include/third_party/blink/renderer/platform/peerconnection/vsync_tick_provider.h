// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VSYNC_TICK_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VSYNC_TICK_PROVIDER_H_

#include <memory>
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/functional/callback_forward.h"
#include "base/memory/raw_ref.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/peerconnection/vsync_provider.h"
#include "third_party/webrtc_overrides/metronome_source.h"
#include "third_party/webrtc_overrides/timer_based_tick_provider.h"

namespace blink {

// Tick provider that normally generates ticks based on callbacks from VSync
// notifications requested from a VSyncProvider. If VSyncs cease to be reported
// (such as when the tab is occluded), ticks are instead provided by a supplied
// default tick provider. When VSyncs are found to be reported again, the class
// switches back to be driven by them.
//
// After construction, all access including destruction must happen on the
// `sequence` specified on construction.
class PLATFORM_EXPORT VSyncTickProvider : public MetronomeSource::TickProvider {
 public:
  // The VSync frequency is at least 30 Hz under RTC conferencing in Chromium,
  // but can be higher depending on animation complexity on the web page.
  static constexpr base::TimeDelta kVSyncTickPeriod = base::Hertz(30);

  // Create using a begin frame provider.
  static scoped_refptr<VSyncTickProvider> Create(
      VSyncProvider& vsync_provider,
      scoped_refptr<base::SequencedTaskRunner> sequence,
      scoped_refptr<MetronomeSource::TickProvider> default_tick_provider);
  ~VSyncTickProvider() override;

  // TickProvider overrides.
  void RequestCallOnNextTick(base::OnceClosure callback) override;
  base::TimeDelta TickPeriod() override;

 private:
  VSyncTickProvider(
      VSyncProvider& vsync_provider,
      scoped_refptr<base::SequencedTaskRunner> sequence,
      scoped_refptr<MetronomeSource::TickProvider> default_tick_provider);
  void Initialize();

  // Requests a callback to OnVSync.
  void ScheduleVSync();

  // Requests a callback to OnTick.
  void ScheduleDefaultTick();

  // Called on tick from the default provider.
  void OnDefaultTick();

  // Called on vsync from the vsync provider.
  void OnVSync();

  // Processes a tick, i.e. calls out to the requested callback, if present.
  void MaybeCalloutToClient();

  // Called on tab visibility changes from the vsync provider.
  void OnTabVisibilityChange(bool visible);

  enum class State {
    // The provider is driven by the default provider.
    kDrivenByDefault,
    // The provider is switching to be driven by vsyncs, but is still awaiting a
    // first vsync callback from the provider and drives using default callbacks
    // until then.
    kAwaitingVSync,
    // The provider is driven solely by vsyncs.
    kDrivenByVSync
  };

  const raw_ref<VSyncProvider> vsync_provider_;
  const scoped_refptr<base::SequencedTaskRunner> sequence_;
  SEQUENCE_CHECKER(sequence_checker_);

  // The default tick provider, used when the tab is occluded.
  const scoped_refptr<MetronomeSource::TickProvider> default_tick_provider_
      GUARDED_BY_CONTEXT(sequence_checker_);

  // The currently scheduled tick callback.
  WTF::Vector<base::OnceClosure> tick_callbacks_
      GUARDED_BY_CONTEXT(sequence_checker_);

  // The state of this tick provider.
  State state_ GUARDED_BY_CONTEXT(sequence_checker_) = State::kDrivenByDefault;

  // Weak factory used to cancel old installed callbacks.
  base::WeakPtrFactory<VSyncTickProvider> weak_tick_factory_{this};
  base::WeakPtrFactory<VSyncTickProvider> weak_factory_{this};
};
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VSYNC_TICK_PROVIDER_H_

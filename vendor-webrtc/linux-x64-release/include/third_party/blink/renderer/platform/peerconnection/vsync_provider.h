// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VSYNC_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VSYNC_PROVIDER_H_

#include "base/functional/callback_forward.h"
#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/graphics/video_frame_sink_bundle.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc_overrides/metronome_source.h"

namespace blink {

// Interface for a class that can request begin frame callbacks, and request a
// callback to be called when the tab visibility changes.
class PLATFORM_EXPORT VSyncProvider {
 public:
  virtual ~VSyncProvider() = default;

  // Initializes with a callback called with a boolean indicating whether VSync
  // callbacks can be expected, whenever the value changes (for example on tab
  // occlusion). The callback can be called on any context.
  virtual void Initialize(
      base::RepeatingCallback<void(bool /*visible*/)> callback) = 0;

  // Sets a vsync callback. The callback can be called on any context.
  virtual void SetVSyncCallback(base::OnceClosure callback) = 0;
};

// Concrete implementation that connects to the VideoFrameSinkBundle
// corresponding to the task runner provided in the constructor.
class PLATFORM_EXPORT VSyncProviderImpl : public VSyncProvider {
 public:
  VSyncProviderImpl(scoped_refptr<base::SequencedTaskRunner> task_runner,
                    uint32_t frame_sink_client_id);

  // BeginFrameProvider overrides.
  void Initialize(
      base::RepeatingCallback<void(bool /*visible*/)> callback) override;
  void SetVSyncCallback(base::OnceClosure callback) override;

 private:
  class BeginFrameObserver;
  const scoped_refptr<base::SequencedTaskRunner> task_runner_;
  const uint32_t frame_sink_client_id_;

  // Weak ptr to be dereferenced only on the video frame compositor thread.
  base::WeakPtr<BeginFrameObserver> weak_observer_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_VSYNC_PROVIDER_H_

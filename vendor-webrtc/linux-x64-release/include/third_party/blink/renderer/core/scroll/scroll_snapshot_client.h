// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_SNAPSHOT_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_SNAPSHOT_CLIENT_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class LocalFrame;

class ScrollSnapshotClient : public GarbageCollectedMixin {
 public:
  explicit ScrollSnapshotClient(LocalFrame*);

  // Called exactly once per frame after scroll update and before animation
  // update.
  virtual void UpdateSnapshot() = 0;

  // Called for all ScrollSnapshotClients once per frame, after layout is
  // finished. ValidateSnapshot is an opportunity for the client to update
  // its snapshot again in the same frame (taking information from the recently
  // finished layout into account).
  //
  // A return value of 'true' means the snapshot was valid(and therefore not
  // updated by this function). A return value of 'false' means the snapshot was
  // invalid (and therefore was updated by this function), and that the style
  // and layout phases need to run again.
  virtual bool ValidateSnapshot() = 0;

  // Compares the last snapshot with the current state, and returns true if a
  // new animation frame should be scheduled due to snapshot difference.
  virtual bool ShouldScheduleNextService() = 0;

  virtual bool IsAnchorPositionScrollData() const { return false; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCROLL_SCROLL_SNAPSHOT_CLIENT_H_

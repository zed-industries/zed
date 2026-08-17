// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_BUFFER_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_BUFFER_RUNNER_H_

#include "third_party/blink/renderer/core/editing/finder/find_options.h"
#include "third_party/blink/renderer/core/editing/range_in_flat_tree.h"

namespace blink {

// This is a base for derived classes that are used as a wrapper around
// FindBuffer to provide a callback-based interface.
class FindBufferRunner : public GarbageCollected<FindBufferRunner> {
 public:
  using Callback = base::OnceCallback<void(const EphemeralRangeInFlatTree&)>;

  virtual void FindMatchInRange(RangeInFlatTree* search_range,
                                String search_text,
                                FindOptions options,
                                Callback completeCallback) = 0;
  virtual void Cancel() = 0;
  virtual bool IsActive() = 0;

  virtual void Trace(Visitor*) const {}
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_FIND_BUFFER_RUNNER_H_

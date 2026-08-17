// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_SYNC_FIND_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_SYNC_FIND_BUFFER_H_

#include "third_party/blink/renderer/core/editing/finder/find_buffer_runner.h"
#include "third_party/blink/renderer/core/editing/finder/find_options.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/range_in_flat_tree.h"

namespace blink {

// This is used as a synchronous wrapper around FindBuffer to provide a
// callback-based interface.
class SyncFindBuffer : public FindBufferRunner {
 public:
  explicit SyncFindBuffer() = default;
  ~SyncFindBuffer() = default;

  void FindMatchInRange(RangeInFlatTree* search_range,
                        String search_text,
                        FindOptions options,
                        Callback completeCallback) override;
  void Cancel() override {}
  bool IsActive() override { return false; }
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_SYNC_FIND_BUFFER_H_

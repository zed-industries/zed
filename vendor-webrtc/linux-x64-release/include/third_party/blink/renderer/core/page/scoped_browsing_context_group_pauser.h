// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCOPED_BROWSING_CONTEXT_GROUP_PAUSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCOPED_BROWSING_CONTEXT_GROUP_PAUSER_H_

#include <stdint.h>

#include "base/unguessable_token.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Page;

// A RAII class that pauses pages belong to a browsing context group.
class CORE_EXPORT ScopedBrowsingContextGroupPauser final {
  USING_FAST_MALLOC(ScopedBrowsingContextGroupPauser);

 public:
  // Returns true if there is at least one pauser for the browsing context
  // group `page` belongs to.
  static bool IsActive(Page& page);

  explicit ScopedBrowsingContextGroupPauser(Page&);
  ScopedBrowsingContextGroupPauser(const ScopedBrowsingContextGroupPauser&) =
      delete;
  ScopedBrowsingContextGroupPauser& operator=(
      const ScopedBrowsingContextGroupPauser&) = delete;
  ~ScopedBrowsingContextGroupPauser();

 private:
  uint64_t& PausedCount();

  void SetPaused(bool);

  base::UnguessableToken browsing_context_group_token_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCOPED_BROWSING_CONTEXT_GROUP_PAUSER_H_

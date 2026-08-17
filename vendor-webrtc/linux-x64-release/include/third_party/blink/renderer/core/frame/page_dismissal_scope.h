// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAGE_DISMISSAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAGE_DISMISSAL_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT PageDismissalScope final {
  STACK_ALLOCATED();

 public:
  PageDismissalScope();
  PageDismissalScope(const PageDismissalScope&) = delete;
  PageDismissalScope& operator=(const PageDismissalScope&) = delete;
  ~PageDismissalScope();

  static bool IsActive();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAGE_DISMISSAL_SCOPE_H_

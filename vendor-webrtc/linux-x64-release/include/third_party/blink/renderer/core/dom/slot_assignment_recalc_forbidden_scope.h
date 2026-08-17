// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SLOT_ASSIGNMENT_RECALC_FORBIDDEN_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SLOT_ASSIGNMENT_RECALC_FORBIDDEN_SCOPE_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/dom/document.h"

namespace blink {

#if DCHECK_IS_ON()
class SlotAssignmentRecalcForbiddenScope {
  STACK_ALLOCATED();

 public:
  explicit SlotAssignmentRecalcForbiddenScope(Document& document)
      : count_(document.SlotAssignmentRecalcForbiddenRecursionDepth()) {
    ++count_;
  }

  SlotAssignmentRecalcForbiddenScope(
      const SlotAssignmentRecalcForbiddenScope&) = delete;
  SlotAssignmentRecalcForbiddenScope& operator=(
      const SlotAssignmentRecalcForbiddenScope&) = delete;
  ~SlotAssignmentRecalcForbiddenScope() { --count_; }

 private:
  unsigned& count_;
};
#else
class SlotAssignmentRecalcForbiddenScope {
  STACK_ALLOCATED();

 public:
  explicit SlotAssignmentRecalcForbiddenScope(Document&) {}
  SlotAssignmentRecalcForbiddenScope(
      const SlotAssignmentRecalcForbiddenScope&) = delete;
  SlotAssignmentRecalcForbiddenScope& operator=(
      const SlotAssignmentRecalcForbiddenScope&) = delete;
};
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SLOT_ASSIGNMENT_RECALC_FORBIDDEN_SCOPE_H_

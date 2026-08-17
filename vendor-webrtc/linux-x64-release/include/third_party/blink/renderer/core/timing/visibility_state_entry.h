// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_VISIBILITY_STATE_ENTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_VISIBILITY_STATE_ENTRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/timing/performance_entry.h"

namespace blink {

class CORE_EXPORT VisibilityStateEntry final : public PerformanceEntry {
  DEFINE_WRAPPERTYPEINFO();

 public:
  VisibilityStateEntry(AtomicString name, double start_time, DOMWindow* source);
  ~VisibilityStateEntry() override;

  const AtomicString& entryType() const override;
  PerformanceEntryType EntryTypeEnum() const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_VISIBILITY_STATE_ENTRY_H_

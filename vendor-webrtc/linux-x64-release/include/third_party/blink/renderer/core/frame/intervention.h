// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_INTERVENTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_INTERVENTION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class LocalFrame;

class CORE_EXPORT Intervention {
  DISALLOW_NEW();

 public:
  Intervention() = default;
  Intervention(const Intervention&) = delete;
  Intervention& operator=(const Intervention&) = delete;
  ~Intervention() = default;

  // Generates a intervention report, to be routed to the Reporting API and any
  // ReportingObservers. Also sends the intervention message to the console.
  static void GenerateReport(LocalFrame*,
                             const String& id,
                             const String& message);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_INTERVENTION_H_

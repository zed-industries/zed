// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_INTEGRITY_REPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_INTEGRITY_REPORT_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/use_counter_and_console_logger.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// During resource loading, integrity checks can fail in contexts that make
// directly reporting an error difficult. Activities we wish to measure can
// likewise occur when we lack the ability to directly access a UseCounter.
//
// This class wraps up errors and counters that occur during integrity checks,
// allowing them to be reported after processing has completed, and we return
// to a context where reporting is possible.
class PLATFORM_EXPORT IntegrityReport final {
  DISALLOW_NEW();

 public:
  void AddUseCount(WebFeature);
  void AddConsoleErrorMessage(const String&);
  void Clear();

  const Vector<String>& Messages() const { return messages_; }

  void SendReports(UseCounterAndConsoleLogger*) const;

  const Vector<WebFeature>& UseCountersForTesting() const { return use_counts_; }

 private:
  Vector<WebFeature> use_counts_;
  Vector<String> messages_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_INTEGRITY_REPORT_H_

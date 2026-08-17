// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PERFORMANCE_METRICS_FOR_NESTED_CONTEXTS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PERFORMANCE_METRICS_FOR_NESTED_CONTEXTS_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

namespace blink {

class WindowPerformance;

// This class is used to pass non-web-exposed metrics to navigation params.
class BLINK_EXPORT WebPerformanceMetricsForNestedContexts {
 public:
  ~WebPerformanceMetricsForNestedContexts() { Reset(); }

  WebPerformanceMetricsForNestedContexts() = default;

  WebPerformanceMetricsForNestedContexts(
      const WebPerformanceMetricsForNestedContexts& p) {
    Assign(p);
  }

  WebPerformanceMetricsForNestedContexts& operator=(
      const WebPerformanceMetricsForNestedContexts& p) {
    Assign(p);
    return *this;
  }

  void Reset();
  void Assign(const WebPerformanceMetricsForNestedContexts&);

#if INSIDE_BLINK
  explicit WebPerformanceMetricsForNestedContexts(WindowPerformance*);
  WebPerformanceMetricsForNestedContexts& operator=(WindowPerformance*);
#endif

  std::optional<base::TimeTicks> UnloadStart() const;
  std::optional<base::TimeTicks> UnloadEnd() const;
  std::optional<base::TimeTicks> CommitNavigationEnd() const;

 private:
  WebPrivatePtrForGC<WindowPerformance> private_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PERFORMANCE_METRICS_FOR_NESTED_CONTEXTS_H_

// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_STATISTICS_COLLECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_STATISTICS_COLLECTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Document;
struct WebDistillabilityFeatures;

class CORE_EXPORT DocumentStatisticsCollector {
  STATIC_ONLY(DocumentStatisticsCollector);

 public:
  static WebDistillabilityFeatures CollectStatistics(Document&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_STATISTICS_COLLECTOR_H_

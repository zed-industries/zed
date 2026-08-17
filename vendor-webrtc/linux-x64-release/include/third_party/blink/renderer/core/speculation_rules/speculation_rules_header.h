// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULES_HEADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULES_HEADER_H_

#include <utility>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Document;
class LocalDOMWindow;
class ResourceResponse;
enum class SpeculationRulesLoadOutcome;

// Responsible for parsing the Speculation-Rules header.
class SpeculationRulesHeader {
 public:
  // This does all of the below -- it determines whether fetching from the
  // Speculation-Rules header is possible, and then initiates any speculation
  // rules fetches that are required.
  CORE_EXPORT static void ProcessHeadersForDocumentResponse(
      const ResourceResponse&,
      LocalDOMWindow&);

 private:
  SpeculationRulesHeader();
  ~SpeculationRulesHeader();

  void ParseSpeculationRulesHeader(const String& header_value,
                                   const KURL& base_url);

  // If errors were encountered, report the unsuccessful outcome for metrics
  // purposes and also inform the developer.
  void ReportErrors(LocalDOMWindow&);

  // Start fetching the rule sets found in the Speculation-Rules header.
  void StartFetches(Document&);

  // Successfully parsed speculation rules fetches to make.
  Vector<KURL> urls_;

  // Error information to be reported if the feature is found to be enabled.
  Vector<std::pair<SpeculationRulesLoadOutcome, String>> errors_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULES_HEADER_H_

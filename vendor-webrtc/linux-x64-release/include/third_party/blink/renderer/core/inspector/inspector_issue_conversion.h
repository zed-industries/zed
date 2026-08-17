// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ISSUE_CONVERSION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ISSUE_CONVERSION_H_

#include "third_party/blink/renderer/core/inspector/protocol/audits.h"

namespace blink {

class InspectorIssue;

CORE_EXPORT std::unique_ptr<protocol::Audits::InspectorIssue>
ConvertInspectorIssueToProtocolFormat(InspectorIssue*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_ISSUE_CONVERSION_H_

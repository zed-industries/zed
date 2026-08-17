// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_DETECT_JAVASCRIPT_FRAMEWORKS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_DETECT_JAVASCRIPT_FRAMEWORKS_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class Document;

CORE_EXPORT void DetectJavascriptFrameworksOnLoad(Document&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_DETECT_JAVASCRIPT_FRAMEWORKS_H_

// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_HISTORY_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_HISTORY_UTIL_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class KURL;
class SecurityOrigin;

CORE_EXPORT bool CanChangeToUrlForHistoryApi(const KURL&,
                                             const SecurityOrigin*,
                                             const KURL& document_url);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_HISTORY_UTIL_H_

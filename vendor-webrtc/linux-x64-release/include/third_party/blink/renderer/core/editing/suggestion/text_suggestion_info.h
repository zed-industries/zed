// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUGGESTION_TEXT_SUGGESTION_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUGGESTION_TEXT_SUGGESTION_INFO_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

struct TextSuggestionInfo {
  int32_t marker_tag;
  uint32_t suggestion_index;

  int32_t span_start;
  int32_t span_end;

  String prefix;
  String suggestion;
  String suffix;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SUGGESTION_TEXT_SUGGESTION_INFO_H_

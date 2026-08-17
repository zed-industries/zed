// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_NAMED_GRID_LINES_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_NAMED_GRID_LINES_MAP_H_

#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

using NamedGridLinesMap = HashMap<String, Vector<wtf_size_t>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_NAMED_GRID_LINES_MAP_H_

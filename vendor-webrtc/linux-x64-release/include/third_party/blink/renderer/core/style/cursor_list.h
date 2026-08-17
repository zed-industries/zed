// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_CURSOR_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_CURSOR_LIST_H_

#include "third_party/blink/renderer/core/style/cursor_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

using CursorList = GCedHeapVector<CursorData>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_CURSOR_LIST_H_

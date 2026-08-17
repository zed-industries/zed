// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_RECORD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_RECORD_H_

#include "cc/paint/paint_record.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace blink {
using cc::PaintRecord;
}

namespace WTF {
template <>
struct CrossThreadCopier<cc::PaintRecord>
    : public CrossThreadCopierPassThrough<cc::PaintRecord> {};
}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINT_RECORD_H_

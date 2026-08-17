// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_HARFBUZZ_FACE_FROM_TYPEFACE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_HARFBUZZ_FACE_FROM_TYPEFACE_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkTypeface.h"

#include <hb.h>
#include <hb-cplusplus.hh>

namespace blink {

// Creates a scoped HarfBuzz hb_face_t based on accessing the underlying SkData
// of the SkTypeface (using SkTypeface::openStream() and
// SkStream::getMemoryBase().
//
// Warning regarding usage on Mac: Using this for FreeType-backed SkTypeface
// objects on Mac is okay. Do not use this on Mac for CoreText-backed SkTypeface
// objects. For those, accessing the font blob does not work efficiently since
// what is returned from typeface->openStream is a synthesized font assembled
// from copying all font tables on Mac into newly allocated memory, causing a
// potentially quite large allocations (in the megabytes range). See the
// implementation of SkTypeface_Mac::onOpenStream.
PLATFORM_EXPORT hb::unique_ptr<hb_face_t> HbFaceFromSkTypeface(
    sk_sp<SkTypeface> typeface);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_HARFBUZZ_FACE_FROM_TYPEFACE_H_

// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_GEOMETRY_HASH_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_GEOMETRY_HASH_TRAITS_H_

#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/skia/include/core/SkRect.h"
#include "ui/gfx/geometry/size_f.h"

namespace WTF {

template <>
struct HashTraits<gfx::SizeF> : GenericHashTraits<gfx::SizeF> {
  STATIC_ONLY(HashTraits);
  static unsigned GetHash(const gfx::SizeF& key) {
    return HashInts(WTF::GetHash(key.width()), WTF::GetHash(key.height()));
  }
  static bool Equal(const gfx::SizeF& a, const gfx::SizeF& b) {
    return HashTraits<float>::Equal(a.width(), b.width()) &&
           HashTraits<float>::Equal(a.height(), b.height());
  }

  static constexpr bool kEmptyValueIsZero = false;
  static constexpr gfx::SizeF EmptyValue() {
    return gfx::SizeF(std::numeric_limits<float>::infinity(), 0);
  }
  static constexpr gfx::SizeF DeletedValue() {
    return gfx::SizeF(0, std::numeric_limits<float>::infinity());
  }
};

template <>
struct HashTraits<SkIRect> : GenericHashTraits<SkIRect> {
  STATIC_ONLY(HashTraits);
  static unsigned GetHash(const SkIRect& key) {
    return HashInts(HashInts(key.x(), key.y()),
                    HashInts(key.right(), key.bottom()));
  }

  static constexpr bool kEmptyValueIsZero = false;
  static SkIRect EmptyValue() { return SkIRect::MakeWH(-1, 0); }
  static SkIRect DeletedValue() { return SkIRect::MakeWH(0, -1); }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_GEOMETRY_HASH_TRAITS_H_

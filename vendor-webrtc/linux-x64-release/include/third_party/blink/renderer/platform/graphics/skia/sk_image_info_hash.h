// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SKIA_SK_IMAGE_INFO_HASH_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SKIA_SK_IMAGE_INFO_HASH_H_

#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/skia/include/core/SkImageInfo.h"

namespace WTF {

template <>
struct HashTraits<SkImageInfo> : GenericHashTraits<SkImageInfo> {
  static unsigned GetHash(const SkImageInfo& key) {
    unsigned result = HashInts(key.width(), key.height());
    result = HashInts(result, key.colorType());
    result = HashInts(result, key.alphaType());
    if (auto* cs = key.colorSpace())
      result = HashInts(result, static_cast<uint32_t>(cs->hash()));
    return result;
  }

  static const bool kEmptyValueIsZero = true;
  static SkImageInfo EmptyValue() { return SkImageInfo::MakeUnknown(); }
  static void ConstructDeletedValue(SkImageInfo& slot) {
    new (NotNullTag::kNotNull, &slot)
        SkImageInfo(SkImageInfo::MakeUnknown(-1, -1));
  }
  static bool IsDeletedValue(const SkImageInfo& value) {
    return value == SkImageInfo::MakeUnknown(-1, -1);
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_SKIA_SK_IMAGE_INFO_HASH_H_

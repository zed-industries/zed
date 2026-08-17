// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_WTF_UCHAR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_WTF_UCHAR_H_

#if defined(USING_SYSTEM_ICU)

#include <unicode/umachine.h>  // IWYU pragma: export

#else

#include <stdint.h>

// These definitions should be matched to
// third_party/icu/source/common/unicode/umachine.h.
typedef char16_t UChar;
typedef int32_t UChar32;

#endif

static_assert(sizeof(UChar) == 2, "UChar should be two bytes");

// Define platform neutral 8 bit character type (L is for Latin-1).
typedef unsigned char LChar;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_WTF_UCHAR_H_

/*
 * Copyright (C) 2011, 2012 Apple Inc. All rights reserved.
 * Copyright (C) 2011 Nokia Corporation and/or its subsidiary(-ies).
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_COPY_LCHARS_FROM_UCHAR_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_COPY_LCHARS_FROM_UCHAR_SOURCE_H_

#include "base/check.h"
#include "base/dcheck_is_on.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/text/ascii_ctype.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

#if defined(ARCH_CPU_X86_FAMILY)
#include <emmintrin.h>

#if !BUILDFLAG(IS_MAC)
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"
#endif
#endif

namespace WTF {

inline void CopyLCharsFromUCharSource(LChar* destination,
                                      const UChar* source,
                                      size_t length) {
#if defined(ARCH_CPU_X86_FAMILY)
  const uintptr_t kMemoryAccessSize =
      16;  // Memory accesses on 16 byte (128 bit) alignment
  const uintptr_t kMemoryAccessMask = kMemoryAccessSize - 1;

  size_t i = 0;
  for (; i < length &&
         reinterpret_cast<uintptr_t>(&source[i]) & kMemoryAccessMask;
       ++i) {
    DCHECK(!(source[i] & 0xff00));
    destination[i] = static_cast<LChar>(source[i]);
  }

  const uintptr_t kSourceLoadSize =
      32;  // Process 32 bytes (16 UChars) each iteration
  const size_t kUcharsPerLoop = kSourceLoadSize / sizeof(UChar);
  if (length > kUcharsPerLoop) {
    const size_t end_length = length - kUcharsPerLoop + 1;
    for (; i < end_length; i += kUcharsPerLoop) {
#if DCHECK_IS_ON()
      for (unsigned check_index = 0; check_index < kUcharsPerLoop;
           ++check_index)
        DCHECK(!(source[i + check_index] & 0xff00));
#endif
      __m128i first8u_chars =
          _mm_load_si128(reinterpret_cast<const __m128i*>(&source[i]));
      __m128i second8u_chars =
          _mm_load_si128(reinterpret_cast<const __m128i*>(&source[i + 8]));
      __m128i packed_chars = _mm_packus_epi16(first8u_chars, second8u_chars);
      _mm_storeu_si128(reinterpret_cast<__m128i*>(&destination[i]),
                       packed_chars);
    }
  }

  for (; i < length; ++i) {
    DCHECK(!(source[i] & 0xff00));
    destination[i] = static_cast<LChar>(source[i]);
  }
#else
  // In practice, the compiler does vectorize this loop at "-O2".
  for (size_t i = 0; i < length; ++i) {
    // Moving the DCHECK() path out of the way in case it disables
    // auto-vectorization.
#if DCHECK_IS_ON()
    DCHECK(!(source[i] & 0xff00));
#endif
    destination[i] = static_cast<LChar>(source[i]);
  }
#endif
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_COPY_LCHARS_FROM_UCHAR_SOURCE_H_

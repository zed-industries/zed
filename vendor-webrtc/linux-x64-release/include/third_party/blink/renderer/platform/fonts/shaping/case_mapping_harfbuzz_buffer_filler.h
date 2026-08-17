// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_CASE_MAPPING_HARFBUZZ_BUFFER_FILLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_CASE_MAPPING_HARFBUZZ_BUFFER_FILLER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

#include <hb.h>

namespace blink {

enum class CaseMapIntend { kKeepSameCase, kUpperCase, kLowerCase };

class CaseMappingHarfBuzzBufferFiller {
  STACK_ALLOCATED();

 public:
  CaseMappingHarfBuzzBufferFiller(CaseMapIntend,
                                  const AtomicString& locale,
                                  hb_buffer_t* harfbuzz_buffer,
                                  const String& text,
                                  unsigned start_index,
                                  unsigned num_characters);

 private:
  void FillSlowCase(CaseMapIntend,
                    const AtomicString& locale,
                    base::span<const UChar> buffer,
                    unsigned start_index,
                    unsigned num_characters);
  hb_buffer_t* harfbuzz_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_CASE_MAPPING_HARFBUZZ_BUFFER_FILLER_H_

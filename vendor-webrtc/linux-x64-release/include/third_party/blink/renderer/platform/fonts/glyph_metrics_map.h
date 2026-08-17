/*
 * Copyright (C) 2006, 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_GLYPH_METRICS_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_GLYPH_METRICS_MAP_H_

#include <memory>
#include <optional>

#include "base/compiler_specific.h"
#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/platform/fonts/glyph.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/assertions.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

template <class T>
class GlyphMetricsMap {
  USING_FAST_MALLOC(GlyphMetricsMap);

 public:
  GlyphMetricsMap() : filled_primary_page_(false) {}
  GlyphMetricsMap(const GlyphMetricsMap&) = delete;
  GlyphMetricsMap& operator=(const GlyphMetricsMap&) = delete;
  std::optional<T> MetricsForGlyph(Glyph glyph) {
    return LocatePage(glyph / GlyphMetricsPage::kSize)->MetricsForGlyph(glyph);
  }

  void SetMetricsForGlyph(Glyph glyph, const T& metrics) {
    LocatePage(glyph / GlyphMetricsPage::kSize)
        ->SetMetricsForGlyph(glyph, metrics);
  }

 private:
  class GlyphMetricsPage {
    USING_FAST_MALLOC(GlyphMetricsPage);

   public:
    GlyphMetricsPage(const GlyphMetricsPage&) = delete;
    GlyphMetricsPage& operator=(const GlyphMetricsPage&) = delete;
    static const size_t kSize =
        256;  // Usually covers Latin-1 in a single page.
    GlyphMetricsPage() {}

    std::optional<T> MetricsForGlyph(Glyph glyph) const {
      // SAFETY: `glyph` is unsigned, `kSize` is the size of the
      // `metrics_` array, so `glyph % kSize` is in-bounds.
      T value = UNSAFE_BUFFERS(metrics_[glyph % kSize]);
      if (value == UnknownMetrics())
        return std::nullopt;
      return value;
    }
    void SetMetricsForGlyph(Glyph glyph, const T& metrics) {
      // SAFETY: `glyph` is unsigned, `kSize` is the size of the
      // `metrics_` array, so `glyph % kSize` is in-bounds.
      UNSAFE_BUFFERS(SetMetricsForIndex(glyph % kSize, metrics));
    }
    UNSAFE_BUFFER_USAGE void SetMetricsForIndex(unsigned index,
                                                const T& metrics) {
      // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE.
      SECURITY_DCHECK(index < kSize);
      UNSAFE_BUFFERS(metrics_[index] = metrics);
    }

   private:
    T metrics_[kSize];
  };

  GlyphMetricsPage* LocatePage(unsigned page_number) {
    if (!page_number && filled_primary_page_)
      return &primary_page_;
    return LocatePageSlowCase(page_number);
  }

  GlyphMetricsPage* LocatePageSlowCase(unsigned page_number);

  static constexpr T UnknownMetrics();

  bool filled_primary_page_;
  // We optimize for the page that contains glyph indices 0-255.
  GlyphMetricsPage primary_page_;
  std::unique_ptr<HashMap<int, std::unique_ptr<GlyphMetricsPage>>> pages_;
};

template <>
inline constexpr float GlyphMetricsMap<float>::UnknownMetrics() {
  return -1;
}

template <>
inline constexpr gfx::RectF GlyphMetricsMap<gfx::RectF>::UnknownMetrics() {
  return gfx::RectF(std::numeric_limits<float>::min(), 0, 0, 0);
}

template <class T>
typename GlyphMetricsMap<T>::GlyphMetricsPage*
GlyphMetricsMap<T>::LocatePageSlowCase(unsigned page_number) {
  GlyphMetricsPage* page;
  if (!page_number) {
    DCHECK(!filled_primary_page_);
    page = &primary_page_;
    filled_primary_page_ = true;
  } else {
    if (pages_) {
      auto it = pages_->find(page_number);
      if (it != pages_->end())
        return it->value.get();
    } else {
      pages_ =
          std::make_unique<HashMap<int, std::unique_ptr<GlyphMetricsPage>>>();
    }
    page = new GlyphMetricsPage;
    pages_->Set(page_number, base::WrapUnique(page));
  }

  // Fill in the whole page with the unknown glyph information.
  for (unsigned i = 0; i < GlyphMetricsPage::kSize; i++) {
    // SAFETY: `kSize` is the size of the metrics array to be indexed.
    UNSAFE_BUFFERS(page->SetMetricsForIndex(i, UnknownMetrics()));
  }
  return page;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_GLYPH_METRICS_MAP_H_

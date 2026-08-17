/*
 * Copyright (C) 2013 Google, Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CUSTOM_FONT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CUSTOM_FONT_DATA_H_

#include "third_party/blink/renderer/core/css/css_font_face_source.h"
#include "third_party/blink/renderer/platform/fonts/custom_font_data.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class CSSCustomFontData final : public CustomFontData {
 public:
  enum FallbackVisibility { kInvisibleFallback, kVisibleFallback };

  CSSCustomFontData(CSSFontFaceSource* source, FallbackVisibility visibility)
      : font_face_source_(source), fallback_visibility_(visibility) {
    if (source) {
      is_loading_ = source->IsLoading();
    }
  }
  ~CSSCustomFontData() override = default;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(font_face_source_);
    CustomFontData::Trace(visitor);
  }

  bool ShouldSkipDrawing() const override {
    if (font_face_source_) {
      font_face_source_->PaintRequested();
    }
    return fallback_visibility_ == kInvisibleFallback && is_loading_;
  }

  void BeginLoadIfNeeded() const override {
    if (!is_loading_ && font_face_source_) {
      is_loading_ = true;
      font_face_source_->BeginLoadIfNeeded();
    }
  }

  bool IsLoading() const override { return is_loading_; }
  bool IsLoadingFallback() const override { return true; }

  bool IsPendingDataUrl() const override {
    return font_face_source_ && font_face_source_->IsPendingDataUrl();
  }

 private:
  Member<CSSFontFaceSource> font_face_source_;
  FallbackVisibility fallback_visibility_;
  mutable bool is_loading_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CUSTOM_FONT_DATA_H_

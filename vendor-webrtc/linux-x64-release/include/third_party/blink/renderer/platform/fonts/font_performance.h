// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_PERFORMANCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_PERFORMANCE_H_

#include "base/check.h"
#include "base/time/time.h"
#include "base/timer/elapsed_timer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf.h"

namespace blink {

// This class collects performance data for font-related operations in main
// thread.
class PLATFORM_EXPORT FontPerformance {
 public:
  static void Reset() {
    primary_font_ = base::TimeDelta();
    primary_font_in_style_ = base::TimeDelta();
    system_fallback_ = base::TimeDelta();
  }

  // The aggregated time spent in |DeterminePrimarySimpleFontData|.
  static base::TimeDelta PrimaryFontTime() {
    return primary_font_ + primary_font_in_style_;
  }
  static const base::TimeDelta& PrimaryFontTimeInStyle() {
    return primary_font_in_style_;
  }
  static void AddPrimaryFontTime(base::TimeDelta time) {
    if (!IsMainThread()) [[unlikely]] {
      return;
    }
    if (in_style_)
      primary_font_in_style_ += time;
    else
      primary_font_ += time;
  }

  // The aggregated time spent in |FallbackFontForCharacter|.
  static base::TimeDelta SystemFallbackFontTime() { return system_fallback_; }
  static void AddSystemFallbackFontTime(base::TimeDelta time) {
    if (!IsMainThread()) [[unlikely]] {
      return;
    }
    system_fallback_ += time;
  }

  static void MarkFirstContentfulPaint();
  static void MarkDomContentLoaded();

  class StyleScope {
   public:
    StyleScope() { ++in_style_; }
    ~StyleScope() {
      DCHECK(in_style_);
      --in_style_;
    }
  };

 private:
  static base::TimeDelta primary_font_;
  static base::TimeDelta primary_font_in_style_;
  static base::TimeDelta system_fallback_;
  static unsigned in_style_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_PERFORMANCE_H_

/*
 * Copyright (C) 2006, 2007, 2008 Apple Computer, Inc.  All rights reserved.
 * Copyright (C) 2008 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2007-2008 Torch Mobile, Inc.
 * Copyright (C) 2013 Google, Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PATTERN_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PATTERN_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_shader.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

class SkMatrix;

namespace blink {

class PLATFORM_EXPORT Pattern : public RefCounted<Pattern> {
 public:
  enum RepeatMode {
    kRepeatModeX = 1 << 0,
    kRepeatModeY = 1 << 1,

    kRepeatModeNone = 0,
    kRepeatModeXY = kRepeatModeX | kRepeatModeY
  };

  static scoped_refptr<Pattern> CreateImagePattern(scoped_refptr<Image>,
                                                   RepeatMode = kRepeatModeXY);
  static scoped_refptr<Pattern> CreatePaintRecordPattern(
      PaintRecord,
      const gfx::RectF& record_bounds,
      RepeatMode = kRepeatModeXY);
  Pattern(const Pattern&) = delete;
  Pattern& operator=(const Pattern&) = delete;
  virtual ~Pattern();

  void ApplyToFlags(cc::PaintFlags&, const SkMatrix&) const;

  bool IsRepeatX() const { return repeat_mode_ & kRepeatModeX; }
  bool IsRepeatY() const { return repeat_mode_ & kRepeatModeY; }
  bool IsRepeatXY() const { return repeat_mode_ == kRepeatModeXY; }

  virtual bool IsTextureBacked() const { return false; }

 protected:
  explicit Pattern(RepeatMode);

  virtual sk_sp<PaintShader> CreateShader(const SkMatrix&) const = 0;

  RepeatMode repeat_mode_;

  mutable sk_sp<PaintShader> cached_shader_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PATTERN_H_

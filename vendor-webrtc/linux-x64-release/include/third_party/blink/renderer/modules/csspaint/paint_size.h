// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_SIZE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_SIZE_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class PaintSize : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit PaintSize(gfx::SizeF size) : size_(size) {}

  PaintSize(const PaintSize&) = delete;
  PaintSize& operator=(const PaintSize&) = delete;

  ~PaintSize() override = default;

  float width() const { return size_.width(); }
  float height() const { return size_.height(); }

 private:
  gfx::SizeF size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_SIZE_H_

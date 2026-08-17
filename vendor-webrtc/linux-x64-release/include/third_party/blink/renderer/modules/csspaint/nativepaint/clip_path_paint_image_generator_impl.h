// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_CLIP_PATH_PAINT_IMAGE_GENERATOR_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_CLIP_PATH_PAINT_IMAGE_GENERATOR_IMPL_H_

#include "third_party/blink/renderer/core/css/clip_path_paint_image_generator.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Image;
class ClipPathPaintDefinition;

class MODULES_EXPORT ClipPathPaintImageGeneratorImpl final
    : public ClipPathPaintImageGenerator {
 public:
  static ClipPathPaintImageGenerator* Create(LocalFrame&);

  explicit ClipPathPaintImageGeneratorImpl(ClipPathPaintDefinition*);
  ~ClipPathPaintImageGeneratorImpl() override = default;

  scoped_refptr<Image> Paint(float zoom,
                             const gfx::RectF& reference_box,
                             const gfx::SizeF& clip_area_size,
                             const Node&) final;
  Animation* GetAnimationIfCompositable(const Element* element) final;

  void Shutdown() final;

  void Trace(Visitor*) const override;

 private:
  Member<ClipPathPaintDefinition> clip_path_paint_definition_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_CLIP_PATH_PAINT_IMAGE_GENERATOR_IMPL_H_

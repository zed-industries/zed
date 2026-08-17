// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_PAINT_IMAGES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_PAINT_IMAGES_H_

#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class PaintImages final : public GarbageCollected<PaintImages> {
 public:
  PaintImages* Clone() const {
    return MakeGarbageCollected<PaintImages>(*this);
  }

  using ImageList = HeapVector<Member<const StyleImage>>;

  ImageList& Images() { return images_; }
  const ImageList& Images() const { return images_; }

  void Trace(Visitor* visitor) const { visitor->Trace(images_); }

 private:
  ImageList images_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_PAINT_IMAGES_H_

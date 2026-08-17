// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVER_ENTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVER_ENTRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class DOMRectReadOnly;
class Element;
class LayoutBox;
class LayoutObject;
class ResizeObserverSize;

template <typename IDLType>
class FrozenArray;

class CORE_EXPORT ResizeObserverEntry final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ResizeObserverEntry(Element* target);

  Element* target() const { return target_.Get(); }
  DOMRectReadOnly* contentRect() const { return content_rect_.Get(); }
  const FrozenArray<ResizeObserverSize>& contentBoxSize() const {
    return *content_box_size_.Get();
  }
  const FrozenArray<ResizeObserverSize>& borderBoxSize() const {
    return *border_box_size_.Get();
  }
  const FrozenArray<ResizeObserverSize>& devicePixelContentBoxSize() const {
    return *device_pixel_content_box_size_.Get();
  }

  void Trace(Visitor*) const override;

 private:
  void PopulateFromLayoutBox(
      const LayoutBox&,
      HeapVector<Member<ResizeObserverSize>>& content_box_size,
      HeapVector<Member<ResizeObserverSize>>& border_box_size,
      HeapVector<Member<ResizeObserverSize>>& device_pixel_content_box_size);
  void PopulateFromSVGChild(
      const LayoutObject&,
      HeapVector<Member<ResizeObserverSize>>& content_box_size,
      HeapVector<Member<ResizeObserverSize>>& border_box_size,
      HeapVector<Member<ResizeObserverSize>>& device_pixel_content_box_size);

  Member<Element> target_;
  Member<DOMRectReadOnly> content_rect_;
  Member<FrozenArray<ResizeObserverSize>> content_box_size_;
  Member<FrozenArray<ResizeObserverSize>> border_box_size_;
  Member<FrozenArray<ResizeObserverSize>> device_pixel_content_box_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_RESIZE_OBSERVER_RESIZE_OBSERVER_ENTRY_H_

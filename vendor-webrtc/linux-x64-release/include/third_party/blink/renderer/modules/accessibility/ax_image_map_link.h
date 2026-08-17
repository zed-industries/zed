/*
 * Copyright (C) 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_IMAGE_MAP_LINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_IMAGE_MAP_LINK_H_

#include "third_party/blink/renderer/core/html/html_area_element.h"
#include "third_party/blink/renderer/modules/accessibility/ax_node_object.h"

namespace blink {

class AXObjectCacheImpl;
class HTMLMapElement;

class AXImageMapLink final : public AXNodeObject {
 public:
  explicit AXImageMapLink(HTMLAreaElement*, AXObjectCacheImpl&);

  AXImageMapLink(const AXImageMapLink&) = delete;
  AXImageMapLink& operator=(const AXImageMapLink&) = delete;

  ~AXImageMapLink() override;
  void Trace(Visitor*) const override;

  HTMLAreaElement* AreaElement() const {
    return To<HTMLAreaElement>(GetNode());
  }

  HTMLMapElement* MapElement() const;

  ax::mojom::blink::Role NativeRoleIgnoringAria() const override;
  bool CanHaveChildren() const override {
    // If the area has child nodes, those will be rendered, and the combination
    // of Role::kGenericContainer and CanHaveChildren() = true allows for those
    // children to show in the AX hierarchy.
    return RoleValue() == ax::mojom::blink::Role::kGenericContainer;
  }

  Element* AnchorElement() const override;
  Element* ActionElement() const override;
  KURL Url() const override;
  bool IsLinked() const override { return true; }
  // For an <area>, return an <img> that should be used as its parent, or null.
  static AXObject* GetAXObjectForImageMap(AXObjectCacheImpl& cache, Node* area);
  void GetRelativeBounds(AXObject** out_container,
                         gfx::RectF& out_bounds_in_container,
                         gfx::Transform& out_container_transform,
                         bool* clips_children = nullptr) const override;

 private:
  bool IsImageMapLink() const override;
};

template <>
struct DowncastTraits<AXImageMapLink> {
  static bool AllowFrom(const AXObject& object) {
    return object.IsImageMapLink();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_IMAGE_MAP_LINK_H_

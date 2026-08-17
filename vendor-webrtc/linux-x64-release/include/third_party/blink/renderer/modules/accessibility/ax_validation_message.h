// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_VALIDATION_MESSAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_VALIDATION_MESSAGE_H_

#include "third_party/blink/renderer/modules/accessibility/ax_object.h"

namespace blink {

class AXObjectCacheImpl;
class ListedElement;

// The AXValidationMessage is a mock object that exposes an alert for a native
// error message popup for an invalid HTML control, aka a validation message.
// The alert is exposed with a name containing the text of the popup..

class AXValidationMessage final : public AXObject {
 public:
  explicit AXValidationMessage(AXObjectCacheImpl&);

  AXValidationMessage(const AXValidationMessage&) = delete;
  AXValidationMessage& operator=(const AXValidationMessage&) = delete;

  ~AXValidationMessage() override;

  // AXObject overrides.
  Document* GetDocument() const override;

 private:
  // AXObject:
  // Always a leaf.
  bool CanHaveChildren() const override { return false; }
  void AddChildren() override {}
  bool ComputeIsIgnored(IgnoredReasons* = nullptr) const override;
  void GetRelativeBounds(AXObject** out_container,
                         gfx::RectF& out_bounds_in_container,
                         gfx::Transform& out_container_transform,
                         bool* clips_children) const override;
  const AtomicString& LiveRegionStatus() const override;
  const AtomicString& LiveRegionRelevant() const override;
  bool IsValidationMessage() const override { return true; }
  bool IsVisible() const override;
  String TextAlternative(bool recursive,
                         const AXObject* aria_label_or_description_root,
                         AXObjectSet& visited,
                         ax::mojom::NameFrom&,
                         AXRelatedObjectVector*,
                         NameSources*) const override;
  ax::mojom::blink::Role NativeRoleIgnoringAria() const override;

  ListedElement* RelatedFormControlIfVisible() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_VALIDATION_MESSAGE_H_

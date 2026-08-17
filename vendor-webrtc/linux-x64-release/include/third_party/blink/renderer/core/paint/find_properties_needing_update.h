// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIND_PROPERTIES_NEEDING_UPDATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIND_PROPERTIES_NEEDING_UPDATE_H_

#include "base/dcheck_is_on.h"

#if DCHECK_IS_ON()

#include "base/check_op.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/core/paint/object_paint_properties.h"
#include "third_party/blink/renderer/core/paint/paint_property_tree_builder.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This file contains a scope class for catching cases where paint properties
// needed an update but were not marked as such. If paint properties will
// change, the object must be marked as needing a paint property update
// using LayoutObject::SetNeedsPaintPropertyUpdate() or by forcing a subtree
// update (see: PaintPropertyTreeBuilderContext::force_subtree_update).
//
// This scope class works by marking the paint property state as immutable
// before rebuilding properties, forcing the properties to get updated, which
// causes object paint properties to DCHECK that property values are not
// changed.

class FindPropertiesNeedingUpdateScope {
  STACK_ALLOCATED();

 public:
  FindPropertiesNeedingUpdateScope(const LayoutObject& object,
                                   const FragmentData& fragment_data,
                                   bool force_subtree_update)
      : object_(object),
        fragment_data_(fragment_data),
        needed_paint_property_update_(object.NeedsPaintPropertyUpdate()),
        needed_forced_subtree_update_(force_subtree_update) {
    if (needed_paint_property_update_ || needed_forced_subtree_update_)
      return;

    // Mark the properties as needing an update to ensure they are rebuilt.
    object.GetMutableForPainting().SetOnlyThisNeedsPaintPropertyUpdate();

    if (const auto* properties = fragment_data_.PaintProperties()) {
      had_original_properties_ = true;
      properties->SetImmutable();
    }

    if (fragment_data_.HasLocalBorderBoxProperties()) {
      original_local_border_box_properties_ =
          fragment_data_.LocalBorderBoxProperties();
    }
  }

  ~FindPropertiesNeedingUpdateScope() {
    // No need to check if an update was already needed.
    if (needed_paint_property_update_ || needed_forced_subtree_update_)
      return;

    const auto* properties = fragment_data_.PaintProperties();
    if (properties) {
      DCHECK(had_original_properties_);
      DCHECK(properties->IsImmutable());
      properties->SetMutable();
    } else {
      DCHECK(!had_original_properties_);
    }

    if (!original_local_border_box_properties_.IsInitialized() &&
        fragment_data_.HasLocalBorderBoxProperties()) {
      const auto object_border_box = fragment_data_.LocalBorderBoxProperties();
      DCHECK_EQ(&original_local_border_box_properties_.Transform(),
                &object_border_box.Transform())
          << object_.DebugName();
      DCHECK_EQ(&original_local_border_box_properties_.Clip(),
                &object_border_box.Clip())
          << object_.DebugName();
      DCHECK_EQ(&original_local_border_box_properties_.Effect(),
                &object_border_box.Effect())
          << object_.DebugName();
    } else {
      DCHECK_EQ(original_local_border_box_properties_.IsInitialized(),
                fragment_data_.HasLocalBorderBoxProperties())
          << object_.DebugName();
    }

    // Restore original clean bit.
    object_.GetMutableForPainting().ClearNeedsPaintPropertyUpdateForTesting();
  }

 private:
  const LayoutObject& object_;
  const FragmentData& fragment_data_;
  bool needed_paint_property_update_ = false;
  bool needed_forced_subtree_update_ = false;
  PropertyTreeStateOrAlias original_local_border_box_properties_{
      PropertyTreeState::kUninitialized};
  bool had_original_properties_ = false;
};

}  // namespace blink
#endif  // DCHECK_IS_ON()

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIND_PROPERTIES_NEEDING_UPDATE_H_

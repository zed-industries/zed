// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_VIEW_TRANSITION_STATE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_VIEW_TRANSITION_STATE_H_

#include <optional>

#include "base/containers/flat_map.h"
#include "components/viz/common/view_transition_element_resource_id.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/frame/view_transition_state.mojom-shared.h"
#include "ui/gfx/geometry/rect_f.h"
#include "ui/gfx/geometry/size_f.h"
#include "ui/gfx/geometry/transform.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

// The following two classes represent a view transition state necessary for a
// cross-document same-origin view transition. See
// third_party/blink/public/mojom/frame/view_transition_state.mojom for more
// comments.
struct BLINK_COMMON_EXPORT ViewTransitionElement {
  struct LayeredBoxProperties {
    gfx::RectF content_box;
    gfx::RectF padding_box;
    mojom::ViewTransitionElementBoxSizing box_sizing;
  };

 private:
  // IMPORTANT:
  // This is private + friends, because it is not meant to be used anywhere
  // outside of ViewTransition feature implementation. Do not add friends unless
  // it is necessary for view transitions. Data stored here comes from an
  // untrusthworthy renderer process and should not be parsed or used by the
  // browser process or in the renderer process for non-ViewTransition purposes.
  friend class ViewTransitionStyleTracker;
  friend struct mojo::StructTraits<blink::mojom::ViewTransitionElementDataView,
                                   ViewTransitionElement>;

  std::string tag_name;
  gfx::RectF border_box_rect_in_enclosing_layer_css_space;
  gfx::Transform viewport_matrix;
  gfx::RectF overflow_rect_in_layout_space;
  viz::ViewTransitionElementResourceId snapshot_id;
  int32_t paint_order = 0;
  std::optional<gfx::RectF> captured_rect_in_layout_space;
  base::flat_map<blink::mojom::ViewTransitionPropertyId, std::string>
      captured_css_properties;

  std::vector<std::string> class_list;
  std::string containing_group_name;

  std::optional<LayeredBoxProperties> layered_box_properties;
};

struct BLINK_COMMON_EXPORT ViewTransitionState {
 public:
  bool IsValid() const {
    return next_element_resource_id !=
           viz::ViewTransitionElementResourceId::kInvalidLocalId;
  }
  bool HasSubframeSnapshot() const { return subframe_snapshot_id.IsValid(); }

 private:
  // IMPORTANT:
  // This is private + friends, because it is not meant to be used anywhere
  // outside of ViewTransition feature implementation. Do not add friends unless
  // it is necessary for view transitions. Data stored here comes from an
  // untrusthworthy renderer process and should not be parsed or used by the
  // browser process or in the renderer process for non-ViewTransition purposes.
  friend class ViewTransitionStyleTracker;
  friend class ViewTransition;
  friend struct mojo::StructTraits<blink::mojom::ViewTransitionStateDataView,
                                   ViewTransitionState>;

  std::vector<ViewTransitionElement> elements;
  ViewTransitionToken transition_token;
  gfx::Size snapshot_root_size_at_capture;
  float device_pixel_ratio = 1.f;
  uint32_t next_element_resource_id =
      viz::ViewTransitionElementResourceId::kInvalidLocalId;
  viz::ViewTransitionElementResourceId subframe_snapshot_id;
  base::flat_map<std::string, std::string> id_to_auto_name_map;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_VIEW_TRANSITION_STATE_H_

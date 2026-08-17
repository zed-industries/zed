/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAGE_SCALE_CONSTRAINTS_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAGE_SCALE_CONSTRAINTS_SET_H_

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/page_scale_constraints.h"
#include "third_party/blink/renderer/core/page/viewport_description.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class Page;

// This class harmonizes the viewport (particularly page scale) constraints from
// the meta viewport tag and other sources.
class CORE_EXPORT PageScaleConstraintsSet
    : public GarbageCollected<PageScaleConstraintsSet> {
 public:
  explicit PageScaleConstraintsSet(Page* page);

  void Trace(Visitor*) const;

  void SetDefaultConstraints(const PageScaleConstraints&);
  const PageScaleConstraints& DefaultConstraints() const;

  // Settings defined in the website's viewport tag, if viewport tag support
  // is enabled.
  const PageScaleConstraints& PageDefinedConstraints() const {
    return page_defined_constraints_;
  }
  void UpdatePageDefinedConstraints(const ViewportDescription&,
                                    const Length& legacy_fallback_width);
  void AdjustForAndroidWebViewQuirks(const ViewportDescription&,
                                     int layout_fallback_width,
                                     bool support_target_density_dpi,
                                     bool wide_viewport_quirk_enabled,
                                     bool use_wide_viewport,
                                     bool load_with_overview_mode,
                                     bool non_user_scalable_quirk_enabled);
  void ClearPageDefinedConstraints();

  // Constraints may also be set from Chromium -- this overrides any
  // page-defined values.
  const PageScaleConstraints& UserAgentConstraints() const {
    return user_agent_constraints_;
  }
  void SetUserAgentConstraints(const PageScaleConstraints&);

  const PageScaleConstraints& FullscreenConstraints() const {
    return fullscreen_constraints_;
  }
  void SetFullscreenConstraints(const PageScaleConstraints&);

  // Actual computed values, taking into account the above plus the current
  // viewport size and document width.
  const PageScaleConstraints& FinalConstraints() const {
    return final_constraints_;
  }
  void ComputeFinalConstraints();
  void DidChangeContentsSize(gfx::Size contents_size,
                             int vertical_scrollbar_width,
                             float page_scale_factor);

  // This should be set to true on each page load to note that the page scale
  // factor needs to be reset to its initial value.
  void SetNeedsReset(bool);
  bool NeedsReset() const { return needs_reset_; }

  // This is set when one of the inputs to finalConstraints changes.
  bool ConstraintsDirty() const { return constraints_dirty_; }

  void DidChangeInitialContainingBlockSize(const gfx::Size&);

  gfx::Size GetLayoutSize() const;

 private:
  PageScaleConstraints ComputeConstraintsStack() const;

  void AdjustFinalConstraintsToContentsSize();

  PageScaleConstraints default_constraints_;
  PageScaleConstraints page_defined_constraints_;
  PageScaleConstraints user_agent_constraints_;
  PageScaleConstraints fullscreen_constraints_;
  PageScaleConstraints final_constraints_;

  Member<Page> page_;

  int last_contents_width_;
  int last_vertical_scrollbar_width_;
  gfx::Size icb_size_;

  bool needs_reset_;
  bool constraints_dirty_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PAGE_SCALE_CONSTRAINTS_SET_H_

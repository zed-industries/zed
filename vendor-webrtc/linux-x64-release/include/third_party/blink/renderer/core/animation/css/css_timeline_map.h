// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_TIMELINE_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_TIMELINE_MAP_H_

#include "third_party/blink/renderer/core/core_export.h"

#include "third_party/blink/renderer/core/style/scoped_css_name.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

class DeferredTimeline;
class ScrollSnapshotTimeline;
class ScrollTimeline;
class ViewTimeline;

template <typename TimelineType>
using CSSTimelineMap =
    HeapHashMap<Member<const ScopedCSSName>, Member<TimelineType>>;

using CSSViewTimelineMap = CSSTimelineMap<ViewTimeline>;
using CSSScrollTimelineMap = CSSTimelineMap<ScrollTimeline>;
using CSSDeferredTimelineMap = CSSTimelineMap<DeferredTimeline>;
using TimelineAttachmentMap =
    HeapHashMap<Member<ScrollSnapshotTimeline>, Member<DeferredTimeline>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_TIMELINE_MAP_H_

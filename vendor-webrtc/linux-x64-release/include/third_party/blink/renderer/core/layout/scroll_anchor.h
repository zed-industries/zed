// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SCROLL_ANCHOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SCROLL_ANCHOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_offset.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace gfx {
class Vector2d;
}

namespace blink {

class LayoutObject;
class Node;
class ScrollableArea;
struct ScrollAnchorData;

static const int kMaxSerializedSelectorLength = 500;

struct SerializedAnchor {
  SerializedAnchor() : simhash(0) {}
  SerializedAnchor(const String& s, const LogicalOffset& p)
      : selector(s), relative_offset(p), simhash(0) {}
  SerializedAnchor(const ScrollAnchorData& data,
                   const ScrollableArea& scroller);

  bool IsValid() const { return !selector.empty(); }

  ScrollOffset GetScrollOffset(const ScrollableArea& scroller) const;

  // Used to locate an element previously used as a scroll anchor.
  const String selector;
  // Used to restore the previous offset of the element within its scroller.
  const LogicalOffset relative_offset;
  // Used to compare the similarity of a prospective anchor's contents to the
  // contents at the time the previous anchor was saved.
  const uint64_t simhash;
};

// Scrolls to compensate for layout movements (bit.ly/scroll-anchoring).
class CORE_EXPORT ScrollAnchor final {
  DISALLOW_NEW();

 public:
  ScrollAnchor();
  explicit ScrollAnchor(ScrollableArea*);
  ~ScrollAnchor();
  void Trace(Visitor* visitor) const;

  // The scroller that is scrolled to componsate for layout movements. Note
  // that the scroller can only be initialized once.
  void SetScroller(ScrollableArea*);

  // Returns true if the underlying scroller is set.
  bool HasScroller() const { return scroller_ != nullptr; }

  // The LayoutObject we are currently anchored to. Lazily computed during
  // notifyBeforeLayout() and cached until the next call to clear().
  LayoutObject* AnchorObject() const { return anchor_object_.Get(); }

  // Called when the scroller attached to this anchor is being destroyed.
  void Dispose();

  // Indicates that this ScrollAnchor, and all ancestor ScrollAnchors, should
  // compute new anchor nodes on their next notifyBeforeLayout().
  void Clear();

  // Indicates that this ScrollAnchor should compute a new anchor node on the
  // next call to notifyBeforeLayout().
  void ClearSelf();

  // Records the anchor's location in relation to the scroller. Should be
  // called when the scroller is about to be laid out.
  void NotifyBeforeLayout();

  // Scrolls to compensate for any change in the anchor's relative location.
  // Should be called at the end of the animation frame.
  void Adjust();

  enum class Corner {
    kTopLeft = 0,
    kBottomLeft,
    kTopRight,
  };
  // Which corner of the anchor object we are currently anchored to.
  // Only meaningful if anchorObject() is non-null.
  Corner GetCorner() const { return corner_; }

  // Attempt to restore |serialized_anchor| by scrolling to the element
  // identified by its selector, adjusting by its relative_offset.
  bool RestoreAnchor(const SerializedAnchor&);

  // Get the serialized representation of the current anchor_object_.
  // If there is not currently an anchor_object_, this will attempt to find one.
  // Repeated calls will re-use the previously calculated selector until the
  // anchor_object it corresponds to is cleared.
  const SerializedAnchor GetSerializedAnchor();

  // Checks if we hold any references to the specified object.
  bool RefersTo(const LayoutObject*) const;

  // Notifies us that an object will be removed from the layout tree.
  void NotifyRemoved(LayoutObject*);

  // This anchor is not active because we are applying scroll-start.
  void CancelAdjustment() { queued_ = false; }

 private:
  enum WalkStatus { kSkip = 0, kConstrain, kContinue, kReturn };

  static bool IsViable(WalkStatus status) {
    return status == kConstrain || status == kReturn;
  }

  void FindAnchor();
  // Search for an anchor inside the specified object. The result is stored in
  // anchor_object_. The status returned indicates whether it found something
  // viable or not, in which case we may stop searching. Note that if kConstrain
  // is returned, which is generally considered viable, we may need to take an
  // additional look for OOFs inside enclosing NG fragmentation contexts. OOFs
  // are direct children of fragmentainers, rather than being a child of their
  // actual containing block.
  WalkStatus FindAnchorRecursive(LayoutObject*);
  WalkStatus FindAnchorInOOFs(LayoutObject*);
  bool ComputeScrollAnchorDisablingStyleChanged();

  // Find viable anchor among the priority candidates. Returns true if anchor
  // has been found; returns false if anchor was not found, and we should look
  // for an anchor in the DOM order traversal.
  bool FindAnchorInPriorityCandidates();
  // Returns a closest ancestor layout object from the given node which isn't a
  // non-atomic inline and is not anonymous.
  LayoutObject* PriorityCandidateFromNode(const Node*) const;

  struct ExamineResult {
    explicit ExamineResult(WalkStatus s) : status(s), corner(Corner::kTopLeft) {
      DCHECK(!IsViable(status));
    }

    ExamineResult(WalkStatus s, Corner c) : status(s), corner(c) {
      DCHECK(IsViable(status));
    }

    WalkStatus status;
    Corner corner;
  };

  ExamineResult Examine(const LayoutObject*) const;
  // Examines a given priority candidate. Note that this is similar to Examine()
  // but it also checks that the given object is a descendant of the scroller
  // and that there is no object that has overflow-anchor: none between the
  // given object and the scroller.
  ExamineResult ExaminePriorityCandidate(const LayoutObject*) const;

  gfx::Vector2d ComputeAdjustment() const;

  // Previously calculated css selector that uniquely locates the current
  // anchor_object_. Cleared when the anchor_object_ is cleared.
  String saved_selector_;

  // The scroller to be adjusted by this ScrollAnchor. This is also the scroller
  // that owns us, unless it is the RootFrameViewport in which case we are owned
  // by the layout viewport.
  Member<ScrollableArea> scroller_;

  // The LayoutObject we should anchor to.
  Member<LayoutObject> anchor_object_;

  // Which corner of m_anchorObject's bounding box to anchor to.
  Corner corner_;

  // Location of anchor_object_ relative to scroller block-start at the time of
  // NotifyBeforeLayout().
  LogicalOffset saved_relative_offset_;

  // We suppress scroll anchoring after a style change on the anchor node or
  // one of its ancestors, if that change might have caused the node to move.
  // This bit tracks whether we have had a scroll-anchor-disabling style
  // change since the last layout.  It is recomputed in notifyBeforeLayout(),
  // and used to suppress adjustment in adjust().  See http://bit.ly/sanaclap.
  bool scroll_anchor_disabling_style_changed_;

  // True iff an adjustment check has been queued with the FrameView but not yet
  // performed.
  bool queued_;

  // This is set to true if the last anchor we have selected is a
  // 'content-visibility: auto' element that did not yet have a layout after
  // becoming visible.
  bool anchor_is_cv_auto_without_layout_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SCROLL_ANCHOR_H_

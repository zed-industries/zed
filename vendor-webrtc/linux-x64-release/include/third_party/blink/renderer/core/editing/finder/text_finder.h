/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_TEXT_FINDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_TEXT_FINDER_H_

#include "base/cancelable_callback.h"
#include "build/build_config.h"
#include "third_party/blink/public/mojom/frame/find_in_page.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class FindTaskController;
class LocalFrame;
class Range;
class WebLocalFrameImpl;

class CORE_EXPORT TextFinder final : public GarbageCollected<TextFinder> {
 public:
  bool Find(int identifier,
            const String& search_text,
            const mojom::blink::FindOptions& options,
            bool wrap_within_frame,
            bool* active_now = nullptr);
  void InitNewSession(const mojom::blink::FindOptions&  options);
  void ClearActiveFindMatch();
  void SetFindEndstateFocusAndSelection();
  void StopFindingAndClearSelection();
  void IncreaseMatchCount(int identifier, int count);
  int FindMatchMarkersVersion() const { return find_match_markers_version_; }
#if BUILDFLAG(IS_ANDROID)
  gfx::RectF ActiveFindMatchRect();
  Vector<gfx::RectF> FindMatchRects();
  int SelectNearestFindMatch(const gfx::PointF&, gfx::Rect* selection_rect);
#endif

  // Starts brand new scoping request: resets the scoping state and
  // asynchronously calls scopeStringMatches().
  void StartScopingStringMatches(int identifier,
                                 const String& search_text,
                                 const mojom::blink::FindOptions& options);

  // Cancels any outstanding requests for scoping string matches on the frame.
  void CancelPendingScopingEffort();

  // This function is called to reset the total number of matches found during
  // the scoping effort.
  void ResetMatchCount();

#if BUILDFLAG(IS_ANDROID)
  // Return the index in the find-in-page cache of the match closest to the
  // provided point in find-in-page coordinates, or -1 in case of error.
  // The squared distance to the closest match is returned in the
  // |distanceSquared| parameter.
  int NearestFindMatch(const gfx::PointF&, float* distance_squared);
#endif

  // Returns whether this frame has the active match.
  bool ActiveMatchFrame() const { return current_active_match_frame_; }

  // Returns the active match in the current frame. Could be a null range if
  // the local frame has no active match.
  Range* ActiveMatch() const { return active_match_.Get(); }

  void FlushCurrentScoping();

  // A match has been found by the current finding effort, so we should
  // update the marker for that match and push it to our cache.
  void DidFindMatch(int identifier,
                    int current_total_matches,
                    Range* result_range);

  void ResetActiveMatch() { active_match_ = nullptr; }

  bool FrameScoping() const { return frame_scoping_; }
  int TotalMatchCount() const { return total_match_count_; }
  bool ScopingInProgress() const { return scoping_in_progress_; }
  void IncreaseMarkerVersion();

  // Finishes the current scoping effort and triggers any updates if
  // appropriate.
  void FinishCurrentScopingEffort(int identifier);

  // A finding effort finished one run of scoping and found some matches,
  // so we should update markers, match count, etc.
  void UpdateMatches(int identifier,
                     int found_match_count,
                     bool finished_whole_request);

  explicit TextFinder(WebLocalFrameImpl& owner_frame);
  TextFinder(const TextFinder&) = delete;
  TextFinder& operator=(const TextFinder&) = delete;

  class FindMatch {
    DISALLOW_NEW();

   public:
    FindMatch(Range*, int ordinal);

    void Trace(Visitor*) const;

    Member<Range> range_;

    // 1-based index within this frame.
    int ordinal_;

    // In find-in-page coordinates.
    // Lazily calculated by updateFindMatchRects.
    gfx::RectF rect_;
  };

  void Trace(Visitor*) const;

 private:
  // Context needed by asynchronous tasks used for beforematch and scrolling.
  struct AsyncScrollContext {
    // Copy of parameters to FindInternal so that it can be called again later.
    int identifier;
    String search_text;
    mojom::blink::FindOptions options;
    bool wrap_within_frame;
    Persistent<Range> first_match;
    bool wrapped_around;

    // Range to fire beforematch on and scroll to. active_match_ may get
    // unassigned during the async steps, so we need to save it here.
    Persistent<Range> range;
  };

  // Same as Find but with extra internal parameters used to track incremental
  // attempts to scroll to the next match when the first/previous was hidden.
  bool FindInternal(int identifier,
                    const String& search_text,
                    const mojom::blink::FindOptions& options,
                    bool wrap_within_frame,
                    bool* active_now = nullptr,
                    Range* first_match = nullptr,
                    bool wrapped_around = false);

  // Notifies the delegate about a new selection rect.
  void ReportFindInPageSelection(const gfx::Rect& selection_rect,
                                 int active_match_ordinal,
                                 int identifier);

  void ReportFindInPageResultToAccessibility(int identifier);
  void ReportFindInPageTerminationToAccessibility();

  // Clear the find-in-page matches cache forcing rects to be fully
  // calculated again next time updateFindMatchRects is called.
  void ClearFindMatchesCache();

  // Forcing rects to be fully recomputed again next time UpdateFindMatchRects
  // is called. This is different from ClearFindMatchesCache which will clear
  // the matches cache.
  void InvalidateFindMatchRects();

#if BUILDFLAG(IS_ANDROID)
  // Select a find-in-page match marker in the current frame using a cache
  // match index returned by nearestFindMatch. Returns the ordinal of the new
  // selected match or -1 in case of error. Also provides the bounding box of
  // the marker in window coordinates if selectionRect is not null.
  int SelectFindMatch(unsigned index, gfx::Rect* selection_rect);
#endif

  // Compute and cache the rects for FindMatches if required.
  // Rects are automatically invalidated in case of content size changes.
  void UpdateFindMatchRects();

  // Sets the markers within a range as active or inactive. Returns true if at
  // least one such marker found.
  bool SetMarkerActive(Range*, bool active);

  // Removes all markers.
  void UnmarkAllTextMatches();

  // Removes the current frame from the global scoping effort and triggers any
  // updates if appropriate. This method does not mark the scoping operation
  // as finished.
  void FlushCurrentScopingEffort(int identifier);

  // Determines whether to invalidate the content area and scrollbar.
  void InvalidateIfNecessary();

  // Issues a paint invalidation on the layout viewport's vertical scrollbar,
  // which is responsible for painting the tickmarks.
  void InvalidatePaintForTickmarks();

  LocalFrame* GetFrame() const;

  WebLocalFrameImpl& OwnerFrame() const {
    DCHECK(owner_frame_);
    return *owner_frame_;
  }

  void Scroll(std::unique_ptr<AsyncScrollContext> context);

  Member<WebLocalFrameImpl> owner_frame_;

  Member<FindTaskController> find_task_controller_;

  // Indicates whether this frame currently has the active match.
  bool current_active_match_frame_;

  // The range of the active match for the current frame.
  Member<Range> active_match_;

  // The index of the active match for the current frame.
  int active_match_index_;

  // This variable keeps a cumulative total of matches found so far in this
  // frame, and is only incremented by calling IncreaseMatchCount.
  int total_match_count_;

  // Keeps track of whether the frame is currently scoping (being searched for
  // matches).
  bool frame_scoping_;

  // Identifier of the latest find-in-page request. Required to be stored in
  // the frame in order to reply if required in case the frame is detached.
  int find_request_identifier_;

  // Keeps track of when the scoping effort should next invalidate the scrollbar
  // and the frame area.
  int next_invalidate_after_;

  // Version number incremented whenever this frame's find-in-page match
  // markers change.
  int find_match_markers_version_;

  // Local cache of the find match markers currently displayed for this frame.
  HeapVector<FindMatch> find_matches_cache_;

  // Contents size when find-in-page match rects were last computed for this
  // frame's cache.
  gfx::Size document_size_for_current_find_match_rects_;

  // This flag is used by the scoping effort to determine if we need to figure
  // out which rectangle is the active match. Once we find the active
  // rectangle we clear this flag.
  bool should_locate_active_rect_;

  // Keeps track of whether there is an scoping effort ongoing in the frame.
  bool scoping_in_progress_;

  // Determines if the rects in the find-in-page matches cache of this frame
  // are invalid and should be recomputed.
  bool find_match_rects_are_valid_;

  base::CancelableOnceClosure scroll_task_;
};

}  // namespace blink

WTF_ALLOW_INIT_WITH_MEM_FUNCTIONS(blink::TextFinder::FindMatch)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FINDER_TEXT_FINDER_H_

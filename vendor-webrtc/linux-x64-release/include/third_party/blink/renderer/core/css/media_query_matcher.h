/*
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Library General Public
 *  License as published by the Free Software Foundation; either
 *  version 2 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Library General Public License for more details.
 *
 *  You should have received a copy of the GNU Library General Public License
 *  along with this library; see the file COPYING.LIB.  If not, write to
 *  the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 *  Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_MATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_MATCHER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/resolver/media_query_result.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class Document;
class MediaQueryList;
class MediaQueryListListener;
class MediaQueryEvaluator;
class MediaQuerySet;

// MediaQueryMatcher class is responsible for keeping a vector of pairs
// MediaQueryList x MediaQueryListListener. It is responsible for evaluating the
// queries whenever it is needed and to call the listeners if the corresponding
// query has changed. The listeners must be called in the very same order in
// which they have been added.

class CORE_EXPORT MediaQueryMatcher final
    : public GarbageCollected<MediaQueryMatcher> {
 public:
  explicit MediaQueryMatcher(Document&);
  MediaQueryMatcher(const MediaQueryMatcher&) = delete;
  MediaQueryMatcher& operator=(const MediaQueryMatcher&) = delete;
  ~MediaQueryMatcher();

  Document* GetDocument() const { return document_.Get(); }
  void DocumentDetached();

  void AddMediaQueryList(MediaQueryList*);
  void RemoveMediaQueryList(MediaQueryList*);

  void AddViewportListener(MediaQueryListListener*);
  void RemoveViewportListener(MediaQueryListListener*);

  MediaQueryList* MatchMedia(const String&);

  void MediaFeaturesChanged();
  void ViewportChanged();
  // Invokes ViewportChanged, if this matcher depends on the dynamic viewport.
  void DynamicViewportChanged();
  bool Evaluate(const MediaQuerySet*);

  void Trace(Visitor*) const;

 private:
  MediaQueryEvaluator* CreateEvaluator() const;

  Member<Document> document_;
  Member<MediaQueryEvaluator> evaluator_;

  using MediaQueryListSet = HeapLinkedHashSet<WeakMember<MediaQueryList>>;
  MediaQueryListSet media_lists_;

  using ViewportListenerSet = HeapLinkedHashSet<Member<MediaQueryListListener>>;
  ViewportListenerSet viewport_listeners_;

  // The set of result flags seen by Evaluate.
  //
  // We currently only act on the unit flag kDynamicViewport. In the future we
  // could also look at the other values to improve invalidation in those
  // cases.
  //
  // See MediaQueryExpValue::UnitFlags.
  MediaQueryResultFlags media_query_result_flags_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_MATCHER_H_

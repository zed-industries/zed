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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_LIST_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ExecutionContext;
class MediaQueryListListener;
class MediaQueryMatcher;
class MediaQuerySet;

// MediaQueryList interface is specified at
// https://drafts.csswg.org/cssom-view/#the-mediaquerylist-interface
// The objects of this class are returned by window.matchMedia. They may be used
// to retrieve the current value of the given media query and to add/remove
// listeners that will be called whenever the value of the query changes.

class CORE_EXPORT MediaQueryList final
    : public EventTarget,
      public ActiveScriptWrappable<MediaQueryList>,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  MediaQueryList(ExecutionContext*, MediaQueryMatcher*, MediaQuerySet*);
  MediaQueryList(const MediaQueryList&) = delete;
  MediaQueryList& operator=(const MediaQueryList&) = delete;
  ~MediaQueryList() override;

  String media() const;
  bool matches();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)

  // These two functions are provided for compatibility with JS code
  // written before the change listener became a DOM event.
  void addDeprecatedListener(V8EventListener*);
  void removeDeprecatedListener(V8EventListener*);

  // C++ code can use these functions to listen to changes instead of having to
  // use DOM event listeners.
  void AddListener(MediaQueryListListener*);
  void RemoveListener(MediaQueryListListener*);

  // Will return true if a DOM event should be scheduled.
  bool MediaFeaturesChanged(
      HeapVector<Member<MediaQueryListListener>>* listeners_to_notify);

  void Trace(Visitor*) const override;

  // From ScriptWrappable
  bool HasPendingActivity() const final;

  // From ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

 private:
  bool UpdateMatches();

  Member<MediaQueryMatcher> matcher_;
  Member<MediaQuerySet> media_;
  using ListenerList = HeapLinkedHashSet<Member<MediaQueryListListener>>;
  ListenerList listeners_;
  bool matches_dirty_;
  bool matches_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_LIST_H_

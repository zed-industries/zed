// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_API_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_API_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/navigation/navigation_api_history_entry_arrays.mojom-blink.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/public/web/web_history_item.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/core/navigation_api/navigate_event_dispatch_params.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class DOMException;
class HistoryItem;
class NavigationApiMethodTracker;
class NavigationUpdateCurrentEntryOptions;
class NavigationHistoryEntry;
class NavigateEvent;
class NavigationActivation;
class NavigationNavigateOptions;
class NavigationReloadOptions;
class NavigationResult;
class NavigationOptions;
class NavigationTransition;
class RegisteredEventListener;
class SerializedScriptValue;

class CORE_EXPORT NavigationApi final : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit NavigationApi(LocalDOMWindow*);
  ~NavigationApi() final = default;

  void InitializeForNewWindow(
      HistoryItem& current,
      WebFrameLoadType,
      CommitReason,
      NavigationApi* previous,
      const std::vector<WebHistoryItem>& back_entries,
      const std::vector<WebHistoryItem>& forward_entries,
      HistoryItem* previous_entry);
  void UpdateForNavigation(HistoryItem&, WebFrameLoadType);
  void SetEntriesForRestore(
      const mojom::blink::NavigationApiHistoryEntryArraysPtr&,
      mojom::blink::NavigationApiEntryRestoreReason);

  void UpdateCurrentEntryForTesting(HistoryItem& item);

  // The entries indicated by |keys| have been removed from the session history
  // in the browser process and should be disposed. In many cases, this won't
  // do anything because those entries have already been synchronously removed
  // in UpdateForNavigation(). However, if the entries are being removed due to
  // a navigation in a different frame or due to the user manually removing
  // things from their history, this callback will be our only notification
  // that those entries are no longer valid.
  void DisposeEntriesForSessionHistoryRemoval(const Vector<String>& keys);

  // From the navigation API's perspective, a dropped navigation is still
  // "ongoing"; that is, ongoing_navigation_event_ and ongoing_navigation_ are
  // non-null. (The impact of this is that another navigation will cancel that
  // ongoing navigation, in a web-developer-visible way.) But from the
  // perspective of other parts of Chromium which interface with the navigation
  // API, e.g. to deal with the loading spinner, dropped navigations are not
  // what they care about.
  bool HasNonDroppedOngoingNavigation() const;

  // Web-exposed:
  NavigationHistoryEntry* currentEntry() const;
  HeapVector<Member<NavigationHistoryEntry>> entries();
  void updateCurrentEntry(NavigationUpdateCurrentEntryOptions*,
                          ExceptionState&);
  NavigationTransition* transition() const { return transition_.Get(); }
  NavigationActivation* activation() const;

  bool canGoBack() const;
  bool canGoForward() const;

  NavigationResult* navigate(ScriptState*,
                             const String& url,
                             NavigationNavigateOptions*);
  NavigationResult* reload(ScriptState*, NavigationReloadOptions*);

  NavigationResult* traverseTo(ScriptState*,
                               const String& key,
                               NavigationOptions*);
  NavigationResult* back(ScriptState*, NavigationOptions*);
  NavigationResult* forward(ScriptState*, NavigationOptions*);

  // onnavigate is defined manually so that a UseCounter can be applied to just
  // the setter
  void setOnnavigate(EventListener* listener);
  EventListener* onnavigate() {
    return GetAttributeEventListener(event_type_names::kNavigate);
  }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(navigatesuccess, kNavigatesuccess)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(navigateerror, kNavigateerror)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(currententrychange, kCurrententrychange)

  enum class DispatchResult { kContinue, kAbort, kIntercept };
  DispatchResult DispatchNavigateEvent(NavigateEventDispatchParams*);

  // In the spec, we are only informed about canceled navigations. But in the
  // implementation we need to handle other cases:
  // - "Dropped" navigations, e.g. navigations to 204/205/Content-Disposition:
  //   attachment, need to be tracked for |HasNonDroppedOngoingNavigation()|.
  //   (See https://github.com/WICG/navigation-api/issues/137 for more on why
  //   they must be ignored.) This distinction is handled via the |reason|
  //   argument.
  void InformAboutCanceledNavigation(
      CancelNavigationReason reason = CancelNavigationReason::kOther);

  // Called when a traverse is cancelled before its navigate event is fired.
  void TraverseCancelled(const String& key,
                         mojom::blink::TraverseCancelledReason reason);

  int GetIndexFor(NavigationHistoryEntry*);
  NavigationHistoryEntry* GetExistingEntryFor(const String& key,
                                              const String& id);

  // EventTarget overrides:
  const AtomicString& InterfaceName() const final;
  ExecutionContext* GetExecutionContext() const final { return window_.Get(); }
  void AddedEventListener(const AtomicString&, RegisteredEventListener&) final;
  void RemovedEventListener(const AtomicString&,
                            const RegisteredEventListener&) final;

  void Trace(Visitor*) const final;

 private:
  friend class NavigateEvent;
  NavigationHistoryEntry* GetEntryForRestore(
      const mojom::blink::NavigationApiHistoryEntryPtr&);
  void PopulateKeySet();
  void UpdateActivation(HistoryItem* previous_entry, WebFrameLoadType);
  void AbortOngoingNavigation(ScriptState*, CancelNavigationReason);
  void DidFinishOngoingNavigation();
  void DidFailOngoingNavigation(ScriptValue);

  NavigationResult* PerformNonTraverseNavigation(
      ScriptState*,
      FrameLoadRequest&,
      scoped_refptr<SerializedScriptValue>,
      NavigationOptions*,
      WebFrameLoadType);

  DOMException* PerformSharedNavigationChecks(
      const String& method_name_for_error_message);

  void PromoteUpcomingNavigationToOngoing(const String& key);

  scoped_refptr<SerializedScriptValue> SerializeState(const ScriptValue&,
                                                      ExceptionState&);

  bool HasEntriesAndEventsDisabled() const;

  NavigationHistoryEntry* MakeEntryFromItem(HistoryItem&);

  Member<LocalDOMWindow> window_;
  HeapVector<Member<NavigationHistoryEntry>> entries_;
  HashMap<String, int> keys_to_indices_;
  int current_entry_index_ = -1;
  bool has_dropped_navigation_ = false;

  Member<NavigationTransition> transition_;
  Member<NavigationActivation> activation_;

  Member<NavigationApiMethodTracker> ongoing_api_method_tracker_;
  HeapHashMap<String, Member<NavigationApiMethodTracker>>
      upcoming_traverse_api_method_trackers_;
  Member<NavigationApiMethodTracker> upcoming_non_traverse_api_method_tracker_;

  Member<NavigateEvent> ongoing_navigate_event_;

  int navigate_event_handler_count_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATION_API_H_

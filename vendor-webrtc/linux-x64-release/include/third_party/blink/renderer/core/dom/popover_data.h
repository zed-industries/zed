// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_POPOVER_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_POPOVER_DATA_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/html/closewatcher/close_watcher.h"
#include "third_party/blink/renderer/core/html/forms/html_form_control_element.h"
#include "third_party/blink/renderer/core/html_element_type_helpers.h"
#include "third_party/blink/renderer/core/inspector/console_message.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

enum class PopoverVisibilityState {
  kHidden,
  kShowing,
};

using PopoverHoverShowMap =
    HeapHashMap<WeakMember<const HTMLFormControlElement>, TaskHandle>;

class PopoverData final : public GarbageCollected<PopoverData>,
                          public ElementRareDataField {
 public:
  PopoverData() = default;
  PopoverData(const PopoverData&) = delete;
  PopoverData& operator=(const PopoverData&) = delete;

  PopoverVisibilityState visibilityState() const { return visibility_state_; }
  void setVisibilityState(PopoverVisibilityState visibility_state) {
    visibility_state_ = visibility_state;
  }

  PopoverValueType type() const { return type_; }
  void setType(PopoverValueType type) {
    type_ = type;
    DCHECK_NE(type, PopoverValueType::kNone)
        << "Remove PopoverData rather than setting kNone type";
  }

  Element* invoker() const { return invoker_.Get(); }
  void setInvoker(Element* element) { invoker_ = element; }

  Element* previouslyFocusedElement() const {
    return previously_focused_element_.Get();
  }
  void setPreviouslyFocusedElement(Element* element) {
    previously_focused_element_ = element;
  }

  bool hasPendingToggleEventTask() const {
    return pending_toggle_event_task_.IsActive();
  }
  void cancelPendingToggleEventTask() { pending_toggle_event_task_.Cancel(); }
  void setPendingToggleEventTask(TaskHandle&& task) {
    DCHECK(!pending_toggle_event_task_.IsActive());
    pending_toggle_event_task_ = std::move(task);
  }

  bool pendingToggleEventStartedClosed() const {
    DCHECK(hasPendingToggleEventTask());
    return pending_toggle_event_started_closed_;
  }
  void setPendingToggleEventStartedClosed(bool was_closed) {
    DCHECK(!hasPendingToggleEventTask());
    pending_toggle_event_started_closed_ = was_closed;
  }

  class ScopedStartShowingOrHiding {
    STACK_ALLOCATED();

   public:
    explicit ScopedStartShowingOrHiding(const Element& popover,
                                        bool show_warning = true)
        : popover_(popover),
          was_set_(popover.GetPopoverData()->hiding_or_showing_this_popover_) {
      if (was_set_ && show_warning) {
        popover_.GetDocument().AddConsoleMessage(MakeGarbageCollected<
                                                 ConsoleMessage>(
            mojom::blink::ConsoleMessageSource::kOther,
            mojom::blink::ConsoleMessageLevel::kWarning,
            "The `beforetoggle` event handler for a popover triggered another "
            "popover to be shown or hidden. This is not recommended."));
      } else {
        popover_.GetPopoverData()->hiding_or_showing_this_popover_ = true;
      }
    }
    ~ScopedStartShowingOrHiding() {
      if (!was_set_ && popover_.GetPopoverData()) {
        popover_.GetPopoverData()->hiding_or_showing_this_popover_ = false;
      }
    }
    explicit operator bool() const { return was_set_; }

   private:
    const Element& popover_;
    bool was_set_;
  };

  PopoverHoverShowMap& hoverShowTasks() { return hover_show_tasks_; }

  void setHoverHideTask(TaskHandle&& task) {
    if (hover_hide_task_.IsActive()) {
      hover_hide_task_.Cancel();
    }
    hover_hide_task_ = std::move(task);
  }

  Element* implicitAnchor() const { return implicit_anchor_.Get(); }
  void setImplicitAnchor(Element* element) { implicit_anchor_ = element; }

  CloseWatcher* closeWatcher() { return close_watcher_.Get(); }
  void setCloseWatcher(CloseWatcher* close_watcher) {
    close_watcher_ = close_watcher;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(invoker_);
    visitor->Trace(previously_focused_element_);
    visitor->Trace(hover_show_tasks_);
    visitor->Trace(implicit_anchor_);
    visitor->Trace(close_watcher_);
    ElementRareDataField::Trace(visitor);
  }

 private:
  PopoverVisibilityState visibility_state_ = PopoverVisibilityState::kHidden;
  PopoverValueType type_ = PopoverValueType::kNone;
  WeakMember<Element> invoker_;
  WeakMember<Element> previously_focused_element_;

  // Any pending 'toggle' event waiting to be fired. Used for coalescing
  // behavior so that only one such event is fired.
  TaskHandle pending_toggle_event_task_;
  bool pending_toggle_event_started_closed_;

  // True when we're in the middle of trying to hide/show this popover.
  bool hiding_or_showing_this_popover_;

  // Map from elements with the 'popovertarget' attribute and
  // `popovertargetaction=hover` to a task that will show the popover after a
  // delay.
  PopoverHoverShowMap hover_show_tasks_;
  // A task that hides the popover after a delay.
  TaskHandle hover_hide_task_;

  // Used to set up an anchor relationship separately from CSS `anchor`
  // references.
  WeakMember<Element> implicit_anchor_;

  Member<CloseWatcher> close_watcher_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_POPOVER_DATA_H_

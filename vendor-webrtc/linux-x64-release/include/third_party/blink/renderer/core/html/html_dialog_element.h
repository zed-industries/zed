/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DIALOG_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DIALOG_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/events/toggle_event.h"
#include "third_party/blink/renderer/core/html/closewatcher/close_watcher.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace blink {

class Document;
class ExceptionState;
class PointerEvent;

enum class ClosedByState {
  kAny,
  kCloseRequest,
  kNone,
};

class CORE_EXPORT HTMLDialogElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLDialogElement(Document&);

  void Trace(Visitor*) const override;

  // open_attribute_being_removed should only be true when `close()` is being
  // run from the attribute change steps for the `open` attribute.
  void close(const String& return_value = String(),
             bool open_attribute_being_removed = false);
  void requestClose(ExceptionState& exception_state) {
    requestClose(String(), exception_state);
  }
  void requestClose(const String& return_value, ExceptionState&);
  void show(ExceptionState&);
  void showModal(ExceptionState&);
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  bool IsModal() const { return is_modal_; }
  bool IsOpen() const { return FastHasAttribute(html_names::kOpenAttr); }
  bool IsOpenAndActive() const { return IsOpen() && InActiveDocument(); }

  String returnValue() const { return return_value_; }
  void setReturnValue(const String& return_value) {
    return_value_ = return_value;
  }

  ClosedByState ClosedBy() const;
  String closedBy() const;
  void setClosedBy(const String& return_value);

  static void HandleDialogLightDismiss(const PointerEvent& pointer_event,
                                       const Node& target_node);

  void CloseWatcherFiredCancel(Event*);
  void CloseWatcherFiredClose();

  // Dialogs support focus, since the dialog focus algorithm
  // https://html.spec.whatwg.org/multipage/interactive-elements.html#dialog-focusing-steps
  // can decide to focus the dialog itself if the dialog does not have a focus
  // delegate.
  FocusableState SupportsFocus(UpdateBehavior) const override {
    return FocusableState::kFocusable;
  }
  bool IsKeyboardFocusableSlow(
      UpdateBehavior update_behavior =
          UpdateBehavior::kStyleAndLayout) const override;

  // https://html.spec.whatwg.org/C/#the-dialog-element
  // Chooses the focused element when show() or showModal() is invoked.
  void SetFocusForDialog();

  // This is the old dialog initial focus behavior which is currently being
  // replaced by SetFocusForDialog.
  // TODO(http://crbug.com/383230): Remove this when DialogNewFocusBehavior gets
  // to stable with no issues.
  static void SetFocusForDialogLegacy(HTMLDialogElement* dialog);

  bool IsValidBuiltinCommand(HTMLElement& invoker,
                             CommandEventType command) override;
  bool HandleCommandInternal(HTMLElement& invoker,
                             CommandEventType command) override;

  void AttributeChanged(const AttributeModificationParams&) override;
  void ParseAttribute(const AttributeModificationParams&) override;

 private:
  void SetCloseWatcherEnabledState();
  void CreateCloseWatcher();

  void SetIsModal(bool is_modal);
  void ScheduleCloseEvent();

  bool DispatchToggleEvents(bool opening, bool asModal = false);
  void DispatchPendingToggleEvent();

  bool is_modal_;
  // is_closing_ is set to true at the beginning of close() and is reset to
  // false after the call to close() finishes.
  bool is_closing_ = false;
  String return_value_;
  String request_close_return_value_;
  WeakMember<Element> previously_focused_element_;

  Member<CloseWatcher> close_watcher_;

  TaskHandle pending_toggle_event_task_;
  Member<ToggleEvent> pending_toggle_event_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_DIALOG_ELEMENT_H_

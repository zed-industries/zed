// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_EDIT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_EDIT_CONTEXT_H_

#include "third_party/blink/public/platform/web_text_input_type.h"
#include "third_party/blink/public/web/web_input_method_controller.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/base/ime/ime_text_span.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class DOMRect;
class EditContext;
class EditContextInit;
class HTMLElement;
class ExceptionState;
class InputMethodController;

// The goal of the EditContext is to expose the lower-level APIs provided by
// modern operating systems to facilitate various input modalities to unlock
// advanced editing scenarios. For more information please refer
// https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/master/EditContext/explainer.md.

class CORE_EXPORT EditContext final : public EventTarget,
                                      public ActiveScriptWrappable<EditContext>,
                                      public WebInputMethodController,
                                      public ElementRareDataField {
  DEFINE_WRAPPERTYPEINFO();

 public:
  EditContext(ScriptState* script_state, const EditContextInit* dict);
  static EditContext* Create(ScriptState* script_state,
                             const EditContextInit* dict);
  ~EditContext() override;

  // Event listeners for an EditContext.
  DEFINE_ATTRIBUTE_EVENT_LISTENER(textupdate, kTextupdate)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(textformatupdate, kTextformatupdate)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(characterboundsupdate, kCharacterboundsupdate)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(compositionstart, kCompositionstart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(compositionend, kCompositionend)

  // Public APIs for an EditContext (called from JS).

  // This API should be called when the selection has changed.
  // It takes as parameters a start and end character offsets, which are based
  // on the text held by the EditContext. This may be called by the web app when
  // a combination of control keys (e.g. Shift + Arrow) or mouse events result
  // in a change to the selection on the edited document.
  void updateSelection(uint32_t start,
                       uint32_t end,
                       ExceptionState& exception_state);

  // This API must be called whenever the client coordinates (i.e. relative to
  // the origin of the viewport) of the view of the EditContext have changed.
  // This includes if the viewport is scrolled or the position of the editable
  // contents changes in response to other updates to the view. The argument
  // describes a bounding box in client coordinates for the editable region.
  void updateControlBounds(DOMRect* control_bounds);

  // This API must be called whenever the client coordinates of the view of
  // the EditContext have changed. The argument describes a bounding box in
  // client coordinates for the current selection (or the caret if the
  // selection is collapsed.)
  void updateSelectionBounds(DOMRect* selection_bounds);

  // This API should be called when the consumer of the EditContext receives
  // CharacterBoundsUpdateEvent. The arguments to this method describe a
  // sequence of bounding boxes that are requested by
  // CharacterBoundsUpdateEvent.
  void updateCharacterBounds(
      uint32_t range_start,
      const HeapVector<Member<DOMRect>>& character_bounds);

  // Updates to the text driven by the webpage/javascript are performed
  // by calling this API on the EditContext. It accepts a range (start and end
  // offsets over the text) and the characters to insert at that
  // range. This should be called anytime the editable contents have been
  // updated. However, in general this should be avoided during the firing of
  // textupdate as it will result in a canceled composition.
  // Also, after calling this API, update the selection by calling
  // updateSelection.
  void updateText(uint32_t start,
                  uint32_t end,
                  const String& new_text,
                  ExceptionState& exception_state);

  // Get elements that are associated with this EditContext.
  const HeapVector<Member<HTMLElement>>& attachedElements();

  // Returns the text of the EditContext.
  String text() const;

  // Returns the current selectionStart of the EditContext.
  uint32_t selectionStart() const;

  // Returns the current selectionEnd of the EditContext.
  uint32_t selectionEnd() const;

  // Returns the start position of the range of the current cached character
  // bounds.
  uint32_t characterBoundsRangeStart() const;

  // Returns the current cached character bounds.
  const HeapVector<Member<DOMRect>> characterBounds();

  // Internal APIs (called from Blink).

  // EventTarget overrides
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  void SetExecutionContext(ExecutionContext* context);

  LocalDOMWindow* DomWindow() const;

  // ActiveScriptWrappable overrides.
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

  // WebInputMethodController overrides.
  bool SetComposition(const WebString& text,
                      const std::vector<ui::ImeTextSpan>& ime_text_spans,
                      const WebRange& replacement_range,
                      int selection_start,
                      int selection_end) override;
  bool CommitText(const WebString& text,
                  const std::vector<ui::ImeTextSpan>& ime_text_spans,
                  const WebRange& replacement_range,
                  int relative_caret_position) override;
  bool FinishComposingText(
      ConfirmCompositionBehavior selection_behavior) override;
  WebTextInputInfo TextInputInfo() override;
  int ComputeWebTextInputNextPreviousFlags() override { return 0; }
  WebRange CompositionRange() const override;
  // Populate `bounds` with the bounds of each item in EditContext's
  // stored character bounds, scaled to physical pixels.
  bool GetCompositionCharacterBounds(std::vector<gfx::Rect>& bounds) override;
  WebRange GetSelectionOffsets() const override;

  // When focus is called on an EditContext, it sets the active EditContext in
  // the document so it can use the text input state to send info about the
  // EditContext to text input clients in the browser process which will also
  // set focus of the text input clients on the corresponding context document.
  void Focus();

  // This API sets the active EditContext to null in the document that results
  // in a focus change event to the text input clients in the browser process
  // which will also remove focus from the corresponding context document.
  void Blur();

  // Populate |control_bounds| and |selection_bounds| with the bounds fetched
  // from the active EditContext, in physical pixels.
  void GetLayoutBounds(gfx::Rect* control_bounds,
                       gfx::Rect* selection_bounds) override;

  // Sets the composition range from the already existing text
  // This is used for reconversion scenarios in JPN IME.
  bool SetCompositionFromExistingText(
      int composition_start,
      int composition_end,
      const std::vector<ui::ImeTextSpan>& ime_text_spans);

  // For English typing.
  bool InsertText(const WebString& text);

  void DeleteBackward();
  void DeleteForward();
  void DeleteWordBackward();
  void DeleteWordForward();

  bool IsEditContextActive() const override;
  // Returns whether show()/hide() API is called from virtualkeyboard or not.
  ui::mojom::VirtualKeyboardVisibilityRequest
  GetLastVirtualKeyboardVisibilityRequest() const override;
  // Sets the VirtualKeyboard visibility request(show/hide/none).
  void SetVirtualKeyboardVisibilityRequest(
      ui::mojom::VirtualKeyboardVisibilityRequest vk_visibility_request)
      override;
  // Called from WebLocalFrame for English compositions
  // such as shape-writing, handwriting panels etc.
  // Extends the current selection range and removes the
  // characters from the buffer.
  void ExtendSelectionAndDelete(int before, int after);
  // Delete `before` characters preceding the current `selection_start_` and
  // `after` characters following the current `selection_end_`.
  void DeleteSurroundingText(int before, int after);

  // Change the selection range.
  // Optionally dispatch TextInputEvent to notify the
  // page that the selection has changed.
  void SetSelection(int start,
                    int end,
                    bool dispatch_text_update_event = false);

  // Sets rect_in_viewport to the surrounding rect, in physical pixels,
  // for the character range specified by `location` and `length`.
  // Returns true on success, false on failure (in which case
  // rect_in_viewport) is not changed.
  bool FirstRectForCharacterRange(uint32_t location,
                                  uint32_t length,
                                  gfx::Rect& rect_in_viewport);

  void AttachElement(HTMLElement* element_to_attach);
  void DetachElement(HTMLElement* element_to_detach);

 private:
  InputMethodController& GetInputMethodController() const;

  void DeleteCurrentSelection();

  // Events fired to JS.
  // Fires compositionstart event to JS whenever user starts a composition.
  bool DispatchCompositionStartEvent(const String& text);

  // Fires compositionend event to JS whenever composition is terminated by the
  // user.
  void DispatchCompositionEndEvent(const String& text);

  // The textformatupdate event is fired when the input method desires a
  // specific region to be styled in a certain fashion, limited to the style
  // properties that correspond with the properties that are exposed on
  // TextFormatUpdateEvent (e.g. backgroundColor, textDecoration, etc.). The
  // consumer of the EditContext should update their view accordingly to provide
  // the user with visual feedback as prescribed by the software keyboard.
  void DispatchTextFormatEvent(
      const std::vector<ui::ImeTextSpan>& ime_text_spans);

  // The textupdate event will be fired on the EditContext when user input has
  // resulted in characters being applied to the editable region. The event
  // signals the fact that the software keyboard or IME updated the text (and as
  // such that state is reflected in the shared buffer at the time the event is
  // fired). This can be a single character update, in the case of typical
  // typing scenarios, or multiple-character insertion based on the user
  // changing composition candidates. Even though text updates are the results
  // of the software keyboard modifying the buffer, the creator of the
  // EditContext is ultimately responsible for keeping its underlying model
  // up-to-date with the content that is being edited as well as telling the
  // EditContext about such changes. These could get out of sync, for example,
  // when updates to the editable content come in through other means (the
  // backspace key is a canonical example — no textupdate is fired in this case,
  // and the consumer of the EditContext should detect the keydown event and
  // remove characters as appropriate).
  void DispatchTextUpdateEvent(const String& text,
                               uint32_t update_range_start,
                               uint32_t update_range_end,
                               uint32_t new_selection_start,
                               uint32_t new_selection_end);

  // The characterboundsupdate event is fired when the range of the composition
  // is changed. The arguments indicates the range of the composition
  // where the character bounds are needed by Text Input Service. The consumer
  // of the EditContext should call updateCharacterBounds() to provide the
  // requested bounding boxes when receiving this event.
  void DispatchCharacterBoundsUpdateEvent(uint32_t range_start,
                                          uint32_t range_end);

  bool HasValidCompositionBounds() const;

  // Delete the characters in the existing composition range and end the
  // composition.
  void CancelComposition();

  void ClearCompositionState();

  // Returns selection_start_ if selection_start_ <= selection_end_,
  // otherwise returns selection_end_.
  uint32_t OrderedSelectionStart() const;
  // Returns selection_end_ if selection_end_ >= selection_start_,
  // otherwise returns selection_start_.
  uint32_t OrderedSelectionEnd() const;

  // EditContext member variables.
  String text_;

  // It is possible that selection_start > selection_end_,
  // indicating a "backwards" selection (e.g. when the user
  // drags the selection from right to left).
  // These should always be modified via SetSelection() to ensure
  // that the proper notifications are fired.
  uint32_t selection_start_ = 0;
  uint32_t selection_end_ = 0;

  // The following bounds are in CSS pixels.
  gfx::Rect control_bounds_;
  gfx::Rect selection_bounds_;
  Vector<gfx::Rect> character_bounds_;
  uint32_t character_bounds_range_start_ = 0;

  // This flag is set when the input method controller receives a
  // composition event from the IME. It keeps track of the start and
  // end composition events and fires JS events accordingly.
  bool has_composition_ = false;
  // This is used to keep track of the active composition text range.
  // It is reset once the composition ends.
  // composition_range_end_ should always be >= composition_range_start_.
  uint32_t composition_range_start_ = 0;
  uint32_t composition_range_end_ = 0;
  // Elements that are associated with this EditContext.
  HeapVector<Member<HTMLElement>> attached_elements_;

  WeakMember<ExecutionContext> execution_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_EDIT_CONTEXT_H_

// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_INPUT_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_INPUT_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_input_event_init.h"
#include "third_party/blink/renderer/core/dom/range.h"
#include "third_party/blink/renderer/core/dom/static_range.h"
#include "third_party/blink/renderer/core/events/ui_event.h"

namespace blink {

class DataTransfer;

class InputEvent final : public UIEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static InputEvent* Create(const AtomicString& type,
                            const InputEventInit* initializer,
                            ExceptionState& exception_state);

  // https://w3c.github.io/input-events/#h-interface-inputevent-attributes
  enum class InputType {
    kNone,
    // Insertion.
    kInsertText,
    kInsertLineBreak,
    kInsertParagraph,
    kInsertOrderedList,
    kInsertUnorderedList,
    kInsertHorizontalRule,
    kInsertFromPaste,
    kInsertFromDrop,
    kInsertFromYank,
    kInsertTranspose,
    kInsertReplacementText,
    kInsertCompositionText,
    kInsertLink,
    // Deletion.
    kDeleteWordBackward,
    kDeleteWordForward,
    kDeleteSoftLineBackward,
    kDeleteSoftLineForward,
    kDeleteHardLineBackward,
    kDeleteHardLineForward,
    kDeleteContentBackward,
    kDeleteContentForward,
    kDeleteByCut,
    kDeleteByDrag,
    // History.
    kHistoryUndo,
    kHistoryRedo,
    // Formatting.
    kFormatBold,
    kFormatItalic,
    kFormatUnderline,
    kFormatStrikeThrough,
    kFormatSuperscript,
    kFormatSubscript,
    kFormatJustifyCenter,
    kFormatJustifyFull,
    kFormatJustifyRight,
    kFormatJustifyLeft,
    kFormatIndent,
    kFormatOutdent,
    kFormatRemove,
    kFormatSetBlockTextDirection,

    // Add new input types immediately above this line.
    kNumberOfInputTypes,
  };

  enum EventIsComposing : bool {
    kNotComposing = false,
    kIsComposing = true,
  };

  static InputEvent* CreateBeforeInput(InputType,
                                       const String& data,
                                       EventIsComposing,
                                       const GCedStaticRangeVector*);
  static InputEvent* CreateBeforeInput(InputType,
                                       DataTransfer*,
                                       EventIsComposing,
                                       const GCedStaticRangeVector*);
  static InputEvent* CreateInput(InputType,
                                 const String& data,
                                 EventIsComposing,
                                 const GCedStaticRangeVector*);

  InputEvent(const AtomicString&, const InputEventInit*, ExceptionState&);
  // This variant of the constructor is more efficient than the InputEventInit
  // variant.
  InputEvent(const AtomicString& type,
             const UIEventInit& init,
             InputType input_type,
             const String& data,
             DataTransfer* data_transfer,
             EventIsComposing is_composing,
             const GCedStaticRangeVector* ranges);

  String inputType() const;
  const String& data() const { return data_; }
  DataTransfer* dataTransfer() const { return data_transfer_.Get(); }
  bool isComposing() const { return is_composing_; }
  // Returns a copy of target ranges during event dispatch, and returns an empty
  // vector after dispatch.
  StaticRangeVector getTargetRanges() const;

  bool IsInputEvent() const override;

  DispatchEventResult DispatchEvent(EventDispatcher&) override;

  void Trace(Visitor*) const override;

 private:
  InputType input_type_;
  String data_;
  Member<DataTransfer> data_transfer_;
  bool is_composing_;

  // We have to stored |Range| internally and only expose |StaticRange|, please
  // see comments in |dispatchEvent()|.
  RangeVector ranges_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_INPUT_EVENT_H_

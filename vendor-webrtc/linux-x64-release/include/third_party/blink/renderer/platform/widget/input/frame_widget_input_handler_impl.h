// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_FRAME_WIDGET_INPUT_HANDLER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_FRAME_WIDGET_INPUT_HANDLER_IMPL_H_

#include "third_party/blink/public/mojom/input/input_handler.mojom-blink.h"
#include "third_party/blink/public/mojom/input/stylus_writing_gesture.mojom-blink.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {
class MainThreadEventQueue;
class WidgetBase;

// This class provides an implementation of FrameWidgetInputHandler mojo
// interface. When a compositor thread is being used in the renderer the mojo
// channel is bound on the compositor thread. Method calls, and events received
// on the compositor thread are then placed in the MainThreadEventQueue for the
// associated WidgetBase. This is done as to ensure that input related
// messages and events that are handled on the compositor thread aren't
// executed before other input events that need to be processed on the
// main thread. ie. Since some messages flow to the compositor thread
// all input needs to flow there so the ordering of events is kept in sequence.
//
// eg. (B = Browser, CT = Compositor Thread, MT = Main Thread)
//   B sends MouseEvent
//   B sends Copy message
//   CT receives MouseEvent (CT might do something with the MouseEvent)
//   CT places MouseEvent in MainThreadEventQueue
//   CT receives Copy message (CT has no use for the Copy message)
//   CT places Copy message in MainThreadEventQueue
//   MT receives MouseEvent
//   MT receives Copy message
//
// When a compositor thread isn't used the mojo channel is just bound
// on the main thread and messages are handled right away.
class PLATFORM_EXPORT FrameWidgetInputHandlerImpl
    : public mojom::blink::FrameWidgetInputHandler {
 public:
  // The `widget` and `frame_widget_input_handler` should be invalidated
  // at the same time.
  FrameWidgetInputHandlerImpl(
      base::WeakPtr<WidgetBase> widget,
      base::WeakPtr<mojom::blink::FrameWidgetInputHandler>
          frame_widget_input_handler,
      scoped_refptr<MainThreadEventQueue> input_event_queue);
  FrameWidgetInputHandlerImpl(const FrameWidgetInputHandlerImpl&) = delete;
  FrameWidgetInputHandlerImpl& operator=(const FrameWidgetInputHandlerImpl&) =
      delete;
  ~FrameWidgetInputHandlerImpl() override;

  void AddImeTextSpansToExistingText(
      uint32_t start,
      uint32_t end,
      const Vector<ui::ImeTextSpan>& ime_text_spans) override;
  void ClearImeTextSpansByType(uint32_t start,
                               uint32_t end,
                               ui::ImeTextSpan::Type type) override;
  void SetCompositionFromExistingText(
      int32_t start,
      int32_t end,
      const Vector<ui::ImeTextSpan>& ime_text_spans) override;
  void ExtendSelectionAndDelete(int32_t before, int32_t after) override;
  void ExtendSelectionAndReplace(uint32_t before,
                                 uint32_t after,
                                 const String& replacement_text) override;
  void DeleteSurroundingText(int32_t before, int32_t after) override;
  void DeleteSurroundingTextInCodePoints(int32_t before,
                                         int32_t after) override;
  void SetEditableSelectionOffsets(int32_t start, int32_t end) override;
  void HandleStylusWritingGestureAction(
      mojom::blink::StylusWritingGestureDataPtr gesture_data,
      HandleStylusWritingGestureActionCallback callback) override;
  void ExecuteEditCommand(const String& command, const String& value) override;
  void Undo() override;
  void Redo() override;
  void Cut() override;
  void Copy() override;
  void CopyToFindPboard() override;
  void CenterSelection() override;
  void Paste() override;
  void PasteAndMatchStyle() override;
  void Replace(const String& word) override;
  void ReplaceMisspelling(const String& word) override;
  void Delete() override;
  void SelectAll() override;
  void CollapseSelection() override;
  void SelectRange(const gfx::Point& base, const gfx::Point& extent) override;
  void SelectAroundCaret(mojom::blink::SelectionGranularity granularity,
                         bool should_show_handle,
                         bool should_show_context_menu,
                         SelectAroundCaretCallback callback) override;
  void AdjustSelectionByCharacterOffset(
      int32_t start,
      int32_t end,
      blink::mojom::SelectionMenuBehavior selection_menu_behavior) override;
  void MoveRangeSelectionExtent(const gfx::Point& extent) override;
  void ScrollFocusedEditableNodeIntoView() override;
  void WaitForPageScaleAnimationForTesting(
      WaitForPageScaleAnimationForTestingCallback callback) override;
  void MoveCaret(const gfx::Point& point) override;

#if BUILDFLAG(IS_IOS)
  void StartAutoscrollForSelectionToPoint(const gfx::PointF& point) override;
  void StopAutoscroll() override;

  void RectForEditFieldChars(const gfx::Range& range,
                             RectForEditFieldCharsCallback callback) override;
#endif  // BUILDFLAG(IS_IOS)

 private:
  enum class UpdateState { kNone, kIsPasting, kIsSelectingRange };

  static void SetCompositionFromExistingTextMainThread(
      base::WeakPtr<WidgetBase> widget,
      int32_t start,
      int32_t end,
      const Vector<ui::ImeTextSpan>& ui_ime_text_spans);
  static void ExtendSelectionAndDeleteMainThread(
      base::WeakPtr<WidgetBase> widget,
      int32_t before,
      int32_t after);

  class HandlingState {
   public:
    HandlingState(const base::WeakPtr<WidgetBase>& widget, UpdateState state);
    ~HandlingState();

   private:
    base::WeakPtr<WidgetBase> widget_;
    bool original_select_range_value_;
    bool original_pasting_value_;
  };

  void RunOnMainThread(base::OnceClosure closure);
  static void ExecuteCommandOnMainThread(
      base::WeakPtr<WidgetBase> widget,
      base::WeakPtr<mojom::blink::FrameWidgetInputHandler> handler,
      const char* command,
      UpdateState state);

  bool ThreadedCompositingEnabled() { return input_event_queue_ != nullptr; }

  // These should only be accessed on the main thread.
  base::WeakPtr<WidgetBase> widget_;
  base::WeakPtr<mojom::blink::FrameWidgetInputHandler>
      main_thread_frame_widget_input_handler_;

  scoped_refptr<MainThreadEventQueue> input_event_queue_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_FRAME_WIDGET_INPUT_HANDLER_IMPL_H_

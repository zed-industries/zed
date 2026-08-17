// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_WIDGET_INPUT_HANDLER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_WIDGET_INPUT_HANDLER_IMPL_H_

#include <variant>

#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "build/build_config.h"
#include "cc/input/browser_controls_offset_tag_modifications.h"
#include "cc/input/browser_controls_state.h"
#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "mojo/public/cpp/bindings/direct_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/common/input/web_coalesced_input_event.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom-blink.h"

namespace blink {
class MainThreadEventQueue;
class WidgetBase;
class WidgetInputHandlerManager;

// This class provides an implementation of the mojo WidgetInputHandler
// interface. If threaded compositing is used this thread will live on
// the compositor thread and proxy events to the main thread. This
// is done so that events stay in order relative to other events.
class WidgetInputHandlerImpl : public mojom::blink::WidgetInputHandler {
 public:
  // The `widget` and `frame_widget_input_handler` should be invalidated
  // at the same time.
  WidgetInputHandlerImpl(
      scoped_refptr<WidgetInputHandlerManager> manager,
      scoped_refptr<MainThreadEventQueue> input_event_queue,
      base::WeakPtr<WidgetBase> widget,
      base::WeakPtr<mojom::blink::FrameWidgetInputHandler>
          frame_widget_input_handler);
  WidgetInputHandlerImpl(const WidgetInputHandlerImpl&) = delete;
  WidgetInputHandlerImpl& operator=(const WidgetInputHandlerImpl&) = delete;
  ~WidgetInputHandlerImpl() override;

  void SetReceiver(mojo::PendingReceiver<mojom::blink::WidgetInputHandler>
                       interface_receiver);

  void SetFocus(mojom::blink::FocusState focus_state) override;
  void MouseCaptureLost() override;
  void SetEditCommandsForNextKeyEvent(
      Vector<mojom::blink::EditCommandPtr> commands) override;
  void CursorVisibilityChanged(bool visible) override;
  void ImeSetComposition(const String& text,
                         const Vector<ui::ImeTextSpan>& ime_text_spans,
                         const gfx::Range& range,
                         int32_t start,
                         int32_t end,
                         ImeSetCompositionCallback callback) override;
  void ImeCommitText(const String& text,
                     const Vector<ui::ImeTextSpan>& ime_text_spans,
                     const gfx::Range& range,
                     int32_t relative_cursor_position,
                     ImeCommitTextCallback callback) override;
  void ImeFinishComposingText(bool keep_selection) override;
  void RequestTextInputStateUpdate() override;
  void RequestCompositionUpdates(bool immediate_request,
                                 bool monitor_request) override;
  void DispatchEvent(std::unique_ptr<WebCoalescedInputEvent>,
                     DispatchEventCallback callback) override;
  void DispatchNonBlockingEvent(
      std::unique_ptr<WebCoalescedInputEvent>) override;
  void WaitForInputProcessed(WaitForInputProcessedCallback callback) override;
#if BUILDFLAG(IS_ANDROID)
  void AttachSynchronousCompositor(
      mojo::PendingRemote<mojom::blink::SynchronousCompositorControlHost>
          control_host,
      mojo::PendingAssociatedRemote<mojom::blink::SynchronousCompositorHost>
          host,
      mojo::PendingAssociatedReceiver<mojom::blink::SynchronousCompositor>
          compositor_receiver) override;
#endif
  void GetFrameWidgetInputHandler(
      mojo::PendingAssociatedReceiver<mojom::blink::FrameWidgetInputHandler>
          interface_request) override;
  void UpdateBrowserControlsState(
      cc::BrowserControlsState constraints,
      cc::BrowserControlsState current,
      bool animate,
      const std::optional<cc::BrowserControlsOffsetTagModifications>&
          offset_tag_modifications) override;

  void InputWasProcessed();

 private:
  bool ShouldProxyToMainThread() const;
  void RunOnMainThread(base::OnceClosure closure);
  void Release();

  bool ThreadedCompositingEnabled() { return input_event_queue_ != nullptr; }

  scoped_refptr<WidgetInputHandlerManager> input_handler_manager_;
  scoped_refptr<MainThreadEventQueue> input_event_queue_;
  base::WeakPtr<WidgetBase> widget_;
  base::WeakPtr<mojom::blink::FrameWidgetInputHandler>
      frame_widget_input_handler_;

  // This callback is used to respond to the WaitForInputProcessed Mojo
  // message. We keep it around so that we can respond even if the renderer is
  // killed before we actually fully process the input.
  WaitForInputProcessedCallback input_processed_ack_;

  using Receiver = mojo::Receiver<mojom::blink::WidgetInputHandler>;
  using DirectReceiver = mojo::DirectReceiver<mojom::blink::WidgetInputHandler>;
  std::variant<std::monostate, Receiver, DirectReceiver> receiver_;

  base::WeakPtrFactory<WidgetInputHandlerImpl> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_WIDGET_INPUT_HANDLER_IMPL_H_

// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WEB_TEST_WEB_TEST_WEB_FRAME_WIDGET_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WEB_TEST_WEB_TEST_WEB_FRAME_WIDGET_IMPL_H_

#include <memory>
#include <utility>

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/test/frame_widget_test_helper.h"
#include "third_party/blink/renderer/core/frame/web_frame_widget_impl.h"

namespace blink {

// WebTestWebFrameWidgetImpl is used to run web tests. This class is a subclass
// of WebFrameWidgetImpl that overrides the minimal necessary portions. These
// portions are limited:
// 1) Handling single threaded compositing.
// 2) Emulating drag and drop.
//
// This class exists inside blink so that it can subclass the WebFrameWidgetImpl
// yet still depends on content/web_test/renderer classes. This will eventually
// be cleaned up with more content code moving into blink but it is fine for
// now since it is only used in tests.
class WebTestWebFrameWidgetImpl : public WebFrameWidgetImpl,
                                  public FrameWidgetTestHelper {
 public:
  WebTestWebFrameWidgetImpl(
      base::PassKey<WebLocalFrame>,
      CrossVariantMojoAssociatedRemote<
          mojom::blink::FrameWidgetHostInterfaceBase> frame_widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::blink::FrameWidgetInterfaceBase>
          frame_widget,
      CrossVariantMojoAssociatedRemote<mojom::blink::WidgetHostInterfaceBase>
          widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::blink::WidgetInterfaceBase>
          widget,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      const viz::FrameSinkId& frame_sink_id,
      bool hidden,
      bool never_composited,
      bool is_for_child_local_root,
      bool is_for_nested_main_frame,
      bool is_for_scalable_page,
      content::TestRunner* test_runner);

  ~WebTestWebFrameWidgetImpl() override;

  // FrameWidgetTestHelper overrides.
  void Reset() override;
  content::EventSender* GetEventSender() override;
  void SynchronouslyCompositeAfterTest(base::OnceClosure callback) override;
  void UpdateAllLifecyclePhasesAndComposite(
      base::OnceClosure completion_callback) override;

  // WebFrameWidget overrides.
  FrameWidgetTestHelper* GetFrameWidgetTestHelperForTesting() override;

  // FrameWidget overrides.
  void RequestDecode(const cc::DrawImage&,
                     base::OnceCallback<void(bool)>) override;

 private:
  // WebFrameWidgetImpl overrides.
  void BindLocalRoot(WebLocalFrame&) override;
  void StartDragging(LocalFrame* source_frame,
                     const WebDragData& drag_data,
                     DragOperationsMask operations_allowed,
                     const SkBitmap& drag_image,
                     const gfx::Vector2d& cursor_offset,
                     const gfx::Rect& drag_obj_rect) override;
  void DidAutoResize(const gfx::Size& size) override;

  // WidgetBaseClient overrides:
  void ScheduleAnimation(bool urgent) override;
  void WillBeginMainFrame() override;
  void ScheduleAnimationForWebTests() override;
  bool AllowsScrollResampling() override { return false; }
  void WasShown(bool was_evicted) override;

  content::TestRunner* GetTestRunner();

  void ScheduleAnimationInternal(bool do_raster);
  void AnimateNow();
  bool RequestedMainFramePending() override;

  // When |do_raster| is false, only a main frame animation step is performed,
  // but when true, a full composite is performed and a frame submitted to the
  // display compositor if there is any damage.
  // Note that compositing has the potential to detach the current frame and
  // thus destroy |this| before returning.
  void SynchronouslyComposite(base::OnceClosure callback, bool do_raster);

  // Perform the synchronous composite step for a given LayerTreeHost.
  static void DoComposite(cc::LayerTreeHost* layer_tree_host,
                          bool do_raster,
                          base::OnceClosure callback);
  std::unique_ptr<content::EventSender> event_sender_;

  content::TestRunner* const test_runner_;

  // For collapsing multiple simulated ScheduleAnimation() calls.
  bool animation_scheduled_ = false;
  // When using the single thread compositor, scheduling an animation will
  // silently drop the BeginMainFrame if the widget isn't visible. This can
  // lead to test waits racing with a visibility change event so this flag is
  // used to defer requested animation frames to run after the widget comes out
  // of being hidden.
  bool animation_deferred_while_hidden_ = false;
  // When true, an AnimateNow() is scheduled that will perform a full composite.
  // Otherwise, any scheduled AnimateNow() calls will only perform the animation
  // step, which calls out to blink but doesn't composite for performance
  // reasons. See setAnimationRequiresRaster() in
  // https://chromium.googlesource.com/chromium/src/+/main/docs/testing/writing_web_tests.md
  // for details on the optimization.
  bool composite_requested_ = false;
  // Synchronous composites should not be nested inside another
  // composite, and this bool is used to guard against that.
  bool in_synchronous_composite_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WEB_TEST_WEB_TEST_WEB_FRAME_WIDGET_IMPL_H_

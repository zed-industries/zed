// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_TEST_FRAME_WIDGET_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_PUBLIC_TEST_FRAME_WIDGET_TEST_HELPER_H_

#include "base/functional/callback_forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/web/web_frame_widget.h"

namespace content {
class EventSender;
class TestRunner;
}  // namespace content

namespace blink {

// This class is used for content/web_test to create a subclass of
// `WebFrameWidgetImpl` that has special test hooks for controlling web tests.
// Since `WebFrameWidgetImpl` is not exported outside blink we need to have
// a special interface that exposes some additional hooks. These special
// hooks are contained on the `FrameWidgetTestHelper` class and the subclass
// implements `WebFrameWidget::GetFrameWidgetTestHelperForTesting()`.
//
// We allow usage of content::EventSender/TestRunner here temporarily while
// these methods are eventually moved into blink. This code is built and
// linked for tests.
class FrameWidgetTestHelper {
 public:
  // Creates a special subclass of WebFrameWidget that also implements
  // the FrameWidgetTestHelper interface.
  static WebFrameWidget* CreateTestWebFrameWidget(
      base::PassKey<WebLocalFrame>,
      CrossVariantMojoAssociatedRemote<mojom::FrameWidgetHostInterfaceBase>
          frame_widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::FrameWidgetInterfaceBase>
          frame_widget,
      CrossVariantMojoAssociatedRemote<mojom::WidgetHostInterfaceBase>
          widget_host,
      CrossVariantMojoAssociatedReceiver<mojom::WidgetInterfaceBase> widget,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      const viz::FrameSinkId& frame_sink_id,
      bool hidden,
      bool never_composited,
      bool is_for_child_local_root,
      bool is_for_nested_main_frame,
      bool is_for_scalable_page,
      content::TestRunner* test_runner);

  // Reset state for web tests.
  virtual void Reset() = 0;

  // Return a handle to content::EventSender.
  virtual content::EventSender* GetEventSender() = 0;

  // Called to composite when the test has ended, in order to ensure the test
  // produces up-to-date pixel output. This is a separate path as most
  // compositing paths stop running when the test ends, to avoid tests running
  // forever.
  virtual void SynchronouslyCompositeAfterTest(base::OnceClosure callback) = 0;

  // Forces a redraw and invokes the callback once the frame's been displayed
  // to the user in the display compositor.
  virtual void UpdateAllLifecyclePhasesAndComposite(
      base::OnceClosure completion_callback) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_TEST_FRAME_WIDGET_TEST_HELPER_H_

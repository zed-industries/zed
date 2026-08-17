// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_PAGE_TEST_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_PAGE_TEST_BASE_H_

#include <memory>

#include <gtest/gtest.h>
#include "services/service_manager/public/cpp/interface_provider.h"
#include "third_party/blink/renderer/core/testing/dummy_page_holder.h"
#include "third_party/blink/renderer/core/testing/mock_clipboard_host.h"
#include "third_party/blink/renderer/core/testing/scoped_mock_overlay_scrollbars.h"
#include "third_party/blink/renderer/platform/testing/task_environment.h"
#include "third_party/blink/renderer/platform/testing/testing_platform_support.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace base {
class TickClock;
}

namespace blink {

class AnimationClock;
class BrowserInterfaceBrokerProxy;
class Document;
class FrameSelection;
class LayoutObject;
class LocalFrame;
class PendingAnimations;
class StyleEngine;

class PageTestBase : public testing::Test, public ScopedMockOverlayScrollbars {
  USING_FAST_MALLOC(PageTestBase);

 public:
  // Helper class to provide a mock clipboard host for a LocalFrame.
  class MockClipboardHostProvider {
   public:
    explicit MockClipboardHostProvider(
        const blink::BrowserInterfaceBrokerProxy& interface_broker);
    MockClipboardHostProvider();
    ~MockClipboardHostProvider();

    // Installs a mock clipboard in the given interface provider.
    // This is called automatically from the ctor that takes an
    // |interface_broker| argument.
    void Install(const blink::BrowserInterfaceBrokerProxy& interface_broker);

    MockClipboardHost* clipboard_host() { return &host_; }

   private:
    void BindClipboardHost(mojo::ScopedMessagePipeHandle handle);

    const blink::BrowserInterfaceBrokerProxy* interface_broker_ = nullptr;
    MockClipboardHost host_;
  };

  PageTestBase();
  PageTestBase(base::test::TaskEnvironment::TimeSource time_source);
  ~PageTestBase() override;

  void EnableCompositing();

  void SetUp() override;
  void TearDown() override;

  using FrameSettingOverrideFunction = void (*)(Settings&);

  void SetUp(gfx::Size);
  void SetupPageWithClients(ChromeClient* = nullptr,
                            LocalFrameClient* = nullptr,
                            FrameSettingOverrideFunction = nullptr,
                            gfx::Size size = gfx::Size(800, 600));
  // TODO(shanmuga.m@samsung.com): These two function to be unified.
  void SetBodyContent(const std::string&);
  void SetBodyInnerHTML(const String&);
  void SetHtmlInnerHTML(const std::string&);

  // Insert STYLE element with |style_rules|, no need to have "<style>", into
  // HEAD.
  void InsertStyleElement(const std::string& style_rules);

  // Navigate to |url| providing an empty response but
  // URL and security origin of the Document will be set to |url|.
  void NavigateTo(const KURL& url,
                  const WTF::HashMap<String, String>& headers = {});

  Document& GetDocument() const;
  Page& GetPage() const;
  LocalFrame& GetFrame() const;
  FrameSelection& Selection() const;
  DummyPageHolder& GetDummyPageHolder() const { return *dummy_page_holder_; }
  StyleEngine& GetStyleEngine();
  Element* GetElementById(const char* id) const;
  Element* QuerySelector(const char* selector) const;
  AnimationClock& GetAnimationClock();
  PendingAnimations& GetPendingAnimations();
  FocusController& GetFocusController() const;

  void UpdateAllLifecyclePhasesForTest();

  // Load the 'Ahem' font to the LocalFrame.
  // The 'Ahem' font is the only font whose font metrics is consistent across
  // platforms, but it's not guaranteed to be available.
  // See external/wpt/css/fonts/ahem/README for more about the 'Ahem' font.
  static void LoadAhem(LocalFrame&);

  // Install the font specified by `font_path` as `family_name` in `frame`.
  static void LoadFontFromFile(LocalFrame& fame,
                               String font_path,
                               const AtomicString& family_name);

  static void LoadNoto(LocalFrame&);

  static std::string ToSimpleLayoutTree(const LayoutObject& layout_object);

  void SetPreferCompositingToLCDText(bool enable);

 protected:
  void LoadAhem();
  void LoadNoto();
  void EnablePlatform();

  // Used by subclasses to provide a different tick clock. At the moment is only
  // used to initialize DummyPageHolder. Note that subclasses calling
  // EnablePlatform() do not need to redefine this because the platform's mock
  // tick clock will be automatically used (see the default implementation in
  // the source file).
  virtual const base::TickClock* GetTickClock();

  TestingPlatformSupport* platform() {
    DCHECK(platform_);
    return platform_->GetTestingPlatformSupport();
  }

  test::TaskEnvironment& task_environment() { return task_environment_; }

  void FastForwardBy(base::TimeDelta);
  void FastForwardUntilNoTasksRemain();
  void AdvanceClock(base::TimeDelta);

 private:
  test::TaskEnvironment task_environment_;
  // The order is important: |platform_| must be destroyed after
  // |dummy_page_holder_| is destroyed.
  std::unique_ptr<ScopedTestingPlatformSupport<TestingPlatformSupport>>
      platform_;
  std::unique_ptr<DummyPageHolder> dummy_page_holder_;
  bool enable_compositing_ = false;

  MockClipboardHostProvider mock_clipboard_host_provider_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_PAGE_TEST_BASE_H_

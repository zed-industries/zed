// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_VIEW_OBSERVER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_VIEW_OBSERVER_H_

#include "base/memory/raw_ptr.h"
#include "base/observer_list_types.h"
#include "third_party/blink/public/mojom/page/page_visibility_state.mojom-shared.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {
class WebView;
class WebViewImpl;

// Base class for objects that want to get notified of changes to the view.
class BLINK_EXPORT WebViewObserver : public base::CheckedObserver {
 public:
  // A subclass can use this to delete itself.
  virtual void OnDestruct() = 0;

  // Notification that the output of a BeginMainFrame was committed to the
  // compositor (thread), though would not be submitted to the display
  // compositor yet. This will only be called for local main frames.
  virtual void DidCommitCompositorFrame() {}

  // Indicates two things:
  //   1) This view may have a new layout now.
  //   2) Layout is up-to-date.
  // After calling WebWidget::updateAllLifecyclePhases(), expect to get this
  // notification unless the view did not need a layout.
  virtual void DidUpdateMainFrameLayout() {}

  // Called when the View's zoom has changed.
  virtual void OnZoomLevelChanged() {}

  // Called when the View's visibility changes.
  virtual void OnPageVisibilityChanged(
      blink::mojom::PageVisibilityState visibility_state) {}

  // Retrieves the WebView that is being observed. Can be null.
  WebView* GetWebView() const;

 protected:
  friend class WebViewImpl;
  explicit WebViewObserver(WebView* web_view);
  ~WebViewObserver() override;

  // Called when |web_view_| was destroyed.
  void WebViewDestroyed();

  // Sets |web_view_| to track.
  // Removes itself of previous (if any) |web_view_| observer list and adds
  // to the new |web_view|.
  void Observe(WebView* web_view);

 private:
  raw_ptr<WebViewImpl> web_view_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_VIEW_OBSERVER_H_

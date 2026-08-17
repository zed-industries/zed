// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LOCAL_FRAME_OBSERVER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LOCAL_FRAME_OBSERVER_H_

#include "base/observer_list_types.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

namespace blink {
class WebFormElement;
class WebLocalFrame;
class WebLocalFrameImpl;

// Base class for objects that want to get notified of changes to the local
// frame.
class BLINK_EXPORT WebLocalFrameObserver : public base::CheckedObserver {
 public:
  // A subclass can use this to delete itself.
  virtual void OnFrameDetached() = 0;

  // A form submission has been requested, but the page's submit event handler
  // hasn't yet had a chance to run (and possibly alter/interrupt the submit.)
  virtual void WillSendSubmitEvent(const WebFormElement&) {}

  // Retrieves the WebLocalFrame that is being observed. Can be null.
  WebLocalFrame* GetWebLocalFrame() const;

 protected:
  friend class WebLocalFrameImpl;
  explicit WebLocalFrameObserver(WebLocalFrame* web_local_frame);
  ~WebLocalFrameObserver() override;

 private:
  // Called when `web_local_frame_` was detached.
  void WebLocalFrameDetached();

  // Sets `WebLocalFrame` to track.
  // Removes itself of previous (if any) `web_local_frame_` observer list and
  // adds to the new `web_local_frame`.
  void Observe(WebLocalFrameImpl* web_local_frame);

  WebPrivatePtrForGC<WebLocalFrameImpl,
                     WebPrivatePtrDestruction::kSameThread,
                     WebPrivatePtrStrength::kWeak>
      web_local_frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LOCAL_FRAME_OBSERVER_H_

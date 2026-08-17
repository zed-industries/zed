// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_CONTENT_CAPTURE_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_CONTENT_CAPTURE_CLIENT_H_

#include <vector>

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"

namespace blink {

class WebContentHolder;

// This interface is called by ContentCaptureManager to talk to the embedder,
// the ContentCapture is disabled without implementation of this interface.
class WebContentCaptureClient {
 public:
  // Adjusts the ContentCaptureTask delay time, has no effect for the existing
  // tasks.
  virtual base::TimeDelta GetTaskInitialDelay() const = 0;

  // Invoked to notify that a batch of Content Capture has completed.
  virtual void DidCompleteBatchCaptureContent() = 0;

  // Invoked when a list of |content| is captured, |first_content| indicates if
  // this is first captured content in the current document.
  virtual void DidCaptureContent(const std::vector<WebContentHolder>& content,
                                 bool first_data) = 0;

  // Invoked when a list of |content| is updated.
  virtual void DidUpdateContent(
      const std::vector<WebContentHolder>& content) = 0;

  // Invoked when the previously captured content is removed, |content_ids| is a
  // list of removed content id.
  virtual void DidRemoveContent(std::vector<int64_t> content_ids) = 0;

 protected:
  virtual ~WebContentCaptureClient() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_CONTENT_CAPTURE_CLIENT_H_

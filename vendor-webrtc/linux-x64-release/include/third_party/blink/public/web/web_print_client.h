// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PRINT_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PRINT_CLIENT_H_

namespace blink {

class WebPrintClient {
 public:
  // Let the client know it will be destroyed soon, to give it a chance to do
  // any necessary cleanup work.
  virtual void WillBeDestroyed() {}

 protected:
  virtual ~WebPrintClient() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PRINT_CLIENT_H_

// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_AUTOFILL_WEB_FORM_ELEMENT_OBSERVER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_AUTOFILL_WEB_FORM_ELEMENT_OBSERVER_H_

#include "base/functional/callback.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class WebFormControlElement;
class WebFormElement;

// TODO(tonikitoo): This uses BLINK_EXPORT instead of BLINK_MODULES_EXPORT
// because it is implemented by renderer/core/autofill code, where
// BLINK_MODULES_IMPLEMENTATION is not defined and compilation fails on win-dbg.
//
class BLINK_EXPORT WebFormElementObserver {
 public:
  // Creates a WebFormElementObserver. Delete this WebFormElementObsrver by
  // calling WebFormElementObserver::Disconnect.
  static WebFormElementObserver* Create(WebFormElement&, base::OnceClosure);
  static WebFormElementObserver* Create(WebFormControlElement&,
                                        base::OnceClosure);

  virtual void Disconnect() = 0;

 protected:
  WebFormElementObserver() = default;
  virtual ~WebFormElementObserver() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_AUTOFILL_WEB_FORM_ELEMENT_OBSERVER_H_

// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_FORM_ELEMENT_OBSERVER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_FORM_ELEMENT_OBSERVER_IMPL_H_

#include "base/types/pass_key.h"
#include "third_party/blink/public/web/modules/autofill/web_form_element_observer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"

namespace blink {

class HTMLElement;

class CORE_EXPORT WebFormElementObserverImpl final
    : public GarbageCollected<WebFormElementObserverImpl>,
      public WebFormElementObserver {
 public:
  WebFormElementObserverImpl(base::PassKey<WebFormElementObserver>,
                             HTMLElement&,
                             base::OnceClosure);
  WebFormElementObserverImpl(const WebFormElementObserverImpl&) = delete;
  WebFormElementObserverImpl& operator=(const WebFormElementObserverImpl&) =
      delete;
  ~WebFormElementObserverImpl() override;

  // WebFormElementObserver implementation.
  void Disconnect() override;

  void Trace(Visitor*) const;

 private:
  class ObserverCallback;

  Member<ObserverCallback> mutation_callback_;

  // WebFormElementObserverImpl must remain alive until Disconnect() is called.
  SelfKeepAlive<WebFormElementObserverImpl> self_keep_alive_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_FORM_ELEMENT_OBSERVER_IMPL_H_

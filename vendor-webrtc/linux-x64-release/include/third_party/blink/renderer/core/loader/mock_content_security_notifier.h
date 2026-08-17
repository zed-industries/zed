// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MOCK_CONTENT_SECURITY_NOTIFIER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MOCK_CONTENT_SECURITY_NOTIFIER_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/mojom/loader/content_security_notifier.mojom-blink.h"

namespace blink {

class MockContentSecurityNotifier
    : public mojom::blink::ContentSecurityNotifier {
 public:
  MockContentSecurityNotifier() = default;
  ~MockContentSecurityNotifier() override = default;

  MockContentSecurityNotifier(const MockContentSecurityNotifier&) = delete;
  MockContentSecurityNotifier& operator=(const MockContentSecurityNotifier&) =
      delete;

  mojo::PendingRemote<mojom::blink::ContentSecurityNotifier>
  BindNewPipeAndPassRemote() {
    return receiver_.BindNewPipeAndPassRemote();
  }

  void Bind(mojo::PendingReceiver<mojom::blink::ContentSecurityNotifier>
                pending_receiver) {
    receiver_.Bind(std::move(pending_receiver));
  }

  MOCK_METHOD0(NotifyContentWithCertificateErrorsRan, void());
  MOCK_METHOD0(NotifyContentWithCertificateErrorsDisplayed, void());
  MOCK_METHOD2(NotifyInsecureContentRan,
               void(const KURL& origin, const KURL& insecure_url));

 private:
  mojo::Receiver<mojom::blink::ContentSecurityNotifier> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MOCK_CONTENT_SECURITY_NOTIFIER_H_

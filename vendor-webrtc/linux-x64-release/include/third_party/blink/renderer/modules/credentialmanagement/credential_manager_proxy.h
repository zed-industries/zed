// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_MANAGER_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_MANAGER_PROXY_H_

#include "base/functional/callback_forward.h"
#include "third_party/blink/public/mojom/credentialmanagement/credential_manager.mojom-blink.h"
#include "third_party/blink/public/mojom/payments/secure_payment_confirmation_service.mojom-blink.h"
#include "third_party/blink/public/mojom/sms/webotp_service.mojom-blink.h"
#include "third_party/blink/public/mojom/webauthn/authenticator.mojom-blink.h"
#include "third_party/blink/public/mojom/webid/digital_identity_request.mojom-blink.h"
#include "third_party/blink/public/mojom/webid/federated_auth_request.mojom-blink.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ScriptState;

// Owns the client end of the mojo::CredentialManager interface connection to an
// implementation that services requests in the security context of the window
// supplemented by this CredentialManagerProxy instance.
//
// This facilitates routing API calls to be serviced in the correct security
// context, even if the `window.navigator.credentials` instance from one
// browsing context was passed to another; in which case the Credential
// Management API call must still be serviced in the browsing context
// responsible for actually calling the API method, as opposed to the context
// whose global object owns the CredentialsContainer instance on which the
// method was called.
class MODULES_EXPORT CredentialManagerProxy
    : public GarbageCollected<CredentialManagerProxy>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  explicit CredentialManagerProxy(LocalDOMWindow&);
  virtual ~CredentialManagerProxy();

  mojom::blink::CredentialManager* CredentialManager();

  mojom::blink::Authenticator* Authenticator();

  mojom::blink::WebOTPService* WebOTPService();

  payments::mojom::blink::SecurePaymentConfirmationService*
  SecurePaymentConfirmationService();

  mojom::blink::FederatedAuthRequest* FederatedAuthRequest();

  mojom::blink::DigitalIdentityRequest* DigitalIdentityRequest();

  void Trace(Visitor*) const override;

  // Must be called only with argument representing a valid
  // context corresponding to an attached window.
  static CredentialManagerProxy* From(ScriptState*);
  static CredentialManagerProxy* From(ExecutionContext*);

  static CredentialManagerProxy* From(LocalDOMWindow*);

 private:
  template <typename Interface>
  void BindRemoteForFedCm(HeapMojoRemote<Interface>& remote,
                          base::OnceClosure disconnect_closure);
  void OnFederatedAuthRequestConnectionError();
  void OnDigitalIdentityRequestConnectionError();

  HeapMojoRemote<mojom::blink::Authenticator> authenticator_;
  HeapMojoRemote<mojom::blink::CredentialManager> credential_manager_;
  HeapMojoRemote<mojom::blink::WebOTPService> webotp_service_;
  HeapMojoRemote<payments::mojom::blink::SecurePaymentConfirmationService>
      spc_service_;
  HeapMojoRemote<mojom::blink::FederatedAuthRequest> federated_auth_request_;
  HeapMojoRemote<mojom::blink::DigitalIdentityRequest>
      digital_identity_request_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_MANAGER_PROXY_H_

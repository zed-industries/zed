// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_MANAGER_TYPE_CONVERTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_MANAGER_TYPE_CONVERTERS_H_

#include <optional>

#include "mojo/public/cpp/bindings/type_converter.h"
#include "third_party/blink/public/mojom/credentialmanagement/credential_manager.mojom-blink.h"
#include "third_party/blink/public/mojom/webauthn/authenticator.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/webid/digital_identity_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/webid/federated_auth_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/webid/federated_auth_request.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_all_accepted_credentials_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_unknown_credential_options.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class AllAcceptedCredentialsOptions;
class AuthenticationExtensionsClientInputs;
class AuthenticationExtensionsClientOutputs;
class AuthenticationExtensionsPRFInputs;
class AuthenticationExtensionsPRFValues;
class AuthenticationExtensionsPaymentOutputs;
class AuthenticationExtensionsSupplementalPubKeysInputs;
class AuthenticationExtensionsSupplementalPubKeysOutputs;
class AuthenticatorSelectionCriteria;
class CableAuthenticationData;
class Credential;
class CurrentUserDetailsOptions;
class IdentityCredentialDisconnectOptions;
class IdentityProviderAccount;
class IdentityProviderConfig;
class IdentityProviderRequestOptions;
class LoginStatusOptions;
class IdentityUserInfo;
class PublicKeyCredentialCreationOptions;
class PublicKeyCredentialDescriptor;
class PublicKeyCredentialParameters;
class PublicKeyCredentialRequestOptions;
class PublicKeyCredentialRpEntity;
class PublicKeyCredentialUserEntity;
class RemoteDesktopClientOverride;
class UserVerificationRequirement;
class LoginStatusOptions;
class V8IdentityCredentialRequestOptionsContext;
class V8IdentityCredentialRequestOptionsMode;
class V8UnionArrayBufferOrArrayBufferView;
}  // namespace blink

namespace mojo {

// blink::mojom::blink::CredentialManager --------------------------

template <>
struct TypeConverter<blink::mojom::blink::CredentialInfoPtr,
                     blink::Credential*> {
  static blink::mojom::blink::CredentialInfoPtr Convert(blink::Credential*);
};

template <>
struct TypeConverter<blink::Credential*,
                     blink::mojom::blink::CredentialInfoPtr> {
  static blink::Credential* Convert(
      const blink::mojom::blink::CredentialInfoPtr&);
};

// "Reverse" converters. These convert from from Mojo structures to
// IDL-generated structures.

template <>
struct MODULES_EXPORT TypeConverter<
    blink::AuthenticationExtensionsClientOutputs*,
    blink::mojom::blink::AuthenticationExtensionsClientOutputsPtr> {
  static blink::AuthenticationExtensionsClientOutputs* Convert(
      const blink::mojom::blink::AuthenticationExtensionsClientOutputsPtr&);
};

template <>
struct TypeConverter<blink::AuthenticationExtensionsSupplementalPubKeysOutputs*,
                     blink::mojom::blink::SupplementalPubKeysResponsePtr> {
  static blink::AuthenticationExtensionsSupplementalPubKeysOutputs* Convert(
      const blink::mojom::blink::SupplementalPubKeysResponsePtr&);
};

template <>
struct TypeConverter<
    blink::AuthenticationExtensionsPaymentOutputs*,
    blink::mojom::blink::AuthenticationExtensionsPaymentResponsePtr> {
  static blink::AuthenticationExtensionsPaymentOutputs* Convert(
      const blink::mojom::blink::AuthenticationExtensionsPaymentResponsePtr&);
};

// blink::mojom::blink::Authenticator ---------------------------------------
template <>
struct TypeConverter<Vector<uint8_t>,
                     blink::V8UnionArrayBufferOrArrayBufferView*> {
  static Vector<uint8_t> Convert(
      const blink::V8UnionArrayBufferOrArrayBufferView*);
};

template <>
struct TypeConverter<
    std::optional<blink::mojom::blink::PublicKeyCredentialType>,
    String> {
  static std::optional<blink::mojom::blink::PublicKeyCredentialType> Convert(
      const String&);
};

template <>
struct TypeConverter<
    WTF::Vector<blink::mojom::blink::PublicKeyCredentialParametersPtr>,
    blink::HeapVector<blink::Member<blink::PublicKeyCredentialParameters>>> {
  static WTF::Vector<blink::mojom::blink::PublicKeyCredentialParametersPtr>
  Convert(const blink::HeapVector<
          blink::Member<blink::PublicKeyCredentialParameters>>&
              input_pub_key_cred_params);
};

template <>
struct TypeConverter<std::optional<blink::mojom::blink::AuthenticatorTransport>,
                     String> {
  static std::optional<blink::mojom::blink::AuthenticatorTransport> Convert(
      const String&);
};

template <>
struct TypeConverter<String, blink::mojom::blink::AuthenticatorTransport> {
  static String Convert(const blink::mojom::blink::AuthenticatorTransport&);
};

template <>
struct TypeConverter<std::optional<blink::mojom::blink::ResidentKeyRequirement>,
                     String> {
  static std::optional<blink::mojom::blink::ResidentKeyRequirement> Convert(
      const String&);
};

template <>
struct TypeConverter<
    std::optional<blink::mojom::blink::UserVerificationRequirement>,
    String> {
  static std::optional<blink::mojom::blink::UserVerificationRequirement>
  Convert(const String&);
};

template <>
struct TypeConverter<
    std::optional<blink::mojom::blink::AttestationConveyancePreference>,
    String> {
  static std::optional<blink::mojom::blink::AttestationConveyancePreference>
  Convert(const String&);
};

template <>
struct TypeConverter<
    std::optional<blink::mojom::blink::AuthenticatorAttachment>,
    std::optional<String>> {
  static std::optional<blink::mojom::blink::AuthenticatorAttachment> Convert(
      const std::optional<String>&);
};

template <>
struct TypeConverter<blink::mojom::blink::LargeBlobSupport,
                     std::optional<String>> {
  static blink::mojom::blink::LargeBlobSupport Convert(
      const std::optional<String>&);
};

template <>
struct TypeConverter<blink::mojom::blink::AuthenticatorSelectionCriteriaPtr,
                     blink::AuthenticatorSelectionCriteria> {
  static blink::mojom::blink::AuthenticatorSelectionCriteriaPtr Convert(
      const blink::AuthenticatorSelectionCriteria&);
};

template <>
struct TypeConverter<blink::mojom::blink::PublicKeyCredentialUserEntityPtr,
                     blink::PublicKeyCredentialUserEntity> {
  static blink::mojom::blink::PublicKeyCredentialUserEntityPtr Convert(
      const blink::PublicKeyCredentialUserEntity&);
};

template <>
struct TypeConverter<blink::mojom::blink::PublicKeyCredentialRpEntityPtr,
                     blink::PublicKeyCredentialRpEntity> {
  static blink::mojom::blink::PublicKeyCredentialRpEntityPtr Convert(
      const blink::PublicKeyCredentialRpEntity&);
};

template <>
struct TypeConverter<blink::mojom::blink::PublicKeyCredentialDescriptorPtr,
                     blink::PublicKeyCredentialDescriptor> {
  static blink::mojom::blink::PublicKeyCredentialDescriptorPtr Convert(
      const blink::PublicKeyCredentialDescriptor&);
};

template <>
struct TypeConverter<blink::mojom::blink::PublicKeyCredentialParametersPtr,
                     blink::PublicKeyCredentialParameters> {
  static blink::mojom::blink::PublicKeyCredentialParametersPtr Convert(
      const blink::PublicKeyCredentialParameters&);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::PublicKeyCredentialCreationOptionsPtr,
                  blink::PublicKeyCredentialCreationOptions> {
  static blink::mojom::blink::PublicKeyCredentialCreationOptionsPtr Convert(
      const blink::PublicKeyCredentialCreationOptions&);
};

template <>
struct TypeConverter<blink::mojom::blink::CableAuthenticationPtr,
                     blink::CableAuthenticationData> {
  static blink::mojom::blink::CableAuthenticationPtr Convert(
      const blink::CableAuthenticationData&);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::PublicKeyCredentialRequestOptionsPtr,
                  blink::PublicKeyCredentialRequestOptions> {
  static blink::mojom::blink::PublicKeyCredentialRequestOptionsPtr Convert(
      const blink::PublicKeyCredentialRequestOptions&);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::AuthenticationExtensionsClientInputsPtr,
                  blink::AuthenticationExtensionsClientInputs> {
  static blink::mojom::blink::AuthenticationExtensionsClientInputsPtr Convert(
      const blink::AuthenticationExtensionsClientInputs&);
};

template <>
struct TypeConverter<blink::mojom::blink::RemoteDesktopClientOverridePtr,
                     blink::RemoteDesktopClientOverride> {
  static blink::mojom::blink::RemoteDesktopClientOverridePtr Convert(
      const blink::RemoteDesktopClientOverride&);
};

template <>
struct TypeConverter<blink::mojom::blink::IdentityProviderConfigPtr,
                     blink::IdentityProviderConfig> {
  static blink::mojom::blink::IdentityProviderConfigPtr Convert(
      const blink::IdentityProviderConfig&);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::IdentityProviderRequestOptionsPtr,
                  blink::IdentityProviderRequestOptions> {
  static blink::mojom::blink::IdentityProviderRequestOptionsPtr Convert(
      const blink::IdentityProviderRequestOptions&);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::RpContext,
                  blink::V8IdentityCredentialRequestOptionsContext> {
  static blink::mojom::blink::RpContext Convert(
      const blink::V8IdentityCredentialRequestOptionsContext&);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::RpMode,
                  blink::V8IdentityCredentialRequestOptionsMode> {
  static blink::mojom::blink::RpMode Convert(
      const blink::V8IdentityCredentialRequestOptionsMode&);
};

template <>
struct TypeConverter<blink::mojom::blink::IdentityUserInfoPtr,
                     blink::IdentityUserInfo> {
  static blink::mojom::blink::IdentityUserInfoPtr Convert(
      const blink::IdentityUserInfo&);
};

template <>
struct TypeConverter<
    std::optional<blink::mojom::blink::SupplementalPubKeysRequestPtr>,
    blink::AuthenticationExtensionsSupplementalPubKeysInputs> {
  static std::optional<blink::mojom::blink::SupplementalPubKeysRequestPtr>
  Convert(const blink::AuthenticationExtensionsSupplementalPubKeysInputs&);
};

template <>
struct TypeConverter<blink::mojom::blink::PRFValuesPtr,
                     blink::AuthenticationExtensionsPRFValues> {
  static StructPtr<blink::mojom::blink::PRFValues> Convert(
      const blink::AuthenticationExtensionsPRFValues&);
};

template <>
struct TypeConverter<Vector<blink::mojom::blink::PRFValuesPtr>,
                     blink::AuthenticationExtensionsPRFInputs> {
  static Vector<StructPtr<blink::mojom::blink::PRFValues>> Convert(
      const blink::AuthenticationExtensionsPRFInputs&);
};

template <>
struct TypeConverter<
    blink::mojom::blink::IdentityCredentialDisconnectOptionsPtr,
    blink::IdentityCredentialDisconnectOptions> {
  static blink::mojom::blink::IdentityCredentialDisconnectOptionsPtr Convert(
      const blink::IdentityCredentialDisconnectOptions&);
};

template <>
struct TypeConverter<Vector<blink::mojom::blink::Hint>, Vector<String>> {
  static Vector<blink::mojom::blink::Hint> Convert(const Vector<String>&);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::PublicKeyCredentialReportOptionsPtr,
                  blink::UnknownCredentialOptions> {
  static blink::mojom::blink::PublicKeyCredentialReportOptionsPtr Convert(
      const blink::UnknownCredentialOptions&);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::PublicKeyCredentialReportOptionsPtr,
                  blink::AllAcceptedCredentialsOptions> {
  static blink::mojom::blink::PublicKeyCredentialReportOptionsPtr Convert(
      const blink::AllAcceptedCredentialsOptions&);
};

template <>
struct MODULES_EXPORT
    TypeConverter<blink::mojom::blink::PublicKeyCredentialReportOptionsPtr,
                  blink::CurrentUserDetailsOptions> {
  static blink::mojom::blink::PublicKeyCredentialReportOptionsPtr Convert(
      const blink::CurrentUserDetailsOptions&);
};

template <>
struct MODULES_EXPORT TypeConverter<blink::mojom::blink::LoginStatusAccountPtr,
                                    blink::IdentityProviderAccount> {
  static blink::mojom::blink::LoginStatusAccountPtr Convert(
      const blink::IdentityProviderAccount&);
};

template <>
struct MODULES_EXPORT TypeConverter<blink::mojom::blink::LoginStatusOptionsPtr,
                                    blink::LoginStatusOptions> {
  static blink::mojom::blink::LoginStatusOptionsPtr Convert(
      const blink::LoginStatusOptions&);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_MANAGER_TYPE_CONVERTERS_H_

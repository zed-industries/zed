// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_MOJOM_AUTHENTICATOR_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_MOJOM_AUTHENTICATOR_MOJOM_TRAITS_H_

#include <optional>
#include <string>
#include <vector>

#include "base/containers/flat_tree.h"
#include "base/notreached.h"
#include "device/fido/authenticator_selection_criteria.h"
#include "device/fido/cable/cable_discovery_data.h"
#include "device/fido/fido_constants.h"
#include "device/fido/fido_transport_protocol.h"
#include "device/fido/public_key_credential_descriptor.h"
#include "device/fido/public_key_credential_params.h"
#include "device/fido/public_key_credential_rp_entity.h"
#include "device/fido/public_key_credential_user_entity.h"
#include "mojo/public/cpp/bindings/array_traits_stl.h"
#include "mojo/public/cpp/bindings/enum_traits.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/webauthn/authenticator.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT EnumTraits<blink::mojom::AuthenticatorTransport,
                                      device::FidoTransportProtocol> {
  static blink::mojom::AuthenticatorTransport ToMojom(
      device::FidoTransportProtocol input);
  static bool FromMojom(blink::mojom::AuthenticatorTransport input,
                        device::FidoTransportProtocol* output);
};

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::PublicKeyCredentialType, device::CredentialType> {
  static blink::mojom::PublicKeyCredentialType ToMojom(
      device::CredentialType input);
  static bool FromMojom(blink::mojom::PublicKeyCredentialType input,
                        device::CredentialType* output);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::PublicKeyCredentialParametersDataView,
                 device::PublicKeyCredentialParams::CredentialInfo> {
  static device::CredentialType type(
      const device::PublicKeyCredentialParams::CredentialInfo& in) {
    return in.type;
  }

  static int32_t algorithm_identifier(
      const device::PublicKeyCredentialParams::CredentialInfo& in) {
    return in.algorithm;
  }

  static bool Read(blink::mojom::PublicKeyCredentialParametersDataView data,
                   device::PublicKeyCredentialParams::CredentialInfo* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::PublicKeyCredentialDescriptorDataView,
                 device::PublicKeyCredentialDescriptor> {
  static device::CredentialType type(
      const device::PublicKeyCredentialDescriptor& in) {
    return in.credential_type;
  }

  static const std::vector<uint8_t>& id(
      const device::PublicKeyCredentialDescriptor& in) {
    return in.id;
  }

  static const std::vector<device::FidoTransportProtocol> transports(
      const device::PublicKeyCredentialDescriptor& in) {
    std::vector<device::FidoTransportProtocol> protocols;
    for (const auto& protocol : in.transports) {
      protocols.push_back(protocol);
    }
    return protocols;
  }

  static bool Read(blink::mojom::PublicKeyCredentialDescriptorDataView data,
                   device::PublicKeyCredentialDescriptor* out);
};

template <>
struct BLINK_COMMON_EXPORT EnumTraits<blink::mojom::AuthenticatorAttachment,
                                      device::AuthenticatorAttachment> {
  static blink::mojom::AuthenticatorAttachment ToMojom(
      device::AuthenticatorAttachment input);
  static bool FromMojom(blink::mojom::AuthenticatorAttachment input,
                        device::AuthenticatorAttachment* output);
};

template <>
struct BLINK_COMMON_EXPORT EnumTraits<blink::mojom::ResidentKeyRequirement,
                                      device::ResidentKeyRequirement> {
  static blink::mojom::ResidentKeyRequirement ToMojom(
      device::ResidentKeyRequirement input);
  static bool FromMojom(blink::mojom::ResidentKeyRequirement input,
                        device::ResidentKeyRequirement* output);
};

template <>
struct BLINK_COMMON_EXPORT EnumTraits<blink::mojom::UserVerificationRequirement,
                                      device::UserVerificationRequirement> {
  static blink::mojom::UserVerificationRequirement ToMojom(
      device::UserVerificationRequirement input);
  static bool FromMojom(blink::mojom::UserVerificationRequirement input,
                        device::UserVerificationRequirement* output);
};

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::LargeBlobSupport, device::LargeBlobSupport> {
  static blink::mojom::LargeBlobSupport ToMojom(device::LargeBlobSupport input);
  static bool FromMojom(blink::mojom::LargeBlobSupport input,
                        device::LargeBlobSupport* output);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AuthenticatorSelectionCriteriaDataView,
                 device::AuthenticatorSelectionCriteria> {
  static device::AuthenticatorAttachment authenticator_attachment(
      const device::AuthenticatorSelectionCriteria& in) {
    return in.authenticator_attachment;
  }

  static device::ResidentKeyRequirement resident_key(
      const device::AuthenticatorSelectionCriteria& in) {
    return in.resident_key;
  }

  static device::UserVerificationRequirement user_verification(
      const device::AuthenticatorSelectionCriteria& in) {
    return in.user_verification_requirement;
  }

  static bool Read(blink::mojom::AuthenticatorSelectionCriteriaDataView data,
                   device::AuthenticatorSelectionCriteria* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::PublicKeyCredentialRpEntityDataView,
                 device::PublicKeyCredentialRpEntity> {
  static const std::string& id(const device::PublicKeyCredentialRpEntity& in) {
    return in.id;
  }

  static const std::optional<std::string>& name(
      const device::PublicKeyCredentialRpEntity& in) {
    return in.name;
  }

  static bool Read(blink::mojom::PublicKeyCredentialRpEntityDataView data,
                   device::PublicKeyCredentialRpEntity* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::PublicKeyCredentialUserEntityDataView,
                 device::PublicKeyCredentialUserEntity> {
  static const std::vector<uint8_t>& id(
      const device::PublicKeyCredentialUserEntity& in) {
    return in.id;
  }

  static const std::optional<std::string>& name(
      const device::PublicKeyCredentialUserEntity& in) {
    return in.name;
  }

  static const std::optional<std::string>& display_name(
      const device::PublicKeyCredentialUserEntity& in) {
    return in.display_name;
  }

  static bool Read(blink::mojom::PublicKeyCredentialUserEntityDataView data,
                   device::PublicKeyCredentialUserEntity* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::CableAuthenticationDataView,
                 device::CableDiscoveryData> {
  static uint8_t version(const device::CableDiscoveryData& in) {
    switch (in.version) {
      case device::CableDiscoveryData::Version::V1:
        return 1;
      case device::CableDiscoveryData::Version::V2:
        return 2;
      case device::CableDiscoveryData::Version::INVALID:
        NOTREACHED();
    }
  }

  static std::optional<device::CableEidArray> client_eid(
      const device::CableDiscoveryData& in) {
    if (in.version == device::CableDiscoveryData::Version::V1) {
      return in.v1->client_eid;
    }
    return std::nullopt;
  }

  static const std::optional<device::CableEidArray> authenticator_eid(
      const device::CableDiscoveryData& in) {
    if (in.version == device::CableDiscoveryData::Version::V1) {
      return in.v1->authenticator_eid;
    }
    return std::nullopt;
  }

  static const std::optional<device::CableSessionPreKeyArray> session_pre_key(
      const device::CableDiscoveryData& in) {
    if (in.version == device::CableDiscoveryData::Version::V1) {
      return in.v1->session_pre_key;
    }
    return std::nullopt;
  }

  static const std::optional<std::vector<uint8_t>> server_link_data(
      const device::CableDiscoveryData& in) {
    if (in.version == device::CableDiscoveryData::Version::V2) {
      return in.v2->server_link_data;
    }
    return std::nullopt;
  }

  static const std::optional<std::vector<uint8_t>> experiments(
      const device::CableDiscoveryData& in) {
    if (in.version == device::CableDiscoveryData::Version::V2) {
      return in.v2->experiments;
    }
    return std::nullopt;
  }

  static bool Read(blink::mojom::CableAuthenticationDataView data,
                   device::CableDiscoveryData* out);
};

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::AttestationConveyancePreference,
               device::AttestationConveyancePreference> {
  static blink::mojom::AttestationConveyancePreference ToMojom(
      device::AttestationConveyancePreference input);
  static bool FromMojom(blink::mojom::AttestationConveyancePreference input,
                        device::AttestationConveyancePreference* output);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_MOJOM_AUTHENTICATOR_MOJOM_TRAITS_H_

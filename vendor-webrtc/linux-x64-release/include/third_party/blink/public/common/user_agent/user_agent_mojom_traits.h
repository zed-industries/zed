// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_USER_AGENT_USER_AGENT_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_USER_AGENT_USER_AGENT_MOJOM_TRAITS_H_

#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"

#include <string>

#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/user_agent/user_agent_metadata.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::UserAgentBrandVersionDataView,
                 ::blink::UserAgentBrandVersion> {
  static const std::string& brand(const ::blink::UserAgentBrandVersion& data) {
    return data.brand;
  }

  static const std::string& version(
      const ::blink::UserAgentBrandVersion& data) {
    return data.version;
  }

  static bool Read(blink::mojom::UserAgentBrandVersionDataView data,
                   ::blink::UserAgentBrandVersion* out);
};

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::UserAgentMetadataDataView,
                                        ::blink::UserAgentMetadata> {
  static const blink::UserAgentBrandList& brand_version_list(
      const ::blink::UserAgentMetadata& data) {
    return data.brand_version_list;
  }

  static const blink::UserAgentBrandList& brand_full_version_list(
      const ::blink::UserAgentMetadata& data) {
    return data.brand_full_version_list;
  }

  static const std::string& full_version(
      const ::blink::UserAgentMetadata& data) {
    return data.full_version;
  }
  static const std::string& platform(const ::blink::UserAgentMetadata& data) {
    return data.platform;
  }
  static const std::string& platform_version(
      const ::blink::UserAgentMetadata& data) {
    return data.platform_version;
  }
  static const std::string& architecture(
      const ::blink::UserAgentMetadata& data) {
    return data.architecture;
  }
  static const std::string& model(const ::blink::UserAgentMetadata& data) {
    return data.model;
  }

  static const bool& mobile(const ::blink::UserAgentMetadata& data) {
    return data.mobile;
  }

  static const std::string& bitness(const ::blink::UserAgentMetadata& data) {
    return data.bitness;
  }

  static bool wow64(const ::blink::UserAgentMetadata& data) {
    return data.wow64;
  }

  static const std::vector<std::string>& form_factors(
      const ::blink::UserAgentMetadata& data) {
    return data.form_factors;
  }

  static bool Read(blink::mojom::UserAgentMetadataDataView data,
                   ::blink::UserAgentMetadata* out);
};

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::UserAgentOverrideDataView,
                                        ::blink::UserAgentOverride> {
  static const std::string& ua_string_override(
      const ::blink::UserAgentOverride& data) {
    return data.ua_string_override;
  }

  static const std::optional<::blink::UserAgentMetadata>& ua_metadata_override(
      const ::blink::UserAgentOverride& data) {
    return data.ua_metadata_override;
  }

  static bool Read(blink::mojom::UserAgentOverrideDataView,
                   ::blink::UserAgentOverride* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_USER_AGENT_USER_AGENT_MOJOM_TRAITS_H_

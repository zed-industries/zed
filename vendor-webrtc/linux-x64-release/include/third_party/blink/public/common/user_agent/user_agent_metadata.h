// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_USER_AGENT_USER_AGENT_METADATA_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_USER_AGENT_USER_AGENT_METADATA_H_

#include <optional>
#include <string>
#include <vector>

#include "third_party/blink/public/common/common_export.h"

namespace blink {

// Values for the Sec-CH-UA-Form-Factors header.
// https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors
// LINT.IfChange
inline constexpr char kDesktopFormFactor[] = "Desktop";
inline constexpr char kAutomotiveFormFactor[] = "Automotive";
inline constexpr char kMobileFormFactor[] = "Mobile";
inline constexpr char kTabletFormFactor[] = "Tablet";
inline constexpr char kXRFormFactor[] = "XR";
inline constexpr char kEInkFormFactor[] = "EInk";
inline constexpr char kWatchFormFactor[] = "Watch";
// LINT.ThenChange(/android_webview/java/src/org/chromium/android_webview/client_hints/AwUserAgentMetadata.java)

struct BLINK_COMMON_EXPORT UserAgentBrandVersion {
  UserAgentBrandVersion() = default;
  UserAgentBrandVersion(const std::string& ua_brand,
                        const std::string& ua_version);

  bool operator==(const UserAgentBrandVersion& a) const;

  std::string brand;
  // Version type is either "full version" or "major version".
  // For brands, `version` is populated with the major version for each brand.
  // For the full version list, `version` is populated with the full version for
  // each brand.
  // https://wicg.github.io/ua-client-hints/#interface
  std::string version;
};

using UserAgentBrandList = std::vector<UserAgentBrandVersion>;

struct BLINK_COMMON_EXPORT UserAgentMetadata {
 private:
  // Common private function turning the brand list into a structured header
  // comes up often enough and is just non-trivial enough that it's better to be
  // in one place.
  const std::string SerializeBrandVersionList(
      const blink::UserAgentBrandList& ua_brand_version_list);

 public:
  // Turning the brand list into a structured header with full version and major
  // version.
  const std::string SerializeBrandFullVersionList();
  const std::string SerializeBrandMajorVersionList();
  const std::string SerializeFormFactors();

  static std::optional<UserAgentMetadata> Demarshal(
      const std::optional<std::string>& encoded);
  static std::optional<std::string> Marshal(
      const std::optional<UserAgentMetadata>& ua_metadata);
  UserAgentBrandList brand_version_list;
  UserAgentBrandList brand_full_version_list;

  std::string full_version;
  std::string platform;
  std::string platform_version;
  std::string architecture;
  std::string model;
  bool mobile = false;
  std::string bitness;
  bool wow64 = false;

  // The form-factors list. It is up to the embedder to ensure that this is
  // compliant with the w3c draft spec:
  // https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors.
  std::vector<std::string> form_factors;
};

// Used when customizing the sent User-Agent and Sec-CH-UA-* for
// features like "request desktop site", which override those from defaults
// for some individual navigations. WebContents::SetUserAgentOverride()
// is the main entry point used for the functionality.
struct BLINK_COMMON_EXPORT UserAgentOverride {
  // Helper which sets only UA with blank client hints.
  static UserAgentOverride UserAgentOnly(const std::string& ua);

  // Empty |ua_string_override| means no override;
  // |ua_metadata_override| must also be null in that case.
  std::string ua_string_override;

  // Non-nullopt if custom values for user agent client hint properties
  // should be used. If this is null, and |ua_string_override| is non-empty,
  // no UA client hints will be sent.
  std::optional<UserAgentMetadata> ua_metadata_override;
};

bool BLINK_COMMON_EXPORT operator==(const UserAgentMetadata& a,
                                    const UserAgentMetadata& b);

bool BLINK_COMMON_EXPORT operator==(const UserAgentOverride& a,
                                    const UserAgentOverride& b);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_USER_AGENT_USER_AGENT_METADATA_H_

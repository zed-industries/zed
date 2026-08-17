// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WEBID_LOGIN_STATUS_ACCOUNT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WEBID_LOGIN_STATUS_ACCOUNT_H_

#include <optional>
#include <string>

#include "base/types/optional_ref.h"
#include "third_party/blink/public/common/common_export.h"
#include "url/gurl.h"

namespace blink::common::webid {

// Plain-Old-Data struct containing information needed to display a useful
// account list entry for Lightweight FedCM. Members are a subset of fields in
// type `IdentityProviderAccount` in
// third_party/blink/renderer/modules/credentialmanagement/identity_provider_account.idl
// When a new field is added, be sure to update the base::Value conversion logic
// in InMemoryFederatedPermissionContext::GetAccounts
struct BLINK_COMMON_EXPORT LoginStatusAccount {
  LoginStatusAccount();

  LoginStatusAccount(const std::string& id,
                     const std::string& email,
                     const std::string& name,
                     base::optional_ref<const std::string> given_name,
                     base::optional_ref<const GURL> picture_url);
  ~LoginStatusAccount();
  bool operator==(const LoginStatusAccount& account) const;

  // STL strings are used here in accordance with the exceptions described in
  // third_party/blink/renderer/README.md
  std::string id;
  std::string email;
  std::string name;
  std::optional<std::string> given_name;

  // A URL representing a picture associated with the user account.
  // GURL is used here in accordance with the exceptions described in
  // third_party/blink/renderer/README.md
  std::optional<GURL> picture;
};

}  // namespace blink::common::webid

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WEBID_LOGIN_STATUS_ACCOUNT_H_

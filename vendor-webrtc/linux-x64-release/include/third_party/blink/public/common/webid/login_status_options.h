// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WEBID_LOGIN_STATUS_OPTIONS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WEBID_LOGIN_STATUS_OPTIONS_H_
#include <optional>
#include <vector>

#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/webid/login_status_account.h"

namespace blink::common::webid {

// Plain-Old-Data struct containing configuration data for a login status
// declaration by an IdP. Corresponds to `LoginStatusOptions` in
// third_party/blink/renderer/modules/credentialmanagement/navigator_login.idl
struct BLINK_COMMON_EXPORT LoginStatusOptions {
  LoginStatusOptions();
  LoginStatusOptions(std::vector<LoginStatusAccount> accounts,
                     const std::optional<base::TimeDelta>& expiration);
  ~LoginStatusOptions();

  bool operator==(const LoginStatusOptions& account) const;

  // List of accounts associated with the login status
  std::vector<LoginStatusAccount> accounts;

  // The duration after which the account information should be cleared.
  std::optional<base::TimeDelta> expiration;
};

}  // namespace blink::common::webid

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WEBID_LOGIN_STATUS_OPTIONS_H_

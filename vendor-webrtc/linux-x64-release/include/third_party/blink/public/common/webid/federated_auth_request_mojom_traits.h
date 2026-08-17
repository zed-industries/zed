// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WEBID_FEDERATED_AUTH_REQUEST_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WEBID_FEDERATED_AUTH_REQUEST_MOJOM_TRAITS_H_

#include <optional>

#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/webid/login_status_account.h"
#include "third_party/blink/public/common/webid/login_status_options.h"
#include "third_party/blink/public/mojom/webid/federated_auth_request.mojom-shared.h"
#include "third_party/blink/public/mojom/webid/federated_auth_request.mojom.h"

namespace mojo {

template <>
class BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::LoginStatusAccountDataView,
                 blink::common::webid::LoginStatusAccount> {
 public:
  static const std::string& id(
      const blink::common::webid::LoginStatusAccount& a) {
    return a.id;
  }
  static const std::string& email(
      const blink::common::webid::LoginStatusAccount& a) {
    return a.email;
  }
  static const std::string& name(
      const blink::common::webid::LoginStatusAccount& a) {
    return a.name;
  }
  static const std::optional<std::string>& given_name(
      const blink::common::webid::LoginStatusAccount& a) {
    return a.given_name;
  }
  static const std::optional<GURL>& picture(
      const blink::common::webid::LoginStatusAccount& a) {
    return a.picture;
  }

  static bool Read(blink::mojom::LoginStatusAccountDataView data,
                   blink::common::webid::LoginStatusAccount* out);
};

template <>
class BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::LoginStatusOptionsDataView,
                 blink::common::webid::LoginStatusOptions> {
 public:
  static const std::vector<blink::common::webid::LoginStatusAccount>& accounts(
      const blink::common::webid::LoginStatusOptions& o) {
    return o.accounts;
  }
  static const std::optional<base::TimeDelta>& expiration(
      const blink::common::webid::LoginStatusOptions& o) {
    return o.expiration;
  }

  static bool Read(blink::mojom::LoginStatusOptionsDataView data,
                   blink::common::webid::LoginStatusOptions* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WEBID_FEDERATED_AUTH_REQUEST_MOJOM_TRAITS_H_

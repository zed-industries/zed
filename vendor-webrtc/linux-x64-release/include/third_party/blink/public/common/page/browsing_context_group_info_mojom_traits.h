// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_BROWSING_CONTEXT_GROUP_INFO_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_BROWSING_CONTEXT_GROUP_INFO_MOJOM_TRAITS_H_

#include "base/unguessable_token.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/page/browsing_context_group_info.h"
#include "third_party/blink/public/mojom/page/browsing_context_group_info.mojom-shared.h"

namespace mojo {

template <>
class BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::BrowsingContextGroupInfoDataView,
                 blink::BrowsingContextGroupInfo> {
 public:
  static const base::UnguessableToken& browsing_context_group_token(
      const blink::BrowsingContextGroupInfo& bcgi) {
    return bcgi.browsing_context_group_token;
  }
  static const base::UnguessableToken& coop_related_group_token(
      const blink::BrowsingContextGroupInfo& bcgi) {
    return bcgi.coop_related_group_token;
  }

  static bool Read(
      blink::mojom::BrowsingContextGroupInfoDataView data,
      blink::BrowsingContextGroupInfo* out_browsing_context_group_info);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_BROWSING_CONTEXT_GROUP_INFO_MOJOM_TRAITS_H_

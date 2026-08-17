// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_BROWSING_CONTEXT_GROUP_INFO_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_BROWSING_CONTEXT_GROUP_INFO_H_

#include "base/unguessable_token.h"
#include "third_party/blink/public/common/common_export.h"

#include "mojo/public/cpp/bindings/default_construct_tag.h"

namespace blink {

// Represents the information necessary to put a given Page in a specific
// browsing context group.
struct BLINK_COMMON_EXPORT BrowsingContextGroupInfo {
 public:
  // Create a BrowsingContextGroupInfo referencing a new, completely independent
  // browsing context group, in a new CoopRelatedGroup. This can be used in
  // tests or other places where the BrowsingContextGroupInfo is irrelevant,
  // and will not trigger COOP: restrict-properties Window restrictions, because
  // it does not affect cross-CoopRelatedGroup accesses.
  static BrowsingContextGroupInfo CreateUnique();

  // Create a BrowsingContextGroupInfo based on the passed in tokens.
  BrowsingContextGroupInfo(
      const base::UnguessableToken& browsing_context_group_token,
      const base::UnguessableToken& coop_related_group_token);

  // Mojo serialization constructor.
  explicit BrowsingContextGroupInfo(mojo::DefaultConstruct::Tag);

  // The token uniquely identifying the browsing context group of the page.
  base::UnguessableToken browsing_context_group_token;

  // The token uniquely identifying the CoopRelatedGroup of the page.
  base::UnguessableToken coop_related_group_token;
};

bool BLINK_COMMON_EXPORT operator==(const BrowsingContextGroupInfo& lhs,
                                    const BrowsingContextGroupInfo& rhs);
bool BLINK_COMMON_EXPORT operator!=(const BrowsingContextGroupInfo& lhs,
                                    const BrowsingContextGroupInfo& rhs);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_BROWSING_CONTEXT_GROUP_INFO_H_

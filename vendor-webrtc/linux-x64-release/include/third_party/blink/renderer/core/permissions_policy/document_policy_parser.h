// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_DOCUMENT_POLICY_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_DOCUMENT_POLICY_PARSER_H_

#include <optional>

#include "third_party/blink/public/common/permissions_policy/document_policy.h"
#include "third_party/blink/public/common/permissions_policy/document_policy_features.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/permissions_policy/policy_helper.h"

namespace blink {

class CORE_EXPORT DocumentPolicyParser {
  STATIC_ONLY(DocumentPolicyParser);

 public:
  // Parse document policy header and 'policy' attribute on iframe to
  // DocumentPolicy::FeatureState.
  static std::optional<DocumentPolicy::ParsedDocumentPolicy> Parse(
      const String& policy_string,
      PolicyParserMessageBuffer&);

  // Internal parsing method for testing.
  static std::optional<DocumentPolicy::ParsedDocumentPolicy> ParseInternal(
      const String& policy_string,
      const DocumentPolicyNameFeatureMap& name_feature_map,
      const DocumentPolicyFeatureInfoMap& feature_info_map,
      const DocumentPolicyFeatureSet& available_features,
      PolicyParserMessageBuffer&);
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_DOCUMENT_POLICY_PARSER_H_

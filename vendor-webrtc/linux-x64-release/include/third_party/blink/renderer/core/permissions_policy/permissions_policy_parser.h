// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_PERMISSIONS_POLICY_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_PERMISSIONS_POLICY_PARSER_H_

#include "base/memory/scoped_refptr.h"
#include "services/network/public/cpp/permissions_policy/origin_with_possible_wildcards.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/permissions_policy/policy_helper.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExecutionContext;

// Returns the list of features which are currently available in this context,
// including any features which have been made available by an origin trial.
CORE_EXPORT const Vector<String> GetAvailableFeatures(ExecutionContext*);

// PermissionsPolicyParser is a collection of methods which are used to convert
// Permissions Policy declarations, in headers and iframe attributes, into
// network::ParsedPermissionsPolicy structs. This class encapsulates all of the
// logic for parsing feature names, origin lists, and threshold values. Note
// that code outside of /renderer/ should not be parsing policy directives from
// strings, but if necessary, should be constructing
// network::ParsedPermissionsPolicy structs directly.
class CORE_EXPORT PermissionsPolicyParser {
  STATIC_ONLY(PermissionsPolicyParser);

 public:
  // Following is the intermediate representation(IR) of permissions policy.
  // Parsing of syntax structures is done in this IR, but semantic checks, e.g.
  // whether feature_name is valid, are not yet performed.
  struct Declaration {
    String feature_name;
    Vector<String> allowlist;
    String endpoint;
  };
  // We need to keep track of the source of the list of declarations as
  // different features (e.g., wildcards) might be active per-context.
  struct Node {
    network::OriginWithPossibleWildcards::NodeType type{
        network::OriginWithPossibleWildcards::NodeType::kUnknown};
    Vector<Declaration> declarations;
  };

  // Converts a header policy string into a vector of allowlists, one for each
  // feature specified. Unrecognized features are filtered out. The optional
  // ExecutionContext is used to determine if any origin trials affect the
  // parsing. Example of a permissions policy string:
  //     "vibrate a.com b.com; fullscreen 'none'; payment 'self', payment *".
  static network::ParsedPermissionsPolicy ParseHeader(
      const String& feature_policy_header,
      const String& permission_policy_header,
      scoped_refptr<const SecurityOrigin>,
      PolicyParserMessageBuffer& feature_policy_logger,
      PolicyParserMessageBuffer& permissions_policy_logger,
      ExecutionContext* = nullptr);

  // Converts a container policy string into a vector of allowlists, given self
  // and src origins provided, one for each feature specified. Unrecognized
  // features are filtered out. Example of a
  // permissions policy string:
  //     "vibrate a.com 'src'; fullscreen 'none'; payment 'self', payment *".
  static network::ParsedPermissionsPolicy ParseAttribute(
      const String& policy,
      scoped_refptr<const SecurityOrigin> self_origin,
      scoped_refptr<const SecurityOrigin> src_origin,
      PolicyParserMessageBuffer& logger,
      ExecutionContext* = nullptr);

  // Converts a PermissionsPolicy::Node into a network::ParsedPermissionsPolicy
  // Unrecognized features are filtered out.
  static network::ParsedPermissionsPolicy ParsePolicyFromNode(
      Node&,
      scoped_refptr<const SecurityOrigin>,
      PolicyParserMessageBuffer& logger,
      ExecutionContext* = nullptr);

  static network::ParsedPermissionsPolicy ParseFeaturePolicyForTest(
      const String& policy,
      scoped_refptr<const SecurityOrigin> self_origin,
      scoped_refptr<const SecurityOrigin> src_origin,
      PolicyParserMessageBuffer& logger,
      const FeatureNameMap& feature_names,
      ExecutionContext* = nullptr);

  static network::ParsedPermissionsPolicy ParsePermissionsPolicyForTest(
      const String& policy,
      scoped_refptr<const SecurityOrigin> self_origin,
      scoped_refptr<const SecurityOrigin> src_origin,
      PolicyParserMessageBuffer& logger,
      const FeatureNameMap& feature_names,
      ExecutionContext* = nullptr);
};

// Returns true iff any declaration in the policy is for the given feature.
CORE_EXPORT bool IsFeatureDeclared(network::mojom::PermissionsPolicyFeature,
                                   const network::ParsedPermissionsPolicy&);

// Removes any declaration in the policy for the given feature. Returns true if
// the policy was modified.
CORE_EXPORT bool RemoveFeatureIfPresent(
    network::mojom::PermissionsPolicyFeature,
    network::ParsedPermissionsPolicy&);

// If no declaration in the policy exists already for the feature, adds a
// declaration which disallows the feature in all origins. Returns true if the
// policy was modified.
CORE_EXPORT bool DisallowFeatureIfNotPresent(
    network::mojom::PermissionsPolicyFeature,
    network::ParsedPermissionsPolicy&);

// If no declaration in the policy exists already for the feature, adds a
// declaration which allows the feature in all origins. Returns true if the
// policy was modified.
CORE_EXPORT bool AllowFeatureEverywhereIfNotPresent(
    network::mojom::PermissionsPolicyFeature,
    network::ParsedPermissionsPolicy&);

// Replaces any existing declarations in the policy for the given feature with
// a declaration which disallows the feature in all origins.
CORE_EXPORT void DisallowFeature(network::mojom::PermissionsPolicyFeature,
                                 network::ParsedPermissionsPolicy&);

// Returns true iff the feature should not be exposed to script.
CORE_EXPORT bool IsFeatureForMeasurementOnly(
    network::mojom::PermissionsPolicyFeature);

// Replaces any existing declarations in the policy for the given feature with
// a declaration which allows the feature in all origins.
CORE_EXPORT void AllowFeatureEverywhere(
    network::mojom::PermissionsPolicyFeature,
    network::ParsedPermissionsPolicy&);

CORE_EXPORT const String
GetNameForFeature(network::mojom::PermissionsPolicyFeature,
                  bool is_isolated_context);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_PERMISSIONS_POLICY_PARSER_H_

// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_POLICY_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_POLICY_HELPER_H_

#include "base/memory/stack_allocated.h"
#include "services/network/public/mojom/permissions_policy/permissions_policy_feature.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink.h"
#include "third_party/blink/public/mojom/permissions_policy/document_policy_feature.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class PolicyParserMessageBuffer {
  STACK_ALLOCATED();

 public:
  struct Message {
    mojom::blink::ConsoleMessageLevel level;
    String content;

    Message(mojom::blink::ConsoleMessageLevel level, const String& content)
        : level(level), content(content) {}
  };

  PolicyParserMessageBuffer() : discard_message_(false) {}
  explicit PolicyParserMessageBuffer(const StringView& prefix,
                                     bool discard_message = false)
      : prefix_(prefix), discard_message_(discard_message) {}

  ~PolicyParserMessageBuffer() = default;

  void Warn(const String& message) {
    if (!discard_message_) {
      message_buffer_.emplace_back(mojom::blink::ConsoleMessageLevel::kWarning,
                                   prefix_ + message);
    }
  }

  void Error(const String& message) {
    if (!discard_message_) {
      message_buffer_.emplace_back(mojom::blink::ConsoleMessageLevel::kError,
                                   prefix_ + message);
    }
  }

  const Vector<Message>& GetMessages() { return message_buffer_; }

 private:
  const StringView prefix_;
  Vector<Message> message_buffer_;
  // If a dummy message buffer is desired, i.e. messages are not needed for
  // the caller, this flag can be set to true and the message buffer will
  // discard any incoming messages.
  const bool discard_message_;
};

struct FeatureNameMapCacheKey {
  bool IsIsolatedContext = false;
  bool is_deleted_value = false;
  bool is_empty_value = false;

  FeatureNameMapCacheKey() : is_empty_value(true) {}
  explicit FeatureNameMapCacheKey(const bool& IsIsolatedContext)
      : IsIsolatedContext(IsIsolatedContext) {}
  explicit FeatureNameMapCacheKey(WTF::HashTableDeletedValueType)
      : is_deleted_value(true) {}

  bool IsHashTableDeletedValue() const { return is_deleted_value; }

  bool operator==(const FeatureNameMapCacheKey& other) const {
    return IsIsolatedContext == other.IsIsolatedContext &&
           is_deleted_value == other.is_deleted_value &&
           is_empty_value == other.is_empty_value;
  }
  bool operator!=(const FeatureNameMapCacheKey& other) const {
    return !(*this == other);
  }
};
using FeatureNameMap =
    HashMap<String, network::mojom::PermissionsPolicyFeature>;
using FeatureNameMapCache = HashMap<FeatureNameMapCacheKey, FeatureNameMap>;

using DocumentPolicyFeatureSet = HashSet<mojom::blink::DocumentPolicyFeature>;

class ExecutionContext;
class FeatureContext;

// This method defines the feature names which will be recognized by the parser
// for the Permissions-Policy HTTP header and the <iframe> "allow" attribute, as
// well as the features which will be recognized by the document or iframe
// policy object.
CORE_EXPORT const FeatureNameMap
GetDefaultFeatureNameMap(bool is_isolated_context);

// This method defines the feature names which will be recognized by the parser
// for the Document-Policy HTTP header and the <iframe> "policy" attribute, as
// well as the features which will be recognized by the document or iframe
// policy object.
const DocumentPolicyFeatureSet& GetAvailableDocumentPolicyFeatures();

// Refresh the set content based on current RuntimeFeatures environment.
CORE_EXPORT void ResetAvailableDocumentPolicyFeaturesForTest();

// Returns if the given API has been configured as privacy sensitive. If
// sensitive, access to the feature may be denied in some circumstances.
bool IsPrivacySensitiveFeature(
    network::mojom::blink::PermissionsPolicyFeature feature);

// Returns true if this PermissionsPolicyFeature is currently disabled by an
// origin trial (it is origin trial controlled, and the origin trial is not
// enabled). The first String param should be a name of
// PermissionsPolicyFeature.
bool DisabledByOriginTrial(const String&, FeatureContext*);

// Returns true if this DocumentPolicyFeature is currently disabled by an origin
// trial (it is origin trial controlled, and the origin trial is not enabled).
bool DisabledByOriginTrial(mojom::blink::DocumentPolicyFeature,
                           FeatureContext*);

// Converts |network::mojom::PermissionsPolicyFeature| to enum used in devtools
// protocol.
String PermissionsPolicyFeatureToProtocol(
    network::mojom::PermissionsPolicyFeature,
    ExecutionContext*);

}  // namespace blink

namespace WTF {

// A helper that defines the hash function and the invalid 'empty value' that
// HashMap should use internally.
template <>
struct HashTraits<blink::FeatureNameMapCacheKey>
    : SimpleClassHashTraits<blink::FeatureNameMapCacheKey> {
  static unsigned GetHash(const blink::FeatureNameMapCacheKey& key) {
    unsigned hash = HashInt(key.IsIsolatedContext);
    AddIntToHash(hash, key.is_deleted_value);
    AddIntToHash(hash, key.is_empty_value);
    return hash;
  }
  static const bool kEmptyValueIsZero = false;
  static blink::FeatureNameMapCacheKey EmptyValue() {
    return blink::FeatureNameMapCacheKey();
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_POLICY_HELPER_H_

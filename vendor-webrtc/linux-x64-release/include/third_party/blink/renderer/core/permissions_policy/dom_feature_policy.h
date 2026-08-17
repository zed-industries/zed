// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_DOM_FEATURE_POLICY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_DOM_FEATURE_POLICY_H_

#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"

namespace blink {

class ScriptState;
class SecurityOrigin;

// DOMFeaturePolicy provides an interface for permissions policy introspection
// of a document (DocumentPolicy) or an iframe (IFramePolicy).
class CORE_EXPORT DOMFeaturePolicy : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit DOMFeaturePolicy(ExecutionContext*);
  ~DOMFeaturePolicy() override = default;

  // Implementation of methods of the policy interface:
  // Returns whether or not the given feature is allowed on the origin of the
  // context that owns the policy.
  bool allowsFeature(ScriptState* script_state, const String& feature) const;
  // Returns whether or not the given feature is allowed on the origin of the
  // given URL.
  bool allowsFeature(ScriptState* script_state,
                     const String& feature,
                     const String& url) const;
  // Returns a list of feature names that are supported by the user agent.
  Vector<String> features(ScriptState* script_state) const;
  // Returns a list of feature names that are allowed on the self origin.
  Vector<String> allowedFeatures(ScriptState* script_state) const;
  // Returns a list of feature name that are allowed on the origin of the given
  // URL.
  Vector<String> getAllowlistForFeature(ScriptState* script_state,
                                        const String& url) const;

  // Inform the DOMFeaturePolicy object when the container policy on its frame
  // element has changed.
  virtual void UpdateContainerPolicy(
      const network::ParsedPermissionsPolicy& container_policy = {},
      scoped_refptr<const SecurityOrigin> src_origin = nullptr) {}

  void Trace(Visitor*) const override;

 protected:
  virtual const network::PermissionsPolicy* GetPolicy() const {
    return context_->GetSecurityContext().GetPermissionsPolicy();
  }

  virtual bool IsIFramePolicy() const { return false; }

  Member<ExecutionContext> context_;

 private:
  // Add console message to the containing document.
  void AddWarningForUnrecognizedFeature(const String& message) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PERMISSIONS_POLICY_DOM_FEATURE_POLICY_H_

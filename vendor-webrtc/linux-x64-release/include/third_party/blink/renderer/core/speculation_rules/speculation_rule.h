// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULE_H_

#include "base/types/strong_alias.h"
#include "services/network/public/mojom/no_vary_search.mojom-blink-forward.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/speculation_rules/speculation_rules.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class DocumentRulePredicate;

// A single speculation rule which permits some set of URLs to be speculated,
// subject to some conditions.
//
// https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rule
class CORE_EXPORT SpeculationRule final
    : public GarbageCollected<SpeculationRule> {
 public:
  using RequiresAnonymousClientIPWhenCrossOrigin =
      base::StrongAlias<class RequiresAnonymousClientIPWhenCrossOriginTag,
                        bool>;

  SpeculationRule(
      Vector<KURL>,
      DocumentRulePredicate*,
      RequiresAnonymousClientIPWhenCrossOrigin,
      std::optional<mojom::blink::SpeculationTargetHint> target_hint,
      std::optional<network::mojom::ReferrerPolicy>,
      mojom::blink::SpeculationEagerness,
      network::mojom::blink::NoVarySearchPtr,
      mojom::blink::SpeculationInjectionType,
      WTF::String ruleset_tag,
      WTF::String rule_tag);
  ~SpeculationRule();

  const Vector<KURL>& urls() const { return urls_; }
  DocumentRulePredicate* predicate() const { return predicate_.Get(); }
  bool requires_anonymous_client_ip_when_cross_origin() const {
    return requires_anonymous_client_ip_.value();
  }
  std::optional<mojom::blink::SpeculationTargetHint>
  target_browsing_context_name_hint() const {
    return target_browsing_context_name_hint_;
  }
  std::optional<network::mojom::ReferrerPolicy> referrer_policy() const {
    return referrer_policy_;
  }
  mojom::blink::SpeculationEagerness eagerness() const { return eagerness_; }
  const network::mojom::blink::NoVarySearchPtr& no_vary_search_hint() const {
    return no_vary_search_hint_;
  }
  mojom::blink::SpeculationInjectionType injection_type() const {
    return injection_type_;
  }
  WTF::String ruleset_tag() const { return ruleset_tag_; }
  WTF::String rule_tag() const { return rule_tag_; }

  void Trace(Visitor*) const;

 private:
  const Vector<KURL> urls_;
  const Member<DocumentRulePredicate> predicate_;
  const RequiresAnonymousClientIPWhenCrossOrigin requires_anonymous_client_ip_;
  const std::optional<mojom::blink::SpeculationTargetHint>
      target_browsing_context_name_hint_;
  const std::optional<network::mojom::ReferrerPolicy> referrer_policy_;
  mojom::blink::SpeculationEagerness eagerness_;
  network::mojom::blink::NoVarySearchPtr no_vary_search_hint_;
  mojom::blink::SpeculationInjectionType injection_type_ =
      mojom::blink::SpeculationInjectionType::kNone;
  // TODO(crbug.com/381687257): make `ruleset_tag_` owned by
  // `SpeculationRuleSet`.
  const WTF::String ruleset_tag_;
  const WTF::String rule_tag_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULE_H_

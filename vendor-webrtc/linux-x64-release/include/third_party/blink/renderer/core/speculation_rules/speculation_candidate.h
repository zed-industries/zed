// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_CANDIDATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_CANDIDATE_H_

#include "third_party/blink/public/mojom/speculation_rules/speculation_rules.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"

namespace blink {

class HTMLAnchorElementBase;
class KURL;
struct Referrer;
class SpeculationRuleSet;

// See documentation for "SpeculationCandidate" in
// third_party/blink/public/mojom/speculation_rules/speculation_rules.mojom.
// Largely equivalent to the mojom type, but stores some extra fields that
// are used by DevTools.
class CORE_EXPORT SpeculationCandidate
    : public GarbageCollected<SpeculationCandidate> {
 public:
  SpeculationCandidate(const KURL& url,
                       mojom::blink::SpeculationAction action,
                       const Referrer& referrer,
                       bool requires_anonymous_client_ip_when_cross_origin,
                       mojom::blink::SpeculationTargetHint target_hint,
                       mojom::blink::SpeculationEagerness eagerness,
                       network::mojom::blink::NoVarySearchPtr no_vary_search,
                       mojom::blink::SpeculationInjectionType injection_type,
                       Vector<WTF::String> tags,
                       SpeculationRuleSet* rule_set,
                       HTMLAnchorElementBase* anchor);
  virtual ~SpeculationCandidate() = default;

  void Trace(Visitor* visitor) const;

  mojom::blink::SpeculationCandidatePtr ToMojom() const;

  const KURL& url() const { return url_; }
  mojom::blink::SpeculationAction action() const { return action_; }
  mojom::blink::SpeculationTargetHint target_hint() const {
    return target_hint_;
  }
  mojom::blink::SpeculationEagerness eagerness() const { return eagerness_; }
  SpeculationRuleSet* rule_set() const { return rule_set_.Get(); }
  // Only set for candidates derived from a document rule (is null for
  // candidates derived from list rules).
  HTMLAnchorElementBase* anchor() const { return anchor_.Get(); }
  const Vector<WTF::String>& tags() const { return tags_; }

  // Returns true if the two candidates are similar from the author's
  // perspective. This means that the two candidates are for the same URL and
  // have the same action, and the other properties are similar enough that
  // the author would consider them to be the same candidate, except for tags.
  bool IsSimilarFromAuthorPerspectiveExceptForTags(
      const SpeculationCandidate& other) const;

 private:
  const KURL url_;
  const mojom::blink::SpeculationAction action_;
  const Referrer referrer_;
  const bool requires_anonymous_client_ip_when_cross_origin_;
  const mojom::blink::SpeculationTargetHint target_hint_;
  const mojom::blink::SpeculationEagerness eagerness_;
  const network::mojom::blink::NoVarySearchPtr no_vary_search_;
  const mojom::blink::SpeculationInjectionType injection_type_;
  const Vector<WTF::String> tags_;
  const Member<SpeculationRuleSet> rule_set_;
  const Member<HTMLAnchorElementBase> anchor_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_CANDIDATE_H_

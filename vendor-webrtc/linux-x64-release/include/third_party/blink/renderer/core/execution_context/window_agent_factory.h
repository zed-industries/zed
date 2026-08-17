// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_WINDOW_AGENT_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_WINDOW_AGENT_FACTORY_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class AgentGroupScheduler;
class SecurityOrigin;
class WindowAgent;

// This is a helper class to assign WindowAgent to Document.
// The instance should be created for each group of Documents that are mutually
// reachable via `window.opener`, `window.frames` or others. These Documents
// may have different origins.
//
// The instance is intended to have the same granularity to the group of
// browsing context, that contains auxiliary browsing contexts.
// https://html.spec.whatwg.org/C#tlbc-group
// https://html.spec.whatwg.org/C#auxiliary-browsing-context
class WindowAgentFactory final : public GarbageCollected<WindowAgentFactory> {
 public:
  explicit WindowAgentFactory(AgentGroupScheduler& agent_group_scheduler);

  // Returns an instance of WindowAgent for |origin|.
  // This returns the same instance for origin A and origin B if either:
  //  - |has_potential_universal_access_privilege| is true,
  //  - both A and B have `file:` scheme,
  //  - or, they have the same scheme and the same registrable origin.
  // If |is_origin_agent_cluster| is true though, then the same instance will
  // only return the same instance for an exact match (scheme, host, port) to
  // |origin|.
  //
  // Set |has_potential_universal_access_privilege| if an agent may be able to
  // access all other agents synchronously.
  // I.e. pass true to if either:
  //   * --disable-web-security is set,
  //   * --run-web-tests is set,
  //   * or, the Blink instance is running for Android WebView.
  WindowAgent* GetAgentForOrigin(bool has_potential_universal_access_privilege,
                                 const SecurityOrigin* origin,
                                 bool is_origin_agent_cluster,
                                 bool origin_agent_cluster_left_as_default);

  void Trace(Visitor*) const;

 private:
  struct SchemeAndRegistrableDomain {
    String scheme;
    String registrable_domain;

    SchemeAndRegistrableDomain(const String& scheme,
                               const String& registrable_domain)
        : scheme(scheme), registrable_domain(registrable_domain) {}
  };

  struct SchemeAndRegistrableDomainTraits
      : TwoFieldsHashTraits<SchemeAndRegistrableDomain,
                            &SchemeAndRegistrableDomain::scheme,
                            &SchemeAndRegistrableDomain::registrable_domain> {};

  // Use a shared instance of Agent for all frames if a frame may have the
  // universal access privilege.
  WeakMember<WindowAgent> universal_access_agent_;

  // `file:` scheme URLs are hard for tracking the equality. Use a shared
  // Agent for them.
  WeakMember<WindowAgent> file_url_agent_;

  // Use the SecurityOrigin itself as the key for opaque origins.
  HeapHashMap<scoped_refptr<const SecurityOrigin>, WeakMember<WindowAgent>>
      opaque_origin_agents_;

  // Use the SecurityOrigin itself as the key for origin-keyed origins.
  // TODO(wjmaclean,domenic): In future when logical cross-origin-isolation
  // (COI) is implemented, we should unify it with logical-OAC so that all the
  // origin-keyed isolation relies on a single mechanism.
  HeapHashMap<scoped_refptr<const SecurityOrigin>, WeakMember<WindowAgent>>
      origin_keyed_agent_cluster_agents_;

  // Use registerable domain as the key for general tuple origins.
  using TupleOriginAgents = HeapHashMap<SchemeAndRegistrableDomain,
                                        WeakMember<WindowAgent>,
                                        SchemeAndRegistrableDomainTraits>;
  TupleOriginAgents tuple_origin_agents_;
  Member<AgentGroupScheduler> agent_group_scheduler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_WINDOW_AGENT_FACTORY_H_

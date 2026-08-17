// Copyright (c) 2022, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/x509.h>

#include <assert.h>

#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/stack.h>

#include "../internal.h"
#include "internal.h"


// This file computes the X.509 policy tree, as described in RFC 5280, section
// 6.1. It differs in that:
//
//  (1) It does not track "qualifier_set". This is not needed as it is not
//      output by this implementation.
//
//  (2) It builds a directed acyclic graph, rather than a tree. When a given
//      policy matches multiple parents, RFC 5280 makes a separate node for
//      each parent. This representation condenses them into one node with
//      multiple parents. Thus we refer to this structure as a "policy graph",
//      rather than a "policy tree".
//
//  (3) "expected_policy_set" is not tracked explicitly and built temporarily
//      as part of building the graph.
//
//  (4) anyPolicy nodes are not tracked explicitly.
//
//  (5) Some pruning steps are deferred to when policies are evaluated, as a
//      reachability pass.

// An X509_POLICY_NODE is a node in the policy graph. It corresponds to a node
// from RFC 5280, section 6.1.2, step (a), but we store some fields differently.
typedef struct x509_policy_node_st {
  // policy is the "valid_policy" field from RFC 5280.
  ASN1_OBJECT *policy;

  // parent_policies, if non-empty, is the list of "valid_policy" values for all
  // nodes which are a parent of this node. In this case, no entry in this list
  // will be anyPolicy. This list is in no particular order and may contain
  // duplicates if the corresponding certificate had duplicate mappings.
  //
  // If empty, this node has a single parent, anyPolicy. The node is then a root
  // policies, and is in authorities-constrained-policy-set if it has a path to
  // a leaf node.
  //
  // Note it is not possible for a policy to have both anyPolicy and a
  // concrete policy as a parent. Section 6.1.3, step (d.1.ii) only runs if
  // there was no match in step (d.1.i). We do not need to represent a parent
  // list of, say, {anyPolicy, OID1, OID2}.
  STACK_OF(ASN1_OBJECT) *parent_policies;

  // mapped is one if this node matches a policy mapping in the certificate and
  // zero otherwise.
  int mapped;

  // reachable is one if this node is reachable from some valid policy in the
  // end-entity certificate. It is computed during |has_explicit_policy|.
  int reachable;
} X509_POLICY_NODE;

DEFINE_STACK_OF(X509_POLICY_NODE)

// An X509_POLICY_LEVEL is the collection of nodes at the same depth in the
// policy graph. This structure can also be used to represent a level's
// "expected_policy_set" values. See |process_policy_mappings|.
typedef struct x509_policy_level_st {
  // nodes is the list of nodes at this depth, except for the anyPolicy node, if
  // any. This list is sorted by policy OID for efficient lookup.
  STACK_OF(X509_POLICY_NODE) *nodes;

  // has_any_policy is one if there is an anyPolicy node at this depth, and zero
  // otherwise.
  int has_any_policy;
} X509_POLICY_LEVEL;

DEFINE_STACK_OF(X509_POLICY_LEVEL)

static int is_any_policy(const ASN1_OBJECT *obj) {
  return OBJ_obj2nid(obj) == NID_any_policy;
}

static void x509_policy_node_free(X509_POLICY_NODE *node) {
  if (node != NULL) {
    ASN1_OBJECT_free(node->policy);
    sk_ASN1_OBJECT_pop_free(node->parent_policies, ASN1_OBJECT_free);
    OPENSSL_free(node);
  }
}

static X509_POLICY_NODE *x509_policy_node_new(const ASN1_OBJECT *policy) {
  assert(!is_any_policy(policy));
  X509_POLICY_NODE *node = OPENSSL_zalloc(sizeof(X509_POLICY_NODE));
  if (node == NULL) {
    return NULL;
  }
  node->policy = OBJ_dup(policy);
  node->parent_policies = sk_ASN1_OBJECT_new_null();
  if (node->policy == NULL || node->parent_policies == NULL) {
    x509_policy_node_free(node);
    return NULL;
  }
  return node;
}

static int x509_policy_node_cmp(const X509_POLICY_NODE *const *a,
                                const X509_POLICY_NODE *const *b) {
  return OBJ_cmp((*a)->policy, (*b)->policy);
}

static void x509_policy_level_free(X509_POLICY_LEVEL *level) {
  if (level != NULL) {
    sk_X509_POLICY_NODE_pop_free(level->nodes, x509_policy_node_free);
    OPENSSL_free(level);
  }
}

static X509_POLICY_LEVEL *x509_policy_level_new(void) {
  X509_POLICY_LEVEL *level = OPENSSL_zalloc(sizeof(X509_POLICY_LEVEL));
  if (level == NULL) {
    return NULL;
  }
  level->nodes = sk_X509_POLICY_NODE_new(x509_policy_node_cmp);
  if (level->nodes == NULL) {
    x509_policy_level_free(level);
    return NULL;
  }
  return level;
}

static int x509_policy_level_is_empty(const X509_POLICY_LEVEL *level) {
  return !level->has_any_policy && sk_X509_POLICY_NODE_num(level->nodes) == 0;
}

static void x509_policy_level_clear(X509_POLICY_LEVEL *level) {
  level->has_any_policy = 0;
  for (size_t i = 0; i < sk_X509_POLICY_NODE_num(level->nodes); i++) {
    x509_policy_node_free(sk_X509_POLICY_NODE_value(level->nodes, i));
  }
  sk_X509_POLICY_NODE_zero(level->nodes);
}

// x509_policy_level_find returns the node in |level| corresponding to |policy|,
// or NULL if none exists.
static X509_POLICY_NODE *x509_policy_level_find(X509_POLICY_LEVEL *level,
                                                const ASN1_OBJECT *policy) {
  assert(sk_X509_POLICY_NODE_is_sorted(level->nodes));
  X509_POLICY_NODE node;
  node.policy = (ASN1_OBJECT *)policy;
  size_t idx;
  if (!sk_X509_POLICY_NODE_find_awslc(level->nodes, &idx, &node)) {
    return NULL;
  }
  return sk_X509_POLICY_NODE_value(level->nodes, idx);
}

// x509_policy_level_add_nodes adds the nodes in |nodes| to |level|. It returns
// one on success and zero on error. No policy in |nodes| may already be present
// in |level|. This function modifies |nodes| to avoid making a copy, but the
// caller is still responsible for releasing |nodes| itself.
//
// This function is used to add nodes to |level| in bulk, and avoid resorting
// |level| after each addition.
static int x509_policy_level_add_nodes(X509_POLICY_LEVEL *level,
                                       STACK_OF(X509_POLICY_NODE) *nodes) {
  for (size_t i = 0; i < sk_X509_POLICY_NODE_num(nodes); i++) {
    X509_POLICY_NODE *node = sk_X509_POLICY_NODE_value(nodes, i);
    if (!sk_X509_POLICY_NODE_push(level->nodes, node)) {
      return 0;
    }
    sk_X509_POLICY_NODE_set(nodes, i, NULL);
  }
  sk_X509_POLICY_NODE_sort(level->nodes);

#if !defined(NDEBUG)
  // There should be no duplicate nodes.
  for (size_t i = 1; i < sk_X509_POLICY_NODE_num(level->nodes); i++) {
    assert(OBJ_cmp(sk_X509_POLICY_NODE_value(level->nodes, i - 1)->policy,
                   sk_X509_POLICY_NODE_value(level->nodes, i)->policy) != 0);
  }
#endif
  return 1;
}

static int policyinfo_cmp(const POLICYINFO *const *a,
                          const POLICYINFO *const *b) {
  return OBJ_cmp((*a)->policyid, (*b)->policyid);
}

static int delete_if_not_in_policies(X509_POLICY_NODE *node, void *data) {
  const CERTIFICATEPOLICIES *policies = data;
  assert(sk_POLICYINFO_is_sorted(policies));
  POLICYINFO info;
  info.policyid = node->policy;
  if (sk_POLICYINFO_find_awslc(policies, NULL, &info)) {
    return 0;
  }
  x509_policy_node_free(node);
  return 1;
}

// process_certificate_policies updates |level| to incorporate |x509|'s
// certificate policies extension. This implements steps (d) and (e) of RFC
// 5280, section 6.1.3. |level| must contain the previous level's
// "expected_policy_set" information. For all but the top-most level, this is
// the output of |process_policy_mappings|. |any_policy_allowed| specifies
// whether anyPolicy is allowed or inhibited, taking into account the exception
// for self-issued certificates.
static int process_certificate_policies(const X509 *x509,
                                        X509_POLICY_LEVEL *level,
                                        int any_policy_allowed) {
  int ret = 0;
  int critical;
  STACK_OF(X509_POLICY_NODE) *new_nodes = NULL;
  CERTIFICATEPOLICIES *policies =
      X509_get_ext_d2i(x509, NID_certificate_policies, &critical, NULL);
  if (policies == NULL) {
    if (critical != -1) {
      return 0;  // Syntax error in the extension.
    }

    // RFC 5280, section 6.1.3, step (e).
    x509_policy_level_clear(level);
    return 1;
  }

  // certificatePolicies may not be empty. See RFC 5280, section 4.2.1.4.
  // TODO(https://crbug.com/boringssl/443): Move this check into the parser.
  if (sk_POLICYINFO_num(policies) == 0) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_POLICY_EXTENSION);
    goto err;
  }

  sk_POLICYINFO_set_cmp_func(policies, policyinfo_cmp);
  sk_POLICYINFO_sort(policies);
  int cert_has_any_policy = 0;
  for (size_t i = 0; i < sk_POLICYINFO_num(policies); i++) {
    const POLICYINFO *policy = sk_POLICYINFO_value(policies, i);
    if (is_any_policy(policy->policyid)) {
      cert_has_any_policy = 1;
    }
    if (i > 0 && OBJ_cmp(sk_POLICYINFO_value(policies, i - 1)->policyid,
                         policy->policyid) == 0) {
      // Per RFC 5280, section 4.2.1.4, |policies| may not have duplicates.
      OPENSSL_PUT_ERROR(X509, X509_R_INVALID_POLICY_EXTENSION);
      goto err;
    }
  }

  // This does the same thing as RFC 5280, section 6.1.3, step (d), though in
  // a slighty different order. |level| currently contains "expected_policy_set"
  // values of the previous level. See |process_policy_mappings| for details.
  const int previous_level_has_any_policy = level->has_any_policy;

  // First, we handle steps (d.1.i) and (d.2). The net effect of these two steps
  // is to intersect |level| with |policies|, ignoring anyPolicy if it is
  // inhibited.
  if (!cert_has_any_policy || !any_policy_allowed) {
    sk_X509_POLICY_NODE_delete_if(level->nodes, delete_if_not_in_policies,
                                  policies);
    level->has_any_policy = 0;
  }

  // Step (d.1.ii) may attach new nodes to the previous level's anyPolicy node.
  if (previous_level_has_any_policy) {
    new_nodes = sk_X509_POLICY_NODE_new_null();
    if (new_nodes == NULL) {
      goto err;
    }
    for (size_t i = 0; i < sk_POLICYINFO_num(policies); i++) {
      const POLICYINFO *policy = sk_POLICYINFO_value(policies, i);
      // Though we've reordered the steps slightly, |policy| is in |level| if
      // and only if it would have been a match in step (d.1.ii).
      if (!is_any_policy(policy->policyid) &&
          x509_policy_level_find(level, policy->policyid) == NULL) {
        X509_POLICY_NODE *node = x509_policy_node_new(policy->policyid);
        if (node == NULL ||  //
            !sk_X509_POLICY_NODE_push(new_nodes, node)) {
          x509_policy_node_free(node);
          goto err;
        }
      }
    }
    if (!x509_policy_level_add_nodes(level, new_nodes)) {
      goto err;
    }
  }

  ret = 1;

err:
  sk_X509_POLICY_NODE_pop_free(new_nodes, x509_policy_node_free);
  CERTIFICATEPOLICIES_free(policies);
  return ret;
}

static int compare_issuer_policy(const POLICY_MAPPING *const *a,
                                 const POLICY_MAPPING *const *b) {
  return OBJ_cmp((*a)->issuerDomainPolicy, (*b)->issuerDomainPolicy);
}

static int compare_subject_policy(const POLICY_MAPPING *const *a,
                                  const POLICY_MAPPING *const *b) {
  return OBJ_cmp((*a)->subjectDomainPolicy, (*b)->subjectDomainPolicy);
}

static int delete_if_mapped(X509_POLICY_NODE *node, void *data) {
  const POLICY_MAPPINGS *mappings = data;
  // |mappings| must have been sorted by |compare_issuer_policy|.
  assert(sk_POLICY_MAPPING_is_sorted(mappings));
  POLICY_MAPPING mapping;
  mapping.issuerDomainPolicy = node->policy;
  if (!sk_POLICY_MAPPING_find_awslc(mappings, NULL, &mapping)) {
    return 0;
  }
  x509_policy_node_free(node);
  return 1;
}

// process_policy_mappings processes the policy mappings extension of |cert|,
// whose corresponding graph level is |level|. |mapping_allowed| specifies
// whether policy mapping is inhibited at this point. On success, it returns an
// |X509_POLICY_LEVEL| containing the "expected_policy_set" for |level|. On
// error, it returns NULL. This implements steps (a) and (b) of RFC 5280,
// section 6.1.4.
//
// We represent the "expected_policy_set" as an |X509_POLICY_LEVEL|.
// |has_any_policy| indicates whether there is an anyPolicy node with
// "expected_policy_set" of {anyPolicy}. If a node with policy oid P1 contains
// P2 in its "expected_policy_set", the level will contain a node of policy P2
// with P1 in |parent_policies|.
//
// This is equivalent to the |X509_POLICY_LEVEL| that would result if the next
// certificats contained anyPolicy. |process_certificate_policies| will filter
// this result down to compute the actual level.
static X509_POLICY_LEVEL *process_policy_mappings(const X509 *cert,
                                                  X509_POLICY_LEVEL *level,
                                                  int mapping_allowed) {
  int ok = 0;
  STACK_OF(X509_POLICY_NODE) *new_nodes = NULL;
  X509_POLICY_LEVEL *next = NULL;
  int critical;
  POLICY_MAPPINGS *mappings =
      X509_get_ext_d2i(cert, NID_policy_mappings, &critical, NULL);
  if (mappings == NULL && critical != -1) {
    // Syntax error in the policy mappings extension.
    goto err;
  }

  if (mappings != NULL) {
    // PolicyMappings may not be empty. See RFC 5280, section 4.2.1.5.
    // TODO(https://crbug.com/boringssl/443): Move this check into the parser.
    if (sk_POLICY_MAPPING_num(mappings) == 0) {
      OPENSSL_PUT_ERROR(X509, X509_R_INVALID_POLICY_EXTENSION);
      goto err;
    }

    // RFC 5280, section 6.1.4, step (a).
    for (size_t i = 0; i < sk_POLICY_MAPPING_num(mappings); i++) {
      POLICY_MAPPING *mapping = sk_POLICY_MAPPING_value(mappings, i);
      if (is_any_policy(mapping->issuerDomainPolicy) ||
          is_any_policy(mapping->subjectDomainPolicy)) {
        goto err;
      }
    }

    // Sort to group by issuerDomainPolicy.
    sk_POLICY_MAPPING_set_cmp_func(mappings, compare_issuer_policy);
    sk_POLICY_MAPPING_sort(mappings);

    if (mapping_allowed) {
      // Mark nodes as mapped, and add any nodes to |level| which may be needed
      // as part of RFC 5280, section 6.1.4, step (b.1).
      new_nodes = sk_X509_POLICY_NODE_new_null();
      if (new_nodes == NULL) {
        goto err;
      }
      const ASN1_OBJECT *last_policy = NULL;
      for (size_t i = 0; i < sk_POLICY_MAPPING_num(mappings); i++) {
        const POLICY_MAPPING *mapping = sk_POLICY_MAPPING_value(mappings, i);
        // There may be multiple mappings with the same |issuerDomainPolicy|.
        if (last_policy != NULL &&
            OBJ_cmp(mapping->issuerDomainPolicy, last_policy) == 0) {
          continue;
        }
        last_policy = mapping->issuerDomainPolicy;

        X509_POLICY_NODE *node =
            x509_policy_level_find(level, mapping->issuerDomainPolicy);
        if (node == NULL) {
          if (!level->has_any_policy) {
            continue;
          }
          node = x509_policy_node_new(mapping->issuerDomainPolicy);
          if (node == NULL ||  //
              !sk_X509_POLICY_NODE_push(new_nodes, node)) {
            x509_policy_node_free(node);
            goto err;
          }
        }
        node->mapped = 1;
      }
      if (!x509_policy_level_add_nodes(level, new_nodes)) {
        goto err;
      }
    } else {
      // RFC 5280, section 6.1.4, step (b.2). If mapping is inhibited, delete
      // all mapped nodes.
      sk_X509_POLICY_NODE_delete_if(level->nodes, delete_if_mapped, mappings);
      sk_POLICY_MAPPING_pop_free(mappings, POLICY_MAPPING_free);
      mappings = NULL;
    }
  }

  // If a node was not mapped, it retains the original "explicit_policy_set"
  // value, itself. Add those to |mappings|.
  if (mappings == NULL) {
    mappings = sk_POLICY_MAPPING_new_null();
    if (mappings == NULL) {
      goto err;
    }
  }
  for (size_t i = 0; i < sk_X509_POLICY_NODE_num(level->nodes); i++) {
    X509_POLICY_NODE *node = sk_X509_POLICY_NODE_value(level->nodes, i);
    if (!node->mapped) {
      POLICY_MAPPING *mapping = POLICY_MAPPING_new();
      if (mapping == NULL) {
        goto err;
      }
      mapping->issuerDomainPolicy = OBJ_dup(node->policy);
      mapping->subjectDomainPolicy = OBJ_dup(node->policy);
      if (mapping->issuerDomainPolicy == NULL ||
          mapping->subjectDomainPolicy == NULL ||
          !sk_POLICY_MAPPING_push(mappings, mapping)) {
        POLICY_MAPPING_free(mapping);
        goto err;
      }
    }
  }

  // Sort to group by subjectDomainPolicy.
  sk_POLICY_MAPPING_set_cmp_func(mappings, compare_subject_policy);
  sk_POLICY_MAPPING_sort(mappings);

  // Convert |mappings| to our "expected_policy_set" representation.
  next = x509_policy_level_new();
  if (next == NULL) {
    goto err;
  }
  next->has_any_policy = level->has_any_policy;

  X509_POLICY_NODE *last_node = NULL;
  for (size_t i = 0; i < sk_POLICY_MAPPING_num(mappings); i++) {
    POLICY_MAPPING *mapping = sk_POLICY_MAPPING_value(mappings, i);
    // Skip mappings where |issuerDomainPolicy| does not appear in the graph.
    if (!level->has_any_policy &&
        x509_policy_level_find(level, mapping->issuerDomainPolicy) == NULL) {
      continue;
    }

    if (last_node == NULL ||
        OBJ_cmp(last_node->policy, mapping->subjectDomainPolicy) != 0) {
      last_node = x509_policy_node_new(mapping->subjectDomainPolicy);
      if (last_node == NULL ||
          !sk_X509_POLICY_NODE_push(next->nodes, last_node)) {
        x509_policy_node_free(last_node);
        goto err;
      }
    }

    if (!sk_ASN1_OBJECT_push(last_node->parent_policies,
                             mapping->issuerDomainPolicy)) {
      goto err;
    }
    mapping->issuerDomainPolicy = NULL;
  }

  sk_X509_POLICY_NODE_sort(next->nodes);
  ok = 1;

err:
  if (!ok) {
    x509_policy_level_free(next);
    next = NULL;
  }

  sk_POLICY_MAPPING_pop_free(mappings, POLICY_MAPPING_free);
  sk_X509_POLICY_NODE_pop_free(new_nodes, x509_policy_node_free);
  return next;
}

// apply_skip_certs, if |skip_certs| is non-NULL, sets |*value| to the minimum
// of its current value and |skip_certs|. It returns one on success and zero if
// |skip_certs| is negative.
static int apply_skip_certs(const ASN1_INTEGER *skip_certs, size_t *value) {
  if (skip_certs == NULL) {
    return 1;
  }

  // TODO(https://crbug.com/boringssl/443): Move this check into the parser.
  if (skip_certs->type & V_ASN1_NEG) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_POLICY_EXTENSION);
    return 0;
  }

  // If |skip_certs| does not fit in |uint64_t|, it must exceed |*value|.
  uint64_t u64;
  if (ASN1_INTEGER_get_uint64(&u64, skip_certs) && u64 < *value) {
    *value = (size_t)u64;
  }
  ERR_clear_error();
  return 1;
}

// process_policy_constraints updates |*explicit_policy|, |*policy_mapping|, and
// |*inhibit_any_policy| according to |x509|'s policy constraints and inhibit
// anyPolicy extensions. It returns one on success and zero on error. This
// implements steps (i) and (j) of RFC 5280, section 6.1.4.
static int process_policy_constraints(const X509 *x509, size_t *explicit_policy,
                                      size_t *policy_mapping,
                                      size_t *inhibit_any_policy) {
  int critical;
  POLICY_CONSTRAINTS *constraints =
      X509_get_ext_d2i(x509, NID_policy_constraints, &critical, NULL);
  if (constraints == NULL && critical != -1) {
    return 0;
  }
  if (constraints != NULL) {
    if (constraints->requireExplicitPolicy == NULL &&
        constraints->inhibitPolicyMapping == NULL) {
      // Per RFC 5280, section 4.2.1.11, at least one of the fields must be
      // present.
      OPENSSL_PUT_ERROR(X509, X509_R_INVALID_POLICY_EXTENSION);
      POLICY_CONSTRAINTS_free(constraints);
      return 0;
    }
    int ok =
        apply_skip_certs(constraints->requireExplicitPolicy, explicit_policy) &&
        apply_skip_certs(constraints->inhibitPolicyMapping, policy_mapping);
    POLICY_CONSTRAINTS_free(constraints);
    if (!ok) {
      return 0;
    }
  }

  ASN1_INTEGER *inhibit_any_policy_ext =
      X509_get_ext_d2i(x509, NID_inhibit_any_policy, &critical, NULL);
  if (inhibit_any_policy_ext == NULL && critical != -1) {
    return 0;
  }
  int ok = apply_skip_certs(inhibit_any_policy_ext, inhibit_any_policy);
  ASN1_INTEGER_free(inhibit_any_policy_ext);
  return ok;
}

// has_explicit_policy returns one if the set of authority-space policy OIDs
// |levels| has some non-empty intersection with |user_policies|, and zero
// otherwise. This mirrors the logic in RFC 5280, section 6.1.5, step (g). This
// function modifies |levels| and should only be called at the end of policy
// evaluation.
static int has_explicit_policy(STACK_OF(X509_POLICY_LEVEL) *levels,
                               const STACK_OF(ASN1_OBJECT) *user_policies) {
  assert(sk_ASN1_OBJECT_is_sorted(user_policies));

  // Step (g.i). If the policy graph is empty, the intersection is empty.
  size_t num_levels = sk_X509_POLICY_LEVEL_num(levels);
  X509_POLICY_LEVEL *level = sk_X509_POLICY_LEVEL_value(levels, num_levels - 1);
  if (x509_policy_level_is_empty(level)) {
    return 0;
  }

  // If |user_policies| is empty, we interpret it as having a single anyPolicy
  // value. The caller may also have supplied anyPolicy explicitly.
  int user_has_any_policy = sk_ASN1_OBJECT_num(user_policies) == 0;
  for (size_t i = 0; i < sk_ASN1_OBJECT_num(user_policies); i++) {
    if (is_any_policy(sk_ASN1_OBJECT_value(user_policies, i))) {
      user_has_any_policy = 1;
      break;
    }
  }

  // Step (g.ii). If the policy graph is not empty and the user set contains
  // anyPolicy, the intersection is the entire (non-empty) graph.
  if (user_has_any_policy) {
    return 1;
  }

  // Step (g.iii) does not delete anyPolicy nodes, so if the graph has
  // anyPolicy, some explicit policy will survive. The actual intersection may
  // synthesize some nodes in step (g.iii.3), but we do not return the policy
  // list itself, so we skip actually computing this.
  if (level->has_any_policy) {
    return 1;
  }

  // We defer pruning the tree, so as we look for nodes with parent anyPolicy,
  // step (g.iii.1), we must limit to nodes reachable from the bottommost level.
  // Start by marking each of those nodes as reachable.
  for (size_t i = 0; i < sk_X509_POLICY_NODE_num(level->nodes); i++) {
    sk_X509_POLICY_NODE_value(level->nodes, i)->reachable = 1;
  }

  for (size_t i = num_levels - 1; i < num_levels; i--) {
    level = sk_X509_POLICY_LEVEL_value(levels, i);
    for (size_t j = 0; j < sk_X509_POLICY_NODE_num(level->nodes); j++) {
      X509_POLICY_NODE *node = sk_X509_POLICY_NODE_value(level->nodes, j);
      if (!node->reachable) {
        continue;
      }
      if (sk_ASN1_OBJECT_num(node->parent_policies) == 0) {
        // |node|'s parent is anyPolicy and is part of "valid_policy_node_set".
        // If it exists in |user_policies|, the intersection is non-empty and we
        // can return immediately.
        if (sk_ASN1_OBJECT_find_awslc(user_policies, NULL, node->policy)) {
          return 1;
        }
      } else if (i > 0) {
        // |node|'s parents are concrete policies. Mark the parents reachable,
        // to be inspected by the next loop iteration.
        X509_POLICY_LEVEL *prev = sk_X509_POLICY_LEVEL_value(levels, i - 1);
        for (size_t k = 0; k < sk_ASN1_OBJECT_num(node->parent_policies); k++) {
          X509_POLICY_NODE *parent = x509_policy_level_find(
              prev, sk_ASN1_OBJECT_value(node->parent_policies, k));
          if (parent != NULL) {
            parent->reachable = 1;
          }
        }
      }
    }
  }

  return 0;
}

static int asn1_object_cmp(const ASN1_OBJECT *const *a,
                           const ASN1_OBJECT *const *b) {
  return OBJ_cmp(*a, *b);
}

int X509_policy_check(const STACK_OF(X509) *certs,
                      const STACK_OF(ASN1_OBJECT) *user_policies,
                      unsigned long flags, X509 **out_current_cert) {
  *out_current_cert = NULL;
  int ret = X509_V_ERR_OUT_OF_MEM;
  X509_POLICY_LEVEL *level = NULL;
  STACK_OF(X509_POLICY_LEVEL) *levels = NULL;
  STACK_OF(ASN1_OBJECT) *user_policies_sorted = NULL;
  size_t num_certs = sk_X509_num(certs);

  // Skip policy checking if the chain is just the trust anchor.
  if (num_certs <= 1) {
    return X509_V_OK;
  }

  // See RFC 5280, section 6.1.2, steps (d) through (f).
  size_t explicit_policy =
      (flags & X509_V_FLAG_EXPLICIT_POLICY) ? 0 : num_certs + 1;
  size_t inhibit_any_policy =
      (flags & X509_V_FLAG_INHIBIT_ANY) ? 0 : num_certs + 1;
  size_t policy_mapping =
      (flags & X509_V_FLAG_INHIBIT_MAP) ? 0 : num_certs + 1;

  levels = sk_X509_POLICY_LEVEL_new_null();
  if (levels == NULL) {
    goto err;
  }

  for (size_t i = num_certs - 2; i < num_certs; i--) {
    X509 *cert = sk_X509_value(certs, i);
    if (!x509v3_cache_extensions(cert)) {
      goto err;
    }
    const int is_self_issued = (cert->ex_flags & EXFLAG_SI) != 0;

    if (level == NULL) {
      assert(i == num_certs - 2);
      level = x509_policy_level_new();
      if (level == NULL) {
        goto err;
      }
      level->has_any_policy = 1;
    }

    // RFC 5280, section 6.1.3, steps (d) and (e). |any_policy_allowed| is
    // computed as in step (d.2).
    const int any_policy_allowed =
        inhibit_any_policy > 0 || (i > 0 && is_self_issued);
    if (!process_certificate_policies(cert, level, any_policy_allowed)) {
      ret = X509_V_ERR_INVALID_POLICY_EXTENSION;
      *out_current_cert = cert;
      goto err;
    }

    // RFC 5280, section 6.1.3, step (f).
    if (explicit_policy == 0 && x509_policy_level_is_empty(level)) {
      ret = X509_V_ERR_NO_EXPLICIT_POLICY;
      goto err;
    }

    // Insert into the list.
    if (!sk_X509_POLICY_LEVEL_push(levels, level)) {
      goto err;
    }
    X509_POLICY_LEVEL *current_level = level;
    level = NULL;

    // If this is not the leaf certificate, we go to section 6.1.4. If it
    // is the leaf certificate, we go to section 6.1.5 instead.
    if (i != 0) {
      // RFC 5280, section 6.1.4, steps (a) and (b).
      level = process_policy_mappings(cert, current_level, policy_mapping > 0);
      if (level == NULL) {
        ret = X509_V_ERR_INVALID_POLICY_EXTENSION;
        *out_current_cert = cert;
        goto err;
      }
    }

    // RFC 5280, section 6.1.4, step (h-j) for non-leaves, and section 6.1.5,
    // step (a-b) for leaves. In the leaf case, RFC 5280 says only to update
    // |explicit_policy|, but |policy_mapping| and |inhibit_any_policy| are no
    // longer read at this point, so we use the same process.
    if (i == 0 || !is_self_issued) {
      if (explicit_policy > 0) {
        explicit_policy--;
      }
      if (policy_mapping > 0) {
        policy_mapping--;
      }
      if (inhibit_any_policy > 0) {
        inhibit_any_policy--;
      }
    }
    if (!process_policy_constraints(cert, &explicit_policy, &policy_mapping,
                                    &inhibit_any_policy)) {
      ret = X509_V_ERR_INVALID_POLICY_EXTENSION;
      *out_current_cert = cert;
      goto err;
    }
  }

  // RFC 5280, section 6.1.5, step (g). We do not output the policy set, so it
  // is only necessary to check if the user-constrained-policy-set is not empty.
  if (explicit_policy == 0) {
    // Build a sorted copy of |user_policies| for more efficient lookup.
    user_policies_sorted = sk_ASN1_OBJECT_dup(user_policies);
    if (user_policies_sorted == NULL) {
      goto err;
    }
    sk_ASN1_OBJECT_set_cmp_func(user_policies_sorted, asn1_object_cmp);
    sk_ASN1_OBJECT_sort(user_policies_sorted);

    if (!has_explicit_policy(levels, user_policies_sorted)) {
      ret = X509_V_ERR_NO_EXPLICIT_POLICY;
      goto err;
    }
  }

  ret = X509_V_OK;

err:
  x509_policy_level_free(level);
  // |user_policies_sorted|'s contents are owned by |user_policies|, so we do
  // not use |sk_ASN1_OBJECT_pop_free|.
  sk_ASN1_OBJECT_free(user_policies_sorted);
  sk_X509_POLICY_LEVEL_pop_free(levels, x509_policy_level_free);
  return ret;
}

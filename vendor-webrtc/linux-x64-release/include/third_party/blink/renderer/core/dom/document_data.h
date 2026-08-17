// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_DATA_H_

#include "services/network/public/mojom/trust_tokens.mojom-blink.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/renderer/platform/bindings/script_regexp.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

// The purpose of blink::DocumentData is to reduce the size of document.h.
// Data members which require huge headers should be stored in
// blink::DocumentData instead of blink::Document.
//
// Ownership: A Document has a strong reference to a single DocumentData.
//   Other instances should not have strong references to the DocumentData.
// Lifetime: A DocumentData instance is created on a Document creation, and
//   is never destructed before the Document.
class DocumentData final : public GarbageCollected<DocumentData> {
 public:
  explicit DocumentData(ExecutionContext* context)
      : permission_service_(context), trust_token_query_answerer_(context) {}

  void Trace(Visitor* visitor) const {
    visitor->Trace(permission_service_);
    visitor->Trace(trust_token_query_answerer_);
    visitor->Trace(pending_trust_token_query_resolvers_);
    visitor->Trace(email_regexp_);
  }

 private:
  // Mojo remote used to determine if the document has permission to access
  // storage or not.
  HeapMojoRemote<mojom::blink::PermissionService> permission_service_;

  // Mojo remote used to answer API calls asking whether the user has trust
  // tokens (https://github.com/wicg/trust-token-api). The other endpoint
  // is in the network service, which may crash and restart. To handle this:
  //   1. |pending_trust_token_query_resolvers_| keeps track of promises
  // depending on |trust_token_query_answerer_|'s answers;
  //   2. |TrustTokenQueryAnswererConnectionError| handles connection errors by
  // rejecting all pending promises and clearing the pending set.
  HeapMojoRemote<network::mojom::blink::TrustTokenQueryAnswerer>
      trust_token_query_answerer_;

  // In order to be able to answer promises when the Mojo remote disconnects,
  // maintain all pending promises here, deleting them on successful completion
  // or on connection error, whichever comes first.
  HeapHashSet<Member<ScriptPromiseResolverBase>>
      pending_trust_token_query_resolvers_;

  // To do email regex checks.
  Member<ScriptRegexp> email_regexp_;

  // The number of immediate child frames created within this document so far.
  // This count doesn't include this document's frame nor descendant frames.
  int immediate_child_frame_creation_count_ = 0;

  // LCPP's LCP ElementLocator was matched against a tag against html
  // during preload scanning.
  bool lcpp_encountered_lcp_in_html = false;

  // Measures SVGImage performance per document.
  int svg_image_processed_count_ = 0;
  base::TimeDelta accumulated_svg_image_elapsed_time_;

  friend class Document;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_DATA_H_

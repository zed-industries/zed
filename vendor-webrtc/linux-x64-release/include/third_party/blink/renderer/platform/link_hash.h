/*
 * Copyright (C) 2008 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LINK_HASH_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LINK_HASH_H_

#include "base/check_op.h"
#include "net/base/schemeful_site.h"
#include "third_party/blink/public/platform/web_security_origin.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class KURL;

typedef uint64_t LinkHash;

// Use the low 32-bits of the 64-bit LinkHash as the key for HashSets.
struct LinkHashHashTraits : GenericHashTraits<LinkHash> {
  static unsigned GetHash(LinkHash key) { return static_cast<unsigned>(key); }
};

// Resolves the potentially relative URL "attributeURL" relative to the given
// base URL, and returns the hash of the string that will be used for visited
// link coloring. It will return the special value of 0 if attributeURL does not
// look like a relative URL.
PLATFORM_EXPORT LinkHash VisitedLinkHash(const KURL& base,
                                         const AtomicString& attribute_url);

// Returns the fingerprint representing this triple-partition key that will be
// used for visited link coloring. It will return the special value of 0 ("the
// null fingerprint") if the key has invalid elements or a fingerprint could not
// be computed.
PLATFORM_EXPORT LinkHash
PartitionedVisitedLinkFingerprint(const KURL& base_link_url,
                                  const AtomicString& relative_link_url,
                                  const net::SchemefulSite& top_level_site,
                                  const SecurityOrigin* frame_origin);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LINK_HASH_H_

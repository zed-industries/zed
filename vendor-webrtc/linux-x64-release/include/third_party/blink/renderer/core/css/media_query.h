/*
 * CSS Media Query
 *
 * Copyright (C) 2006 Kimmo Kinnunen <kimmo.t.kinnunen@nokia.com>.
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies).
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
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/axis.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class MediaQueryExp;
class MediaQueryExpNode;

using ExpressionHeapVector = Vector<MediaQueryExp>;

class CORE_EXPORT MediaQuery : public GarbageCollected<MediaQuery> {
 public:
  enum class RestrictorType : uint8_t { kOnly, kNot, kNone };

  static MediaQuery* CreateNotAll();

  MediaQuery(RestrictorType, String media_type, const MediaQueryExpNode*);
  MediaQuery(const MediaQuery&);
  MediaQuery& operator=(const MediaQuery&) = delete;
  ~MediaQuery();
  void Trace(Visitor*) const;

  bool HasUnknown() const { return has_unknown_; }
  RestrictorType Restrictor() const;
  const MediaQueryExpNode* ExpNode() const;
  const String& MediaType() const;
  bool operator==(const MediaQuery& other) const;
  String CssText() const;

 private:
  String media_type_;
  String serialization_cache_;
  Member<const MediaQueryExpNode> exp_node_;

  RestrictorType restrictor_;
  // Set if |exp_node_| contains any MediaQueryUnknownExpNode instances.
  //
  // If the runtime flag CSSMediaQueries4 is *not* enabled, this will cause the
  // MediaQuery to appear as a "not all".
  //
  // Knowing whether or not something is unknown is useful for use-counting and
  // testing purposes.
  bool has_unknown_;

  String Serialize() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_QUERY_H_

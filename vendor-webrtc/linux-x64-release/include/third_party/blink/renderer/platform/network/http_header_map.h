/*
 * Copyright (C) 2006 Apple Computer, Inc.  All rights reserved.
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_HEADER_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_HEADER_MAP_H_

#include <memory>
#include <utility>
#include "third_party/blink/renderer/platform/network/http_parsers.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

typedef Vector<std::pair<String, String>> CrossThreadHTTPHeaderMapData;

// FIXME: Not every header fits into a map. Notably, multiple Set-Cookie header
// fields are needed to set multiple cookies.
class PLATFORM_EXPORT HTTPHeaderMap final {
  DISALLOW_NEW();

 public:
  HTTPHeaderMap();
  ~HTTPHeaderMap();

  // Gets a copy of the data suitable for passing to another thread.
  std::unique_ptr<CrossThreadHTTPHeaderMapData> CopyData() const;

  void Adopt(std::unique_ptr<CrossThreadHTTPHeaderMapData>);

  typedef HashMap<AtomicString,
                  AtomicString,
                  CaseFoldingHashTraits<AtomicString>>
      MapType;
  typedef MapType::AddResult AddResult;
  typedef MapType::const_iterator const_iterator;

  wtf_size_t size() const { return headers_.size(); }
  const_iterator begin() const { return headers_.begin(); }
  const_iterator end() const { return headers_.end(); }
  const_iterator Find(const AtomicString& k) const { return headers_.find(k); }
  void Clear() { headers_.clear(); }
  bool Contains(const AtomicString& k) const { return headers_.Contains(k); }
  const AtomicString& Get(const AtomicString& k) const {
    const auto it = headers_.find(k);
    if (it == headers_.end())
      return g_null_atom;
    return it->value;
  }
  AddResult Set(const AtomicString& k, const AtomicString& v) {
    SECURITY_DCHECK(!k.Contains('\n') && !k.Contains('\r'));
    SECURITY_DCHECK(!v.Contains('\n') && !v.Contains('\r'));
    return headers_.Set(k, v);
  }
  AddResult Add(const AtomicString& k, const AtomicString& v) {
    SECURITY_DCHECK(!k.Contains('\n') && !k.Contains('\r'));
    SECURITY_DCHECK(!v.Contains('\n') && !v.Contains('\r'));
    return headers_.insert(k, v);
  }
  void Remove(const AtomicString& k) { headers_.erase(k); }
  bool operator!=(const HTTPHeaderMap& rhs) const {
    return headers_ != rhs.headers_;
  }
  bool operator==(const HTTPHeaderMap& rhs) const {
    return headers_ == rhs.headers_;
  }

  String GetAsRawString(int status_code, String status_message) const;

 private:
  MapType headers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_HEADER_MAP_H_

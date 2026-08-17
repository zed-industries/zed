// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_HEADER_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_HEADER_LIST_H_

#include <map>
#include <utility>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// http://fetch.spec.whatwg.org/#terminology-headers
class CORE_EXPORT FetchHeaderList final
    : public GarbageCollected<FetchHeaderList> {
 public:
  struct ByteCaseInsensitiveCompare {
    bool operator()(const String& lhs, const String& rhs) const {
      return CodeUnitCompareIgnoringASCIICaseLessThan(lhs, rhs);
    }
  };

  typedef std::pair<String, String> Header;
  FetchHeaderList* Clone() const;

  FetchHeaderList();
  ~FetchHeaderList();

  void Append(const String&, const String&);
  void Set(const String&, const String&);
  // FIXME: Implement parse()
  String ExtractMIMEType() const;

  size_t size() const;
  void Remove(const String&);
  bool Get(const String&, String&) const;
  Vector<String> GetSetCookie() const;
  String GetAsRawString(int status_code, String status_message) const;
  bool Has(const String&) const;
  void ClearList();

  Vector<Header> SortAndCombine() const;

  const std::multimap<String, String, ByteCaseInsensitiveCompare>& List()
      const {
    return header_list_;
  }

  static bool IsValidHeaderName(const String&);
  static bool IsValidHeaderValue(const String&);

  void Trace(Visitor* visitor) const {}

 private:
  // While using STL data structures in Blink is not very common or
  // encouraged, we do need a multimap here. The closest WTF structure
  // comparable to what we need would be a
  //   HashMap<String, Vector<String>>
  // but it is not a "flat" data structure like std::multimap is. The
  // size() of the HashMap is the number of distinct header names, not
  // the total number of headers and values on the list.
  // This would cause FetchHeaderList::size() to have to manually
  // iterate through all keys and vectors in the HashMap. Similarly,
  // list() would require callers to manually iterate through the
  // HashMap's keys and value vector.
  std::multimap<String, String, ByteCaseInsensitiveCompare> header_list_
      ALLOW_DISCOURAGED_TYPE("No multimap equivalent in blink");
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_HEADER_LIST_H_

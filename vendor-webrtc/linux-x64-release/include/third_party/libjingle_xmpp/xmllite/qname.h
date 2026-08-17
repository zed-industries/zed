/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef THIRD_PARTY_LIBJINGLE_XMPP_XMLLITE_QNAME_H_
#define THIRD_PARTY_LIBJINGLE_XMPP_XMLLITE_QNAME_H_

#include <string>

namespace jingle_xmpp {

class QName;

// StaticQName is used to represend constant quailified names. They
// can be initialized statically and don't need intializers code, e.g.
//   const StaticQName QN_FOO = { "foo_namespace", "foo" };
//
// Beside this use case, QName should be used everywhere
// else. StaticQName instances are implicitly converted to QName
// objects.
struct StaticQName {
  const char* const ns;
  const char* const local;

  bool operator==(const QName& other) const;
  bool operator!=(const QName& other) const;
};

class QName {
 public:
  QName();
  QName(const QName& qname);
  QName(const StaticQName& const_value);
  QName(const std::string& ns, const std::string& local);
  explicit QName(const std::string& merged_or_local);
  ~QName();

  const std::string& Namespace() const { return namespace_; }
  const std::string& LocalPart() const { return local_part_; }
  std::string Merged() const;
  bool IsEmpty() const;

  int Compare(const StaticQName& other) const;
  int Compare(const QName& other) const;

  bool operator==(const StaticQName& other) const {
    return Compare(other) == 0;
  }
  bool operator==(const QName& other) const {
    return Compare(other) == 0;
  }
  bool operator!=(const StaticQName& other) const {
    return Compare(other) != 0;
  }
  bool operator!=(const QName& other) const {
    return Compare(other) != 0;
  }
  bool operator<(const QName& other) const {
    return Compare(other) < 0;
  }

 private:
  std::string namespace_;
  std::string local_part_;
};

inline bool StaticQName::operator==(const QName& other) const {
  return other.Compare(*this) == 0;
}

inline bool StaticQName::operator!=(const QName& other) const {
  return other.Compare(*this) != 0;
}

}  // namespace jingle_xmpp

#endif  // THIRD_PARTY_LIBJINGLE_XMPP_XMLLITE_QNAME_H_

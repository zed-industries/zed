/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef THIRD_PARTY_LIBJINGLE_XMPP_XMLLITE_XMLELEMENT_H_
#define THIRD_PARTY_LIBJINGLE_XMPP_XMLLITE_XMLELEMENT_H_

#include <iosfwd>
#include <string>
#include <string_view>

#include "third_party/libjingle_xmpp/xmllite/qname.h"

namespace jingle_xmpp {

class XmlChild;
class XmlText;
class XmlElement;
class XmlAttr;

class XmlChild {
 public:
  XmlChild* NextChild() { return next_child_; }
  const XmlChild* NextChild() const { return next_child_; }

  bool IsText() const { return IsTextImpl(); }

  XmlElement* AsElement() { return AsElementImpl(); }
  const XmlElement* AsElement() const { return AsElementImpl(); }

  XmlText* AsText() { return AsTextImpl(); }
  const XmlText* AsText() const { return AsTextImpl(); }


 protected:
  XmlChild() :
    next_child_(NULL) {
  }

  virtual bool IsTextImpl() const = 0;
  virtual XmlElement* AsElementImpl() const = 0;
  virtual XmlText* AsTextImpl() const = 0;


  virtual ~XmlChild();

 private:
  friend class XmlElement;

  XmlChild(const XmlChild& noimpl);

  XmlChild* next_child_;
};

class XmlText : public XmlChild {
 public:
  explicit XmlText(const std::string& text) :
    XmlChild(),
    text_(text) {
  }
  explicit XmlText(const XmlText& t) :
    XmlChild(),
    text_(t.text_) {
  }
  explicit XmlText(const char* cstr, size_t len) :
    XmlChild(),
    text_(cstr, len) {
  }
  virtual ~XmlText();

  const std::string& Text() const { return text_; }
  void SetText(const std::string& text);
  void AddParsedText(const char* buf, int len);
  void AddText(const std::string& text);

 protected:
  virtual bool IsTextImpl() const;
  virtual XmlElement* AsElementImpl() const;
  virtual XmlText* AsTextImpl() const;

 private:
  std::string text_;
};

class XmlAttr {
 public:
  XmlAttr* NextAttr() const { return next_attr_; }
  const QName& Name() const { return name_; }
  const std::string& Value() const { return value_; }

 private:
  friend class XmlElement;

  explicit XmlAttr(const QName& name, std::string_view value)
      : next_attr_(NULL), name_(name), value_(value) {}
  explicit XmlAttr(const XmlAttr& att) :
    next_attr_(NULL),
    name_(att.name_),
    value_(att.value_) {
  }

  XmlAttr* next_attr_;
  QName name_;
  std::string value_;
};

class XmlElement : public XmlChild {
 public:
  explicit XmlElement(const QName& name);
  explicit XmlElement(const QName& name, bool useDefaultNs);
  explicit XmlElement(const XmlElement& elt);

  virtual ~XmlElement();

  const QName& Name() const { return name_; }
  void SetName(const QName& name) { name_ = name; }

  const std::string BodyText() const;
  void SetBodyText(const std::string& text);

  const QName FirstElementName() const;

  XmlAttr* FirstAttr();
  const XmlAttr* FirstAttr() const
    { return const_cast<XmlElement *>(this)->FirstAttr(); }

  // Attr will return an empty string if the attribute isn't there:
  // use HasAttr to test presence of an attribute.
  const std::string Attr(const StaticQName& name) const;
  const std::string Attr(const QName& name) const;
  bool HasAttr(const StaticQName& name) const;
  bool HasAttr(const QName& name) const;
  void SetAttr(const QName& name, std::string_view value);
  void ClearAttr(const QName& name);

  XmlChild* FirstChild();
  const XmlChild* FirstChild() const {
    return const_cast<XmlElement *>(this)->FirstChild();
  }

  XmlElement* FirstElement();
  const XmlElement* FirstElement() const {
    return const_cast<XmlElement *>(this)->FirstElement();
  }

  XmlElement* NextElement();
  const XmlElement* NextElement() const {
    return const_cast<XmlElement *>(this)->NextElement();
  }

  XmlElement* FirstWithNamespace(const std::string& ns);
  const XmlElement* FirstWithNamespace(const std::string& ns) const {
    return const_cast<XmlElement *>(this)->FirstWithNamespace(ns);
  }

  XmlElement* NextWithNamespace(const std::string& ns);
  const XmlElement* NextWithNamespace(const std::string& ns) const {
    return const_cast<XmlElement *>(this)->NextWithNamespace(ns);
  }

  XmlElement* FirstNamed(const StaticQName& name);
  const XmlElement* FirstNamed(const StaticQName& name) const {
    return const_cast<XmlElement *>(this)->FirstNamed(name);
  }

  XmlElement* FirstNamed(const QName& name);
  const XmlElement* FirstNamed(const QName& name) const {
    return const_cast<XmlElement *>(this)->FirstNamed(name);
  }

  XmlElement* NextNamed(const StaticQName& name);
  const XmlElement* NextNamed(const StaticQName& name) const {
    return const_cast<XmlElement *>(this)->NextNamed(name);
  }

  XmlElement* NextNamed(const QName& name);
  const XmlElement* NextNamed(const QName& name) const {
    return const_cast<XmlElement *>(this)->NextNamed(name);
  }

  // Finds the first element named 'name'.  If that element can't be found then
  // adds one and returns it.
  XmlElement* FindOrAddNamedChild(const QName& name);

  const std::string TextNamed(const QName& name) const;

  void InsertChildAfter(XmlChild* predecessor, XmlChild* new_child);
  void RemoveChildAfter(XmlChild* predecessor);

  void AddParsedText(const char* buf, int len);
  // Note: CDATA is not supported by XMPP, therefore using this function will
  // generate non-XMPP compatible XML.
  void AddCDATAText(const char* buf, int len);
  void AddText(const std::string& text);
  void AddText(const std::string& text, int depth);
  void AddElement(XmlElement* child);
  void AddElement(XmlElement* child, int depth);
  void AddAttr(const QName& name, const std::string& value);
  void AddAttr(const QName& name, const std::string& value, int depth);
  void ClearNamedChildren(const QName& name);
  void ClearAttributes();
  void ClearChildren();

  static XmlElement* ForStr(const std::string& str);
  std::string Str() const;

  bool IsCDATA() const { return cdata_; }

 protected:
  virtual bool IsTextImpl() const;
  virtual XmlElement* AsElementImpl() const;
  virtual XmlText* AsTextImpl() const;

 private:
  QName name_;
  XmlAttr* first_attr_;
  XmlAttr* last_attr_;
  XmlChild* first_child_;
  XmlChild* last_child_;
  bool cdata_;
};

}  // namespace jingle_xmpp

#endif  // THIRD_PARTY_LIBJINGLE_XMPP_XMLLITE_XMLELEMENT_H_

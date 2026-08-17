/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef THIRD_PARTY_LIBJINGLE_XMPP_XMLLITE_XMLPARSER_H_
#define THIRD_PARTY_LIBJINGLE_XMPP_XMLLITE_XMLPARSER_H_

#include <string>

#include "third_party/libjingle_xmpp/xmllite/xmlnsstack.h"
#ifdef EXPAT_RELATIVE_PATH
#include "expat.h"
#else
#include "third_party/expat/v2_0_1/Source/lib/expat.h"
#endif  // EXPAT_RELATIVE_PATH

struct XML_ParserStruct;
typedef struct XML_ParserStruct* XML_Parser;

namespace jingle_xmpp {

class XmlParseHandler;
class XmlParseContext;
class XmlParser;

class XmlParseContext {
public:
  virtual ~XmlParseContext() {}
  virtual QName ResolveQName(const char * qname, bool isAttr) = 0;
  virtual void RaiseError(XML_Error err) = 0;
  virtual void GetPosition(unsigned long * line, unsigned long * column,
                           unsigned long * byte_index) = 0;
};

class XmlParseHandler {
public:
  virtual ~XmlParseHandler() {}
  virtual void StartElement(XmlParseContext * pctx,
               const char * name, const char ** atts) = 0;
  virtual void EndElement(XmlParseContext * pctx,
               const char * name) = 0;
  virtual void CharacterData(XmlParseContext * pctx,
               const char * text, int len) = 0;
  virtual void Error(XmlParseContext * pctx,
               XML_Error errorCode) = 0;
};

class XmlParser {
public:
  static void ParseXml(XmlParseHandler * pxph, std::string text);

  explicit XmlParser(XmlParseHandler * pxph);
  bool Parse(const char * data, size_t len, bool isFinal);
  void Reset();
  virtual ~XmlParser();

  // expat callbacks
  void ExpatStartElement(const char * name, const char ** atts);
  void ExpatEndElement(const char * name);
  void ExpatCharacterData(const char * text, int len);
  void ExpatXmlDecl(const char * ver, const char * enc, int standalone);

private:

  class ParseContext : public XmlParseContext {
  public:
    ParseContext();
    virtual ~ParseContext();
    virtual QName ResolveQName(const char * qname, bool isAttr);
    virtual void RaiseError(XML_Error err) { if (!raised_) raised_ = err; }
    virtual void GetPosition(unsigned long * line, unsigned long * column,
                             unsigned long * byte_index);
    XML_Error RaisedError() { return raised_; }
    void Reset();

    void StartElement();
    void EndElement();
    void StartNamespace(const char * prefix, const char * ns);
    void SetPosition(int line, int column, long byte_index);

  private:
    XmlnsStack xmlnsstack_;
    XML_Error raised_;
    XML_Size line_number_;
    XML_Size column_number_;
    XML_Index byte_index_;
  };

  ParseContext context_;
  XML_Parser expat_;
  XmlParseHandler * pxph_;
  bool sentError_;
};

}  // namespace jingle_xmpp

#endif  // THIRD_PARTY_LIBJINGLE_XMPP_XMLLITE_XMLPARSER_H_

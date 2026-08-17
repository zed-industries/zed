/*
 * Copyright 2005 Maksim Orlovich <maksim@kde.org>
 * Copyright (C) 2006 Apple Computer, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_PARSER_H_

#include "third_party/blink/renderer/core/xml/xpath_predicate.h"
#include "third_party/blink/renderer/core/xml/xpath_step.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ExceptionState;
class UseCounter;
class V8XPathNSResolver;

namespace xpath {

class Expression;
class LocationPath;
class Parser;

struct Token {
  STACK_ALLOCATED();

 public:
  int type;
  String str;
  Step::Axis axis;
  NumericOp::Opcode numop;
  EqTestOp::Opcode eqop;

  Token(int t) : type(t) {}
  Token(int t, const String& v) : type(t), str(v) {}
  Token(int t, Step::Axis v) : type(t), axis(v) {}
  Token(int t, NumericOp::Opcode v) : type(t), numop(v) {}
  Token(int t, EqTestOp::Opcode v) : type(t), eqop(v) {}
};

class Parser {
  STACK_ALLOCATED();

 public:
  explicit Parser(UseCounter* use_counter);
  Parser(const Parser&) = delete;
  Parser& operator=(const Parser&) = delete;
  ~Parser();

  V8XPathNSResolver* Resolver() const { return resolver_; }
  bool ExpandQName(const String& q_name,
                   AtomicString& local_name,
                   AtomicString& namespace_uri);

  Expression* ParseStatement(const String& statement,
                             V8XPathNSResolver*,
                             ExceptionState&);

  static Parser* Current() { return current_parser_; }

  int Lex(void* yylval);

  Expression* top_expr_;
  bool got_namespace_error_;

 private:
  bool IsBinaryOperatorContext() const;

  void SkipWS();
  Token MakeTokenAndAdvance(int type, int advance = 1);
  Token MakeTokenAndAdvance(int type, NumericOp::Opcode, int advance = 1);
  Token MakeTokenAndAdvance(int type, EqTestOp::Opcode, int advance = 1);
  char PeekAheadHelper();
  char PeekCurHelper();

  Token LexString();
  Token LexNumber();
  bool LexNCName(String&);
  bool LexQName(String&);

  Token NextToken();
  Token NextTokenInternal();

  void Reset(const String& data);

  static Parser* current_parser_;

  unsigned next_pos_;
  String data_;
  int last_token_type_;
  V8XPathNSResolver* resolver_ = nullptr;
  UseCounter* use_counter_ = nullptr;
};

}  // namespace xpath

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_PARSER_H_

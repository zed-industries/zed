// [The "BSD 3-clause license"]
// Copyright (c) 2005-2007 Terence Parr
// Copyright (c) 2012-2015 Terence Parr
// Copyright (c) 2012-2015 Sam Harwell
// Copyright (c) 2014-2015 Gerald Rosenberg
// Copyright (c) 2023 Google LLC
// All rights reserved.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:

//  1. Redistributions of source code must retain the above copyright
//     notice, this list of conditions and the following disclaimer.
//  2. Redistributions in binary form must reproduce the above copyright
//     notice, this list of conditions and the following disclaimer in the
//     documentation and/or other materials provided with the distribution.
//  3. Neither the name of the copyright holder nor the names of its
//     contributors may be used to endorse or promote products derived from this
//     software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
// OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// Generated from ANTLRv4Lexer.g4 by ANTLR 4.11.2

#ifndef ANTLR4_GEN_ANTLRV_LEXER_H
#define ANTLR4_GEN_ANTLRV_LEXER_H

#include "antlr4-runtime.h"

namespace antlr4_grammar {

class LexerAdaptor : public antlr4::Lexer {
 public:
  LexerAdaptor &self;
  LexerAdaptor(antlr4::CharStream *input) : Lexer(input), self(*this){};
  int currentRuleType = antlr4::Token::INVALID_TYPE;

  int getCurrentRuleType();

  void setCurrentRuleType(int ruleType);

  void handleBeginArgument();

  void handleEndArgument();

  void handleEndAction();

  antlr4::Token *emit();

  bool inLexerRule();

  bool inParserRule();
};

class ANTLRv4Lexer : public LexerAdaptor {
 public:
  enum {
    TOKEN_REF = 1,
    RULE_REF = 2,
    LEXER_CHAR_SET = 3,
    DOC_COMMENT = 4,
    BLOCK_COMMENT = 5,
    LINE_COMMENT = 6,
    INT = 7,
    STRING_LITERAL = 8,
    UNTERMINATED_STRING_LITERAL = 9,
    BEGIN_ARGUMENT = 10,
    BEGIN_ACTION = 11,
    OPTIONS = 12,
    TOKENS = 13,
    CHANNELS = 14,
    IMPORT = 15,
    FRAGMENT = 16,
    LEXER = 17,
    PARSER = 18,
    GRAMMAR = 19,
    PROTECTED = 20,
    PUBLIC = 21,
    PRIVATE = 22,
    RETURNS = 23,
    LOCALS = 24,
    THROWS = 25,
    CATCH = 26,
    FINALLY = 27,
    MODE = 28,
    COLON = 29,
    COLONCOLON = 30,
    COMMA = 31,
    SEMI = 32,
    LPAREN = 33,
    RPAREN = 34,
    LBRACE = 35,
    RBRACE = 36,
    RARROW = 37,
    LT = 38,
    GT = 39,
    ASSIGN = 40,
    QUESTION = 41,
    STAR = 42,
    PLUS_ASSIGN = 43,
    PLUS = 44,
    OR = 45,
    DOLLAR = 46,
    RANGE = 47,
    DOT = 48,
    AT = 49,
    POUND = 50,
    NOT = 51,
    ID = 52,
    WS = 53,
    ERRCHAR = 54,
    END_ARGUMENT = 55,
    UNTERMINATED_ARGUMENT = 56,
    ARGUMENT_CONTENT = 57,
    END_ACTION = 58,
    UNTERMINATED_ACTION = 59,
    ACTION_CONTENT = 60,
    UNTERMINATED_CHAR_SET = 61
  };

  enum { OFF_CHANNEL = 2, COMMENT = 3 };

  enum { Argument = 1, TargetLanguageAction = 2, LexerCharSet = 3 };

  explicit ANTLRv4Lexer(antlr4::CharStream *input);

  ~ANTLRv4Lexer() override;

  std::string getGrammarFileName() const override;

  const std::vector<std::string> &getRuleNames() const override;

  const std::vector<std::string> &getChannelNames() const override;

  const std::vector<std::string> &getModeNames() const override;

  const antlr4::dfa::Vocabulary &getVocabulary() const override;

  antlr4::atn::SerializedATNView getSerializedATN() const override;

  const antlr4::atn::ATN &getATN() const override;

  void action(antlr4::RuleContext *context, size_t ruleIndex,
              size_t actionIndex) override;

  // By default the static state used to implement the lexer is lazily
  // initialized during the first call to the constructor. You can call this
  // function if you wish to initialize the static state ahead of time.
  static void initialize();

 private:
  // Individual action functions triggered by action() above.
  void BEGIN_ARGUMENTAction(antlr4::RuleContext *context, size_t actionIndex);
  void END_ARGUMENTAction(antlr4::RuleContext *context, size_t actionIndex);
  void END_ACTIONAction(antlr4::RuleContext *context, size_t actionIndex);

  // Individual semantic predicate functions triggered by sempred() above.
};

}  // namespace antlr4_grammar

#endif  // ANTLR4_GEN_ANTLRV_LEXER_H

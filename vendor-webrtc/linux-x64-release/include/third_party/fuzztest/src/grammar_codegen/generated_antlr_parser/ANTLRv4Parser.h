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

// Generated from ANTLRv4Parser.g4 by ANTLR 4.11.2

#ifndef ANTLR4_GEN_ANTLRV_PARSER_H
#define ANTLR4_GEN_ANTLRV_PARSER_H

#include "antlr4-runtime.h"

namespace antlr4_grammar {

class ANTLRv4Parser : public antlr4::Parser {
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

  enum {
    RuleGrammarSpec = 0,
    RuleGrammarDecl = 1,
    RuleGrammarType = 2,
    RulePrequelConstruct = 3,
    RuleOptionsSpec = 4,
    RuleOption = 5,
    RuleOptionValue = 6,
    RuleDelegateGrammars = 7,
    RuleDelegateGrammar = 8,
    RuleTokensSpec = 9,
    RuleChannelsSpec = 10,
    RuleIdList = 11,
    RuleAction_ = 12,
    RuleActionScopeName = 13,
    RuleActionBlock = 14,
    RuleArgActionBlock = 15,
    RuleModeSpec = 16,
    RuleRules = 17,
    RuleRuleSpec = 18,
    RuleParserRuleSpec = 19,
    RuleExceptionGroup = 20,
    RuleExceptionHandler = 21,
    RuleFinallyClause = 22,
    RuleRulePrequel = 23,
    RuleRuleReturns = 24,
    RuleThrowsSpec = 25,
    RuleLocalsSpec = 26,
    RuleRuleAction = 27,
    RuleRuleModifiers = 28,
    RuleRuleModifier = 29,
    RuleRuleBlock = 30,
    RuleRuleAltList = 31,
    RuleLabeledAlt = 32,
    RuleLexerRuleSpec = 33,
    RuleLexerRuleBlock = 34,
    RuleLexerAltList = 35,
    RuleLexerAlt = 36,
    RuleLexerElements = 37,
    RuleLexerElement = 38,
    RuleLabeledLexerElement = 39,
    RuleLexerBlock = 40,
    RuleLexerCommands = 41,
    RuleLexerCommand = 42,
    RuleLexerCommandName = 43,
    RuleLexerCommandExpr = 44,
    RuleAltList = 45,
    RuleAlternative = 46,
    RuleElement = 47,
    RuleLabeledElement = 48,
    RuleEbnf = 49,
    RuleBlockSuffix = 50,
    RuleEbnfSuffix = 51,
    RuleLexerAtom = 52,
    RuleAtom = 53,
    RuleNotSet = 54,
    RuleBlockSet = 55,
    RuleSetElement = 56,
    RuleBlock = 57,
    RuleRuleref = 58,
    RuleCharacterRange = 59,
    RuleTerminal = 60,
    RuleElementOptions = 61,
    RuleElementOption = 62,
    RuleIdentifier = 63
  };

  explicit ANTLRv4Parser(antlr4::TokenStream *input);

  ANTLRv4Parser(antlr4::TokenStream *input,
                const antlr4::atn::ParserATNSimulatorOptions &options);

  ~ANTLRv4Parser() override;

  std::string getGrammarFileName() const override;

  const antlr4::atn::ATN &getATN() const override;

  const std::vector<std::string> &getRuleNames() const override;

  const antlr4::dfa::Vocabulary &getVocabulary() const override;

  antlr4::atn::SerializedATNView getSerializedATN() const override;

  class GrammarSpecContext;
  class GrammarDeclContext;
  class GrammarTypeContext;
  class PrequelConstructContext;
  class OptionsSpecContext;
  class OptionContext;
  class OptionValueContext;
  class DelegateGrammarsContext;
  class DelegateGrammarContext;
  class TokensSpecContext;
  class ChannelsSpecContext;
  class IdListContext;
  class Action_Context;
  class ActionScopeNameContext;
  class ActionBlockContext;
  class ArgActionBlockContext;
  class ModeSpecContext;
  class RulesContext;
  class RuleSpecContext;
  class ParserRuleSpecContext;
  class ExceptionGroupContext;
  class ExceptionHandlerContext;
  class FinallyClauseContext;
  class RulePrequelContext;
  class RuleReturnsContext;
  class ThrowsSpecContext;
  class LocalsSpecContext;
  class RuleActionContext;
  class RuleModifiersContext;
  class RuleModifierContext;
  class RuleBlockContext;
  class RuleAltListContext;
  class LabeledAltContext;
  class LexerRuleSpecContext;
  class LexerRuleBlockContext;
  class LexerAltListContext;
  class LexerAltContext;
  class LexerElementsContext;
  class LexerElementContext;
  class LabeledLexerElementContext;
  class LexerBlockContext;
  class LexerCommandsContext;
  class LexerCommandContext;
  class LexerCommandNameContext;
  class LexerCommandExprContext;
  class AltListContext;
  class AlternativeContext;
  class ElementContext;
  class LabeledElementContext;
  class EbnfContext;
  class BlockSuffixContext;
  class EbnfSuffixContext;
  class LexerAtomContext;
  class AtomContext;
  class NotSetContext;
  class BlockSetContext;
  class SetElementContext;
  class BlockContext;
  class RulerefContext;
  class CharacterRangeContext;
  class TerminalContext;
  class ElementOptionsContext;
  class ElementOptionContext;
  class IdentifierContext;

  class GrammarSpecContext : public antlr4::ParserRuleContext {
   public:
    GrammarSpecContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GrammarDeclContext *grammarDecl();
    RulesContext *rules();
    antlr4::tree::TerminalNode *EOF();
    std::vector<PrequelConstructContext *> prequelConstruct();
    PrequelConstructContext *prequelConstruct(size_t i);
    std::vector<ModeSpecContext *> modeSpec();
    ModeSpecContext *modeSpec(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  GrammarSpecContext *grammarSpec();

  class GrammarDeclContext : public antlr4::ParserRuleContext {
   public:
    GrammarDeclContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    GrammarTypeContext *grammarType();
    IdentifierContext *identifier();
    antlr4::tree::TerminalNode *SEMI();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  GrammarDeclContext *grammarDecl();

  class GrammarTypeContext : public antlr4::ParserRuleContext {
   public:
    GrammarTypeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LEXER();
    antlr4::tree::TerminalNode *GRAMMAR();
    antlr4::tree::TerminalNode *PARSER();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  GrammarTypeContext *grammarType();

  class PrequelConstructContext : public antlr4::ParserRuleContext {
   public:
    PrequelConstructContext(antlr4::ParserRuleContext *parent,
                            size_t invokingState);
    virtual size_t getRuleIndex() const override;
    OptionsSpecContext *optionsSpec();
    DelegateGrammarsContext *delegateGrammars();
    TokensSpecContext *tokensSpec();
    ChannelsSpecContext *channelsSpec();
    Action_Context *action_();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  PrequelConstructContext *prequelConstruct();

  class OptionsSpecContext : public antlr4::ParserRuleContext {
   public:
    OptionsSpecContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *OPTIONS();
    antlr4::tree::TerminalNode *RBRACE();
    std::vector<OptionContext *> option();
    OptionContext *option(size_t i);
    std::vector<antlr4::tree::TerminalNode *> SEMI();
    antlr4::tree::TerminalNode *SEMI(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  OptionsSpecContext *optionsSpec();

  class OptionContext : public antlr4::ParserRuleContext {
   public:
    OptionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();
    antlr4::tree::TerminalNode *ASSIGN();
    OptionValueContext *optionValue();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  OptionContext *option();

  class OptionValueContext : public antlr4::ParserRuleContext {
   public:
    OptionValueContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<IdentifierContext *> identifier();
    IdentifierContext *identifier(size_t i);
    std::vector<antlr4::tree::TerminalNode *> DOT();
    antlr4::tree::TerminalNode *DOT(size_t i);
    antlr4::tree::TerminalNode *STRING_LITERAL();
    ActionBlockContext *actionBlock();
    antlr4::tree::TerminalNode *INT();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  OptionValueContext *optionValue();

  class DelegateGrammarsContext : public antlr4::ParserRuleContext {
   public:
    DelegateGrammarsContext(antlr4::ParserRuleContext *parent,
                            size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *IMPORT();
    std::vector<DelegateGrammarContext *> delegateGrammar();
    DelegateGrammarContext *delegateGrammar(size_t i);
    antlr4::tree::TerminalNode *SEMI();
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode *COMMA(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  DelegateGrammarsContext *delegateGrammars();

  class DelegateGrammarContext : public antlr4::ParserRuleContext {
   public:
    DelegateGrammarContext(antlr4::ParserRuleContext *parent,
                           size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<IdentifierContext *> identifier();
    IdentifierContext *identifier(size_t i);
    antlr4::tree::TerminalNode *ASSIGN();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  DelegateGrammarContext *delegateGrammar();

  class TokensSpecContext : public antlr4::ParserRuleContext {
   public:
    TokensSpecContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TOKENS();
    antlr4::tree::TerminalNode *RBRACE();
    IdListContext *idList();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  TokensSpecContext *tokensSpec();

  class ChannelsSpecContext : public antlr4::ParserRuleContext {
   public:
    ChannelsSpecContext(antlr4::ParserRuleContext *parent,
                        size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CHANNELS();
    antlr4::tree::TerminalNode *RBRACE();
    IdListContext *idList();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ChannelsSpecContext *channelsSpec();

  class IdListContext : public antlr4::ParserRuleContext {
   public:
    IdListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<IdentifierContext *> identifier();
    IdentifierContext *identifier(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode *COMMA(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  IdListContext *idList();

  class Action_Context : public antlr4::ParserRuleContext {
   public:
    Action_Context(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *AT();
    IdentifierContext *identifier();
    ActionBlockContext *actionBlock();
    ActionScopeNameContext *actionScopeName();
    antlr4::tree::TerminalNode *COLONCOLON();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  Action_Context *action_();

  class ActionScopeNameContext : public antlr4::ParserRuleContext {
   public:
    ActionScopeNameContext(antlr4::ParserRuleContext *parent,
                           size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();
    antlr4::tree::TerminalNode *LEXER();
    antlr4::tree::TerminalNode *PARSER();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ActionScopeNameContext *actionScopeName();

  class ActionBlockContext : public antlr4::ParserRuleContext {
   public:
    ActionBlockContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *BEGIN_ACTION();
    antlr4::tree::TerminalNode *END_ACTION();
    std::vector<antlr4::tree::TerminalNode *> ACTION_CONTENT();
    antlr4::tree::TerminalNode *ACTION_CONTENT(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ActionBlockContext *actionBlock();

  class ArgActionBlockContext : public antlr4::ParserRuleContext {
   public:
    ArgActionBlockContext(antlr4::ParserRuleContext *parent,
                          size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *BEGIN_ARGUMENT();
    antlr4::tree::TerminalNode *END_ARGUMENT();
    std::vector<antlr4::tree::TerminalNode *> ARGUMENT_CONTENT();
    antlr4::tree::TerminalNode *ARGUMENT_CONTENT(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ArgActionBlockContext *argActionBlock();

  class ModeSpecContext : public antlr4::ParserRuleContext {
   public:
    ModeSpecContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *MODE();
    IdentifierContext *identifier();
    antlr4::tree::TerminalNode *SEMI();
    std::vector<LexerRuleSpecContext *> lexerRuleSpec();
    LexerRuleSpecContext *lexerRuleSpec(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ModeSpecContext *modeSpec();

  class RulesContext : public antlr4::ParserRuleContext {
   public:
    RulesContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<RuleSpecContext *> ruleSpec();
    RuleSpecContext *ruleSpec(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RulesContext *rules();

  class RuleSpecContext : public antlr4::ParserRuleContext {
   public:
    RuleSpecContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ParserRuleSpecContext *parserRuleSpec();
    LexerRuleSpecContext *lexerRuleSpec();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RuleSpecContext *ruleSpec();

  class ParserRuleSpecContext : public antlr4::ParserRuleContext {
   public:
    ParserRuleSpecContext(antlr4::ParserRuleContext *parent,
                          size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *RULE_REF();
    antlr4::tree::TerminalNode *COLON();
    RuleBlockContext *ruleBlock();
    antlr4::tree::TerminalNode *SEMI();
    ExceptionGroupContext *exceptionGroup();
    RuleModifiersContext *ruleModifiers();
    ArgActionBlockContext *argActionBlock();
    RuleReturnsContext *ruleReturns();
    ThrowsSpecContext *throwsSpec();
    LocalsSpecContext *localsSpec();
    std::vector<RulePrequelContext *> rulePrequel();
    RulePrequelContext *rulePrequel(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ParserRuleSpecContext *parserRuleSpec();

  class ExceptionGroupContext : public antlr4::ParserRuleContext {
   public:
    ExceptionGroupContext(antlr4::ParserRuleContext *parent,
                          size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<ExceptionHandlerContext *> exceptionHandler();
    ExceptionHandlerContext *exceptionHandler(size_t i);
    FinallyClauseContext *finallyClause();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ExceptionGroupContext *exceptionGroup();

  class ExceptionHandlerContext : public antlr4::ParserRuleContext {
   public:
    ExceptionHandlerContext(antlr4::ParserRuleContext *parent,
                            size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *CATCH();
    ArgActionBlockContext *argActionBlock();
    ActionBlockContext *actionBlock();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ExceptionHandlerContext *exceptionHandler();

  class FinallyClauseContext : public antlr4::ParserRuleContext {
   public:
    FinallyClauseContext(antlr4::ParserRuleContext *parent,
                         size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *FINALLY();
    ActionBlockContext *actionBlock();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  FinallyClauseContext *finallyClause();

  class RulePrequelContext : public antlr4::ParserRuleContext {
   public:
    RulePrequelContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    OptionsSpecContext *optionsSpec();
    RuleActionContext *ruleAction();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RulePrequelContext *rulePrequel();

  class RuleReturnsContext : public antlr4::ParserRuleContext {
   public:
    RuleReturnsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *RETURNS();
    ArgActionBlockContext *argActionBlock();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RuleReturnsContext *ruleReturns();

  class ThrowsSpecContext : public antlr4::ParserRuleContext {
   public:
    ThrowsSpecContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *THROWS();
    std::vector<IdentifierContext *> identifier();
    IdentifierContext *identifier(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode *COMMA(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ThrowsSpecContext *throwsSpec();

  class LocalsSpecContext : public antlr4::ParserRuleContext {
   public:
    LocalsSpecContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LOCALS();
    ArgActionBlockContext *argActionBlock();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LocalsSpecContext *localsSpec();

  class RuleActionContext : public antlr4::ParserRuleContext {
   public:
    RuleActionContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *AT();
    IdentifierContext *identifier();
    ActionBlockContext *actionBlock();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RuleActionContext *ruleAction();

  class RuleModifiersContext : public antlr4::ParserRuleContext {
   public:
    RuleModifiersContext(antlr4::ParserRuleContext *parent,
                         size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<RuleModifierContext *> ruleModifier();
    RuleModifierContext *ruleModifier(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RuleModifiersContext *ruleModifiers();

  class RuleModifierContext : public antlr4::ParserRuleContext {
   public:
    RuleModifierContext(antlr4::ParserRuleContext *parent,
                        size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *PUBLIC();
    antlr4::tree::TerminalNode *PRIVATE();
    antlr4::tree::TerminalNode *PROTECTED();
    antlr4::tree::TerminalNode *FRAGMENT();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RuleModifierContext *ruleModifier();

  class RuleBlockContext : public antlr4::ParserRuleContext {
   public:
    RuleBlockContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    RuleAltListContext *ruleAltList();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RuleBlockContext *ruleBlock();

  class RuleAltListContext : public antlr4::ParserRuleContext {
   public:
    RuleAltListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<LabeledAltContext *> labeledAlt();
    LabeledAltContext *labeledAlt(size_t i);
    std::vector<antlr4::tree::TerminalNode *> OR();
    antlr4::tree::TerminalNode *OR(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RuleAltListContext *ruleAltList();

  class LabeledAltContext : public antlr4::ParserRuleContext {
   public:
    LabeledAltContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    AlternativeContext *alternative();
    antlr4::tree::TerminalNode *POUND();
    IdentifierContext *identifier();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LabeledAltContext *labeledAlt();

  class LexerRuleSpecContext : public antlr4::ParserRuleContext {
   public:
    LexerRuleSpecContext(antlr4::ParserRuleContext *parent,
                         size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TOKEN_REF();
    antlr4::tree::TerminalNode *COLON();
    LexerRuleBlockContext *lexerRuleBlock();
    antlr4::tree::TerminalNode *SEMI();
    antlr4::tree::TerminalNode *FRAGMENT();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerRuleSpecContext *lexerRuleSpec();

  class LexerRuleBlockContext : public antlr4::ParserRuleContext {
   public:
    LexerRuleBlockContext(antlr4::ParserRuleContext *parent,
                          size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LexerAltListContext *lexerAltList();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerRuleBlockContext *lexerRuleBlock();

  class LexerAltListContext : public antlr4::ParserRuleContext {
   public:
    LexerAltListContext(antlr4::ParserRuleContext *parent,
                        size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<LexerAltContext *> lexerAlt();
    LexerAltContext *lexerAlt(size_t i);
    std::vector<antlr4::tree::TerminalNode *> OR();
    antlr4::tree::TerminalNode *OR(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerAltListContext *lexerAltList();

  class LexerAltContext : public antlr4::ParserRuleContext {
   public:
    LexerAltContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LexerElementsContext *lexerElements();
    LexerCommandsContext *lexerCommands();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerAltContext *lexerAlt();

  class LexerElementsContext : public antlr4::ParserRuleContext {
   public:
    LexerElementsContext(antlr4::ParserRuleContext *parent,
                         size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<LexerElementContext *> lexerElement();
    LexerElementContext *lexerElement(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerElementsContext *lexerElements();

  class LexerElementContext : public antlr4::ParserRuleContext {
   public:
    LexerElementContext(antlr4::ParserRuleContext *parent,
                        size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LabeledLexerElementContext *labeledLexerElement();
    EbnfSuffixContext *ebnfSuffix();
    LexerAtomContext *lexerAtom();
    LexerBlockContext *lexerBlock();
    ActionBlockContext *actionBlock();
    antlr4::tree::TerminalNode *QUESTION();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerElementContext *lexerElement();

  class LabeledLexerElementContext : public antlr4::ParserRuleContext {
   public:
    LabeledLexerElementContext(antlr4::ParserRuleContext *parent,
                               size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();
    antlr4::tree::TerminalNode *ASSIGN();
    antlr4::tree::TerminalNode *PLUS_ASSIGN();
    LexerAtomContext *lexerAtom();
    LexerBlockContext *lexerBlock();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LabeledLexerElementContext *labeledLexerElement();

  class LexerBlockContext : public antlr4::ParserRuleContext {
   public:
    LexerBlockContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LPAREN();
    LexerAltListContext *lexerAltList();
    antlr4::tree::TerminalNode *RPAREN();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerBlockContext *lexerBlock();

  class LexerCommandsContext : public antlr4::ParserRuleContext {
   public:
    LexerCommandsContext(antlr4::ParserRuleContext *parent,
                         size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *RARROW();
    std::vector<LexerCommandContext *> lexerCommand();
    LexerCommandContext *lexerCommand(size_t i);
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode *COMMA(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerCommandsContext *lexerCommands();

  class LexerCommandContext : public antlr4::ParserRuleContext {
   public:
    LexerCommandContext(antlr4::ParserRuleContext *parent,
                        size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LexerCommandNameContext *lexerCommandName();
    antlr4::tree::TerminalNode *LPAREN();
    LexerCommandExprContext *lexerCommandExpr();
    antlr4::tree::TerminalNode *RPAREN();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerCommandContext *lexerCommand();

  class LexerCommandNameContext : public antlr4::ParserRuleContext {
   public:
    LexerCommandNameContext(antlr4::ParserRuleContext *parent,
                            size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();
    antlr4::tree::TerminalNode *MODE();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerCommandNameContext *lexerCommandName();

  class LexerCommandExprContext : public antlr4::ParserRuleContext {
   public:
    LexerCommandExprContext(antlr4::ParserRuleContext *parent,
                            size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();
    antlr4::tree::TerminalNode *INT();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerCommandExprContext *lexerCommandExpr();

  class AltListContext : public antlr4::ParserRuleContext {
   public:
    AltListContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<AlternativeContext *> alternative();
    AlternativeContext *alternative(size_t i);
    std::vector<antlr4::tree::TerminalNode *> OR();
    antlr4::tree::TerminalNode *OR(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  AltListContext *altList();

  class AlternativeContext : public antlr4::ParserRuleContext {
   public:
    AlternativeContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    ElementOptionsContext *elementOptions();
    std::vector<ElementContext *> element();
    ElementContext *element(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  AlternativeContext *alternative();

  class ElementContext : public antlr4::ParserRuleContext {
   public:
    ElementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    LabeledElementContext *labeledElement();
    EbnfSuffixContext *ebnfSuffix();
    AtomContext *atom();
    EbnfContext *ebnf();
    ActionBlockContext *actionBlock();
    antlr4::tree::TerminalNode *QUESTION();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ElementContext *element();

  class LabeledElementContext : public antlr4::ParserRuleContext {
   public:
    LabeledElementContext(antlr4::ParserRuleContext *parent,
                          size_t invokingState);
    virtual size_t getRuleIndex() const override;
    IdentifierContext *identifier();
    antlr4::tree::TerminalNode *ASSIGN();
    antlr4::tree::TerminalNode *PLUS_ASSIGN();
    AtomContext *atom();
    BlockContext *block();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LabeledElementContext *labeledElement();

  class EbnfContext : public antlr4::ParserRuleContext {
   public:
    EbnfContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    BlockContext *block();
    BlockSuffixContext *blockSuffix();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  EbnfContext *ebnf();

  class BlockSuffixContext : public antlr4::ParserRuleContext {
   public:
    BlockSuffixContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    EbnfSuffixContext *ebnfSuffix();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  BlockSuffixContext *blockSuffix();

  class EbnfSuffixContext : public antlr4::ParserRuleContext {
   public:
    EbnfSuffixContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<antlr4::tree::TerminalNode *> QUESTION();
    antlr4::tree::TerminalNode *QUESTION(size_t i);
    antlr4::tree::TerminalNode *STAR();
    antlr4::tree::TerminalNode *PLUS();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  EbnfSuffixContext *ebnfSuffix();

  class LexerAtomContext : public antlr4::ParserRuleContext {
   public:
    LexerAtomContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    CharacterRangeContext *characterRange();
    TerminalContext *terminal();
    NotSetContext *notSet();
    antlr4::tree::TerminalNode *LEXER_CHAR_SET();
    antlr4::tree::TerminalNode *DOT();
    ElementOptionsContext *elementOptions();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  LexerAtomContext *lexerAtom();

  class AtomContext : public antlr4::ParserRuleContext {
   public:
    AtomContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    TerminalContext *terminal();
    RulerefContext *ruleref();
    NotSetContext *notSet();
    antlr4::tree::TerminalNode *DOT();
    ElementOptionsContext *elementOptions();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  AtomContext *atom();

  class NotSetContext : public antlr4::ParserRuleContext {
   public:
    NotSetContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *NOT();
    SetElementContext *setElement();
    BlockSetContext *blockSet();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  NotSetContext *notSet();

  class BlockSetContext : public antlr4::ParserRuleContext {
   public:
    BlockSetContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LPAREN();
    std::vector<SetElementContext *> setElement();
    SetElementContext *setElement(size_t i);
    antlr4::tree::TerminalNode *RPAREN();
    std::vector<antlr4::tree::TerminalNode *> OR();
    antlr4::tree::TerminalNode *OR(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  BlockSetContext *blockSet();

  class SetElementContext : public antlr4::ParserRuleContext {
   public:
    SetElementContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TOKEN_REF();
    ElementOptionsContext *elementOptions();
    antlr4::tree::TerminalNode *STRING_LITERAL();
    CharacterRangeContext *characterRange();
    antlr4::tree::TerminalNode *LEXER_CHAR_SET();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  SetElementContext *setElement();

  class BlockContext : public antlr4::ParserRuleContext {
   public:
    BlockContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LPAREN();
    AltListContext *altList();
    antlr4::tree::TerminalNode *RPAREN();
    antlr4::tree::TerminalNode *COLON();
    OptionsSpecContext *optionsSpec();
    std::vector<RuleActionContext *> ruleAction();
    RuleActionContext *ruleAction(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  BlockContext *block();

  class RulerefContext : public antlr4::ParserRuleContext {
   public:
    RulerefContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *RULE_REF();
    ArgActionBlockContext *argActionBlock();
    ElementOptionsContext *elementOptions();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  RulerefContext *ruleref();

  class CharacterRangeContext : public antlr4::ParserRuleContext {
   public:
    CharacterRangeContext(antlr4::ParserRuleContext *parent,
                          size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<antlr4::tree::TerminalNode *> STRING_LITERAL();
    antlr4::tree::TerminalNode *STRING_LITERAL(size_t i);
    antlr4::tree::TerminalNode *RANGE();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  CharacterRangeContext *characterRange();

  class TerminalContext : public antlr4::ParserRuleContext {
   public:
    TerminalContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *TOKEN_REF();
    ElementOptionsContext *elementOptions();
    antlr4::tree::TerminalNode *STRING_LITERAL();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  TerminalContext *terminal();

  class ElementOptionsContext : public antlr4::ParserRuleContext {
   public:
    ElementOptionsContext(antlr4::ParserRuleContext *parent,
                          size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *LT();
    std::vector<ElementOptionContext *> elementOption();
    ElementOptionContext *elementOption(size_t i);
    antlr4::tree::TerminalNode *GT();
    std::vector<antlr4::tree::TerminalNode *> COMMA();
    antlr4::tree::TerminalNode *COMMA(size_t i);

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ElementOptionsContext *elementOptions();

  class ElementOptionContext : public antlr4::ParserRuleContext {
   public:
    ElementOptionContext(antlr4::ParserRuleContext *parent,
                         size_t invokingState);
    virtual size_t getRuleIndex() const override;
    std::vector<IdentifierContext *> identifier();
    IdentifierContext *identifier(size_t i);
    antlr4::tree::TerminalNode *ASSIGN();
    antlr4::tree::TerminalNode *STRING_LITERAL();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  ElementOptionContext *elementOption();

  class IdentifierContext : public antlr4::ParserRuleContext {
   public:
    IdentifierContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *RULE_REF();
    antlr4::tree::TerminalNode *TOKEN_REF();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;
  };

  IdentifierContext *identifier();

  // By default the static state used to implement the parser is lazily
  // initialized during the first call to the constructor. You can call this
  // function if you wish to initialize the static state ahead of time.
  static void initialize();

 private:
};

}  // namespace antlr4_grammar

#endif  // ANTLR4_GEN_ANTLRV_PARSER_H

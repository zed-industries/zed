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

#ifndef ANTLR4_GEN_ANTLRV_PARSERBASELISTENER_H
#define ANTLR4_GEN_ANTLRV_PARSERBASELISTENER_H

#include "ANTLRv4ParserListener.h"
#include "antlr4-runtime.h"

namespace antlr4_grammar {

/**
 * This class provides an empty implementation of ANTLRv4ParserListener,
 * which can be extended to create a listener which only needs to handle a
 * subset of the available methods.
 */
class ANTLRv4ParserBaseListener : public ANTLRv4ParserListener {
 public:
  virtual void enterGrammarSpec(
      ANTLRv4Parser::GrammarSpecContext* /*ctx*/) override {}
  virtual void exitGrammarSpec(
      ANTLRv4Parser::GrammarSpecContext* /*ctx*/) override {}

  virtual void enterGrammarDecl(
      ANTLRv4Parser::GrammarDeclContext* /*ctx*/) override {}
  virtual void exitGrammarDecl(
      ANTLRv4Parser::GrammarDeclContext* /*ctx*/) override {}

  virtual void enterGrammarType(
      ANTLRv4Parser::GrammarTypeContext* /*ctx*/) override {}
  virtual void exitGrammarType(
      ANTLRv4Parser::GrammarTypeContext* /*ctx*/) override {}

  virtual void enterPrequelConstruct(
      ANTLRv4Parser::PrequelConstructContext* /*ctx*/) override {}
  virtual void exitPrequelConstruct(
      ANTLRv4Parser::PrequelConstructContext* /*ctx*/) override {}

  virtual void enterOptionsSpec(
      ANTLRv4Parser::OptionsSpecContext* /*ctx*/) override {}
  virtual void exitOptionsSpec(
      ANTLRv4Parser::OptionsSpecContext* /*ctx*/) override {}

  virtual void enterOption(ANTLRv4Parser::OptionContext* /*ctx*/) override {}
  virtual void exitOption(ANTLRv4Parser::OptionContext* /*ctx*/) override {}

  virtual void enterOptionValue(
      ANTLRv4Parser::OptionValueContext* /*ctx*/) override {}
  virtual void exitOptionValue(
      ANTLRv4Parser::OptionValueContext* /*ctx*/) override {}

  virtual void enterDelegateGrammars(
      ANTLRv4Parser::DelegateGrammarsContext* /*ctx*/) override {}
  virtual void exitDelegateGrammars(
      ANTLRv4Parser::DelegateGrammarsContext* /*ctx*/) override {}

  virtual void enterDelegateGrammar(
      ANTLRv4Parser::DelegateGrammarContext* /*ctx*/) override {}
  virtual void exitDelegateGrammar(
      ANTLRv4Parser::DelegateGrammarContext* /*ctx*/) override {}

  virtual void enterTokensSpec(
      ANTLRv4Parser::TokensSpecContext* /*ctx*/) override {}
  virtual void exitTokensSpec(
      ANTLRv4Parser::TokensSpecContext* /*ctx*/) override {}

  virtual void enterChannelsSpec(
      ANTLRv4Parser::ChannelsSpecContext* /*ctx*/) override {}
  virtual void exitChannelsSpec(
      ANTLRv4Parser::ChannelsSpecContext* /*ctx*/) override {}

  virtual void enterIdList(ANTLRv4Parser::IdListContext* /*ctx*/) override {}
  virtual void exitIdList(ANTLRv4Parser::IdListContext* /*ctx*/) override {}

  virtual void enterAction_(ANTLRv4Parser::Action_Context* /*ctx*/) override {}
  virtual void exitAction_(ANTLRv4Parser::Action_Context* /*ctx*/) override {}

  virtual void enterActionScopeName(
      ANTLRv4Parser::ActionScopeNameContext* /*ctx*/) override {}
  virtual void exitActionScopeName(
      ANTLRv4Parser::ActionScopeNameContext* /*ctx*/) override {}

  virtual void enterActionBlock(
      ANTLRv4Parser::ActionBlockContext* /*ctx*/) override {}
  virtual void exitActionBlock(
      ANTLRv4Parser::ActionBlockContext* /*ctx*/) override {}

  virtual void enterArgActionBlock(
      ANTLRv4Parser::ArgActionBlockContext* /*ctx*/) override {}
  virtual void exitArgActionBlock(
      ANTLRv4Parser::ArgActionBlockContext* /*ctx*/) override {}

  virtual void enterModeSpec(ANTLRv4Parser::ModeSpecContext* /*ctx*/) override {
  }
  virtual void exitModeSpec(ANTLRv4Parser::ModeSpecContext* /*ctx*/) override {}

  virtual void enterRules(ANTLRv4Parser::RulesContext* /*ctx*/) override {}
  virtual void exitRules(ANTLRv4Parser::RulesContext* /*ctx*/) override {}

  virtual void enterRuleSpec(ANTLRv4Parser::RuleSpecContext* /*ctx*/) override {
  }
  virtual void exitRuleSpec(ANTLRv4Parser::RuleSpecContext* /*ctx*/) override {}

  virtual void enterParserRuleSpec(
      ANTLRv4Parser::ParserRuleSpecContext* /*ctx*/) override {}
  virtual void exitParserRuleSpec(
      ANTLRv4Parser::ParserRuleSpecContext* /*ctx*/) override {}

  virtual void enterExceptionGroup(
      ANTLRv4Parser::ExceptionGroupContext* /*ctx*/) override {}
  virtual void exitExceptionGroup(
      ANTLRv4Parser::ExceptionGroupContext* /*ctx*/) override {}

  virtual void enterExceptionHandler(
      ANTLRv4Parser::ExceptionHandlerContext* /*ctx*/) override {}
  virtual void exitExceptionHandler(
      ANTLRv4Parser::ExceptionHandlerContext* /*ctx*/) override {}

  virtual void enterFinallyClause(
      ANTLRv4Parser::FinallyClauseContext* /*ctx*/) override {}
  virtual void exitFinallyClause(
      ANTLRv4Parser::FinallyClauseContext* /*ctx*/) override {}

  virtual void enterRulePrequel(
      ANTLRv4Parser::RulePrequelContext* /*ctx*/) override {}
  virtual void exitRulePrequel(
      ANTLRv4Parser::RulePrequelContext* /*ctx*/) override {}

  virtual void enterRuleReturns(
      ANTLRv4Parser::RuleReturnsContext* /*ctx*/) override {}
  virtual void exitRuleReturns(
      ANTLRv4Parser::RuleReturnsContext* /*ctx*/) override {}

  virtual void enterThrowsSpec(
      ANTLRv4Parser::ThrowsSpecContext* /*ctx*/) override {}
  virtual void exitThrowsSpec(
      ANTLRv4Parser::ThrowsSpecContext* /*ctx*/) override {}

  virtual void enterLocalsSpec(
      ANTLRv4Parser::LocalsSpecContext* /*ctx*/) override {}
  virtual void exitLocalsSpec(
      ANTLRv4Parser::LocalsSpecContext* /*ctx*/) override {}

  virtual void enterRuleAction(
      ANTLRv4Parser::RuleActionContext* /*ctx*/) override {}
  virtual void exitRuleAction(
      ANTLRv4Parser::RuleActionContext* /*ctx*/) override {}

  virtual void enterRuleModifiers(
      ANTLRv4Parser::RuleModifiersContext* /*ctx*/) override {}
  virtual void exitRuleModifiers(
      ANTLRv4Parser::RuleModifiersContext* /*ctx*/) override {}

  virtual void enterRuleModifier(
      ANTLRv4Parser::RuleModifierContext* /*ctx*/) override {}
  virtual void exitRuleModifier(
      ANTLRv4Parser::RuleModifierContext* /*ctx*/) override {}

  virtual void enterRuleBlock(
      ANTLRv4Parser::RuleBlockContext* /*ctx*/) override {}
  virtual void exitRuleBlock(
      ANTLRv4Parser::RuleBlockContext* /*ctx*/) override {}

  virtual void enterRuleAltList(
      ANTLRv4Parser::RuleAltListContext* /*ctx*/) override {}
  virtual void exitRuleAltList(
      ANTLRv4Parser::RuleAltListContext* /*ctx*/) override {}

  virtual void enterLabeledAlt(
      ANTLRv4Parser::LabeledAltContext* /*ctx*/) override {}
  virtual void exitLabeledAlt(
      ANTLRv4Parser::LabeledAltContext* /*ctx*/) override {}

  virtual void enterLexerRuleSpec(
      ANTLRv4Parser::LexerRuleSpecContext* /*ctx*/) override {}
  virtual void exitLexerRuleSpec(
      ANTLRv4Parser::LexerRuleSpecContext* /*ctx*/) override {}

  virtual void enterLexerRuleBlock(
      ANTLRv4Parser::LexerRuleBlockContext* /*ctx*/) override {}
  virtual void exitLexerRuleBlock(
      ANTLRv4Parser::LexerRuleBlockContext* /*ctx*/) override {}

  virtual void enterLexerAltList(
      ANTLRv4Parser::LexerAltListContext* /*ctx*/) override {}
  virtual void exitLexerAltList(
      ANTLRv4Parser::LexerAltListContext* /*ctx*/) override {}

  virtual void enterLexerAlt(ANTLRv4Parser::LexerAltContext* /*ctx*/) override {
  }
  virtual void exitLexerAlt(ANTLRv4Parser::LexerAltContext* /*ctx*/) override {}

  virtual void enterLexerElements(
      ANTLRv4Parser::LexerElementsContext* /*ctx*/) override {}
  virtual void exitLexerElements(
      ANTLRv4Parser::LexerElementsContext* /*ctx*/) override {}

  virtual void enterLexerElement(
      ANTLRv4Parser::LexerElementContext* /*ctx*/) override {}
  virtual void exitLexerElement(
      ANTLRv4Parser::LexerElementContext* /*ctx*/) override {}

  virtual void enterLabeledLexerElement(
      ANTLRv4Parser::LabeledLexerElementContext* /*ctx*/) override {}
  virtual void exitLabeledLexerElement(
      ANTLRv4Parser::LabeledLexerElementContext* /*ctx*/) override {}

  virtual void enterLexerBlock(
      ANTLRv4Parser::LexerBlockContext* /*ctx*/) override {}
  virtual void exitLexerBlock(
      ANTLRv4Parser::LexerBlockContext* /*ctx*/) override {}

  virtual void enterLexerCommands(
      ANTLRv4Parser::LexerCommandsContext* /*ctx*/) override {}
  virtual void exitLexerCommands(
      ANTLRv4Parser::LexerCommandsContext* /*ctx*/) override {}

  virtual void enterLexerCommand(
      ANTLRv4Parser::LexerCommandContext* /*ctx*/) override {}
  virtual void exitLexerCommand(
      ANTLRv4Parser::LexerCommandContext* /*ctx*/) override {}

  virtual void enterLexerCommandName(
      ANTLRv4Parser::LexerCommandNameContext* /*ctx*/) override {}
  virtual void exitLexerCommandName(
      ANTLRv4Parser::LexerCommandNameContext* /*ctx*/) override {}

  virtual void enterLexerCommandExpr(
      ANTLRv4Parser::LexerCommandExprContext* /*ctx*/) override {}
  virtual void exitLexerCommandExpr(
      ANTLRv4Parser::LexerCommandExprContext* /*ctx*/) override {}

  virtual void enterAltList(ANTLRv4Parser::AltListContext* /*ctx*/) override {}
  virtual void exitAltList(ANTLRv4Parser::AltListContext* /*ctx*/) override {}

  virtual void enterAlternative(
      ANTLRv4Parser::AlternativeContext* /*ctx*/) override {}
  virtual void exitAlternative(
      ANTLRv4Parser::AlternativeContext* /*ctx*/) override {}

  virtual void enterElement(ANTLRv4Parser::ElementContext* /*ctx*/) override {}
  virtual void exitElement(ANTLRv4Parser::ElementContext* /*ctx*/) override {}

  virtual void enterLabeledElement(
      ANTLRv4Parser::LabeledElementContext* /*ctx*/) override {}
  virtual void exitLabeledElement(
      ANTLRv4Parser::LabeledElementContext* /*ctx*/) override {}

  virtual void enterEbnf(ANTLRv4Parser::EbnfContext* /*ctx*/) override {}
  virtual void exitEbnf(ANTLRv4Parser::EbnfContext* /*ctx*/) override {}

  virtual void enterBlockSuffix(
      ANTLRv4Parser::BlockSuffixContext* /*ctx*/) override {}
  virtual void exitBlockSuffix(
      ANTLRv4Parser::BlockSuffixContext* /*ctx*/) override {}

  virtual void enterEbnfSuffix(
      ANTLRv4Parser::EbnfSuffixContext* /*ctx*/) override {}
  virtual void exitEbnfSuffix(
      ANTLRv4Parser::EbnfSuffixContext* /*ctx*/) override {}

  virtual void enterLexerAtom(
      ANTLRv4Parser::LexerAtomContext* /*ctx*/) override {}
  virtual void exitLexerAtom(
      ANTLRv4Parser::LexerAtomContext* /*ctx*/) override {}

  virtual void enterAtom(ANTLRv4Parser::AtomContext* /*ctx*/) override {}
  virtual void exitAtom(ANTLRv4Parser::AtomContext* /*ctx*/) override {}

  virtual void enterNotSet(ANTLRv4Parser::NotSetContext* /*ctx*/) override {}
  virtual void exitNotSet(ANTLRv4Parser::NotSetContext* /*ctx*/) override {}

  virtual void enterBlockSet(ANTLRv4Parser::BlockSetContext* /*ctx*/) override {
  }
  virtual void exitBlockSet(ANTLRv4Parser::BlockSetContext* /*ctx*/) override {}

  virtual void enterSetElement(
      ANTLRv4Parser::SetElementContext* /*ctx*/) override {}
  virtual void exitSetElement(
      ANTLRv4Parser::SetElementContext* /*ctx*/) override {}

  virtual void enterBlock(ANTLRv4Parser::BlockContext* /*ctx*/) override {}
  virtual void exitBlock(ANTLRv4Parser::BlockContext* /*ctx*/) override {}

  virtual void enterRuleref(ANTLRv4Parser::RulerefContext* /*ctx*/) override {}
  virtual void exitRuleref(ANTLRv4Parser::RulerefContext* /*ctx*/) override {}

  virtual void enterCharacterRange(
      ANTLRv4Parser::CharacterRangeContext* /*ctx*/) override {}
  virtual void exitCharacterRange(
      ANTLRv4Parser::CharacterRangeContext* /*ctx*/) override {}

  virtual void enterTerminal(ANTLRv4Parser::TerminalContext* /*ctx*/) override {
  }
  virtual void exitTerminal(ANTLRv4Parser::TerminalContext* /*ctx*/) override {}

  virtual void enterElementOptions(
      ANTLRv4Parser::ElementOptionsContext* /*ctx*/) override {}
  virtual void exitElementOptions(
      ANTLRv4Parser::ElementOptionsContext* /*ctx*/) override {}

  virtual void enterElementOption(
      ANTLRv4Parser::ElementOptionContext* /*ctx*/) override {}
  virtual void exitElementOption(
      ANTLRv4Parser::ElementOptionContext* /*ctx*/) override {}

  virtual void enterIdentifier(
      ANTLRv4Parser::IdentifierContext* /*ctx*/) override {}
  virtual void exitIdentifier(
      ANTLRv4Parser::IdentifierContext* /*ctx*/) override {}

  virtual void enterEveryRule(antlr4::ParserRuleContext* /*ctx*/) override {}
  virtual void exitEveryRule(antlr4::ParserRuleContext* /*ctx*/) override {}
  virtual void visitTerminal(antlr4::tree::TerminalNode* /*node*/) override {}
  virtual void visitErrorNode(antlr4::tree::ErrorNode* /*node*/) override {}
};

}  // namespace antlr4_grammar

#endif  // ANTLR4_GEN_ANTLRV_PARSERBASELISTENER_H

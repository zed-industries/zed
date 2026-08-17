// -*- mode: C++ -*-

// Copyright 2010 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// Original author: Jim Blandy <jimb@mozilla.com> <jimb@red-bean.com>

// cfi_frame_info.h: Define the CFIFrameInfo class, which holds the
// set of 'STACK CFI'-derived register recovery rules that apply at a
// given instruction.

#ifndef PROCESSOR_CFI_FRAME_INFO_H_
#define PROCESSOR_CFI_FRAME_INFO_H_

#include <map>
#include <string>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"

namespace google_breakpad {

using std::map;

class MemoryRegion;

// A set of rules for recovering the calling frame's registers'
// values, when the PC is at a given address in the current frame's
// function. See the description of 'STACK CFI' records at:
//
// https://chromium.googlesource.com/breakpad/breakpad/+/master/docs/symbol_files.md
//
// To prepare an instance of CFIFrameInfo for use at a given
// instruction, first populate it with the rules from the 'STACK CFI
// INIT' record that covers that instruction, and then apply the
// changes given by the 'STACK CFI' records up to our instruction's
// address. Then, use the FindCallerRegs member function to apply the
// rules to the callee frame's register values, yielding the caller
// frame's register values.
class CFIFrameInfo {
 public:
  // A map from register names onto values.
  template<typename ValueType> class RegisterValueMap: 
    public map<string, ValueType> { };

  // Set the expression for computing a call frame address, return
  // address, or register's value. At least the CFA rule and the RA
  // rule must be set before calling FindCallerRegs.
  void SetCFARule(const string& expression) { cfa_rule_ = expression; }
  void SetRARule(const string& expression)  { ra_rule_ = expression; }
  void SetRegisterRule(const string& register_name, const string& expression) {
    register_rules_[register_name] = expression;
  }

  // Compute the values of the calling frame's registers, according to
  // this rule set. Use ValueType in expression evaluation; this
  // should be uint32_t on machines with 32-bit addresses, or
  // uint64_t on machines with 64-bit addresses.
  //
  // Return true on success, false otherwise.
  //
  // MEMORY provides access to the contents of the stack. REGISTERS is
  // a dictionary mapping the names of registers whose values are
  // known in the current frame to their values. CALLER_REGISTERS is
  // populated with the values of the recoverable registers in the
  // frame that called the current frame.
  //
  // In addition, CALLER_REGISTERS[".ra"] will be the return address,
  // and CALLER_REGISTERS[".cfa"] will be the call frame address.
  // These may be helpful in computing the caller's PC and stack
  // pointer, if their values are not explicitly specified.
  template<typename ValueType>
  bool FindCallerRegs(const RegisterValueMap<ValueType>& registers,
                      const MemoryRegion& memory,
                      RegisterValueMap<ValueType>* caller_registers) const;

  // Serialize the rules in this object into a string in the format
  // of STACK CFI records.
  string Serialize() const;

 private:

  // A map from register names onto evaluation rules. 
  typedef map<string, string> RuleMap;

  // In this type, a "postfix expression" is an expression of the sort
  // interpreted by google_breakpad::PostfixEvaluator.

  // A postfix expression for computing the current frame's CFA (call
  // frame address). The CFA is a reference address for the frame that
  // remains unchanged throughout the frame's lifetime. You should
  // evaluate this expression with a dictionary initially populated
  // with the values of the current frame's known registers.
  string cfa_rule_;

  // The following expressions should be evaluated with a dictionary
  // initially populated with the values of the current frame's known
  // registers, and with ".cfa" set to the result of evaluating the
  // cfa_rule expression, above.

  // A postfix expression for computing the current frame's return
  // address. 
  string ra_rule_;

  // For a register named REG, rules[REG] is a postfix expression
  // which leaves the value of REG in the calling frame on the top of
  // the stack. You should evaluate this expression
  RuleMap register_rules_;
};

// A parser for STACK CFI-style rule sets.
// This may seem bureaucratic: there's no legitimate run-time reason
// to use a parser/handler pattern for this, as it's not a likely
// reuse boundary. But doing so makes finer-grained unit testing
// possible.
class CFIRuleParser {
 public:

  class Handler {
   public:
    Handler() { }
    virtual ~Handler() { }

    // The input specifies EXPRESSION as the CFA/RA computation rule.
    virtual void CFARule(const string& expression) = 0;
    virtual void RARule(const string& expression) = 0;

    // The input specifies EXPRESSION as the recovery rule for register NAME.
    virtual void RegisterRule(const string& name, const string& expression) = 0;
  };
    
  // Construct a parser which feeds its results to HANDLER.
  CFIRuleParser(Handler* handler) : handler_(handler) { }

  // Parse RULE_SET as a set of CFA computation and RA/register
  // recovery rules, as appearing in STACK CFI records. Report the
  // results of parsing by making the appropriate calls to handler_.
  // Return true if parsing was successful, false otherwise.
  bool Parse(const string& rule_set);

 private:
  // Report any accumulated rule to handler_
  bool Report();

  // The handler to which the parser reports its findings.
  Handler* handler_;

  // Working data.
  string name_, expression_;
};

// A handler for rule set parsing that populates a CFIFrameInfo with
// the results.
class CFIFrameInfoParseHandler: public CFIRuleParser::Handler {
 public:
  // Populate FRAME_INFO with the results of parsing.
  CFIFrameInfoParseHandler(CFIFrameInfo* frame_info)
      : frame_info_(frame_info) { }

  void CFARule(const string& expression);
  void RARule(const string& expression);
  void RegisterRule(const string& name, const string& expression);

 private:
  CFIFrameInfo* frame_info_;
};

// A utility class template for simple 'STACK CFI'-driven stack walkers.
// Given a CFIFrameInfo instance, a table describing the architecture's
// register set, and a context holding the last frame's registers, an
// instance of this class can populate a new context with the caller's
// registers.
//
// This class template doesn't use any internal knowledge of CFIFrameInfo
// or the other stack walking structures; it just uses the public interface
// of CFIFrameInfo to do the usual things. But the logic it handles should
// be common to many different architectures' stack walkers, so wrapping it
// up in a class should allow the walkers to share code.
//
// RegisterType should be the type of this architecture's registers, either
// uint32_t or uint64_t. RawContextType should be the raw context
// structure type for this architecture.
template <typename RegisterType, class RawContextType>
class SimpleCFIWalker {
 public:
  // A structure describing one architecture register.
  struct RegisterSet {
    // The register name, as it appears in STACK CFI rules.
    const char* name;

    // An alternate name that the register's value might be found
    // under in a register value dictionary, or NULL. When generating
    // names, prefer NAME to this value. It's common to list ".cfa" as
    // an alternative name for the stack pointer, and ".ra" as an
    // alternative name for the instruction pointer.
    const char* alternate_name;

    // True if the callee is expected to preserve the value of this
    // register. If this flag is true for some register R, and the STACK
    // CFI records provide no rule to recover R, then SimpleCFIWalker
    // assumes that the callee has not changed R's value, and the caller's
    // value for R is that currently in the callee's context.
    bool callee_saves;

    // The ContextValidity flag representing the register's presence.
    int validity_flag;

    // A pointer to the RawContextType member that holds the
    // register's value.
    RegisterType RawContextType::*context_member;
  };

  // Create a simple CFI-based frame walker, given a description of the
  // architecture's register set. REGISTER_MAP is an array of
  // RegisterSet structures; MAP_SIZE is the number of elements in the
  // array.
  SimpleCFIWalker(const RegisterSet* register_map, size_t map_size)
      : register_map_(register_map), map_size_(map_size) { }

  // Compute the calling frame's raw context given the callee's raw
  // context.
  //
  // Given:
  //
  // - MEMORY, holding the stack's contents,
  // - CFI_FRAME_INFO, describing the called function,
  // - CALLEE_CONTEXT, holding the called frame's registers, and
  // - CALLEE_VALIDITY, indicating which registers in CALLEE_CONTEXT are valid,
  //
  // fill in CALLER_CONTEXT with the caller's register values, and set
  // CALLER_VALIDITY to indicate which registers are valid in
  // CALLER_CONTEXT. Return true on success, or false on failure.
  bool FindCallerRegisters(const MemoryRegion& memory,
                           const CFIFrameInfo& cfi_frame_info,
                           const RawContextType& callee_context,
                           int callee_validity,
                           RawContextType* caller_context,
                           int* caller_validity) const;

 private:
  const RegisterSet* register_map_;
  size_t map_size_;
};

}  // namespace google_breakpad

#include "cfi_frame_info-inl.h"

#endif  // PROCESSOR_CFI_FRAME_INFO_H_

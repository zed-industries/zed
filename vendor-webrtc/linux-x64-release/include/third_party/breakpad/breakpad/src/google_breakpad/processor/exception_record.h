// Copyright 2019 Google LLC
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
//
// exception_record.h: A snapshot of an exception record.
//
// Author: Ivan Penkov

#ifndef THIRD_PARTY_BREAKPAD_SRC_GOOGLE_BREAKPAD_PROCESSOR_EXCEPTION_RECORD_H_
#define THIRD_PARTY_BREAKPAD_SRC_GOOGLE_BREAKPAD_PROCESSOR_EXCEPTION_RECORD_H_

#include <vector>

namespace google_breakpad {

// Additional argument that describes the exception.
class ExceptionParameter {
 public:
  ExceptionParameter(uint64_t value, const string& description)
      : value_(value), description_(description) {}
  // Accessors. See the data declarations below.
  uint64_t value() const { return value_; }
  void set_value(uint64_t value) { value_ = value; }
  const string& description() const { return description_; }
  void set_description(const string& description) {
    description_ = description;
  }

 private:
  // Parameter value.
  uint64_t value_;
  // Human readable description/interpretation of the above value.
  string description_;
};

// A snapshot of an exception record. Contains exception record details: code,
// flags, address, parameters.
class ExceptionRecord {
 public:
  // Accessors. See the data declarations below.
  uint32_t code() const { return code_; }
  const string& code_description() const { return code_description_; }
  void set_code(uint32_t code, const string& description) {
    code_ = code;
    code_description_ = description;
  }

  uint32_t flags() const { return flags_; }
  const string& flags_description() const { return flags_description_; }
  void set_flags(uint32_t flags, const string& description) {
    flags_ = flags;
    flags_description_ = description;
  }

  uint64_t nested_exception_record_address() const {
    return nested_exception_record_address_;
  }
  void set_nested_exception_record_address(
      uint64_t nested_exception_record_address) {
    nested_exception_record_address_ = nested_exception_record_address;
  }

  uint64_t address() const { return address_; }
  void set_address(uint64_t address) { address_ = address; }

  const std::vector<ExceptionParameter>* parameters() const {
    return &parameters_;
  }
  void add_parameter(uint64_t value, const string& description) {
    parameters_.push_back(ExceptionParameter(value, description));
  }

 private:
  // Exception code.
  uint32_t code_;
  string code_description_;

  // Exception flags.
  uint32_t flags_;
  string flags_description_;

  // The address of an associated MDException structure. Exception records can
  // be chained together to provide additional information when nested
  // exceptions occur.
  uint64_t nested_exception_record_address_;

  // The memory address that caused the exception.  For data access errors,
  // this will be the data address that caused the fault.  For code errors,
  // this will be the address of the instruction that caused the fault.
  uint64_t address_;

  // An array of additional arguments that describe the exception.
  std::vector<ExceptionParameter> parameters_;
};

}  // namespace google_breakpad


#endif  // THIRD_PARTY_BREAKPAD_SRC_GOOGLE_BREAKPAD_PROCESSOR_EXCEPTION_RECORD_H_

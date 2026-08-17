// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_LIBFUZZER_RESEARCH_DOMATOLPM_DOMATOLPM_H_
#define TESTING_LIBFUZZER_RESEARCH_DOMATOLPM_DOMATOLPM_H_

#include <string>
#include <string_view>
#include <unordered_map>
#include <unordered_set>

#include "base/strings/string_number_conversions.h"

namespace domatolpm {

// SampleBuilder is used by the internal runtime DomatoLPM generated code to
// build the current sample.
// This is a very dumb interface, but allows for modularity and extensibility
// without changing the generated code.
class SampleBuilder {
 public:
  virtual ~SampleBuilder() = default;
  virtual std::string_view view() = 0;
  virtual void append(std::string_view) = 0;
};

// TextSampleBuilder builds a sample into a string.
class TextSampleBuilder : public SampleBuilder {
 public:
  TextSampleBuilder() = default;
  ~TextSampleBuilder() override = default;
  std::string_view view() override;
  void append(std::string_view v) override;

 private:
  std::string data_;
};

// Context is used by the internal runtime DomatoLPM generated code to interact
// with the current context. It is used to retrieve the current sample builder
// and interact with existing variables.
class Context {
 public:
  Context() = default;
  SampleBuilder* GetBuilder();
  bool HasVar(const std::string& var_type);
  void SetVar(const std::string& var_type, const std::string& var_name);
  std::string_view GetVar(const std::string& var_type, int32_t id);
  std::string GetNewID();

 private:
  TextSampleBuilder builder_;
  std::unordered_map<std::string, std::unordered_set<std::string>> vars_;
  uint64_t id_ = 0;
};

}  // namespace domatolpm

#endif  // TESTING_LIBFUZZER_RESEARCH_DOMATOLPM_DOMATOLPM_H_

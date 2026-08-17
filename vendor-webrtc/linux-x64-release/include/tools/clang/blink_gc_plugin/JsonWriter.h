// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_JSON_WRITER_H_
#define TOOLS_BLINK_GC_PLUGIN_JSON_WRITER_H_

#include <memory>
#include <stack>

#include "llvm/Support/raw_ostream.h"

// Helper to write information for the points-to graph.
class JsonWriter {
 public:
  static JsonWriter* from(std::unique_ptr<llvm::raw_ostream> os) {
    return os ? new JsonWriter(std::move(os)) : 0;
  }
  void OpenList() {
    Separator();
    *os_ << "[";
    state_.push(false);
  }
  void OpenList(const std::string& key) {
    Write(key);
    *os_ << ":";
    OpenList();
  }
  void CloseList() {
    *os_ << "]";
    state_.pop();
  }
  void OpenObject() {
    Separator();
    *os_ << "{";
    state_.push(false);
  }
  void CloseObject() {
    *os_ << "}\n";
    state_.pop();
  }
  void Write(const size_t val) {
    Separator();
    *os_ << val;
  }
  void Write(const std::string& val) {
    Separator();
    *os_ << "\"" << Escape(val) << "\"";
  }
  void Write(const std::string& key, const size_t val) {
    Separator();
    *os_ << "\"" << Escape(key) << "\":" << val;
  }
  void Write(const std::string& key, const std::string& val) {
    Separator();
    *os_ << "\"" << Escape(key) << "\":\"" << Escape(val) << "\"";
  }
 private:
  JsonWriter(std::unique_ptr<llvm::raw_ostream> os) : os_(std::move(os)) {}
  void Separator() {
    if (state_.empty())
      return;
    if (state_.top()) {
      *os_ << ",";
      return;
    }
    state_.top() = true;
  }
  std::string Escape(const std::string& s) {
    std::string copy = s;
    size_t i = 0;
    while ((i = copy.find('\\', i)) != std::string::npos) {
      copy.replace(i, 1, "\\\\");
      i += 2;
    }
    return copy;
  }
  std::unique_ptr<llvm::raw_ostream> os_;
  std::stack<bool> state_;
};

#endif // TOOLS_BLINK_GC_PLUGIN_JSON_WRITER_H_

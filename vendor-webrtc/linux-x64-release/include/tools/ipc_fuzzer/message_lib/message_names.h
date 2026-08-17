// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_IPC_FUZZER_MESSAGE_LIB_MESSAGE_NAMES_H_
#define TOOLS_IPC_FUZZER_MESSAGE_LIB_MESSAGE_NAMES_H_

#include <stdint.h>

#include <string>
#include <unordered_map>
#include "base/check.h"

namespace ipc_fuzzer {

class MessageNames {
 public:
  MessageNames();

  MessageNames(const MessageNames&) = delete;
  MessageNames& operator=(const MessageNames&) = delete;

  ~MessageNames();
  static MessageNames* GetInstance();

  void Add(uint32_t type, const char* name) {
    name_map_[type] = name;
    type_map_[name] = type;
  }

  bool TypeExists(uint32_t type) {
    return name_map_.find(type) != name_map_.end();
  }

  bool NameExists(const std::string& name) {
    return type_map_.find(name) != type_map_.end();
  }

  const std::string& TypeToName(uint32_t type) {
    TypeToNameMap::iterator it = name_map_.find(type);
    CHECK(it != name_map_.end());
    return it->second;
  }

  uint32_t NameToType(const std::string& name) {
    NameToTypeMap::iterator it = type_map_.find(name);
    CHECK(it != type_map_.end());
    return it->second;
  }

 private:
  typedef std::unordered_map<uint32_t, std::string> TypeToNameMap;
  typedef std::unordered_map<std::string, uint32_t> NameToTypeMap;
  TypeToNameMap name_map_;
  NameToTypeMap type_map_;

  static MessageNames* all_names_;
};

}  // namespace ipc_fuzzer

#endif  // TOOLS_IPC_FUZZER_MESSAGE_LIB_MESSAGE_NAMES_H_

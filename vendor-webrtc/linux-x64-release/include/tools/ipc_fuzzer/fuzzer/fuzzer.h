// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_IPC_FUZZER_MUTATE_FUZZER_H_
#define TOOLS_IPC_FUZZER_MUTATE_FUZZER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "base/strings/string_util.h"
#include "ipc/ipc_message.h"

namespace ipc_fuzzer {

// Interface implemented by those who generate basic types.  The types all
// correspond to the types which a pickle from base/pickle.h can pickle,
// plus the floating point types.
class Fuzzer {
 public:
  // Functions for various data types.
  virtual void FuzzBool(bool* value) = 0;
  virtual void FuzzInt(int* value) = 0;
  virtual void FuzzLong(long* value) = 0;
  virtual void FuzzSize(size_t* value) = 0;
  virtual void FuzzUChar(unsigned char* value) = 0;
  virtual void FuzzWChar(wchar_t* value) = 0;
  virtual void FuzzUInt16(uint16_t* value) = 0;
  virtual void FuzzUInt32(uint32_t* value) = 0;
  virtual void FuzzInt64(int64_t* value) = 0;
  virtual void FuzzUInt64(uint64_t* value) = 0;
  virtual void FuzzFloat(float* value) = 0;
  virtual void FuzzDouble(double *value) = 0;
  virtual void FuzzString(std::string* value) = 0;
  virtual void FuzzString16(std::u16string* value) = 0;
  virtual void FuzzData(char* data, int length) = 0;
  virtual void FuzzBytes(void* data, int data_len) = 0;

  // Used to determine if a completely new value should be generated for
  // certain types instead of attempting to modify the existing one.
  virtual bool ShouldGenerate();
};

class NoOpFuzzer : public Fuzzer {
 public:
  NoOpFuzzer() {}
  virtual ~NoOpFuzzer() {}

  void FuzzBool(bool* value) override {}
  void FuzzInt(int* value) override {}
  void FuzzLong(long* value) override {}
  void FuzzSize(size_t* value) override {}
  void FuzzUChar(unsigned char* value) override {}
  void FuzzWChar(wchar_t* value) override {}
  void FuzzUInt16(uint16_t* value) override {}
  void FuzzUInt32(uint32_t* value) override {}
  void FuzzInt64(int64_t* value) override {}
  void FuzzUInt64(uint64_t* value) override {}
  void FuzzFloat(float* value) override {}
  void FuzzDouble(double* value) override {}
  void FuzzString(std::string* value) override {}
  void FuzzString16(std::u16string* value) override {}
  void FuzzData(char* data, int length) override {}
  void FuzzBytes(void* data, int data_len) override {}
};

using FuzzerFunction = std::unique_ptr<IPC::Message> (*)(IPC::Message*,
                                                         Fuzzer*);

// Used for mutating messages. Once populated, the map associates a message ID
// with a FuzzerFunction used for mutation of that message type.
using FuzzerFunctionMap = std::unordered_map<uint32_t, FuzzerFunction>;
void PopulateFuzzerFunctionMap(FuzzerFunctionMap* map);

// Used for generating new messages. Once populated, the vector contains
// FuzzerFunctions for all message types that we know how to generate.
using FuzzerFunctionVector = std::vector<FuzzerFunction>;
void PopulateFuzzerFunctionVector(FuzzerFunctionVector* function_vector);

// Since IPC::Message can be serialized, we also track a global function vector
// to handle generation of new messages while fuzzing.
extern FuzzerFunctionVector g_function_vector;

}  // namespace ipc_fuzzer

#endif  // TOOLS_IPC_FUZZER_MUTATE_FUZZER_H_

// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_SYSTEM_RAW_PTR_SYSTEM_TEST_H_
#define TOOLS_CLANG_PLUGINS_TESTS_SYSTEM_RAW_PTR_SYSTEM_TEST_H_
#line 7 "/src/tools/clang/plugins/tests/system/raw_ptr_system_test.h"

#define SYS_INT int
#define SYS_INTP int*
#define SYS_CONST const
#define SYS_ATTR [[fake_attribute]]
#define SYS_INTP_FIELD() int* macro_ptr
#define SYS_TYPE_WITH_SUFFIX(TYP) TYP##Suffix
#define SYS_SYMBOL(SYM) SYM
#define SYS_SYMBOL_WITH_SUFFIX(SYM) SYM##_Suffix
#define SYS_EQ =
#define SYS_NULLPTR nullptr
class SysTypSuffix;

// OK: code owner has no control over system header.
struct SysStructWithSysMacro {
  int* ptr0;                                     // OK.
  SYS_INT* ptr1;                                 // OK.
  SYS_INTP ptr2;                                 // OK.
  int* SYS_CONST ptr3;                           // OK.
  int* SYS_ATTR ptr4;                            // OK.
  SYS_INTP_FIELD();                              // OK.
  SYS_TYPE_WITH_SUFFIX(SysTyp) * ptr5;           // OK.
  int* SYS_SYMBOL(ptr6);                         // OK.
  int* SYS_SYMBOL_WITH_SUFFIX(ptr7);             // OK.
  int* ptr8 SYS_EQ nullptr;                      // OK.
  int* ptr9 = SYS_NULLPTR;                       // OK.
  int* SYS_SYMBOL_WITH_SUFFIX(ptr10) = nullptr;  // OK.
};

// OK: code owner has no control over system header and command line.
struct SysStructWithCmdMacro {
  int* ptr0;                                     // OK.
  CMD_INT* ptr1;                                 // OK.
  CMD_INTP ptr2;                                 // OK.
  int* CMD_CONST ptr3;                           // OK.
  int* CMD_ATTR ptr4;                            // OK.
  CMD_INTP_FIELD();                              // OK.
  CMD_TYPE_WITH_SUFFIX(SysTyp) * ptr5;           // OK.
  int* CMD_SYMBOL(ptr6);                         // OK.
  int* CMD_SYMBOL_WITH_SUFFIX(ptr7);             // OK.
  int* ptr8 CMD_EQ nullptr;                      // OK.
  int* ptr9 = CMD_NULLPTR;                       // OK.
  int* CMD_SYMBOL_WITH_SUFFIX(ptr10) = nullptr;  // OK.
};

// These `USR_***` macro should be defined before including this header,
// in `//tools/clang/plugins/tests/raw_ptr_fields_macro.cpp`.
struct SysStructWithUsrMacro {
  // OK: code owner has no control over system header.
  int* ptr0;
  // OK: code owner has no control over system header.
  USR_INT* ptr1;
  // OK: code owner has no control over system header.
  USR_INTP ptr2;
  // OK: code owner has no control over system header.
  int* USR_CONST ptr3;
  // OK: code owner has no control over system header.
  int* USR_ATTR ptr4;
  // Error: user has control over the macro.
  USR_INTP_FIELD();
  // OK: code owner has no control over system header.
  USR_TYPE_WITH_SUFFIX(SysTyp) * ptr5;
  // OK: code owner has no control over system header.
  int* USR_SYMBOL(ptr6);
  // OK: the source location for this field declaration will be "<scratch
  // space>" and the real file path cannot be detected.
  int* USR_SYMBOL_WITH_SUFFIX(ptr7);
  // OK: code owner has no control over system header.
  int* ptr8 USR_EQ nullptr;
  // OK: code owner has no control over system header.
  int* ptr9 = USR_NULLPTR;
  // OK: the source location for this field declaration will be "<scratch
  // space>" and the real file path cannot be detected.
  int* USR_SYMBOL_WITH_SUFFIX(ptr10) = nullptr;
};

// Same as for SysStructWithSysMacro.
struct SysStructWithThirdPartyMacro {
  int* ptr0;                                    // OK.
  TP_INT* ptr1;                                 // OK.
  TP_INTP ptr2;                                 // OK.
  int* TP_CONST ptr3;                           // OK.
  int* TP_ATTR ptr4;                            // OK.
  TP_INTP_FIELD();                              // OK.
  TP_TYPE_WITH_SUFFIX(SysTyp) * ptr5;           // OK.
  int* TP_SYMBOL(ptr6);                         // OK.
  int* TP_SYMBOL_WITH_SUFFIX(ptr7);             // OK.
  int* ptr8 TP_EQ nullptr;                      // OK.
  int* ptr9 = TP_NULLPTR;                       // OK.
  int* TP_SYMBOL_WITH_SUFFIX(ptr10) = nullptr;  // OK.
};

// Same as for SysStructWithSysMacro.
struct SysStructWithManuallyIgnoredMacro {
  int* ptr0;                                    // OK.
  IG_INT* ptr1;                                 // OK.
  IG_INTP ptr2;                                 // OK.
  int* IG_CONST ptr3;                           // OK.
  int* IG_ATTR ptr4;                            // OK.
  IG_INTP_FIELD();                              // OK.
  IG_TYPE_WITH_SUFFIX(SysTyp) * ptr5;           // OK.
  int* IG_SYMBOL(ptr6);                         // OK.
  int* IG_SYMBOL_WITH_SUFFIX(ptr7);             // OK.
  int* ptr8 IG_EQ nullptr;                      // OK.
  int* ptr9 = IG_NULLPTR;                       // OK.
  int* IG_SYMBOL_WITH_SUFFIX(ptr10) = nullptr;  // OK.
};

// Same as for SysStructWithSysMacro.
struct SysStructWithGeneratedMacro {
  int* ptr0;                                     // OK.
  GEN_INT* ptr1;                                 // OK.
  GEN_INTP ptr2;                                 // OK.
  int* GEN_CONST ptr3;                           // OK.
  int* GEN_ATTR ptr4;                            // OK.
  GEN_INTP_FIELD();                              // OK.
  GEN_TYPE_WITH_SUFFIX(SysTyp) * ptr5;           // OK.
  int* GEN_SYMBOL(ptr6);                         // OK.
  int* GEN_SYMBOL_WITH_SUFFIX(ptr7);             // OK.
  int* ptr8 GEN_EQ nullptr;                      // OK.
  int* ptr9 = GEN_NULLPTR;                       // OK.
  int* GEN_SYMBOL_WITH_SUFFIX(ptr10) = nullptr;  // OK.
};

#endif  // TOOLS_CLANG_PLUGINS_TESTS_SYSTEM_RAW_PTR_SYSTEM_TEST_H_

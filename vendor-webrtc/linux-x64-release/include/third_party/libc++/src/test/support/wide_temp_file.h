//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_WIDE_TEMP_FILE_H
#define TEST_SUPPORT_WIDE_TEMP_FILE_H

#include <codecvt>
#include <locale>
#include <string>

#include "platform_support.h"
#include "test_macros.h"

TEST_DIAGNOSTIC_PUSH
TEST_CLANG_DIAGNOSTIC_IGNORED("-Wdeprecated-declarations")
TEST_GCC_DIAGNOSTIC_IGNORED("-Wdeprecated-declarations")
inline std::wstring get_wide_temp_file_name() {
    return std::wstring_convert<std::codecvt_utf8_utf16<wchar_t> >().from_bytes(get_temp_file_name());
}
TEST_DIAGNOSTIC_POP

#endif // TEST_SUPPORT_WIDE_TEMP_FILE_H

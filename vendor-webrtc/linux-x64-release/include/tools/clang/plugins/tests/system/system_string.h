// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_SYSTEM_SYSTEM_STRING_H_
#define TOOLS_CLANG_PLUGINS_TESTS_SYSTEM_SYSTEM_STRING_H_

// Mock string.h for plugin tests since some bots can't find the
// actual one. Avoids use of size_t since there's no reason to
// believe the bots could find stddef.h, either.
void* memcpy(void* dst, const void* src, __SIZE_TYPE__ size);

#endif  // TOOLS_CLANG_PLUGINS_TESTS_SYSTEM_SYSTEM_STRING_H_

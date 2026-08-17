// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef TOOLS_CLANG_SPANIFY_TESTS_HEADER_ORIGINAL_H_
#define TOOLS_CLANG_SPANIFY_TESTS_HEADER_ORIGINAL_H_

// The goal of this file is to test the plugin's ability to rewrite raw pointers
// when the declaration and implementation are in different files, and

// Declared in first party, implemented in first party.
// ProcessBuffer1 is:
// - Declared in first party
// - Implemented in first party
void ProcessBuffer1(int* buffer, int size);

// ProcessBuffer2 is:
// - Declared in third_party
// - Implemented in third_party

// ProcessBuffer3 is:
// - Declared in first party
// - Implemented in third_party
void ProcessBuffer3(int* buffer, int size);

// ProcessBuffer4 is:
// - Declared in third_party
// - Implemented in first party

#endif  // TOOLS_CLANG_SPANIFY_TESTS_HEADER_ORIGINAL_H_

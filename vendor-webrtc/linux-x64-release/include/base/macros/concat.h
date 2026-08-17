// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MACROS_CONCAT_H_
#define BASE_MACROS_CONCAT_H_

// A macro that expands to the concatenation of its arguments. If the arguments
// are themselves macros, they are first expanded (due to the indirection
// through a second macro). This can be used to construct tokens.
#define BASE_CONCAT(a, b) BASE_INTERNAL_CONCAT(a, b)

// Implementation details: do not use directly.
#define BASE_INTERNAL_CONCAT(a, b) a##b

#endif  // BASE_MACROS_CONCAT_H_

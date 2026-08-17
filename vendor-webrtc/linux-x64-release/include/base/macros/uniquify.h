// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MACROS_UNIQUIFY_H_
#define BASE_MACROS_UNIQUIFY_H_

#include "base/macros/concat.h"

// A macro to create a "unique" token name beginning with `name` by appending
// the current line number. This is generally used inside other macros that need
// to create temporaries while minimizing the likelihood of name conflicts.
#define BASE_UNIQUIFY(name) BASE_CONCAT(name, __LINE__)

#endif  // BASE_MACROS_UNIQUIFY_H_

// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MACROS_IS_EMPTY_H_
#define BASE_MACROS_IS_EMPTY_H_

// A macro that substitutes with 1 if called without arguments, otherwise 0.
#define BASE_IS_EMPTY(...) BASE_INTERNAL_IS_##__VA_OPT__(NON_)##EMPTY

#define BASE_INTERNAL_IS_EMPTY 1
#define BASE_INTERNAL_IS_NON_EMPTY 0

#endif  // BASE_MACROS_IS_EMPTY_H_

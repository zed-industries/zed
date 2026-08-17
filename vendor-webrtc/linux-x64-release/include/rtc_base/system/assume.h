/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SYSTEM_ASSUME_H_
#define RTC_BASE_SYSTEM_ASSUME_H_

// Possibly evaluate `p`, promising the compiler that the result is true; the
// compiler is allowed (but not required) to use this information when
// optimizing the code. USE WITH CAUTION! If you promise the compiler things
// that aren't true, it will build a broken binary for you.
//
// As a simple example, the compiler is allowed to transform this
//
//   RTC_ASSUME(x == 4);
//   return x;
//
// into this
//
//   return 4;
//
// It is even allowed to propagate the assumption "backwards in time", if it can
// prove that it must have held at some earlier time. For example, the compiler
// is allowed to transform this
//
//   int Add(int x, int y) {
//     if (x == 17)
//       y += 1;
//     RTC_ASSUME(x != 17);
//     return x + y;
//   }
//
// into this
//
//   int Add(int x, int y) {
//     return x + y;
//   }
//
// since if `x` isn't 17 on the third line of the function body, the test of `x
// == 17` on the first line must fail since nothing can modify the local
// variable `x` in between.
//
// The intended use is to allow the compiler to optimize better. For example,
// here we allow the compiler to omit an instruction that ensures correct
// rounding of negative arguments:
//
//   int DivBy2(int x) {
//     RTC_ASSUME(x >= 0);
//     return x / 2;
//   }
//
// and here we allow the compiler to possibly omit a null check:
//
//   void Delete(int* p) {
//     RTC_ASSUME(p != nullptr);
//     delete p;
//   }
//
// clang-format off
#if defined(__GNUC__)
#define RTC_ASSUME(p) do { if (!(p)) __builtin_unreachable(); } while (0)
#else
#define RTC_ASSUME(p) do {} while (0)
#endif
// clang-format on

#endif  // RTC_BASE_SYSTEM_ASSUME_H_

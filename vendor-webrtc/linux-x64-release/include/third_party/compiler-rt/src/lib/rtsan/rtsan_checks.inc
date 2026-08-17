//===-- rtsan_checks.inc ----------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// List of suppression checks handled by RTSan runtime.
//
//===----------------------------------------------------------------------===//
#ifndef RTSAN_CHECK
#error "Define RTSAN_CHECK prior to including this file!"
#endif

// RTSAN_CHECK(Name, SummaryKind)
// SummaryKind should be a string literal.

RTSAN_CHECK(CallStackContains, "call-stack-contains")
RTSAN_CHECK(FunctionNameMatches, "function-name-matches")

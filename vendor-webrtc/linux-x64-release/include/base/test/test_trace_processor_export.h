// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_TRACE_PROCESSOR_EXPORT_H_
#define BASE_TEST_TEST_TRACE_PROCESSOR_EXPORT_H_

#if defined(WIN32)

#if defined(TEST_TRACE_PROCESSOR_IMPL)
#define TEST_TRACE_PROCESSOR_EXPORT __declspec(dllexport)
#else
#define TEST_TRACE_PROCESSOR_EXPORT __declspec(dllimport)
#endif  // defined(TEST_TRACE_PROCESSOR_IMPL)

#else  // defined(WIN32)

#if defined(TEST_TRACE_PROCESSOR_IMPL)
#define TEST_TRACE_PROCESSOR_EXPORT __attribute__((visibility("default")))
#else
#define TEST_TRACE_PROCESSOR_EXPORT
#endif  // defined(TEST_TRACE_PROCESSOR_IMPL)

#endif  // defined(WIN32)

#endif  // BASE_TEST_TEST_TRACE_PROCESSOR_EXPORT_H_

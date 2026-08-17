// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PRIVATE_JOIN_AND_COMPUTE_BASE_PRIVATE_JOIN_AND_COMPUTE_EXPORT_H_
#define PRIVATE_JOIN_AND_COMPUTE_BASE_PRIVATE_JOIN_AND_COMPUTE_EXPORT_H_

// PRIVATE_COMPUTE_EXPORT is used to mark symbols as imported or
// exported when private-join-and-compute is built or used as a
// shared library.
//
// When private-join-and-compute is built as a static library the
// PRIVATE_COMPUTE_EXPORT macro expands to nothing.

#ifdef PRIVATE_COMPUTE_ENABLE_SYMBOL_EXPORT

#ifdef PRIVATE_COMPUTE_WIN

#ifdef IS_PRIVATE_COMPUTE_LIBRARY_IMPL
#define PRIVATE_COMPUTE_EXPORT __declspec(dllexport)
#else
#define PRIVATE_COMPUTE_EXPORT __declspec(dllimport)
#endif

#else  // PRIVATE_COMPUTE_WIN

#if __has_attribute(visibility) && defined(IS_PRIVATE_COMPUTE_LIBRARY_IMPL)
#define PRIVATE_COMPUTE_EXPORT __attribute__((visibility("default")))
#endif

#endif  // PRIVATE_COMPUTE_WIN

#endif  // PRIVATE_COMPUTE_ENABLE_SYMBOL_EXPORT

#ifndef PRIVATE_COMPUTE_EXPORT
#define PRIVATE_COMPUTE_EXPORT
#endif

#endif  // PRIVATE_JOIN_AND_COMPUTE_BASE_PRIVATE_JOIN_AND_COMPUTE_EXPORT_H_

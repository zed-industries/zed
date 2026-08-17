// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_SQLITE_SQLITE3_SHIM_FIXUPS_H_
#define THIRD_PARTY_SQLITE_SQLITE3_SHIM_FIXUPS_H_

// This file contains various fixups for the amalgamated SQLite code.
// It is intended to be included in sqlite3_shim.c only.

#if defined(SQLITE_OMIT_COMPILEOPTION_DIAGS)
// When SQLITE_OMIT_COMPILEOPTION_DIAGS is defined, sqlite3.h emits macros
// instead of declarations for sqlite3_compileoption_{get,used}().
//
// In order to avoid a macro redefinition warning, we must undo the #define in
// rename_exports.h.
#if defined(sqlite3_compileoption_get)
#undef sqlite3_compileoption_get
#else
#error "This workaround is no longer needed."
#endif  // !defined(sqlite3_compileoption_get)
#if defined(sqlite3_compileoption_used)
#undef sqlite3_compileoption_used
#else
#error "This workaround is no longer needed."
#endif  // !defined(sqlite3_compileoption_used)
#endif  // defined(SQLITE_OMIT_COMPILEOPTION_DIAGS)

// Linux-specific configuration fixups.
#if defined(__linux__)

// features.h, included below, indirectly includes sys/mman.h. The latter header
// only defines mremap if _GNU_SOURCE is defined. Depending on the order of the
// files in the amalgamation, removing the define below may result in a build
// error on Linux.
#if defined(__GNUC__) && !defined(_GNU_SOURCE)
#define _GNU_SOURCE
#endif
#include <features.h>

// SQLite wants to track malloc sizes. On OSX it uses malloc_size(), on Windows
// _msize(), elsewhere it handles it manually by enlarging the malloc and
// injecting a field. Enable malloc_usable_size() for Linux.
//
// malloc_usable_size() is not exported by the Android NDK. It is not
// implemented by uclibc.
#if !defined(__UCLIBC__) && !defined(__ANDROID__)
#define HAVE_MALLOC_H 1
#define HAVE_MALLOC_USABLE_SIZE 1
#endif

#endif  // defined(__linux__)

// For unfortunately complex reasons, Chrome has release builds where
// DCHECK_IS_ON() (so we want SQLITE_DEBUG to be on) but NDEBUG is also defined.
// This causes declarations for mutex-checking functions used by SQLITE_DEBUG
// code (sqlite3_mutex_held, sqlite3_mutex_notheld) to be omitted, resulting in
// warnings.
//
// The easiest solution for now is to undefine NDEBUG when SQLITE_DEBUG is
// defined. The #undef only takes effect for the SQLite implementation (included
// below), and does not impact any dependency.
#if defined(SQLITE_DEBUG) && defined(NDEBUG)
#undef NDEBUG
#endif  // defined(SQLITE_DEBUG) && defined(NDEBUG)

#endif  // THIRD_PARTY_SQLITE_SQLITE3_SHIM_FIXUPS_H_

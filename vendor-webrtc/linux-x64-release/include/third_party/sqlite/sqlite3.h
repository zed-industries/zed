// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_SQLITE_SQLITE3_H_
#define THIRD_PARTY_SQLITE_SQLITE3_H_

// This is a shim header to include the right sqlite3 headers.
// Use this instead of referencing sqlite3 headers directly.

// We prefix chrome_ to SQLite's exported symbols, so that we don't clash with
// other SQLite libraries loaded by the system libraries. This only matters when
// using the component build, where our SQLite's symbols are visible to the
// dynamic library loader.
#include "third_party/sqlite/src/amalgamation/rename_exports.h"  // IWYU pragma: export

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

#include "third_party/sqlite/src/amalgamation/sqlite3.h"  // IWYU pragma: export

#endif  // THIRD_PARTY_SQLITE_SQLITE3_H_

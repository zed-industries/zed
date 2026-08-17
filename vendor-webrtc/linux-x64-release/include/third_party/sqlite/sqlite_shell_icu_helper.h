// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_SQLITE_SQLITE_SHELL_ICU_HELPER_H_
#define THIRD_PARTY_SQLITE_SQLITE_SHELL_ICU_HELPER_H_

#ifdef __cplusplus
extern "C" {
#endif

// Exposes base::i18n::InitializeICU() to the SQLite shell.
//
// Chrome's startup sequence calls base::i18n::InitializeICU(). This function
// exposes the same logic to SQLite's shell, so Chrome developers can debug
// SQLite with the ICU tables used in production.
void InitializeICUForSqliteShell();

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // THIRD_PARTY_SQLITE_SQLITE_SHELL_ICU_HELPER_H_

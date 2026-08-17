// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SHELL_ENCRYPTION_BASE_SHELL_ENCRYPTION_EXPORT_H_
#define SHELL_ENCRYPTION_BASE_SHELL_ENCRYPTION_EXPORT_H_

// SHELL_ENCRYPTION_EXPORT is used to mark symbols as imported or
// exported when shell-encryption is built or used as a shared library.
// When shell-encryption is built as a static library the
// SHELL_ENCRYPTION_EXPORT macro expands to nothing.
//
// This export macros doesn't support Windows. There will be additional
// component build work to support Windows (see crbug.com/1269714).

#ifdef SHELL_ENCRYPTION_ENABLE_SYMBOL_EXPORT

#if __has_attribute(visibility)
#define SHELL_ENCRYPTION_EXPORT __attribute__((visibility("default")))
#endif

#endif  // SHELL_ENCRYPTION_ENABLE_SYMBOL_EXPORT

#ifndef SHELL_ENCRYPTION_EXPORT
#define SHELL_ENCRYPTION_EXPORT
#endif

#endif  // SHELL_ENCRYPTION_BASE_SHELL_ENCRYPTION_EXPORT_H_

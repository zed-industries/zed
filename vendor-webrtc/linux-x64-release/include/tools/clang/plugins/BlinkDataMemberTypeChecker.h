// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_BLINKDATAMEMBERTYPECHECKER_H_
#define TOOLS_CLANG_PLUGINS_BLINKDATAMEMBERTYPECHECKER_H_

#include "clang/AST/DeclCXX.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Frontend/CompilerInstance.h"
#include "llvm/ADT/StringSet.h"
#include "llvm/Support/Regex.h"

namespace chrome_checker {

class BlinkDataMemberTypeChecker {
 public:
  explicit BlinkDataMemberTypeChecker(clang::CompilerInstance& instance);

  void CheckClass(clang::SourceLocation location,
                  const clang::CXXRecordDecl* record);

 private:
  bool AllowsDiscouragedType(const clang::Decl* decl);
  void CheckField(const clang::FieldDecl* field);

  clang::CompilerInstance& instance_;
  clang::DiagnosticsEngine& diagnostic_;
  unsigned diag_disallowed_blink_data_member_type_;

  // Each entry maps from the qualified name of a discouraged type to a string
  // containing the alternative suggestions.
  llvm::StringMap<const char*> discouraged_types_;

  llvm::Regex included_filenames_regex_;
  llvm::Regex excluded_filenames_regex_;
};

}  // namespace chrome_checker

#endif  // TOOLS_CLANG_PLUGINS_BLINKDATAMEMBERTYPECHECKER_H_

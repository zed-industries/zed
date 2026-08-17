// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This check ensures that 32/64-bit unstable types are not used in IPC.
//
// A type (or typedef) is unstable if it changes size between 32/ 64-bit
// platforms. However, it's impossible to accurately identify unstable
// typedefs, because their definitions rely on the preprocessor. For
// example uintptr_t is either unsigned int or unsigned long.
//
// So we're not trying to be accurate, and just blocklisting some types
// that are known to be unstable:
// 1. Types: long / unsigned long (but not typedefs to)
// 2. Typedefs: intmax_t, uintmax_t, intptr_t, uintptr_t, wint_t,
//    size_t, rsize_t, ssize_t, ptrdiff_t, dev_t, off_t, clock_t,
//    time_t, suseconds_t (including typedefs to)
//
// Additionally, templates referencing blocklisted types (e.g. vector<long>)
// are also blocklisted.
//
// Blacklisted types are checked in:
// 1. IPC::WriteParam() calls
// 2. IPC::CheckedTuple<> specializations
//

#ifndef TOOLS_CLANG_PLUGINS_CHECKIPC_VISITOR_H_
#define TOOLS_CLANG_PLUGINS_CHECKIPC_VISITOR_H_

#include <vector>

#include "clang/AST/AST.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Frontend/CompilerInstance.h"
#include "llvm/ADT/StringSet.h"

namespace chrome_checker {

class CheckIPCVisitor {
 public:
  explicit CheckIPCVisitor(clang::CompilerInstance& compiler);

  void set_context(clang::ASTContext* context) { context_ = context; }

  void BeginDecl(clang::Decl* decl);
  void EndDecl();
  void VisitTemplateSpecializationType(
      clang::TemplateSpecializationType* spec);
  void VisitCallExpr(clang::CallExpr* call_expr);

 private:
  // ValidateXXX functions return false if validation failed and diagnostic
  // was reported. They return true otherwise (not applicable / validation
  // succeeded).

  bool ValidateWriteParam(const clang::CallExpr* call_expr);
  bool ValidateWriteParamSignature(const clang::CallExpr* call_expr);
  bool ValidateWriteParamArgument(const clang::Expr* arg_expr);
  bool ValidateCheckedTuple(
      const clang::TemplateSpecializationType* spec);

  template <typename T>
  const T* GetParentDecl() const;

  bool IsBlacklistedType(clang::QualType type) const;
  bool IsBlacklistedTypedef(const clang::TypedefNameDecl* tdef) const;

  struct CheckDetails {
    clang::QualType entry_type;
    clang::QualType exit_type;
    llvm::SmallVector<const clang::TypedefType*, 5> typedefs;
  };

  bool CheckType(clang::QualType type, CheckDetails* details) const;
  bool CheckIntegerType(clang::QualType type, CheckDetails* details) const;
  bool CheckTemplateArgument(const clang::TemplateArgument& arg,
                             CheckDetails* details) const;

  void ReportCheckError(const CheckDetails& details,
                        clang::SourceLocation loc,
                        unsigned error);

  clang::CompilerInstance& compiler_;
  clang::ASTContext* context_;

  unsigned error_write_param_bad_type_;
  unsigned error_tuple_bad_type_;
  unsigned error_write_param_bad_signature_;
  unsigned note_see_here_;

  std::vector<const clang::Decl*> decl_stack_;

  llvm::StringSet<> blocklisted_typedefs_;
};

}  // namespace chrome_checker

#endif  // TOOLS_CLANG_PLUGINS_CHECKIPC_VISITOR_H_

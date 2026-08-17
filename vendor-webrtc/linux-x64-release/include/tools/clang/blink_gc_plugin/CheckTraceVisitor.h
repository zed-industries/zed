// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_CHECK_TRACE_VISITOR_H_
#define TOOLS_BLINK_GC_PLUGIN_CHECK_TRACE_VISITOR_H_

#include <string>

#include "RecordInfo.h"
#include "clang/AST/AST.h"
#include "clang/AST/RecursiveASTVisitor.h"

class RecordCache;
class RecordInfo;

// This visitor checks a tracing method by traversing its body.
// - A member field is considered traced if it is referenced in the body.
// - A base is traced if a base-qualified call to a trace method is found.
class CheckTraceVisitor : public clang::RecursiveASTVisitor<CheckTraceVisitor> {
 public:
  CheckTraceVisitor(clang::CXXMethodDecl* trace,
                    RecordInfo* info,
                    RecordCache* cache);

  bool VisitMemberExpr(clang::MemberExpr* member);
  bool VisitCallExpr(clang::CallExpr* call);
  bool VisitStmt(clang::Stmt* stmt);

 private:
  bool IsTraceCallName(const std::string& name);

  clang::CXXRecordDecl* GetDependentTemplatedDecl(
      clang::DependentScopeDeclRefExpr* expr);

  void CheckDependentScopeDeclRefExpr(clang::CallExpr* call,
                                      clang::DependentScopeDeclRefExpr* expr);
  bool CheckTraceBaseCall(clang::CallExpr* call);
  bool CheckTraceFieldMemberCall(clang::CXXMemberCallExpr* call);
  bool CheckTraceFieldCall(const std::string& name,
                           clang::CXXRecordDecl* callee,
                           clang::Expr* arg1,
                           clang::Expr* arg2);
  bool CheckRegisterWeakMembers(clang::CXXMemberCallExpr* call);
  bool CheckImplicitCastExpr(clang::CallExpr* call,
                             clang::ImplicitCastExpr* expr);

  bool IsWeakCallback() const;

  void MarkTraced(RecordInfo::Fields::iterator it);
  void MarkTracedIfNeeded(RecordInfo::Fields::iterator it);
  void FoundField(clang::FieldDecl* field, bool is_trace_if_needed);
  void MarkAllWeakMembersTraced();

  clang::CXXMethodDecl* trace_;
  RecordInfo* info_;
  RecordCache* cache_;
};

#endif  // TOOLS_BLINK_GC_PLUGIN_CHECK_TRACE_VISITOR_H_

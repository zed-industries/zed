// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_CHECK_FINALIZER_VISITOR_H_
#define TOOLS_BLINK_GC_PLUGIN_CHECK_FINALIZER_VISITOR_H_

#include <set>
#include <vector>

#include "Edge.h"
#include "RecordInfo.h"
#include "clang/AST/RecursiveASTVisitor.h"

// This visitor checks that a finalizer method does not have invalid access to
// fields that are potentially finalized. A potentially finalized field is
// either a Member, a heap-allocated collection or an off-heap collection that
// contains Members.  Invalid uses are currently identified as passing the field
// as the argument of a procedure call or using the -> or [] operators on it.
class CheckFinalizerVisitor
    : public clang::RecursiveASTVisitor<CheckFinalizerVisitor> {
 public:
  struct Error {
    Error(clang::MemberExpr* member,
          FieldPoint* field)
        : member(member),
          field(field) {}

    clang::MemberExpr* member;
    FieldPoint* field;
  };

  typedef std::vector<Error> Errors;

  explicit CheckFinalizerVisitor(RecordCache* cache);

  Errors& finalized_fields();

  bool WalkUpFromCXXOperatorCallExpr(clang::CXXOperatorCallExpr* expr);
  bool WalkUpFromCallExpr(clang::CallExpr* expr);

  bool VisitMemberExpr(clang::MemberExpr* member);

 private:
  bool MightBeCollected(FieldPoint* point);

  bool blocklist_context_;
  Errors finalized_fields_;
  std::set<clang::MemberExpr*> seen_members_;
  RecordCache* cache_;
};

#endif  // TOOLS_BLINK_GC_PLUGIN_CHECK_FINALIZER_VISITOR_H_

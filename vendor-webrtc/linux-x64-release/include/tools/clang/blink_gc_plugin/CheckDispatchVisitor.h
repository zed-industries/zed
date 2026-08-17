// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_CHECK_DISPATCH_VISITOR_H_
#define TOOLS_BLINK_GC_PLUGIN_CHECK_DISPATCH_VISITOR_H_

#include "clang/AST/RecursiveASTVisitor.h"

class RecordInfo;

// This visitor checks that a method contains within its body, a call to a
// method on the provided receiver class. This is used to check manual
// dispatching for trace and finalize methods.
class CheckDispatchVisitor
    : public clang::RecursiveASTVisitor<CheckDispatchVisitor> {
 public:
  explicit CheckDispatchVisitor(RecordInfo* receiver);

  bool dispatched_to_receiver();

  bool VisitMemberExpr(clang::MemberExpr* member);
  bool VisitUnresolvedMemberExpr(clang::UnresolvedMemberExpr* member);

 private:
  RecordInfo* receiver_;
  bool dispatched_to_receiver_;
};

#endif  // TOOLS_BLINK_GC_PLUGIN_CHECK_DISPATCH_VISITOR_H_

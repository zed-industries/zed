// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_COLLECT_VISITOR_H_
#define TOOLS_BLINK_GC_PLUGIN_COLLECT_VISITOR_H_

#include <vector>

#include "clang/AST/AST.h"
#include "clang/AST/RecursiveASTVisitor.h"

// This visitor collects the entry points for the checker.
class CollectVisitor : public clang::RecursiveASTVisitor<CollectVisitor> {
 public:
  typedef std::vector<clang::CXXRecordDecl*> RecordVector;
  typedef std::vector<clang::CXXMethodDecl*> MethodVector;

  CollectVisitor();

  RecordVector& record_decls();
  MethodVector& trace_decls();

  // Collect record declarations, including nested declarations.
  bool VisitCXXRecordDecl(clang::CXXRecordDecl* record);

  // Collect tracing method definitions, but don't traverse method bodies.
  bool VisitCXXMethodDecl(clang::CXXMethodDecl* method);

 private:
  RecordVector record_decls_;
  MethodVector trace_decls_;
};

#endif  // TOOLS_BLINK_GC_PLUGIN_COLLECT_VISITOR_H_

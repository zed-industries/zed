// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This check ensures that every LayoutObject method begins with
// CheckIsNotDestroyed() so that LayoutObject instances are not accessed after
// they gets destroyed.

#ifndef TOOLS_CLANG_PLUGINS_CHECKLAYOUTOBJECTMETHODSVISITOR_H_
#define TOOLS_CLANG_PLUGINS_CHECKLAYOUTOBJECTMETHODSVISITOR_H_

#include "clang/AST/ASTConsumer.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Frontend/CompilerInstance.h"

// TODO: Consider moving this checker into a blink plugin when we have it.
namespace chrome_checker {

class CheckLayoutObjectMethodsVisitor {
 public:
  explicit CheckLayoutObjectMethodsVisitor(clang::CompilerInstance& compiler);

  void VisitLayoutObjectMethods(clang::ASTContext& context);

 private:
  static std::string layout_directory;
  static std::string test_directory;

  clang::CompilerInstance& compiler_;
};

}  // namespace chrome_checker

#endif  // TOOLS_CLANG_PLUGINS_CHECKLAYOUTOBJECTMETHODSVISITOR_H_

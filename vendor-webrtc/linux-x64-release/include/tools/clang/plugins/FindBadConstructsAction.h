// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_FINDBADCONSTRUCTIONS_ACTION_H_
#define TOOLS_CLANG_PLUGINS_FINDBADCONSTRUCTIONS_ACTION_H_

#include "clang/Frontend/FrontendAction.h"

#include "Options.h"

namespace chrome_checker {

class FindBadConstructsAction : public clang::PluginASTAction {
 public:
  FindBadConstructsAction();

 protected:
  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance& instance,
      llvm::StringRef ref) override;
  PluginASTAction::ActionType getActionType() override {
    return CmdlineBeforeMainAction;
  }
  bool ParseArgs(const clang::CompilerInstance& instance,
                 const std::vector<std::string>& args) override;

 private:
  Options options_;
};

}  // namespace chrome_checker

#endif  // TOOLS_CLANG_PLUGINS_FINDBADCONSTRUCTIONS_ACTION_H_

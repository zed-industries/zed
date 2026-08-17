// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_RAW_PTR_PLUGIN_RAWPTRPLUGIN_H_
#define TOOLS_CLANG_RAW_PTR_PLUGIN_RAWPTRPLUGIN_H_

#include "Options.h"
#include "clang/Frontend/FrontendAction.h"

namespace raw_ptr_plugin {

class RawPtrPlugin : public clang::PluginASTAction {
 public:
  RawPtrPlugin();

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

}  // namespace raw_ptr_plugin

#endif  // TOOLS_CLANG_RAW_PTR_PLUGIN_RAWPTRPLUGIN_H_

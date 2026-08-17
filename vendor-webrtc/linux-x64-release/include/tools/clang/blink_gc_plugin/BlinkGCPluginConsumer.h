// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_BLINK_GC_PLUGIN_CONSUMER_H_
#define TOOLS_BLINK_GC_PLUGIN_BLINK_GC_PLUGIN_CONSUMER_H_

#include <string>

#include "BlinkGCPluginOptions.h"
#include "Config.h"
#include "DiagnosticsReporter.h"
#include "clang/AST/AST.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Frontend/CompilerInstance.h"

class JsonWriter;
class RecordInfo;

// Main class containing checks for various invariants of the Blink
// garbage collection infrastructure.
class BlinkGCPluginConsumer : public clang::ASTConsumer {
 public:
  BlinkGCPluginConsumer(clang::CompilerInstance& instance,
                        const BlinkGCPluginOptions& options);

  void HandleTranslationUnit(clang::ASTContext& context) override;

 private:
  void ParseFunctionTemplates(clang::TranslationUnitDecl* decl);

  // Main entry for checking a record declaration.
  void CheckRecord(RecordInfo* info);

  // Check a class-like object (eg, class, specialization, instantiation).
  void CheckClass(RecordInfo* info);

  clang::CXXRecordDecl* GetDependentTemplatedDecl(const clang::Type& type);

  void CheckPolymorphicClass(RecordInfo* info, clang::CXXMethodDecl* trace);

  clang::CXXRecordDecl* GetLeftMostBase(clang::CXXRecordDecl* left_most);

  bool DeclaresVirtualMethods(clang::CXXRecordDecl* decl);

  void CheckLeftMostDerived(RecordInfo* info);

  void CheckDispatch(RecordInfo* info);

  void CheckFinalization(RecordInfo* info);

  // This is the main entry for tracing method definitions.
  void CheckTracingMethod(clang::CXXMethodDecl* method);

  // Determine what type of tracing method this is (dispatch or trace).
  void CheckTraceOrDispatchMethod(RecordInfo* parent,
                                  clang::CXXMethodDecl* method);

  // Check an actual trace method.
  void CheckTraceMethod(RecordInfo* parent,
                        clang::CXXMethodDecl* trace,
                        Config::TraceMethodType trace_type);

  void DumpClass(RecordInfo* info);

  // Adds either a warning or error, based on the current handling of -Werror.
  clang::DiagnosticsEngine::Level getErrorLevel();

  std::string GetLocString(clang::SourceLocation loc);

  bool IsIgnored(RecordInfo* info);

  bool IsIgnoredClass(RecordInfo* info);

  bool InIgnoredDirectory(RecordInfo* info);

  bool InCheckedNamespaceOrDirectory(RecordInfo* info);

  bool GetFilename(clang::SourceLocation loc, std::string* filename);

  clang::CompilerInstance& instance_;
  DiagnosticsReporter reporter_;
  BlinkGCPluginOptions options_;
  RecordCache cache_;
  JsonWriter* json_;
};

#endif  // TOOLS_BLINK_GC_PLUGIN_BLINK_GC_PLUGIN_CONSUMER_H_

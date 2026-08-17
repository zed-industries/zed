// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_CHROMECLASSTESTER_H_
#define TOOLS_CLANG_PLUGINS_CHROMECLASSTESTER_H_

#include <set>
#include <vector>

#include "Options.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/TypeLoc.h"
#include "clang/Frontend/CompilerInstance.h"

// A class on top of ASTConsumer that forwards classes defined in Chromium
// headers to subclasses which implement CheckChromeClass().
// TODO(vmpstr): Fold this class into FindBadConstructsConsumer.
class ChromeClassTester {
 public:
  ChromeClassTester(clang::CompilerInstance& instance,
                    const chrome_checker::Options& options);
  virtual ~ChromeClassTester();

  void CheckTag(clang::TagDecl*);

  clang::DiagnosticsEngine::Level getErrorLevel();

 protected:
  clang::CompilerInstance& instance() { return instance_; }
  clang::DiagnosticsEngine& diagnostic() { return diagnostic_; }

  // A classification used to determine how a certain SourceLocation should be
  // handled for diagnostics. The main criteria for classification is the
  // SourceLocation's path (e.g. whether it's in //third_party).
  enum class LocationType {
    // Enforce all default checks.
    kChrome,
    // Enforces a subset of checks for Blink code. This is hopefully a
    // transitional stage, as more plugin checks are gradually enabled in Blink.
    kBlink,
    // Skip all checks. Typically, this is third-party or generated code where
    // it doesn't make sense to enforce Chrome's custom diagnostics.
    kThirdParty,
  };

  // Determines if a SourceLocation is considered part of first-party or
  // third-party code, which can be used to determine how or which diagnostics
  // should be emitted.
  //
  // NOTE: chrome_checker::ClassifySourceLocation() provides finer granularity
  // in its answer.
  LocationType ClassifyLocation(clang::SourceLocation loc);

  // Utility method to check whether the given record has any of the ignored
  // base classes.
  bool HasIgnoredBases(const clang::CXXRecordDecl* record);

  // Utility method for subclasses to check if this class is within an
  // implementation (.cc, .cpp, .mm) file.
  bool InImplementationFile(clang::SourceLocation location);

  // Options.
  const chrome_checker::Options options_;

 private:
  void BuildBannedLists();

  // Filtered versions of tags that are only called with things defined in
  // chrome header files.
  virtual void CheckChromeClass(LocationType location_type,
                                clang::SourceLocation record_location,
                                clang::CXXRecordDecl* record) = 0;

  // Utility methods used for filtering out non-chrome classes (and ones we
  // deliberately ignore) in HandleTagDeclDefinition().
  bool IsIgnoredType(std::string_view base_name);

  clang::CompilerInstance& instance_;
  clang::DiagnosticsEngine& diagnostic_;

  // List of types that we don't check.
  std::set<std::string_view> ignored_record_names_;

  // List of base classes that we skip when checking complex class ctors/dtors.
  std::set<std::string_view> ignored_base_classes_;
};

#endif  // TOOLS_CLANG_PLUGINS_CHROMECLASSTESTER_H_

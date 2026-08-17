// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_RAW_PTR_PLUGIN_STACKALLOCATEDCHECKER_H_
#define TOOLS_CLANG_RAW_PTR_PLUGIN_STACKALLOCATEDCHECKER_H_

#include <map>

namespace clang {
class CompilerInstance;
class CXXRecordDecl;
class FieldDecl;
}  // namespace clang

namespace raw_ptr_plugin {

// This determines a record (class/struct) is annotated with
// |STACK_ALLOCATED()|. Even if it is explicitly annotated with
// |STACK_ALLOCATED()|, this will consider it as "stack allocated" when its
// ancestor has the annotation. Similarly, classes with a "stack allocated"
// template type parameter is considered "stack allocated".
class StackAllocatedPredicate {
 public:
  bool IsStackAllocated(const clang::CXXRecordDecl* record) const;

 private:
  mutable std::map<const clang::CXXRecordDecl*, bool> cache_;
};

// This verifies usage of classes annotated with STACK_ALLOCATED().
// Specifically, it ensures that an instance of such a class cannot be used as a
// member variable in a non-STACK_ALLOCATED() class.
class StackAllocatedChecker {
 public:
  explicit StackAllocatedChecker(clang::CompilerInstance& compiler);

  void Check(clang::CXXRecordDecl* record);

 private:
  clang::CompilerInstance& compiler_;
  unsigned stack_allocated_field_error_signature_;
  StackAllocatedPredicate predicate_;
};

}  // namespace raw_ptr_plugin

#endif  // TOOLS_CLANG_RAW_PTR_PLUGIN_STACKALLOCATEDCHECKER_H_

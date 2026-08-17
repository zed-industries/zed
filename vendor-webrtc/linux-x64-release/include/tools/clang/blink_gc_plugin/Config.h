// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file defines the names used by GC infrastructure.

// TODO: Restructure the name determination to use fully qualified names (ala,
// blink::Foo) so that the plugin can be enabled for all of chromium. Doing so
// would allow us to catch errors with structures outside of blink that might
// have unsafe pointers to GC allocated blink structures.

#ifndef TOOLS_BLINK_GC_PLUGIN_CONFIG_H_
#define TOOLS_BLINK_GC_PLUGIN_CONFIG_H_

#include <cassert>

#include "RecordInfo.h"
#include "clang/AST/AST.h"
#include "clang/AST/Attr.h"

extern const char kNewOperatorName[];
extern const char kCreateName[];
extern const char kTraceName[];
extern const char kTraceMultipleName[];
extern const char kTraceEphemeronName[];
extern const char kFinalizeName[];
extern const char kTraceAfterDispatchName[];
extern const char kRegisterWeakMembersName[];
extern const char kHeapAllocatorName[];
extern const char kTraceIfNeededName[];
extern const char kVisitorDispatcherName[];
extern const char kVisitorVarName[];
extern const char kConstIteratorName[];
extern const char kIteratorName[];
extern const char kConstReverseIteratorName[];
extern const char kReverseIteratorName[];

class Config {
 private:
  // Checks that the namespace matches the expected namespace and that the type
  // takes at least |expected_minimum_arg_count| template arguments. If both
  // requirements are fulfilled, populates |args| with the first
  // |expected_minimum_arg_count| template arguments. Verifying only the minimum
  // expected argument keeps the plugin resistant to changes in the type
  // definitions (to some extent)
  static bool VerifyNamespaceAndArgCount(std::string expected_ns_name,
                                         int expected_minimum_arg_count,
                                         llvm::StringRef ns_name,
                                         RecordInfo* info,
                                         RecordInfo::TemplateArgs* args) {
    return (ns_name == expected_ns_name) &&
           info->GetTemplateArgs(expected_minimum_arg_count, args);
  }

 public:
  static bool IsMember(llvm::StringRef name,
                       llvm::StringRef ns_name,
                       RecordInfo* info,
                       RecordInfo::TemplateArgs* args) {
    if (name == "BasicMember") {
      if (!VerifyNamespaceAndArgCount("cppgc", 2, ns_name, info, args))
        return false;
      return (*args)[1]->getAsRecordDecl()->getName() == "StrongMemberTag";
    }
    return false;
  }

  static bool IsWeakMember(llvm::StringRef name,
                           llvm::StringRef ns_name,
                           RecordInfo* info,
                           RecordInfo::TemplateArgs* args) {
    if (name == "BasicMember") {
      if (!VerifyNamespaceAndArgCount("cppgc", 2, ns_name, info, args))
        return false;
      return (*args)[1]->getAsRecordDecl()->getName() == "WeakMemberTag";
    }
    return false;
  }

  static bool IsPersistent(llvm::StringRef name,
                           llvm::StringRef ns_name,
                           RecordInfo* info,
                           RecordInfo::TemplateArgs* args) {
    if (name == "BasicPersistent") {
      return VerifyNamespaceAndArgCount("cppgc", 1, ns_name, info, args);
    }
    return false;
  }

  static bool IsCrossThreadPersistent(llvm::StringRef name,
                                      llvm::StringRef ns_name,
                                      RecordInfo* info,
                                      RecordInfo::TemplateArgs* args) {
    if (name == "BasicCrossThreadPersistent") {
      return VerifyNamespaceAndArgCount("cppgc", 1, ns_name, info, args);
    }
    return false;
  }

  static bool IsRefPtr(llvm::StringRef name) { return name == "scoped_refptr"; }

  static bool IsWeakPtr(llvm::StringRef name) { return name == "WeakPtr"; }

  static bool IsRefOrWeakPtr(llvm::StringRef name) {
    return IsRefPtr(name) || IsWeakPtr(name);
  }

  static bool IsUniquePtr(llvm::StringRef name) {
    return name == "unique_ptr";
  }

  static bool IsTraceWrapperV8Reference(llvm::StringRef name,
                                        llvm::StringRef ns_name,
                                        RecordInfo* info,
                                        RecordInfo::TemplateArgs* args) {
    return name == "TracedReference" &&
           VerifyNamespaceAndArgCount("v8", 1, ns_name, info, args);
  }

  static bool IsWTFCollection(llvm::StringRef name) {
    return name == "Vector" ||
           name == "Deque" ||
           name == "HashSet" ||
           name == "LinkedHashSet" ||
           name == "HashCountedSet" ||
           name == "HashMap";
  }

  static bool IsSTDCollection(llvm::StringRef name) {
    return name == "vector" || name == "map" || name == "unordered_map" ||
           name == "set" || name == "unordered_set" || name == "array" ||
           name == "optional" || name == "variant";
  }

  static bool IsGCCollection(llvm::StringRef name) {
    return name == "HeapVector" || name == "HeapDeque" ||
           name == "HeapHashSet" || name == "HeapLinkedHashSet" ||
           name == "HeapHashCountedSet" || name == "HeapHashMap" ||
           name == "HeapLinkedStack";
  }

  static bool IsHashMap(llvm::StringRef name) {
    return name == "HashMap" || name == "HeapHashMap" || name == "map" ||
           name == "unordered_map";
  }

  // Assumes name is a valid collection name.
  static size_t CollectionDimension(llvm::StringRef name) {
    // In case we're dealing with a variant, we want to collect the whole
    // parameter pack.
    if (name == "variant") {
      return 0;
    }
    return (IsHashMap(name) || name == "pair") ? 2 : 1;
  }

  static bool IsRefCountedBase(llvm::StringRef name) {
    return name == "RefCounted" ||
           name == "ThreadSafeRefCounted";
  }

  static bool IsGCSimpleBase(llvm::StringRef name) {
    return name == "GarbageCollected";
  }

  static bool IsGCMixinBase(llvm::StringRef name) {
    return name == "GarbageCollectedMixin";
  }

  static bool IsGCBase(llvm::StringRef name) {
    return IsGCSimpleBase(name) || IsGCMixinBase(name);
  }

  static bool IsIterator(llvm::StringRef name) {
    return name == kIteratorName || name == kConstIteratorName ||
           name == kReverseIteratorName || name == kConstReverseIteratorName;
  }

  // Returns true of the base classes that do not need a vtable entry for trace
  // because they cannot possibly initiate a GC during construction.
  static bool IsSafePolymorphicBase(llvm::StringRef name) {
    return IsGCBase(name) || IsRefCountedBase(name);
  }

  static bool IsAnnotated(const clang::Decl* decl, const std::string& anno) {
    clang::AnnotateAttr* attr = decl->getAttr<clang::AnnotateAttr>();
    return attr && (attr->getAnnotation() == anno);
  }

  static bool IsIgnoreAnnotated(const clang::Decl* decl) {
    return IsAnnotated(decl, "blink_gc_plugin_ignore");
  }

  static bool IsStackAllocatedIgnoreAnnotated(const clang::Decl* decl) {
    return IsAnnotated(decl, "stack_allocated_ignore");
  }

  static bool IsVisitor(llvm::StringRef name) { return name == "Visitor"; }

  static bool IsVisitorPtrType(const clang::QualType& formal_type) {
    if (!formal_type->isPointerType())
      return false;

    clang::CXXRecordDecl* pointee_type =
        formal_type->getPointeeType()->getAsCXXRecordDecl();
    if (!pointee_type)
      return false;

    if (!IsVisitor(pointee_type->getName()))
      return false;

    return true;
  }

  static bool IsVisitorDispatcherType(const clang::QualType& formal_type) {
    if (const clang::SubstTemplateTypeParmType* subst_type =
            clang::dyn_cast<clang::SubstTemplateTypeParmType>(
                formal_type.getTypePtr())) {
      if (IsVisitorPtrType(subst_type->getReplacementType())) {
        // VisitorDispatcher template parameter substituted to Visitor*.
        return true;
      }
    } else if (const clang::TemplateTypeParmType* parm_type =
                   clang::dyn_cast<clang::TemplateTypeParmType>(
                       formal_type.getTypePtr())) {
      if (parm_type->getDecl()->getName() == kVisitorDispatcherName) {
        // Unresolved, but its parameter name is VisitorDispatcher.
        return true;
      }
    }

    return IsVisitorPtrType(formal_type);
  }

  enum TraceMethodType {
    NOT_TRACE_METHOD,
    TRACE_METHOD,
    TRACE_AFTER_DISPATCH_METHOD,
  };

  static TraceMethodType GetTraceMethodType(const clang::FunctionDecl* method) {
    if (method->getNumParams() != 1)
      return NOT_TRACE_METHOD;

    const std::string& name = method->getNameAsString();
    if (name != kTraceName && name != kTraceAfterDispatchName)
      return NOT_TRACE_METHOD;

    const clang::QualType& formal_type = method->getParamDecl(0)->getType();
    if (!IsVisitorPtrType(formal_type)) {
      return NOT_TRACE_METHOD;
    }

    if (name == kTraceName)
      return TRACE_METHOD;
    if (name == kTraceAfterDispatchName)
      return TRACE_AFTER_DISPATCH_METHOD;

    assert(false && "Should not reach here");
    return NOT_TRACE_METHOD;
  }

  static bool IsTraceMethod(const clang::FunctionDecl* method) {
    return GetTraceMethodType(method) != NOT_TRACE_METHOD;
  }

  static bool IsTraceWrappersMethod(const clang::FunctionDecl* method);

  static bool StartsWith(const std::string& str, const std::string& prefix) {
    if (prefix.size() > str.size())
      return false;
    return str.compare(0, prefix.size(), prefix) == 0;
  }

  static bool EndsWith(const std::string& str, const std::string& suffix) {
    if (suffix.size() > str.size())
      return false;
    return str.compare(str.size() - suffix.size(), suffix.size(), suffix) == 0;
  }

  // Test if a template specialization is an instantiation.
  static bool IsTemplateInstantiation(clang::CXXRecordDecl* record);
};

#endif  // TOOLS_BLINK_GC_PLUGIN_CONFIG_H_

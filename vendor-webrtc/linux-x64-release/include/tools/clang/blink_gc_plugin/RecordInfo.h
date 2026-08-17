// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides a wrapper for CXXRecordDecl that accumulates GC related
// information about a class. Accumulated information is memoized and the info
// objects are stored in a RecordCache.

#ifndef TOOLS_BLINK_GC_PLUGIN_RECORD_INFO_H_
#define TOOLS_BLINK_GC_PLUGIN_RECORD_INFO_H_

#include <map>
#include <vector>

#include "Edge.h"

#include "clang/AST/AST.h"
#include "clang/AST/CXXInheritance.h"
#include "clang/Frontend/CompilerInstance.h"

class RecordCache;

// A potentially tracable and/or lifetime affecting point in the object graph.
class GraphPoint {
 public:
  virtual ~GraphPoint() {}
  void MarkTraced() { traced_ = TracedStatus::kTraced; }
  void MarkTracedIfNeeded() { traced_ = TracedStatus::kTracedIfNeeded; }
  bool IsProperlyTraced() {
    return (traced_ != TracedStatus::kUntraced) || !NeedsTracing().IsNeeded();
  }
  bool IsInproperlyTraced() {
    return (traced_ == TracedStatus::kTraced) && NeedsTracing().IsIllegal();
  }
  virtual const TracingStatus NeedsTracing() = 0;

 private:
  enum class TracedStatus { kUntraced, kTraced, kTracedIfNeeded };
  TracedStatus traced_ = TracedStatus::kUntraced;
};

class BasePoint : public GraphPoint {
 public:
  BasePoint(const clang::CXXBaseSpecifier& spec,
            RecordInfo* info,
            const TracingStatus& status)
      : spec_(spec), info_(info), status_(status) {}
  const TracingStatus NeedsTracing() override { return status_; }
  const clang::CXXBaseSpecifier& spec() { return spec_; }
  RecordInfo* info() { return info_; }

 private:
  const clang::CXXBaseSpecifier& spec_;
  RecordInfo* info_;
  TracingStatus status_;
};

class FieldPoint : public GraphPoint {
 public:
  FieldPoint(clang::FieldDecl* field, Edge* edge)
      : field_(field), edge_(edge) {}
  const TracingStatus NeedsTracing() override {
    return edge_->NeedsTracing(Edge::kRecursive);
  }
  clang::FieldDecl* field() { return field_; }
  Edge* edge() { return edge_; }

 private:
  clang::FieldDecl* field_;
  Edge* edge_;

  friend class RecordCache;
  void deleteEdge() { delete edge_; }
};

// Wrapper class to lazily collect information about a C++ record.
class RecordInfo {
 public:
  typedef std::vector<std::pair<clang::CXXRecordDecl*, BasePoint>> Bases;

  struct FieldDeclCmp {
    bool operator()(clang::FieldDecl* a, clang::FieldDecl *b) const {
      return a->getBeginLoc() < b->getBeginLoc();
    }
  };
  typedef std::map<clang::FieldDecl*, FieldPoint, FieldDeclCmp> Fields;

  typedef std::vector<const clang::Type*> TemplateArgs;

  ~RecordInfo();

  clang::CXXRecordDecl* record() const { return record_; }
  const std::string& name() const { return name_; }
  Fields& GetFields();
  Bases& GetBases();
  const clang::CXXBaseSpecifier* GetDirectGCBase();
  clang::CXXMethodDecl* GetTraceMethod();
  clang::CXXMethodDecl* GetTraceWrappersMethod();
  clang::CXXMethodDecl* GetTraceDispatchMethod();
  clang::CXXMethodDecl* GetFinalizeDispatchMethod();

  bool HasMultipleTraceDispatchMethods() const {
    return extra_trace_dispatch_method_ != nullptr;
  }
  bool HasMultipleFinalizeDispatchMethods() const {
    return extra_finalize_dispatch_method_ != nullptr;
  }

  clang::CXXMethodDecl* GetExtraTraceDispatchMethod() {
    assert(determined_trace_methods_);
    return extra_trace_dispatch_method_;
  }
  clang::CXXMethodDecl* GetExtraFinalizeDispatchMethod() {
    assert(determined_trace_methods_);
    return extra_finalize_dispatch_method_;
  }

  bool GetTemplateArgs(size_t count, TemplateArgs* output_args);

  bool IsHeapAllocatedCollection();
  bool IsGCDerived();
  bool IsGCDirectlyDerived();
  bool IsGCAllocated();
  bool IsGCMixin();
  bool IsStackAllocated();
  bool IsNewDisallowed();

  bool HasDefinition();

  clang::CXXMethodDecl* DeclaresNewOperator();

  bool RequiresTraceMethod();
  bool NeedsFinalization();
  bool DeclaresLocalTraceMethod();
  TracingStatus NeedsTracing(Edge::NeedsTracingOption);
  clang::CXXMethodDecl* InheritsNonVirtualTrace();
  bool IsConsideredAbstract();

  static clang::CXXRecordDecl* GetDependentTemplatedDecl(const clang::Type&);

 private:
  RecordInfo(clang::CXXRecordDecl* record, RecordCache* cache);

  void walkBases();

  Fields* CollectFields();
  Bases* CollectBases();
  void DetermineTracingMethods();
  bool InheritsTrace();

  Edge* CreateEdge(const clang::Type* type);
  Edge* CreateEdgeFromOriginalType(const clang::Type* type);

  bool HasOptionalFinalizer();

  bool HasTypeAlias(std::string marker_name) const;
  bool GetTemplateArgsInternal(
      const llvm::ArrayRef<clang::TemplateArgument>& args,
      size_t count,
      TemplateArgs* output_args);

  RecordCache* cache_;
  clang::CXXRecordDecl* record_;
  const std::string name_;
  TracingStatus fields_need_tracing_;
  Bases* bases_ = nullptr;
  Fields* fields_ = nullptr;

  enum CachedBool { kFalse = 0, kTrue = 1, kNotComputed = 2 };
  CachedBool is_stack_allocated_ = kNotComputed;
  CachedBool does_need_finalization_ = kNotComputed;
  CachedBool is_declaring_local_trace_ = kNotComputed;

  bool determined_new_operator_ = false;
  clang::CXXMethodDecl* new_operator_ = nullptr;

  bool determined_trace_methods_ = false;
  clang::CXXMethodDecl* trace_method_ = nullptr;
  clang::CXXMethodDecl* trace_dispatch_method_ = nullptr;
  clang::CXXMethodDecl* finalize_dispatch_method_ = nullptr;
  clang::CXXMethodDecl* extra_trace_dispatch_method_ = nullptr;
  clang::CXXMethodDecl* extra_finalize_dispatch_method_ = nullptr;

  bool is_gc_derived_ = false;

  std::vector<std::string> gc_base_names_;

  const clang::CXXBaseSpecifier* directly_derived_gc_base_ = nullptr;

  friend class RecordCache;
};

class RecordCache {
 public:
  RecordCache(clang::CompilerInstance& instance)
    : instance_(instance)
  {
  }

  RecordInfo* Lookup(clang::CXXRecordDecl* record);

  RecordInfo* Lookup(const clang::CXXRecordDecl* record) {
    return Lookup(const_cast<clang::CXXRecordDecl*>(record));
  }

  RecordInfo* Lookup(clang::DeclContext* decl) {
    return Lookup(clang::dyn_cast<clang::CXXRecordDecl>(decl));
  }

  RecordInfo* Lookup(const clang::Type* type) {
    return Lookup(type->getAsCXXRecordDecl());
  }

  RecordInfo* Lookup(const clang::QualType& type) {
    return Lookup(type.getTypePtr());
  }

  ~RecordCache() {
    for (Cache::iterator it = cache_.begin(); it != cache_.end(); ++it) {
      if (!it->second.fields_)
        continue;
      for (RecordInfo::Fields::iterator fit = it->second.fields_->begin();
        fit != it->second.fields_->end();
        ++fit) {
        fit->second.deleteEdge();
      }
    }
  }

  clang::CompilerInstance& instance() const { return instance_; }

 private:
  clang::CompilerInstance& instance_;

  typedef std::map<clang::CXXRecordDecl*, RecordInfo> Cache;
  Cache cache_;
};

#endif  // TOOLS_BLINK_GC_PLUGIN_RECORD_INFO_H_

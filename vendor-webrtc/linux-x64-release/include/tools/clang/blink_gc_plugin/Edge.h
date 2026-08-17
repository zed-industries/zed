// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BLINK_GC_PLUGIN_EDGE_H_
#define TOOLS_BLINK_GC_PLUGIN_EDGE_H_

#include <cassert>
#include <deque>
#include <string>
#include <vector>

#include "TracingStatus.h"

class RecordInfo;

class Edge;
class Collection;
class CrossThreadPersistent;
class Iterator;
class Member;
class Persistent;
class RawPtr;
class RefPtr;
class UniquePtr;
class Value;
class WeakMember;
class TraceWrapperV8Reference;
class ArrayEdge;

// Bare-bones visitor.
class EdgeVisitor {
 public:
  virtual ~EdgeVisitor() {}
  virtual void VisitValue(Value*) {}
  virtual void VisitRawPtr(RawPtr*) {}
  virtual void VisitRefPtr(RefPtr*) {}
  virtual void VisitUniquePtr(UniquePtr*) {}
  virtual void VisitMember(Member*) {}
  virtual void VisitWeakMember(WeakMember*) {}
  virtual void VisitPersistent(Persistent*) {}
  virtual void VisitCrossThreadPersistent(CrossThreadPersistent*) {}
  virtual void VisitCollection(Collection*) {}
  virtual void VisitIterator(Iterator*) {}
  virtual void VisitTraceWrapperV8Reference(TraceWrapperV8Reference*) {}
  virtual void VisitArrayEdge(ArrayEdge*) {}
};

// Recursive edge visitor. The traversed path is accessible in context.
class RecursiveEdgeVisitor : public EdgeVisitor {
 public:
  // Overrides that recursively walk the edges and record the path.
  void VisitValue(Value*) override;
  void VisitRawPtr(RawPtr*) override;
  void VisitRefPtr(RefPtr*) override;
  void VisitUniquePtr(UniquePtr*) override;
  void VisitMember(Member*) override;
  void VisitWeakMember(WeakMember*) override;
  void VisitPersistent(Persistent*) override;
  void VisitCrossThreadPersistent(CrossThreadPersistent*) override;
  void VisitCollection(Collection*) override;
  void VisitIterator(Iterator*) override;
  void VisitTraceWrapperV8Reference(TraceWrapperV8Reference*) override;
  void VisitArrayEdge(ArrayEdge*) override;

 protected:
  typedef std::deque<Edge*> Context;
  Context& context() { return context_; }
  Edge* Parent() { return context_.empty() ? 0 : context_.front(); }
  Edge* GrandParent() {
    return Parent() ? (context_.size() > 1 ? context_[1] : nullptr) : nullptr;
  }
  void Enter(Edge* e) { return context_.push_front(e); }
  void Leave() { context_.pop_front(); }

  // Default callback to overwrite in visitor subclass.
  virtual void AtValue(Value*);
  virtual void AtRawPtr(RawPtr*);
  virtual void AtRefPtr(RefPtr*);
  virtual void AtUniquePtr(UniquePtr*);
  virtual void AtMember(Member*);
  virtual void AtWeakMember(WeakMember*);
  virtual void AtTraceWrapperV8Reference(TraceWrapperV8Reference*);
  virtual void AtPersistent(Persistent*);
  virtual void AtCrossThreadPersistent(CrossThreadPersistent*);
  virtual void AtCollection(Collection*);
  virtual void AtIterator(Iterator*);
  virtual void AtArrayEdge(ArrayEdge*);

 private:
  Context context_;
};

// Base class for all edges.
class Edge {
 public:
  enum NeedsTracingOption { kRecursive, kNonRecursive };
  enum LivenessKind { kWeak, kStrong, kRoot };

  virtual ~Edge() {}
  virtual LivenessKind Kind() = 0;
  virtual void Accept(EdgeVisitor*) = 0;
  virtual bool NeedsFinalization() = 0;
  virtual TracingStatus NeedsTracing(NeedsTracingOption) {
    return TracingStatus::Unknown();
  }

  virtual bool IsValue() { return false; }
  virtual bool IsRawPtr() { return false; }
  virtual bool IsRefPtr() { return false; }
  virtual bool IsUniquePtr() { return false; }
  virtual bool IsMember() { return false; }
  virtual bool IsWeakMember() { return false; }
  virtual bool IsCollection() { return false; }
  virtual bool IsArray() { return false; }
  virtual bool IsTraceWrapperV8Reference() { return false; }
};

// A value edge is a direct edge to some type, eg, part-object edges.
class Value : public Edge {
 public:
  explicit Value(RecordInfo* value) : value_(value) {};
  bool IsValue() override { return true; }
  LivenessKind Kind() override { return kStrong; }
  bool NeedsFinalization() override;
  TracingStatus NeedsTracing(NeedsTracingOption) override;
  void Accept(EdgeVisitor* visitor) override { visitor->VisitValue(this); }
  RecordInfo* value() { return value_; }

 private:
  RecordInfo* value_;
};

class ArrayEdge : public Edge {
 public:
  explicit ArrayEdge(Edge* value) : value_(value){};
  LivenessKind Kind() override { return kStrong; }
  bool NeedsFinalization() override { return false; }
  TracingStatus NeedsTracing(NeedsTracingOption option) override {
    return value_->NeedsTracing(option);
  }
  void Accept(EdgeVisitor* visitor) override { visitor->VisitArrayEdge(this); }
  Edge* element() { return value_; }
  bool IsArray() override { return true; }

 private:
  Edge* value_;
};

// Shared base for smart-pointer edges.
class PtrEdge : public Edge {
 public:
  ~PtrEdge() { delete ptr_; }
  Edge* ptr() { return ptr_; }
 protected:
  PtrEdge(Edge* ptr) : ptr_(ptr) {
    assert(ptr && "EdgePtr pointer must be non-null");
  }
 private:
  Edge* ptr_;
};

class RawPtr : public PtrEdge {
 public:
  RawPtr(Edge* ptr, bool is_ref_type)
      : PtrEdge(ptr)
      , is_ref_type_(is_ref_type)
  {
  }

  bool IsRawPtr() override { return true; }
  LivenessKind Kind() override { return kWeak; }
  bool NeedsFinalization() override { return false; }
  TracingStatus NeedsTracing(NeedsTracingOption) override {
    return TracingStatus::Illegal();
  }
  void Accept(EdgeVisitor* visitor) override { visitor->VisitRawPtr(this); }

  bool HasReferenceType() { return is_ref_type_; }
 private:
  bool is_ref_type_;
};

class RefPtr : public PtrEdge {
 public:
  RefPtr(Edge* ptr, LivenessKind kind) : PtrEdge(ptr), kind_(kind) {}
  bool IsRefPtr() override { return true; }
  LivenessKind Kind() override { return kind_; }
  bool NeedsFinalization() override { return true; }
  TracingStatus NeedsTracing(NeedsTracingOption) override {
    return TracingStatus::Illegal();
  }
  void Accept(EdgeVisitor* visitor) override { visitor->VisitRefPtr(this); }

 private:
  LivenessKind kind_;
};

class UniquePtr : public PtrEdge {
 public:
  explicit UniquePtr(Edge* ptr) : PtrEdge(ptr) { }
  bool IsUniquePtr() override { return true; }
  LivenessKind Kind() override { return kStrong; }
  bool NeedsFinalization() override { return true; }
  TracingStatus NeedsTracing(NeedsTracingOption) override {
    return TracingStatus::Illegal();
  }
  void Accept(EdgeVisitor* visitor) override { visitor->VisitUniquePtr(this); }
};

class Member : public PtrEdge {
 public:
  explicit Member(Edge* ptr) : PtrEdge(ptr) { }
  bool IsMember() override { return true; }
  LivenessKind Kind() override { return kStrong; }
  bool NeedsFinalization() override { return false; }
  TracingStatus NeedsTracing(NeedsTracingOption) override {
    return TracingStatus::Needed();
  }
  void Accept(EdgeVisitor* visitor) override { visitor->VisitMember(this); }
};

class WeakMember : public PtrEdge {
 public:
  explicit WeakMember(Edge* ptr) : PtrEdge(ptr) { }
  bool IsWeakMember() override { return true; }
  LivenessKind Kind() override { return kWeak; }
  bool NeedsFinalization() override { return false; }
  TracingStatus NeedsTracing(NeedsTracingOption) override {
    return TracingStatus::Needed();
  }
  void Accept(EdgeVisitor* visitor) override { visitor->VisitWeakMember(this); }
};

class Persistent : public PtrEdge {
 public:
  explicit Persistent(Edge* ptr) : PtrEdge(ptr) { }
  LivenessKind Kind() override { return kRoot; }
  bool NeedsFinalization() override { return true; }
  TracingStatus NeedsTracing(NeedsTracingOption) override {
    return TracingStatus::Illegal();
  }
  void Accept(EdgeVisitor* visitor) override { visitor->VisitPersistent(this); }
};

class CrossThreadPersistent : public PtrEdge {
 public:
  explicit CrossThreadPersistent(Edge* ptr) : PtrEdge(ptr) { }
  LivenessKind Kind() override { return kRoot; }
  bool NeedsFinalization() override { return true; }
  TracingStatus NeedsTracing(NeedsTracingOption) override {
    return TracingStatus::Illegal();
  }
  void Accept(EdgeVisitor* visitor) override {
    visitor->VisitCrossThreadPersistent(this);
  }
};

class TraceWrapperV8Reference : public PtrEdge {
 public:
  explicit TraceWrapperV8Reference(Edge* ptr) : PtrEdge(ptr) {}
  bool IsTraceWrapperV8Reference() override { return true; }
  LivenessKind Kind() override { return kStrong; }
  bool NeedsFinalization() override { return true; }
  TracingStatus NeedsTracing(NeedsTracingOption) override {
    return TracingStatus::Needed();
  }
  void Accept(EdgeVisitor* visitor) override {
    visitor->VisitTraceWrapperV8Reference(this);
  }
};

class Collection : public Edge {
 public:
  typedef std::vector<Edge*> Members;
  Collection(RecordInfo* info, bool on_heap) : info_(info), on_heap_(on_heap) {}
  ~Collection() {
    for (Members::iterator it = members_.begin(); it != members_.end(); ++it) {
      assert(*it && "Collection-edge members must be non-null");
      delete *it;
    }
  }
  bool IsCollection() override { return true; }
  bool IsSTDCollection();
  LivenessKind Kind() override { return kStrong; }
  bool on_heap() { return on_heap_; }
  Members& members() { return members_; }
  void Accept(EdgeVisitor* visitor) override { visitor->VisitCollection(this); }
  void AcceptMembers(EdgeVisitor* visitor) {
    for (Members::iterator it = members_.begin(); it != members_.end(); ++it)
      (*it)->Accept(visitor);
  }
  bool NeedsFinalization() override;
  TracingStatus NeedsTracing(NeedsTracingOption) override;
  std::string GetCollectionName() const;

 private:
  RecordInfo* info_;
  Members members_;
  bool on_heap_;
};

// An iterator edge is a direct edge to some iterator type.
class Iterator : public Edge {
 public:
  Iterator(RecordInfo* info, bool on_heap) : info_(info), on_heap_(on_heap) {}
  ~Iterator() {}

  void Accept(EdgeVisitor* visitor) override { visitor->VisitIterator(this); }
  LivenessKind Kind() override { return kStrong; }
  bool NeedsFinalization() override { return false; }
  TracingStatus NeedsTracing(NeedsTracingOption) override {
    if (on_heap_)
      return TracingStatus::Illegal();
    return TracingStatus::Unneeded();
  }

  RecordInfo* info() const { return info_; }

  bool on_heap() const { return on_heap_; }

 private:
  RecordInfo* info_;
  bool on_heap_;
};

#endif  // TOOLS_BLINK_GC_PLUGIN_EDGE_H_

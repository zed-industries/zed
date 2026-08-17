// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef MISSING_CTOR_H_
#define MISSING_CTOR_H_

#include <string>

#include "base/containers/span.h"
#include "base/memory/checked_ptr.h"
#include "base/memory/raw_ptr.h"

struct MyString {
  MyString();
  ~MyString();
  MyString(const MyString&);
  MyString(MyString&&);
};

template <class T>
struct MyVector {
  MyVector();
  ~MyVector();
  MyVector(const MyVector&);
  MyVector(MyVector&&);
};

template <class T>
struct TrivialTemplate {
  TrivialTemplate();
};

template <typename T>
using AliasTemplate = T;

// Note: this should warn for an implicit copy constructor too, but currently
// doesn't, due to a plugin bug.
class MissingCtorsArentOKInHeader {
 public:

 private:
  MyVector<int> one_;
  MyVector<MyString> two_;
};

// Inline move ctors shouldn't be warned about. Similar to the previous test
// case, this also incorrectly fails to warn for the implicit copy ctor.
class InlineImplicitMoveCtorOK {
 public:
  InlineImplicitMoveCtorOK();

 private:
  // ctor weight = 12, dtor weight = 9.
  MyString one_;
  MyString two_;
  MyString three_;
  int four_;
  int five_;
  int six_;
};

class ExplicitlyDefaultedInlineAlsoWarns {
 public:
  ExplicitlyDefaultedInlineAlsoWarns() = default;
  ~ExplicitlyDefaultedInlineAlsoWarns() = default;
  ExplicitlyDefaultedInlineAlsoWarns(
      const ExplicitlyDefaultedInlineAlsoWarns&) = default;

 private:
  MyVector<int> one_;
  MyVector<MyString> two_;

};

union UnionDoesNotWarn {
 UnionDoesNotWarn() = default;
 UnionDoesNotWarn(const UnionDoesNotWarn& other) = default;

 int a;
 int b;
 int c;
 int d;
 int e;
 int f;
 int g;
 int h;
 int i;
 int j;
 int k;
 int l;
 int m;
 int n;
 int o;
 int p;
 int q;
 int r;
 int s;
 int t;
 int u;
 int v;
 int w;
 int x;
 int y;
 int z;
};

class StringDoesNotWarn {
 public:
  StringDoesNotWarn() = default;
  ~StringDoesNotWarn() = default;

 private:
  std::string foo_;
};

class ThreeStringsDoesNotWarn {
 public:
  ThreeStringsDoesNotWarn() = default;
  ~ThreeStringsDoesNotWarn() = default;

 private:
  std::string one_;
  std::string two_;
  std::string three_;
};

class FourStringsWarns {
 public:
  FourStringsWarns() = default;
  ~FourStringsWarns() = default;

 private:
  std::string one_;
  std::string two_;
  std::string three_;
  std::string four_;
};

class TrivialTemplateDoesNotWarn {
 public:
  TrivialTemplateDoesNotWarn() = default;
  ~TrivialTemplateDoesNotWarn() = default;

 private:
  TrivialTemplate<int> foo_;
};

class NineTrivialTemplatesDoesNotWarn {
 public:
  NineTrivialTemplatesDoesNotWarn() = default;
  ~NineTrivialTemplatesDoesNotWarn() = default;

 private:
  TrivialTemplate<int> one_;
  TrivialTemplate<int> two_;
  TrivialTemplate<int> three_;
  TrivialTemplate<int> four_;
  TrivialTemplate<int> five_;
  TrivialTemplate<int> six_;
  TrivialTemplate<int> seven_;
  TrivialTemplate<int> eight_;
  TrivialTemplate<int> nine_;
};

class TenTrivialTemplatesWarns {
 public:
  TenTrivialTemplatesWarns() = default;
  ~TenTrivialTemplatesWarns() = default;

 private:
  TrivialTemplate<int> one_;
  TrivialTemplate<int> two_;
  TrivialTemplate<int> three_;
  TrivialTemplate<int> four_;
  TrivialTemplate<int> five_;
  TrivialTemplate<int> six_;
  TrivialTemplate<int> seven_;
  TrivialTemplate<int> eight_;
  TrivialTemplate<int> nine_;
  TrivialTemplate<int> ten_;
};

class TrivialAliasTemplateDoesNotWarn {
 public:
  TrivialAliasTemplateDoesNotWarn() = default;
  ~TrivialAliasTemplateDoesNotWarn() = default;

 private:
  AliasTemplate<int> one_;
};

class NineTrivialAliasTemplatesDoesNotWarn {
 public:
  NineTrivialAliasTemplatesDoesNotWarn() = default;
  ~NineTrivialAliasTemplatesDoesNotWarn() = default;

 private:
  AliasTemplate<int> one_;
  AliasTemplate<int> two_;
  AliasTemplate<int> three_;
  AliasTemplate<int> four_;
  AliasTemplate<int> five_;
  AliasTemplate<int> six_;
  AliasTemplate<int> seven_;
  AliasTemplate<int> eight_;
  AliasTemplate<int> nine_;
};

class TenTrivialAliasTemplatesWarns {
 public:
  TenTrivialAliasTemplatesWarns() = default;
  ~TenTrivialAliasTemplatesWarns() = default;

 private:
  AliasTemplate<int> one_;
  AliasTemplate<int> two_;
  AliasTemplate<int> three_;
  AliasTemplate<int> four_;
  AliasTemplate<int> five_;
  AliasTemplate<int> six_;
  AliasTemplate<int> seven_;
  AliasTemplate<int> eight_;
  AliasTemplate<int> nine_;
  AliasTemplate<int> ten_;
};

class NonTrivialAliasTemplateDoesNotWarn {
 public:
  NonTrivialAliasTemplateDoesNotWarn() = default;
  ~NonTrivialAliasTemplateDoesNotWarn() = default;

 private:
  AliasTemplate<std::string> one_;
};

class ThreeNonTrivialAliasTemplatesDoesNotWarn {
 public:
  ThreeNonTrivialAliasTemplatesDoesNotWarn() = default;
  ~ThreeNonTrivialAliasTemplatesDoesNotWarn() = default;

 private:
  AliasTemplate<std::string> one_;
  AliasTemplate<std::string> two_;
  AliasTemplate<std::string> three_;
};

class FourNonTrivialAliasTemplatesWarns {
 public:
  FourNonTrivialAliasTemplatesWarns() = default;
  ~FourNonTrivialAliasTemplatesWarns() = default;

 private:
  AliasTemplate<std::string> one_;
  AliasTemplate<std::string> two_;
  AliasTemplate<std::string> three_;
  AliasTemplate<std::string> four_;
};

class CheckedPtrDoesNotWarn {
 public:
  CheckedPtrDoesNotWarn() = default;
  ~CheckedPtrDoesNotWarn() = default;

 private:
  CheckedPtr<CheckedPtrDoesNotWarn> foo_;
};

class NineCheckedPtrDoesNotWarn {
 public:
  NineCheckedPtrDoesNotWarn() = default;
  ~NineCheckedPtrDoesNotWarn() = default;

 private:
  CheckedPtr<NineCheckedPtrDoesNotWarn> one_;
  CheckedPtr<NineCheckedPtrDoesNotWarn> two_;
  CheckedPtr<NineCheckedPtrDoesNotWarn> three_;
  CheckedPtr<NineCheckedPtrDoesNotWarn> four_;
  CheckedPtr<NineCheckedPtrDoesNotWarn> five_;
  CheckedPtr<NineCheckedPtrDoesNotWarn> six_;
  CheckedPtr<NineCheckedPtrDoesNotWarn> seven_;
  CheckedPtr<NineCheckedPtrDoesNotWarn> eight_;
  CheckedPtr<NineCheckedPtrDoesNotWarn> nine_;
};

class TenCheckedPtrWarns {
 public:
  TenCheckedPtrWarns() = default;
  ~TenCheckedPtrWarns() = default;

 private:
  CheckedPtr<TenCheckedPtrWarns> one_;
  CheckedPtr<TenCheckedPtrWarns> two_;
  CheckedPtr<TenCheckedPtrWarns> three_;
  CheckedPtr<TenCheckedPtrWarns> four_;
  CheckedPtr<TenCheckedPtrWarns> five_;
  CheckedPtr<TenCheckedPtrWarns> six_;
  CheckedPtr<TenCheckedPtrWarns> seven_;
  CheckedPtr<TenCheckedPtrWarns> eight_;
  CheckedPtr<TenCheckedPtrWarns> nine_;
  CheckedPtr<TenCheckedPtrWarns> ten_;
};

class RawPtrDoesNotWarn {
 public:
  RawPtrDoesNotWarn() = default;
  ~RawPtrDoesNotWarn() = default;

 private:
  raw_ptr<RawPtrDoesNotWarn> foo_;
};

class NineRawPtrDoesNotWarn {
 public:
  NineRawPtrDoesNotWarn() = default;
  ~NineRawPtrDoesNotWarn() = default;

 private:
  raw_ptr<NineRawPtrDoesNotWarn> one_;
  raw_ptr<NineRawPtrDoesNotWarn> two_;
  raw_ptr<NineRawPtrDoesNotWarn> three_;
  raw_ptr<NineRawPtrDoesNotWarn> four_;
  raw_ptr<NineRawPtrDoesNotWarn> five_;
  raw_ptr<NineRawPtrDoesNotWarn> six_;
  raw_ptr<NineRawPtrDoesNotWarn> seven_;
  raw_ptr<NineRawPtrDoesNotWarn> eight_;
  raw_ptr<NineRawPtrDoesNotWarn> nine_;
};

class TenRawPtrWarns {
 public:
  TenRawPtrWarns() = default;
  ~TenRawPtrWarns() = default;

 private:
  raw_ptr<TenRawPtrWarns> one_;
  raw_ptr<TenRawPtrWarns> two_;
  raw_ptr<TenRawPtrWarns> three_;
  raw_ptr<TenRawPtrWarns> four_;
  raw_ptr<TenRawPtrWarns> five_;
  raw_ptr<TenRawPtrWarns> six_;
  raw_ptr<TenRawPtrWarns> seven_;
  raw_ptr<TenRawPtrWarns> eight_;
  raw_ptr<TenRawPtrWarns> nine_;
  raw_ptr<TenRawPtrWarns> ten_;
};

class RawRefDoesNotWarn {
 public:
  RawRefDoesNotWarn() = default;
  ~RawRefDoesNotWarn() = default;

 private:
  raw_ref<RawRefDoesNotWarn> foo_;
};

class NineRawRefDoesNotWarn {
 public:
  NineRawRefDoesNotWarn() = default;
  ~NineRawRefDoesNotWarn() = default;

 private:
  raw_ref<NineRawRefDoesNotWarn> one_;
  raw_ref<NineRawRefDoesNotWarn> two_;
  raw_ref<NineRawRefDoesNotWarn> three_;
  raw_ref<NineRawRefDoesNotWarn> four_;
  raw_ref<NineRawRefDoesNotWarn> five_;
  raw_ref<NineRawRefDoesNotWarn> six_;
  raw_ref<NineRawRefDoesNotWarn> seven_;
  raw_ref<NineRawRefDoesNotWarn> eight_;
  raw_ref<NineRawRefDoesNotWarn> nine_;
};

class TenRawRefWarns {
 public:
  TenRawRefWarns() = default;
  ~TenRawRefWarns() = default;

 private:
  raw_ref<TenRawRefWarns> one_;
  raw_ref<TenRawRefWarns> two_;
  raw_ref<TenRawRefWarns> three_;
  raw_ref<TenRawRefWarns> four_;
  raw_ref<TenRawRefWarns> five_;
  raw_ref<TenRawRefWarns> six_;
  raw_ref<TenRawRefWarns> seven_;
  raw_ref<TenRawRefWarns> eight_;
  raw_ref<TenRawRefWarns> nine_;
  raw_ref<TenRawRefWarns> ten_;
};

class SpanDoesNotWarn {
 public:
  SpanDoesNotWarn() = default;
  ~SpanDoesNotWarn() = default;

 private:
  raw_ref<SpanDoesNotWarn> foo_;
};

class NineSpanDoesNotWarn {
 public:
  NineSpanDoesNotWarn() = default;
  ~NineSpanDoesNotWarn() = default;

 private:
  base::span<NineSpanDoesNotWarn> one_;
  base::span<NineSpanDoesNotWarn> two_;
  base::span<NineSpanDoesNotWarn> three_;
  base::span<NineSpanDoesNotWarn> four_;
  base::span<NineSpanDoesNotWarn> five_;
  base::span<NineSpanDoesNotWarn> six_;
  base::span<NineSpanDoesNotWarn> seven_;
  base::span<NineSpanDoesNotWarn> eight_;
  base::span<NineSpanDoesNotWarn> nine_;
};

class TenSpanWarns {
 public:
  TenSpanWarns() = default;
  ~TenSpanWarns() = default;

 private:
  base::span<TenSpanWarns> one_;
  base::span<TenSpanWarns> two_;
  base::span<TenSpanWarns> three_;
  base::span<TenSpanWarns> four_;
  base::span<TenSpanWarns> five_;
  base::span<TenSpanWarns> six_;
  base::span<TenSpanWarns> seven_;
  base::span<TenSpanWarns> eight_;
  base::span<TenSpanWarns> nine_;
  base::span<TenSpanWarns> ten_;
};

class RawSpanDoesNotWarn {
 public:
  RawSpanDoesNotWarn() = default;
  ~RawSpanDoesNotWarn() = default;

 private:
  base::raw_span<RawSpanDoesNotWarn> foo_;
};

class NineRawSpanDoesNotWarn {
 public:
  NineRawSpanDoesNotWarn() = default;
  ~NineRawSpanDoesNotWarn() = default;

 private:
  base::raw_span<NineRawSpanDoesNotWarn> one_;
  base::raw_span<NineRawSpanDoesNotWarn> two_;
  base::raw_span<NineRawSpanDoesNotWarn> three_;
  base::raw_span<NineRawSpanDoesNotWarn> four_;
  base::raw_span<NineRawSpanDoesNotWarn> five_;
  base::raw_span<NineRawSpanDoesNotWarn> six_;
  base::raw_span<NineRawSpanDoesNotWarn> seven_;
  base::raw_span<NineRawSpanDoesNotWarn> eight_;
  base::raw_span<NineRawSpanDoesNotWarn> nine_;
};

class TenRawSpanWarns {
 public:
  TenRawSpanWarns() = default;
  ~TenRawSpanWarns() = default;

 private:
  base::raw_span<TenRawSpanWarns> one_;
  base::raw_span<TenRawSpanWarns> two_;
  base::raw_span<TenRawSpanWarns> three_;
  base::raw_span<TenRawSpanWarns> four_;
  base::raw_span<TenRawSpanWarns> five_;
  base::raw_span<TenRawSpanWarns> six_;
  base::raw_span<TenRawSpanWarns> seven_;
  base::raw_span<TenRawSpanWarns> eight_;
  base::raw_span<TenRawSpanWarns> nine_;
  base::raw_span<TenRawSpanWarns> ten_;
};
#endif  // MISSING_CTOR_H_

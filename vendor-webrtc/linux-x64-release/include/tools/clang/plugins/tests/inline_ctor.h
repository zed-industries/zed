// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef INLINE_CTOR_H_
#define INLINE_CTOR_H_

#include <string>
#include <vector>

class InlineCtorsArentOKInHeader {
 public:
  InlineCtorsArentOKInHeader() {}
  ~InlineCtorsArentOKInHeader() {}

 private:
  std::vector<int> one_;
  std::vector<std::string> two_;
};

#define INLINE_CTORS_IN_A_MACRO(CLASS_NAME) \
  class CLASS_NAME {                        \
   public:                                  \
    CLASS_NAME() {}                         \
    ~CLASS_NAME() {}                        \
                                            \
   private:                                 \
    std::vector<int> one_;                  \
    std::vector<std::string> two_;          \
  }
INLINE_CTORS_IN_A_MACRO(InlineCtorsBehindAMacroArentOKInHeader);
MACRO_FROM_CPP;

class DeletedMembersInHeaderAreOKThough {
 public:
  DeletedMembersInHeaderAreOKThough() = delete;
  ~DeletedMembersInHeaderAreOKThough() = delete;
  DeletedMembersInHeaderAreOKThough(const DeletedMembersInHeaderAreOKThough&) =
      delete;

 private:
  std::vector<int> one_;
  std::vector<std::string> two_;
};

class ExplicitlyInlinedIsAlsoOK {
  ExplicitlyInlinedIsAlsoOK();
  ~ExplicitlyInlinedIsAlsoOK();
  ExplicitlyInlinedIsAlsoOK(const ExplicitlyInlinedIsAlsoOK&);

 private:
  std::vector<int> one_;
  std::vector<std::string> two_;
};

inline ExplicitlyInlinedIsAlsoOK::ExplicitlyInlinedIsAlsoOK() {
}

inline ExplicitlyInlinedIsAlsoOK::~ExplicitlyInlinedIsAlsoOK() {
}

inline ExplicitlyInlinedIsAlsoOK::ExplicitlyInlinedIsAlsoOK(
    const ExplicitlyInlinedIsAlsoOK&) {
}

struct TrivialStruct {
  int something_;
};

struct NonTrivialStruct {
  NonTrivialStruct();
  ~NonTrivialStruct();

  int something_;
};

// Plugin doesn't warn about inlining trivial member dtor calls.
struct FourTrivialMembers {
  ~FourTrivialMembers();

  TrivialStruct a;
  TrivialStruct b;
  TrivialStruct c;
  TrivialStruct d;
};

// Plugin doesn't warn about inlining three ctor/dtor calls.
struct ThreeNonTrivialMembers {
  NonTrivialStruct a;
  NonTrivialStruct b;
  NonTrivialStruct c;
};

// Plugin does warn about inlining four ctor/dtor calls.
struct FourNonTrivialMembers {
  NonTrivialStruct a;
  NonTrivialStruct b;
  NonTrivialStruct c;
  NonTrivialStruct d;
};

#endif  // INLINE_CTOR_H_

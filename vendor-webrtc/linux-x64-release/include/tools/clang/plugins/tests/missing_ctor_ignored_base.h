// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef MISSING_CTOR_IGNORED_BASE_H_
#define MISSING_CTOR_IGNORED_BASE_H_

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

// IPC::NoParams is an ignored base.
namespace IPC {
class NoParams {};
}

// Note: this should warn for an implicit copy constructor too, but currently
// doesn't, due to a plugin bug.
class MissingCtorsWithIgnoredBase : public IPC::NoParams {
 public:

 private:
  MyVector<int> one_;
  MyVector<MyString> two_;
};

// Inline move ctors shouldn't be warned about. Similar to the previous test
// case, this also incorrectly fails to warn for the implicit copy ctor.
class MissingCtorsWithIgnoredGrandBase : public MissingCtorsWithIgnoredBase {
 public:

 private:
  // ctor weight = 12, dtor weight = 9.
  MyString one_;
  MyString two_;
  MyString three_;
  int four_;
  int five_;
  int six_;
};

#endif  // MISSING_CTOR_IGNORED_BASE_H_

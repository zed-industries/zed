// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_LAYOUT_OBJECT_METHODS_H_
#define TOOLS_CLANG_PLUGINS_TESTS_LAYOUT_OBJECT_METHODS_H_

namespace blink {

class Visitor;

void foo() {}

class LayoutObject {
 public:
  // These methods should be ignored.
  static void StaticMethod() {}
  void CheckIsNotDestroyed() const {}
  void Trace(Visitor*) const {}

  int ShouldPass1() {
    CheckIsNotDestroyed();
    foo();
    return 0;
  }
  int ShouldFail1() {
    // CheckIsNotDestroyed();
    foo();
    return 0;
  }

  virtual void VirtualEmptyMethod() = 0;
  void EmptyMethod() {}
};

class LayoutBoxModelObject : public LayoutObject {
 public:
  int ShouldPass2() const {
    CheckIsNotDestroyed();
    return 0;
  }
  int ShouldFail2() const {
    ShouldPass2();
    CheckIsNotDestroyed();  // This should be the first statement.
    return 0;
  }

  void VirtualEmptyMethod() override {}
};

class LayoutBox : public LayoutBoxModelObject {
 public:
  int ShouldPass3() {
    CheckIsNotDestroyed();
    return 0;
  }
  int ShouldFail3() {
    foo();
    CheckIsNotDestroyed();  // This should be the first statement.
    return 0;
  }

  void VirtualEmptyMethod() override {}
};

}  // namespace blink

#endif  // TOOLS_CLANG_PLUGINS_TESTS_LAYOUT_OBJECT_METHODS_H_

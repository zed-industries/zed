//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_TYPE_CLASSIFICATION_MOVECONSTRUCTIBLE_H
#define TEST_SUPPORT_TYPE_CLASSIFICATION_MOVECONSTRUCTIBLE_H

struct HasDefaultOps {};

struct CustomMoveCtor {
  CustomMoveCtor(CustomMoveCtor&&) noexcept;
};

struct MoveOnly {
  MoveOnly(MoveOnly&&) noexcept = default;
  MoveOnly& operator=(MoveOnly&&) noexcept = default;
  MoveOnly(const MoveOnly&) = delete;
  MoveOnly& operator=(const MoveOnly&) = default;
};

struct CustomMoveAssign {
  CustomMoveAssign(CustomMoveAssign&&) noexcept;
  CustomMoveAssign& operator=(CustomMoveAssign&&) noexcept;
};

struct DeletedMoveCtor {
  DeletedMoveCtor(DeletedMoveCtor&&) = delete;
  DeletedMoveCtor& operator=(DeletedMoveCtor&&) = default;
};

struct ImplicitlyDeletedMoveCtor {
  DeletedMoveCtor X;
};

struct DeletedMoveAssign {
  DeletedMoveAssign& operator=(DeletedMoveAssign&&) = delete;
};

struct ImplicitlyDeletedMoveAssign {
  DeletedMoveAssign X;
};

class MemberLvalueReference {
public:
  MemberLvalueReference(int&);

private:
  int& X;
};

class MemberRvalueReference {
public:
  MemberRvalueReference(int&&);

private:
  int&& X;
};

struct NonMovable {
  NonMovable() = default;
  NonMovable(NonMovable&&) = delete;
  NonMovable& operator=(NonMovable&&) = delete;
};

struct DerivedFromNonMovable : NonMovable {};

struct HasANonMovable {
  NonMovable X;
};

#endif // TEST_SUPPORT_TYPE_CLASSIFICATION_MOVECONSTRUCTIBLE_H

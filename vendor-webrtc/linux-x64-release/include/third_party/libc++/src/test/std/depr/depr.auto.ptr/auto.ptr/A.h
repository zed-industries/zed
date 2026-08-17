//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef A_H
#define A_H

#include <cassert>

class A
{
    int id_;
public:
    explicit A(int id) : id_(id) {++count;}
    A(const A& a) : id_(a.id_) {++count;}
    ~A() {assert(id_ >= 0); id_ = -1; --count;}

    A& operator=(const A& other) { id_ = other.id_; return *this; }

    int id() const {return id_;}

    static int count;
};

int A::count = 0;

#endif // A_H

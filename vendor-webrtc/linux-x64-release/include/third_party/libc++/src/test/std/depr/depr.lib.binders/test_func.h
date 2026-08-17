//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_FUNC_H
#define TEST_FUNC_H

class test_func
{
    int id_;
public:
    typedef int first_argument_type;
    typedef double second_argument_type;
    typedef long double result_type;

    explicit test_func(int id) : id_(id) {}

    int id() const {return id_;}

    result_type operator() (const first_argument_type& x, second_argument_type& y) const
        {return x+y;}
    result_type operator() (const first_argument_type& x, const second_argument_type& y) const
        {return x-y;}
    result_type operator() (first_argument_type& x, const second_argument_type& y) const
        {return x*y;}
};

#endif // TEST_FUNC_H

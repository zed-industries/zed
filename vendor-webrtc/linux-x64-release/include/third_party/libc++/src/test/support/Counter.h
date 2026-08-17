//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef COUNTER_H
#define COUNTER_H

#include <functional> // for std::hash

#include "test_macros.h"

struct Counter_base { static int gConstructed; };

template <typename T>
class Counter : public Counter_base
{
public:
    Counter() : data_()                             { ++gConstructed; }
    Counter(const T &data) : data_(data)            { ++gConstructed; }
    Counter(const Counter& rhs) : data_(rhs.data_)  { ++gConstructed; }
    Counter& operator=(const Counter& rhs)          { data_ = rhs.data_; return *this; }
#if TEST_STD_VER >= 11
    Counter(Counter&& rhs) : data_(std::move(rhs.data_))  { ++gConstructed; }
    Counter& operator=(Counter&& rhs) { data_ = std::move(rhs.data_); return *this; }
#endif
    ~Counter() { --gConstructed; }

    const T& get() const {return data_;}

    bool operator==(const Counter& x) const {return data_ == x.data_;}
    bool operator< (const Counter& x) const {return data_ <  x.data_;}

private:
    T data_;
};

int Counter_base::gConstructed = 0;

template <class T>
struct std::hash<Counter<T> > {
  typedef Counter<T> argument_type;
  typedef std::size_t result_type;

  std::size_t operator()(const Counter<T>& x) const { return std::hash<T>()(x.get()); }
};

#endif

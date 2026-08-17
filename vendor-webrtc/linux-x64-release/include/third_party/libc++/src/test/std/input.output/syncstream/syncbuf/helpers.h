//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_INPUT_OUTPUT_SYNCSTREAM_SYNCBUF_SYNCSTREAM_SYNCBUF_MEMBERS_H
#define TEST_STD_INPUT_OUTPUT_SYNCSTREAM_SYNCBUF_SYNCSTREAM_SYNCBUF_MEMBERS_H

#include <streambuf>
#include <syncstream>

template <class T>
class test_buf : public std::basic_streambuf<T> {
public:
  int id;

  test_buf(int _id = 0) : id(_id) {}

  T* _pptr() { return this->pptr(); }
};

template <class T, class Alloc = std::allocator<T>>
class test_syncbuf : public std::basic_syncbuf<T, std::char_traits<T>, Alloc> {
public:
  test_syncbuf(test_buf<T>* buf, Alloc alloc) : std::basic_syncbuf<T, std::char_traits<T>, Alloc>(buf, alloc) {}

  void _setp(T* begin, T* end) { return this->setp(begin, end); }
};

template <class T>
struct test_allocator : std::allocator<T> {
  int id;
  test_allocator(int _id = 0) : id(_id) {}
};

#endif // TEST_STD_INPUT_OUTPUT_SYNCSTREAM_SYNCBUF_SYNCSTREAM_SYNCBUF_MEMBERS_H

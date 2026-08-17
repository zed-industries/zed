/* -*- Mode: C++; tab-width: 8; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim: set ts=2 et sw=2 tw=80: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

// Original author: ekr@rtfm.com

#ifndef task_utils_h__
#define task_utils_h__


class gmp_args_base : public GMPTask {
 public:
  void Run() = 0;
  void Destroy() {
    delete this;
  }
};

// The generated file contains four major function templates
// (in variants for arbitrary numbers of arguments up to 10,
// which is why it is machine generated). The four templates
// are:
//
// WrapTask(o, m, ...) -- wraps a member function m of an object ptr o
// WrapTaskRet(o, m, ..., r) -- wraps a member function m of an object ptr o
//                              the function returns something that can
//                              be assigned to *r
// WrapTaskNM(f, ...) -- wraps a function f
// WrapTaskNMRet(f, ..., r) -- wraps a function f that returns something
//                             that can be assigned to *r
//
// All of these template functions return a Task* which can be passed
// to Post().
#include "task_utils_generated.h"

#endif

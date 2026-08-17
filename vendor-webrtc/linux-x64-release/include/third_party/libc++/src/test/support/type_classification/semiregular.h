//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_TYPE_CLASSIFICATION_H
#define TEST_SUPPORT_TYPE_CLASSIFICATION_H

#include "copyable.h"

struct no_default_ctor {
  no_default_ctor(int);
};
struct derived_from_non_default_initializable : no_default_ctor {};
struct has_non_default_initializable {
  no_default_ctor x;
};

struct deleted_default_ctor {
  deleted_default_ctor() = delete;
};
struct derived_from_deleted_default_ctor : deleted_default_ctor {};
struct has_deleted_default_ctor {
  deleted_default_ctor x;
};

#endif // TEST_SUPPORT_TYPE_CLASSIFICATION_H

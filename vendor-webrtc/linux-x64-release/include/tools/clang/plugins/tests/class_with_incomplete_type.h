// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// Make sure that the plugin doesn't crash when trying to calculate the weight
// of a class that has fields of incomplete type.

#ifndef CLASS_WITH_INCOMPLETE_TYPE_H_
#define CLASS_WITH_INCOMPLETE_TYPE_H_

struct B;

struct A {
  ~A();
  B incomplete;
};

#endif  // CLASS_WITH_INCOMPLETE_TYPE_H_

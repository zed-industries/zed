// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_MAP_TEST_UTIL_H__
#define GOOGLE_PROTOBUF_MAP_TEST_UTIL_H__

#include "google/protobuf/map_unittest.pb.h"
#include "google/protobuf/reflection_tester.h"
#include "google/protobuf/unittest.pb.h"

#define UNITTEST ::proto2_unittest
#define BRIDGE_UNITTEST ::google::protobuf::bridge_unittest

// Must be included after defining UNITTEST, etc.
#include "google/protobuf/map_test_util.inc"

#undef UNITTEST
#undef BRIDGE_UNITTEST

#endif  // GOOGLE_PROTOBUF_MAP_TEST_UTIL_H__

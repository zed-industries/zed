// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// This header declares the namespace google::protobuf::proto2_unittest in order to expose
// any problems with the generated class names. We use this header to ensure
// unittest.cc will declare the namespace prior to other includes, while obeying
// normal include ordering.
//
// When generating a class name of "foo.Bar" we must ensure we prefix the class
// name with "::", in case the namespace google::protobuf::foo exists. We intentionally
// trigger that case here by declaring google::protobuf::proto2_unittest.
//
// See ClassName in helpers.h for more details.

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_UNITTEST_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_UNITTEST_H__

namespace google {
namespace protobuf {
namespace proto2_unittest {}
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_UNITTEST_H__

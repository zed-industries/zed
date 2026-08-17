// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// This file exists solely to document the google::protobuf::io namespace.
// It is not compiled into anything, but it may be read by an automated
// documentation generator.

namespace google {
namespace protobuf {

// Auxiliary classes used for I/O.
//
// The Protocol Buffer library uses the classes in this package to deal with
// I/O and encoding/decoding raw bytes.  Most users will not need to
// deal with this package.  However, users who want to adapt the system to
// work with their own I/O abstractions -- e.g., to allow Protocol Buffers
// to be read from a different kind of input stream without the need for a
// temporary buffer -- should take a closer look.
namespace io {}

}  // namespace protobuf
}  // namespace google

// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PHP_PROTOBUF_NAMES_H_
#define PHP_PROTOBUF_NAMES_H_

#include "php-upb.h"

// Translates a protobuf symbol name (eg. foo.bar.Baz) into a PHP class name
// (eg. \Foo\Bar\Baz).
char* GetPhpClassname(const upb_FileDef* file, const char* fullname,
                      bool previous);
bool IsPreviouslyUnreservedClassName(const char* fullname);

#endif  // PHP_PROTOBUF_NAMES_H_

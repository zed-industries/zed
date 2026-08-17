// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PHP_PROTOBUF_ARENA_H_
#define PHP_PROTOBUF_ARENA_H_

#include <php.h>

#include "php-upb.h"

// Registers the PHP Arena class.
void Arena_ModuleInit();

// Creates and returns a new arena object that wraps a new upb_Arena*.
void Arena_Init(zval* val);

// Gets the underlying upb_Arena from this arena object.
upb_Arena* Arena_Get(zval* arena);

#endif  // PHP_PROTOBUF_ARENA_H_

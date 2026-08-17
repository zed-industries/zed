// Protocol Buffers - Google's data interchange format
// Copyright 2022 Google Inc.  All rights reserved.
// https://developers.google.com/protocol-buffers/
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// ! THIS FILE ONLY APPROACHING IN-TREE PHP EXTENSION BUILD !
// ! DOES NOT USE NORMALLY.                                 !
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
#ifndef PHP_PROTOBUF_H
#define PHP_PROTOBUF_H

#ifdef HAVE_CONFIG_H
#include "config.h"
#endif

extern zend_module_entry protobuf_module_entry;
#define phpext_protobuf_ptr &protobuf_module_entry

#endif /* PHP_PROTOBUF_H */

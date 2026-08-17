// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PHP_PROTOBUF_CONVERT_H_
#define PHP_PROTOBUF_CONVERT_H_

#include <php.h>

#include "def.h"
#include "php-upb.h"

upb_CType pbphp_dtype_to_type(upb_FieldType type);

// Converts |php_val| to an int64_t. Returns false if the value cannot be
// converted.
bool Convert_PhpToInt64(const zval* php_val, int64_t* i64);

// Converts |php_val| to a upb_MessageValue according to |type|. If type is
// kUpb_CType_Message, then |desc| must be the Descriptor for this message type.
// If type is string, message, or bytes, then |arena| will be used to copy
// string data or fuse this arena to the given message's arena.
bool Convert_PhpToUpb(zval* php_val, upb_MessageValue* upb_val, TypeInfo type,
                      upb_Arena* arena);

// Similar to Convert_PhpToUpb, but supports automatically wrapping the wrapper
// types if a primitive is specified:
//
//   5 -> Int64Wrapper(value=5)
//
// We currently allow this implicit conversion in initializers, but not for
// assignment.
bool Convert_PhpToUpbAutoWrap(zval* val, upb_MessageValue* upb_val,
                              TypeInfo type, upb_Arena* arena);

// Converts |upb_val| to a PHP zval according to |type|. This may involve
// creating a PHP wrapper object. Any newly created wrapper object
// will reference |arena|.
//
// The caller owns a reference to the returned value.
void Convert_UpbToPhp(upb_MessageValue upb_val, zval* php_val, TypeInfo type,
                      zval* arena);

// Registers the GPBUtil class.
void Convert_ModuleInit(void);

#endif  // PHP_PROTOBUF_CONVERT_H_

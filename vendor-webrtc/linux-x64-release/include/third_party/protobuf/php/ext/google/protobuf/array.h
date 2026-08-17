// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PHP_PROTOBUF_ARRAY_H_
#define PHP_PROTOBUF_ARRAY_H_

#include <php.h>

#include "def.h"
#include "php-upb.h"

// Registers PHP classes for RepeatedField.
void Array_ModuleInit();

// Gets a upb_Array* for the PHP object |val|:
//  * If |val| is a RepeatedField object, we first check its type and verify
//    that that the elements have the correct type for |type|. If so, we return
//    the wrapped upb_Array*. We also make sure that this array's arena is fused
//    to |arena|, so the returned upb_Array is guaranteed to live as long as
//    |arena|.
//  * If |val| is a PHP Array, we attempt to create a new upb_Array using
//    |arena| and add all of the PHP elements to it.
//
// If an error occurs, we raise a PHP error and return NULL.
upb_Array* RepeatedField_GetUpbArray(zval* val, TypeInfo type,
                                     upb_Arena* arena);

// Creates a PHP RepeatedField object for the given upb_Array* and |type| and
// returns it in |val|. The PHP object will keep a reference to this |arena| to
// ensure the underlying array data stays alive.
//
// If |arr| is NULL, this will return a PHP null object.
void RepeatedField_GetPhpWrapper(zval* val, upb_Array* arr, TypeInfo type,
                                 zval* arena);

// Returns true if the given arrays are equal. Both arrays must be of this
// |type| and, if the type is |kUpb_CType_Message|, must have the same |m|.
bool ArrayEq(const upb_Array* a1, const upb_Array* a2, TypeInfo type);

#endif  // PHP_PROTOBUF_ARRAY_H_

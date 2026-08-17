// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PHP_PROTOBUF_MAP_H_
#define PHP_PROTOBUF_MAP_H_

#include <php.h>

#include "def.h"
#include "php-upb.h"

void Map_ModuleInit();

typedef struct {
  upb_CType key_type;
  TypeInfo val_type;
} MapField_Type;

MapField_Type MapType_Get(const upb_FieldDef* f);

// Gets a upb_Map* for the PHP object |val|:
//  * If |val| is a RepeatedField object, we first check its type and verify
//    that that the elements have the correct type for |f|. If so, we return the
//    wrapped upb_Map*. We also make sure that this map's arena is fused to
//    |arena|, so the returned upb_Map is guaranteed to live as long as
//    |arena|.
//  * If |val| is a PHP Map, we attempt to create a new upb_Map using
//    |arena| and add all of the PHP elements to it.
//
// If an error occurs, we raise a PHP error and return NULL.
upb_Map* MapField_GetUpbMap(zval* val, MapField_Type type, upb_Arena* arena);

// Creates a PHP MapField object for the given upb_Map* and |f| and returns it
// in |val|. The PHP object will keep a reference to this |arena| to ensure the
// underlying array data stays alive.
//
// If |map| is NULL, this will return a PHP null object.
void MapField_GetPhpWrapper(zval* val, upb_Map* arr, MapField_Type type,
                            zval* arena);

bool MapEq(const upb_Map* m1, const upb_Map* m2, MapField_Type type);

#endif  // PHP_PROTOBUF_MAP_H_

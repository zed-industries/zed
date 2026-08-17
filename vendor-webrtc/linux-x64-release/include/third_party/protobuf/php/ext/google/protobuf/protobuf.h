// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PHP_PROTOBUF_H_
#define PHP_PROTOBUF_H_

#include <php.h>
#include <stdbool.h>

#include "php-upb.h"

upb_DefPool* get_global_symtab();

// In PHP 8.1, mismatched tentative return types emit a deprecation notice.
// https://wiki.php.net/rfc/internal_method_return_types
//
// When compiling for earlier php versions, the return type is dropped.
#if PHP_VERSION_ID < 80100
#define ZEND_BEGIN_ARG_WITH_TENTATIVE_RETURN_TYPE_INFO_EX(       \
    name, return_reference, required_num_args, type, allow_null) \
  ZEND_BEGIN_ARG_INFO_EX(name, return_reference, required_num_args, allow_null)
#endif

ZEND_BEGIN_ARG_INFO(arginfo_void, 0)
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_INFO_EX(arginfo_setter, 0, 0, 1)
ZEND_ARG_INFO(0, value)
ZEND_END_ARG_INFO()

#define PHP_PROTOBUF_VERSION "4.30.2"

// ptr -> PHP object cache. This is a weak map that caches lazily-created
// wrapper objects around upb types:
//  * upb_Message* -> Message
//  * upb_Array* -> RepeatedField
//  * upb_Map*, -> MapField
//  * upb_MessageDef* -> Descriptor
//  * upb_EnumDef* -> EnumDescriptor
//  * upb_MessageDef* -> Descriptor
//
// Each wrapped object should add itself to the map when it is constructed, and
// remove itself from the map when it is destroyed. This is how we ensure that
// the map only contains live objects. The map is weak so it does not actually
// take references to the cached objects.
void ObjCache_Add(const void* key, zend_object* php_obj);
void ObjCache_Delete(const void* key);
bool ObjCache_Get(const void* key, zval* val);

// PHP class name map. This is necessary because the pb_name->php_class_name
// transformation is non-reversible, so when we need to look up a msgdef or
// enumdef by PHP class, we can't turn the class name into a pb_name.
//  * php_class_name -> upb_MessageDef*
//  * php_class_name -> upb_EnumDef*
void NameMap_AddMessage(const upb_MessageDef* m);
void NameMap_AddEnum(const upb_EnumDef* m);
const upb_MessageDef* NameMap_GetMessage(zend_class_entry* ce);
const upb_EnumDef* NameMap_GetEnum(zend_class_entry* ce);
void NameMap_EnterConstructor(zend_class_entry* ce);
void NameMap_ExitConstructor(zend_class_entry* ce);

// Add this descriptor object to the global list of descriptors that will be
// kept alive for the duration of the request but destroyed when the request
// is ending.
void Descriptors_Add(zend_object* desc);

// We need our own assert() because PHP takes control of NDEBUG in its headers.
#ifdef PBPHP_ENABLE_ASSERTS
#define PBPHP_ASSERT(x)                                                    \
  do {                                                                     \
    if (!(x)) {                                                            \
      fprintf(stderr, "Assertion failure at %s:%d %s", __FILE__, __LINE__, \
              #x);                                                         \
      abort();                                                             \
    }                                                                      \
  } while (false)
#else
#define PBPHP_ASSERT(x) \
  do {                  \
  } while (false && (x))
#endif

#endif  // PHP_PROTOBUF_H_

// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_APPLE_FOUNDATION_UTIL_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_APPLE_FOUNDATION_UTIL_H_

#include <CoreFoundation/CoreFoundation.h>

#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base::apple {

// CFCast<>() and CFCastStrict<>() cast a basic CFTypeRef to a more specific
// CoreFoundation type. The compatibility of the passed object is found by
// comparing its opaque type against the requested type identifier. If the
// supplied object is not compatible with the requested return type, CFCast<>()
// returns null and CFCastStrict<>() will CHECK. Providing a null pointer to
// either variant results in null being returned without triggering any CHECK.
//
// Example usage:
// CFNumberRef some_number = base::mac::CFCast<CFNumberRef>(
//     CFArrayGetValueAtIndex(array, index));
//
// CFTypeRef hello = CFSTR("hello world");
// CFStringRef some_string = base::mac::CFCastStrict<CFStringRef>(hello);

template <typename T>
T CFCast(const CFTypeRef& cf_val);

template <typename T>
T CFCastStrict(const CFTypeRef& cf_val);

#define PA_CF_CAST_DECL(TypeCF)                             \
  template <>                                               \
  PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)                 \
  TypeCF##Ref CFCast<TypeCF##Ref>(const CFTypeRef& cf_val); \
                                                            \
  template <>                                               \
  PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)                 \
  TypeCF##Ref CFCastStrict<TypeCF##Ref>(const CFTypeRef& cf_val)

PA_CF_CAST_DECL(CFArray);
PA_CF_CAST_DECL(CFBag);
PA_CF_CAST_DECL(CFBoolean);
PA_CF_CAST_DECL(CFData);
PA_CF_CAST_DECL(CFDate);
PA_CF_CAST_DECL(CFDictionary);
PA_CF_CAST_DECL(CFNull);
PA_CF_CAST_DECL(CFNumber);
PA_CF_CAST_DECL(CFSet);
PA_CF_CAST_DECL(CFString);
PA_CF_CAST_DECL(CFURL);
PA_CF_CAST_DECL(CFUUID);

#undef PA_CF_CAST_DECL

}  // namespace partition_alloc::internal::base::apple

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_APPLE_FOUNDATION_UTIL_H_

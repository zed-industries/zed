// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

#ifndef UPB_REFLECTION_DEF_POOL_H_
#define UPB_REFLECTION_DEF_POOL_H_

#include "upb/base/status.h"
#include "upb/base/string_view.h"
#include "upb/reflection/common.h"
#include "upb/reflection/def_type.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

UPB_API void upb_DefPool_Free(upb_DefPool* s);

UPB_API upb_DefPool* upb_DefPool_New(void);

UPB_API const UPB_DESC(FeatureSetDefaults) *
    upb_DefPool_FeatureSetDefaults(const upb_DefPool* s);

UPB_API bool upb_DefPool_SetFeatureSetDefaults(upb_DefPool* s,
                                               const char* serialized_defaults,
                                               size_t serialized_len,
                                               upb_Status* status);

UPB_API const upb_MessageDef* upb_DefPool_FindMessageByName(
    const upb_DefPool* s, const char* sym);

const upb_MessageDef* upb_DefPool_FindMessageByNameWithSize(
    const upb_DefPool* s, const char* sym, size_t len);

UPB_API const upb_EnumDef* upb_DefPool_FindEnumByName(const upb_DefPool* s,
                                                      const char* sym);

const upb_EnumValueDef* upb_DefPool_FindEnumByNameval(const upb_DefPool* s,
                                                      const char* sym);

const upb_FileDef* upb_DefPool_FindFileByName(const upb_DefPool* s,
                                              const char* name);

const upb_FileDef* upb_DefPool_FindFileByNameWithSize(const upb_DefPool* s,
                                                      const char* name,
                                                      size_t len);

const upb_FieldDef* upb_DefPool_FindExtensionByMiniTable(
    const upb_DefPool* s, const upb_MiniTableExtension* ext);

UPB_API const upb_FieldDef* upb_DefPool_FindExtensionByName(const upb_DefPool* s,
                                                    const char* sym);

const upb_FieldDef* upb_DefPool_FindExtensionByNameWithSize(
    const upb_DefPool* s, const char* name, size_t size);

const upb_FieldDef* upb_DefPool_FindExtensionByNumber(const upb_DefPool* s,
                                                      const upb_MessageDef* m,
                                                      int32_t fieldnum);

UPB_API const upb_ServiceDef* upb_DefPool_FindServiceByName(
  const upb_DefPool* s, const char* name);

const upb_ServiceDef* upb_DefPool_FindServiceByNameWithSize(
    const upb_DefPool* s, const char* name, size_t size);

const upb_FileDef* upb_DefPool_FindFileContainingSymbol(const upb_DefPool* s,
                                                        const char* name);

UPB_API const upb_FileDef* upb_DefPool_AddFile(
    upb_DefPool* s, const UPB_DESC(FileDescriptorProto) * file_proto,
    upb_Status* status);

UPB_API const upb_ExtensionRegistry* upb_DefPool_ExtensionRegistry(
    const upb_DefPool* s);

const upb_FieldDef** upb_DefPool_GetAllExtensions(const upb_DefPool* s,
                                                  const upb_MessageDef* m,
                                                  size_t* count);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_DEF_POOL_H_ */

// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_FILE_DEF_INTERNAL_H_
#define UPB_REFLECTION_FILE_DEF_INTERNAL_H_

#include "upb/reflection/file_def.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

const upb_MiniTableExtension* _upb_FileDef_ExtensionMiniTable(
    const upb_FileDef* f, int i);
const int32_t* _upb_FileDef_PublicDependencyIndexes(const upb_FileDef* f);
const int32_t* _upb_FileDef_WeakDependencyIndexes(const upb_FileDef* f);

// upb_FileDef_Package() returns "" if f->package is NULL, this does not.
const char* _upb_FileDef_RawPackage(const upb_FileDef* f);

void _upb_FileDef_Create(upb_DefBuilder* ctx,
                         const UPB_DESC(FileDescriptorProto) * file_proto);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_FILE_DEF_INTERNAL_H_ */

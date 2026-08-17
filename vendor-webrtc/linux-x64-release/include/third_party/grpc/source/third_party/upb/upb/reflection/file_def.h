// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

#ifndef UPB_REFLECTION_FILE_DEF_H_
#define UPB_REFLECTION_FILE_DEF_H_

#include "upb/reflection/common.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

UPB_API const char* upb_FileDef_EditionName(int edition);

const upb_FileDef* upb_FileDef_Dependency(const upb_FileDef* f, int i);
int upb_FileDef_DependencyCount(const upb_FileDef* f);
bool upb_FileDef_HasOptions(const upb_FileDef* f);
UPB_API const char* upb_FileDef_Name(const upb_FileDef* f);
const UPB_DESC(FileOptions) * upb_FileDef_Options(const upb_FileDef* f);
const UPB_DESC(FeatureSet) * upb_FileDef_ResolvedFeatures(const upb_FileDef* f);
const char* upb_FileDef_Package(const upb_FileDef* f);
UPB_DESC(Edition) upb_FileDef_Edition(const upb_FileDef* f);
UPB_API const upb_DefPool* upb_FileDef_Pool(const upb_FileDef* f);

const upb_FileDef* upb_FileDef_PublicDependency(const upb_FileDef* f, int i);
int upb_FileDef_PublicDependencyCount(const upb_FileDef* f);

const upb_ServiceDef* upb_FileDef_Service(const upb_FileDef* f, int i);
int upb_FileDef_ServiceCount(const upb_FileDef* f);

UPB_API upb_Syntax upb_FileDef_Syntax(const upb_FileDef* f);

const upb_EnumDef* upb_FileDef_TopLevelEnum(const upb_FileDef* f, int i);
int upb_FileDef_TopLevelEnumCount(const upb_FileDef* f);

const upb_FieldDef* upb_FileDef_TopLevelExtension(const upb_FileDef* f, int i);
int upb_FileDef_TopLevelExtensionCount(const upb_FileDef* f);

const upb_MessageDef* upb_FileDef_TopLevelMessage(const upb_FileDef* f, int i);
int upb_FileDef_TopLevelMessageCount(const upb_FileDef* f);

const upb_FileDef* upb_FileDef_WeakDependency(const upb_FileDef* f, int i);
int upb_FileDef_WeakDependencyCount(const upb_FileDef* f);

// Returns whether |symbol| is transitively included by |f|
bool upb_FileDef_Resolves(const upb_FileDef* f, const char* symbol);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_FILE_DEF_H_ */

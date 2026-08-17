// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_UTIL_DEF_TO_PROTO_H_
#define UPB_UTIL_DEF_TO_PROTO_H_

#include "upb/reflection/def.h"

#ifdef __cplusplus
extern "C" {
#endif

// Functions for converting defs back to the equivalent descriptor proto.
// Ultimately the goal is that a round-trip proto->def->proto is lossless.  Each
// function returns a new proto created in arena `a`, or NULL if memory
// allocation failed.
google_protobuf_DescriptorProto* upb_MessageDef_ToProto(const upb_MessageDef* m,
                                                        upb_Arena* a);
google_protobuf_EnumDescriptorProto* upb_EnumDef_ToProto(const upb_EnumDef* e,
                                                         upb_Arena* a);
google_protobuf_EnumValueDescriptorProto* upb_EnumValueDef_ToProto(
    const upb_EnumValueDef* e, upb_Arena* a);
google_protobuf_FieldDescriptorProto* upb_FieldDef_ToProto(
    const upb_FieldDef* f, upb_Arena* a);
google_protobuf_OneofDescriptorProto* upb_OneofDef_ToProto(
    const upb_OneofDef* o, upb_Arena* a);
google_protobuf_FileDescriptorProto* upb_FileDef_ToProto(const upb_FileDef* f,
                                                         upb_Arena* a);
google_protobuf_MethodDescriptorProto* upb_MethodDef_ToProto(
    const upb_MethodDef* m, upb_Arena* a);
google_protobuf_ServiceDescriptorProto* upb_ServiceDef_ToProto(
    const upb_ServiceDef* s, upb_Arena* a);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* UPB_UTIL_DEF_TO_PROTO_H_ */

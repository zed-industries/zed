/* -*- mode: C; c-file-style: "gnu"; indent-tabs-mode: nil; -*-
 * GObject introspection: types
 *
 * Copyright (C) 2005 Matthias Clasen
 * Copyright (C) 2008,2009 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GITYPES_H__
#define __GITYPES_H__

#if !defined (__GIREPOSITORY_H_INSIDE__) && !defined (GI_COMPILATION)
#error "Only <girepository.h> can be included directly."
#endif

#include <giversionmacros.h>

G_BEGIN_DECLS

typedef struct _GIBaseInfoStub {
  /* <private> */
  gint32 dummy1;
  gint32 dummy2;
  gpointer dummy3;
  gpointer dummy4;
  gpointer dummy5;
  guint32  dummy6;
  guint32  dummy7;
  gpointer padding[4];
} GIBaseInfo;

/**
 * GICallableInfo:
 *
 * Represents a callable, either #GIFunctionInfo, #GICallbackInfo or
 * #GIVFuncInfo.
 */
typedef GIBaseInfo GICallableInfo;

/**
 * GIFunctionInfo:
 *
 * Represents a function, eg arguments and return value.
 */
typedef GIBaseInfo GIFunctionInfo;

/**
 * SECTION:gicallbackinfo
 * @title: GICallbackInfo
 * @short_description: Struct representing a callback
 *
 * GICallbackInfo represents a callback.
 *
 * <refsect1 id="gi-gicallbackinfo.struct-hierarchy" role="struct_hierarchy">
 * <title role="struct_hierarchy.title">Struct hierarchy</title>
 * <synopsis>
 *   <link linkend="GIBaseInfo">GIBaseInfo</link>
 *    +----<link linkend="gi-GICallableInfo">GICallableInfo</link>
 *          +----GIFunctionInfo
 *          +----<link linkend="gi-GISignalInfo">GISignalInfo</link>
 *          +----<link linkend="gi-GIVFuncInfo">GIVFuncInfo</link>
 * </synopsis>
 * </refsect1>
 */

/**
 * GICallbackInfo:
 *
 * Represents a callback, eg arguments and return value.
 */
typedef GIBaseInfo GICallbackInfo;

/**
 * GIRegisteredTypeInfo:
 *
 * Represent a registered type.
 */
typedef GIBaseInfo GIRegisteredTypeInfo;

/**
 * GIStructInfo:
 *
 * Represents a struct.
 */
typedef GIBaseInfo GIStructInfo;

/**
 * GIUnionInfo:
 *
 * Represents a union.
 */
typedef GIBaseInfo GIUnionInfo;

/**
 * GIEnumInfo:
 *
 * Represents an enum or a flag.
 */
typedef GIBaseInfo GIEnumInfo;

/**
 * GIObjectInfo:
 *
 * Represents an object.
 */
typedef GIBaseInfo GIObjectInfo;

/**
 * GIInterfaceInfo:
 *
 * Represents an interface.
 */
typedef GIBaseInfo GIInterfaceInfo;

/**
 * GIConstantInfo:
 *
 * Represents a constant.
 */
typedef GIBaseInfo GIConstantInfo;

/**
 * SECTION:givalueinfo
 * @title: GIValueInfo
 * @short_description: Struct representing a value
 *
 * GIValueInfo represents a value.
 *
 * <refsect1 id="gi-givalueinfo.struct-hierarchy" role="struct_hierarchy">
 * <title role="struct_hierarchy.title">Struct hierarchy</title>
 * <synopsis>
 *   <link linkend="GIBaseInfo">GIBaseInfo</link>
 *    +----GIValueInfo
 * </synopsis>
 * </refsect1>
 */

/**
 * GIValueInfo:
 *
 * Represents a enum value of a #GIEnumInfo.
 */
typedef GIBaseInfo GIValueInfo;

/**
 * GISignalInfo:
 *
 * Represents a signal.
 */
typedef GIBaseInfo GISignalInfo;

/**
 * GIVFuncInfo:
 *
 * Represents a virtual function.
 */
typedef GIBaseInfo GIVFuncInfo;

/**
 * GIPropertyInfo:
 *
 * Represents a property of a #GIObjectInfo or a #GIInterfaceInfo.
 */
typedef GIBaseInfo GIPropertyInfo;

/**
 * GIFieldInfo:
 *
 * Represents a field of a #GIStructInfo or a #GIUnionInfo.
 */
typedef GIBaseInfo GIFieldInfo;

/**
 * GIArgInfo:
 *
 * Represents an argument.
 */
typedef GIBaseInfo GIArgInfo;

/**
 * GITypeInfo:
 *
 * Represents type information, direction, transfer etc.
 */
typedef GIBaseInfo GITypeInfo;

/**
 * GIUnresolvedInfo:
 *
 * Represents a unresolved type in a typelib.
 */
typedef struct _GIUnresolvedInfo GIUnresolvedInfo;

union _GIArgument
{
  gboolean v_boolean;
  gint8    v_int8;
  guint8   v_uint8;
  gint16   v_int16;
  guint16  v_uint16;
  gint32   v_int32;
  guint32  v_uint32;
  gint64   v_int64;
  guint64  v_uint64;
  gfloat   v_float;
  gdouble  v_double;
  gshort   v_short;
  gushort  v_ushort;
  gint     v_int;
  guint    v_uint;
  glong    v_long;
  gulong   v_ulong;
  gssize   v_ssize;
  gsize    v_size;
  gchar *  v_string;
  gpointer v_pointer;
};

/**
 * GIArgument:
 * @v_boolean: TODO
 * @v_int8: TODO
 * @v_uint8: TODO
 * @v_int16: TODO
 * @v_uint16: TODO
 * @v_int32: TODO
 * @v_uint32: TODO
 * @v_int64: TODO
 * @v_uint64: TODO
 * @v_float: TODO
 * @v_double: TODO
 * @v_short: TODO
 * @v_ushort: TODO
 * @v_int: TODO
 * @v_uint: TODO
 * @v_long: TODO
 * @v_ulong: TODO
 * @v_ssize: TODO
 * @v_size: TODO
 * @v_string: TODO
 * @v_pointer: TODO
 *
 * Stores an argument of varying type
 */
typedef union _GIArgument GIArgument;

/**
 * GIInfoType:
 * @GI_INFO_TYPE_INVALID: invalid type
 * @GI_INFO_TYPE_FUNCTION: function, see #GIFunctionInfo
 * @GI_INFO_TYPE_CALLBACK: callback, see #GIFunctionInfo
 * @GI_INFO_TYPE_STRUCT: struct, see #GIStructInfo
 * @GI_INFO_TYPE_BOXED: boxed, see #GIStructInfo or #GIUnionInfo
 * @GI_INFO_TYPE_ENUM: enum, see #GIEnumInfo
 * @GI_INFO_TYPE_FLAGS: flags, see #GIEnumInfo
 * @GI_INFO_TYPE_OBJECT: object, see #GIObjectInfo
 * @GI_INFO_TYPE_INTERFACE: interface, see #GIInterfaceInfo
 * @GI_INFO_TYPE_CONSTANT: contant, see #GIConstantInfo
 * @GI_INFO_TYPE_INVALID_0: deleted, used to be GI_INFO_TYPE_ERROR_DOMAIN.
 * @GI_INFO_TYPE_UNION: union, see #GIUnionInfo
 * @GI_INFO_TYPE_VALUE: enum value, see #GIValueInfo
 * @GI_INFO_TYPE_SIGNAL: signal, see #GISignalInfo
 * @GI_INFO_TYPE_VFUNC: virtual function, see #GIVFuncInfo
 * @GI_INFO_TYPE_PROPERTY: GObject property, see #GIPropertyInfo
 * @GI_INFO_TYPE_FIELD: struct or union field, see #GIFieldInfo
 * @GI_INFO_TYPE_ARG: argument of a function or callback, see #GIArgInfo
 * @GI_INFO_TYPE_TYPE: type information, see #GITypeInfo
 * @GI_INFO_TYPE_UNRESOLVED: unresolved type, a type which is not present in
 *   the typelib, or any of its dependencies.
 *
 * The type of a GIBaseInfo struct.
 */
typedef enum
{
  GI_INFO_TYPE_INVALID,
  GI_INFO_TYPE_FUNCTION,
  GI_INFO_TYPE_CALLBACK,
  GI_INFO_TYPE_STRUCT,
  GI_INFO_TYPE_BOXED,
  GI_INFO_TYPE_ENUM,         /*  5 */
  GI_INFO_TYPE_FLAGS,
  GI_INFO_TYPE_OBJECT,
  GI_INFO_TYPE_INTERFACE,
  GI_INFO_TYPE_CONSTANT,
  GI_INFO_TYPE_INVALID_0,    /* 10 */
  GI_INFO_TYPE_UNION,
  GI_INFO_TYPE_VALUE,
  GI_INFO_TYPE_SIGNAL,
  GI_INFO_TYPE_VFUNC,
  GI_INFO_TYPE_PROPERTY,     /* 15 */
  GI_INFO_TYPE_FIELD,
  GI_INFO_TYPE_ARG,
  GI_INFO_TYPE_TYPE,
  GI_INFO_TYPE_UNRESOLVED
} GIInfoType;

/**
 * GITransfer:
 * @GI_TRANSFER_NOTHING: transfer nothing from the callee (function or the type
 * instance the property belongs to) to the caller. The callee retains the
 * ownership of the transfer and the caller doesn't need to do anything to free
 * up the resources of this transfer.
 * @GI_TRANSFER_CONTAINER: transfer the container (list, array, hash table) from
 * the callee to the caller. The callee retains the ownership of the individual
 * items in the container and the caller has to free up the container resources
 * (g_list_free()/g_hash_table_destroy() etc) of this transfer.
 * @GI_TRANSFER_EVERYTHING: transfer everything, eg the container and its
 * contents from the callee to the caller. This is the case when the callee
 * creates a copy of all the data it returns. The caller is responsible for
 * cleaning up the container and item resources of this transfer.
 *
 * The transfer is the exchange of data between two parts, from the callee to
 * the caller. The callee is either a function/method/signal or an
 * object/interface where a property is defined. The caller is the side
 * accessing a property or calling a function.
 * #GITransfer specifies who's responsible for freeing the resources after the
 * ownership transfer is complete. In case of a containing type such as a list,
 * an array or a hash table the container itself is specified differently from
 * the items within the container itself. Each container is freed differently,
 * check the documentation for the types themselves for information on how to
 * free them.
 */
typedef enum {
  GI_TRANSFER_NOTHING,
  GI_TRANSFER_CONTAINER,
  GI_TRANSFER_EVERYTHING
} GITransfer;

/**
 * GIDirection:
 * @GI_DIRECTION_IN: in argument.
 * @GI_DIRECTION_OUT: out argument.
 * @GI_DIRECTION_INOUT: in and out argument.
 *
 * The direction of a #GIArgInfo.
 */
typedef enum  {
  GI_DIRECTION_IN,
  GI_DIRECTION_OUT,
  GI_DIRECTION_INOUT
} GIDirection;

/**
 * GIScopeType:
 * @GI_SCOPE_TYPE_INVALID: The argument is not of callback type.
 * @GI_SCOPE_TYPE_CALL: The callback and associated user_data is only
 * used during the call to this function.
 * @GI_SCOPE_TYPE_ASYNC: The callback and associated user_data is
 * only used until the callback is invoked, and the callback.
 * is invoked always exactly once.
 * @GI_SCOPE_TYPE_NOTIFIED: The callback and and associated
 * user_data is used until the caller is notfied via the destroy_notify.
 *
 * Scope type of a #GIArgInfo representing callback, determines how the
 * callback is invoked and is used to decided when the invoke structs
 * can be freed.
 */
typedef enum {
  GI_SCOPE_TYPE_INVALID,
  GI_SCOPE_TYPE_CALL,
  GI_SCOPE_TYPE_ASYNC,
  GI_SCOPE_TYPE_NOTIFIED
} GIScopeType;

/**
 * GITypeTag:
 * @GI_TYPE_TAG_VOID: void
 * @GI_TYPE_TAG_BOOLEAN: boolean
 * @GI_TYPE_TAG_INT8: 8-bit signed integer
 * @GI_TYPE_TAG_UINT8: 8-bit unsigned integer
 * @GI_TYPE_TAG_INT16: 16-bit signed integer
 * @GI_TYPE_TAG_UINT16: 16-bit unsigned integer
 * @GI_TYPE_TAG_INT32: 32-bit signed integer
 * @GI_TYPE_TAG_UINT32: 32-bit unsigned integer
 * @GI_TYPE_TAG_INT64: 64-bit signed integer
 * @GI_TYPE_TAG_UINT64: 64-bit unsigned integer
 * @GI_TYPE_TAG_FLOAT: float
 * @GI_TYPE_TAG_DOUBLE: double floating point
 * @GI_TYPE_TAG_GTYPE: a #GType
 * @GI_TYPE_TAG_UTF8: a UTF-8 encoded string
 * @GI_TYPE_TAG_FILENAME: a filename, encoded in the same encoding
 *   as the native filesystem is using.
 * @GI_TYPE_TAG_ARRAY: an array
 * @GI_TYPE_TAG_INTERFACE: an extended interface object
 * @GI_TYPE_TAG_GLIST: a #GList
 * @GI_TYPE_TAG_GSLIST: a #GSList
 * @GI_TYPE_TAG_GHASH: a #GHashTable
 * @GI_TYPE_TAG_ERROR: a #GError
 * @GI_TYPE_TAG_UNICHAR: Unicode character
 *
 * The type tag of a #GITypeInfo.
 */
typedef enum {
  /* Basic types */
  GI_TYPE_TAG_VOID      =  0,
  GI_TYPE_TAG_BOOLEAN   =  1,
  GI_TYPE_TAG_INT8      =  2,
  GI_TYPE_TAG_UINT8     =  3,
  GI_TYPE_TAG_INT16     =  4,
  GI_TYPE_TAG_UINT16    =  5,
  GI_TYPE_TAG_INT32     =  6,
  GI_TYPE_TAG_UINT32    =  7,
  GI_TYPE_TAG_INT64     =  8,
  GI_TYPE_TAG_UINT64    =  9,
  GI_TYPE_TAG_FLOAT     = 10,
  GI_TYPE_TAG_DOUBLE    = 11,
  GI_TYPE_TAG_GTYPE     = 12,
  GI_TYPE_TAG_UTF8      = 13,
  GI_TYPE_TAG_FILENAME  = 14,
  /* Non-basic types; compare with G_TYPE_TAG_IS_BASIC */
  GI_TYPE_TAG_ARRAY     = 15,
  GI_TYPE_TAG_INTERFACE = 16,
  GI_TYPE_TAG_GLIST     = 17,
  GI_TYPE_TAG_GSLIST    = 18,
  GI_TYPE_TAG_GHASH     = 19,
  GI_TYPE_TAG_ERROR     = 20,
  /* Another basic type */
  GI_TYPE_TAG_UNICHAR   = 21
  /* Note - there is currently only room for 32 tags */
} GITypeTag;

/**
 * GI_TYPE_TAG_N_TYPES:
 *
 * TODO
 */
#define GI_TYPE_TAG_N_TYPES (GI_TYPE_TAG_UNICHAR+1)

#ifndef __GTK_DOC_IGNORE__
/* These were removed and no longer appear in the typelib;
 * instead, the machine-specific versions like INT32 are
 * always used.
 */
#define GI_TYPE_TAG_SHORT GI_TYPE_TAG_SHORT_WAS_REMOVED
#define GI_TYPE_TAG_INT   GI_TYPE_TAG_INT_WAS_REMOVED
#define GI_TYPE_TAG_LONG  GI_TYPE_TAG_LONG_WAS_REMOVED
#endif

/**
 * GIArrayType:
 * @GI_ARRAY_TYPE_C: a C array, char[] for instance
 * @GI_ARRAY_TYPE_ARRAY: a @GArray array
 * @GI_ARRAY_TYPE_PTR_ARRAY: a #GPtrArray array
 * @GI_ARRAY_TYPE_BYTE_ARRAY: a #GByteArray array
 *
 * The type of array in a #GITypeInfo.
 */
typedef enum {
  GI_ARRAY_TYPE_C,
  GI_ARRAY_TYPE_ARRAY,
  GI_ARRAY_TYPE_PTR_ARRAY,
  GI_ARRAY_TYPE_BYTE_ARRAY
} GIArrayType;

/**
 * GIFieldInfoFlags:
 * @GI_FIELD_IS_READABLE: field is readable.
 * @GI_FIELD_IS_WRITABLE: field is writable.
 *
 * Flags for a #GIFieldInfo.
 */

typedef enum
{
  GI_FIELD_IS_READABLE = 1 << 0,
  GI_FIELD_IS_WRITABLE = 1 << 1
} GIFieldInfoFlags;

/**
 * GIVFuncInfoFlags:
 * @GI_VFUNC_MUST_CHAIN_UP: chains up to the parent type
 * @GI_VFUNC_MUST_OVERRIDE: overrides
 * @GI_VFUNC_MUST_NOT_OVERRIDE: does not override
 * @GI_VFUNC_THROWS: Includes a #GError
 *
 * Flags of a #GIVFuncInfo struct.
 */
typedef enum
{
  GI_VFUNC_MUST_CHAIN_UP     = 1 << 0,
  GI_VFUNC_MUST_OVERRIDE     = 1 << 1,
  GI_VFUNC_MUST_NOT_OVERRIDE = 1 << 2,
  GI_VFUNC_THROWS =            1 << 3
} GIVFuncInfoFlags;

/**
 * GIFunctionInfoFlags:
 * @GI_FUNCTION_IS_METHOD: is a method.
 * @GI_FUNCTION_IS_CONSTRUCTOR: is a constructor.
 * @GI_FUNCTION_IS_GETTER: is a getter of a #GIPropertyInfo.
 * @GI_FUNCTION_IS_SETTER: is a setter of a #GIPropertyInfo.
 * @GI_FUNCTION_WRAPS_VFUNC: represents a virtual function.
 * @GI_FUNCTION_THROWS: the function may throw an error.
 *
 * Flags for a #GIFunctionInfo struct.
 */
typedef enum
{
  GI_FUNCTION_IS_METHOD      = 1 << 0,
  GI_FUNCTION_IS_CONSTRUCTOR = 1 << 1,
  GI_FUNCTION_IS_GETTER      = 1 << 2,
  GI_FUNCTION_IS_SETTER      = 1 << 3,
  GI_FUNCTION_WRAPS_VFUNC    = 1 << 4,
  GI_FUNCTION_THROWS         = 1 << 5
} GIFunctionInfoFlags;

#ifndef __GI_SCANNER__
#ifndef __GTK_DOC_IGNORE__
/* backwards compatibility */
typedef GIArgument GArgument;
typedef struct _GITypelib GTypelib;
#endif
#endif

G_END_DECLS

#endif  /* __GITYPES_H__ */


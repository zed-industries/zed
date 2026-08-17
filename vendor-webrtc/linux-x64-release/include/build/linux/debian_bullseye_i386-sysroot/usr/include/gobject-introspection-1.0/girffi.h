/* -*- mode: C; c-file-style: "gnu"; indent-tabs-mode: nil; -*-
 * GObject introspection: Helper functions for ffi integration
 *
 * Copyright (C) 2008 Red Hat, Inc
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

#ifndef __GIRFFI_H__
#define __GIRFFI_H__

#include <ffi.h>
#include "girepository.h"

G_BEGIN_DECLS

/**
 * GIFFIClosureCallback:
 * @Param1: TODO
 * @Param2: TODO
 * @Param3: TODO
 * @Param4: TODO
 *
 * TODO
 */
typedef void (*GIFFIClosureCallback) (ffi_cif *,
                                      void *,
                                      void **,
                                      void *);

/**
 * GIFunctionInvoker:
 * @cif: the cif
 * @native_address: the native address
 *
 * TODO
 */
typedef struct _GIFunctionInvoker GIFunctionInvoker;

struct _GIFunctionInvoker {
  ffi_cif cif;
  gpointer native_address;
  /* <private> */
  gpointer padding[3];
};

/**
 * GIFFIReturnValue:
 *
 * TODO
 */
typedef GIArgument GIFFIReturnValue;

GI_AVAILABLE_IN_ALL
ffi_type *    gi_type_tag_get_ffi_type            (GITypeTag type_tag, gboolean is_pointer);

GI_AVAILABLE_IN_ALL
ffi_type *    g_type_info_get_ffi_type            (GITypeInfo           *info);

GI_AVAILABLE_IN_1_32
void          gi_type_info_extract_ffi_return_value (GITypeInfo                  *return_info,
                                                     GIFFIReturnValue            *ffi_value,
                                                     GIArgument                  *arg);

GI_AVAILABLE_IN_ALL
gboolean      g_function_info_prep_invoker        (GIFunctionInfo       *info,
                                                   GIFunctionInvoker    *invoker,
                                                   GError              **error);

GI_AVAILABLE_IN_1_32
gboolean      g_function_invoker_new_for_address  (gpointer              addr,
                                                   GICallableInfo       *info,
                                                   GIFunctionInvoker    *invoker,
                                                   GError              **error);

GI_AVAILABLE_IN_ALL
void          g_function_invoker_destroy          (GIFunctionInvoker    *invoker);


GI_AVAILABLE_IN_ALL
ffi_closure * g_callable_info_prepare_closure     (GICallableInfo       *callable_info,
                                                   ffi_cif              *cif,
                                                   GIFFIClosureCallback  callback,
                                                   gpointer              user_data);
GI_AVAILABLE_IN_ALL
void          g_callable_info_free_closure        (GICallableInfo       *callable_info,
                                                   ffi_closure          *closure);

G_END_DECLS

#endif /* __GIRFFI_H__ */

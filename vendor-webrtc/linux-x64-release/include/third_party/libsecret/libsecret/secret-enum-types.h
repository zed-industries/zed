/* libsecret - GLib wrapper for Secret Service
 *
 * Copyright 2012 Red Hat Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published
 * by the Free Software Foundation; either version 2.1 of the licence or (at
 * your option) any later version.
 *
 * See the included COPYING file for more information.
 *
 * Author: Stef Walter <stefw@gnome.org>
 */

/* Generated data (by glib-mkenums) */

#if !defined (__SECRET_INSIDE_HEADER__) && !defined (SECRET_COMPILATION)
#error "Only <secret/secret.h> can be included directly."
#endif

#ifndef __SECRET_ENUM_TYPES_H__
#define __SECRET_ENUM_TYPES_H__

#include <glib-object.h>

G_BEGIN_DECLS

/* enumerations from "secret-collection.h" */
GType secret_collection_flags_get_type (void) G_GNUC_CONST;
#define SECRET_TYPE_COLLECTION_FLAGS (secret_collection_flags_get_type ())
GType secret_collection_create_flags_get_type (void) G_GNUC_CONST;
#define SECRET_TYPE_COLLECTION_CREATE_FLAGS (secret_collection_create_flags_get_type ())

/* enumerations from "secret-item.h" */
GType secret_item_flags_get_type (void) G_GNUC_CONST;
#define SECRET_TYPE_ITEM_FLAGS (secret_item_flags_get_type ())
GType secret_item_create_flags_get_type (void) G_GNUC_CONST;
#define SECRET_TYPE_ITEM_CREATE_FLAGS (secret_item_create_flags_get_type ())

/* enumerations from "secret-schema.h" */
GType secret_schema_attribute_type_get_type (void) G_GNUC_CONST;
#define SECRET_TYPE_SCHEMA_ATTRIBUTE_TYPE (secret_schema_attribute_type_get_type ())
GType secret_schema_flags_get_type (void) G_GNUC_CONST;
#define SECRET_TYPE_SCHEMA_FLAGS (secret_schema_flags_get_type ())

/* enumerations from "secret-service.h" */
GType secret_service_flags_get_type (void) G_GNUC_CONST;
#define SECRET_TYPE_SERVICE_FLAGS (secret_service_flags_get_type ())
GType secret_search_flags_get_type (void) G_GNUC_CONST;
#define SECRET_TYPE_SEARCH_FLAGS (secret_search_flags_get_type ())

/* enumerations from "secret-types.h" */
GType secret_error_get_type (void) G_GNUC_CONST;
#define SECRET_TYPE_ERROR (secret_error_get_type ())
G_END_DECLS

#endif /* __SECRET_ENUM_TYPES_H__ */

/* Generated data ends here */


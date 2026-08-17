/* libsecret - GLib wrapper for Secret Service
 *
 * Copyright 2011 Collabora Ltd.
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

#if !defined (__SECRET_INSIDE_HEADER__) && !defined (SECRET_COMPILATION)
#error "Only <libsecret/secret.h> can be included directly."
#endif

#ifndef __SECRET_VALUE_H__
#define __SECRET_VALUE_H__

#include <gio/gio.h>

#include "secret-types.h"

G_BEGIN_DECLS

typedef struct _SecretValue  SecretValue;

#define             SECRET_TYPE_VALUE              (secret_value_get_type ())

GType               secret_value_get_type          (void) G_GNUC_CONST;

SecretValue *       secret_value_new               (const gchar *secret,
                                                    gssize length,
                                                    const gchar *content_type);

SecretValue *       secret_value_new_full          (gchar *secret,
                                                    gssize length,
                                                    const gchar *content_type,
                                                    GDestroyNotify destroy);

const gchar *       secret_value_get               (SecretValue *value,
                                                    gsize *length);

const gchar *       secret_value_get_text          (SecretValue *value);

const gchar *       secret_value_get_content_type  (SecretValue *value);

SecretValue *       secret_value_ref               (SecretValue *value);

void                secret_value_unref             (gpointer value);

G_END_DECLS

#endif /* __SECRET_VALUE_H___ */

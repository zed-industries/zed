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

#if !defined (__SECRET_INSIDE_HEADER__) && !defined (SECRET_COMPILATION)
#error "Only <libsecret/secret.h> can be included directly."
#endif

#ifndef __SECRET_ATTRIBUTES_H__
#define __SECRET_ATTRIBUTES_H__

#include <glib.h>
#include <stdarg.h>

#include "secret-schema.h"

G_BEGIN_DECLS

GHashTable *         secret_attributes_build         (const SecretSchema *schema,
                                                      ...);

GHashTable *         secret_attributes_buildv        (const SecretSchema *schema,
                                                      va_list va);


G_END_DECLS

#endif /* __SECRET_ATTRIBUTES_H___ */

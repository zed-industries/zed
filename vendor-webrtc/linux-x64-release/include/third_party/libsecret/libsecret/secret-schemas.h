/* libsecret - GLib wrapper for Secret Service
 *
 * Copyright 2012 Stef Walter
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

#ifndef __SECRET_SCHEMAS_H__
#define __SECRET_SCHEMAS_H__

#include <glib.h>

#include "secret-schema.h"

G_BEGIN_DECLS

/*
 * A note or password stored manually by the user.
 */
extern const SecretSchema *  SECRET_SCHEMA_NOTE;

/*
 * This schema is here for compatibility with libgnome-keyring's network
 * password functions.
 */

extern const SecretSchema *  SECRET_SCHEMA_COMPAT_NETWORK;

G_END_DECLS

#endif /* __SECRET_SCHEMAS_H___ */

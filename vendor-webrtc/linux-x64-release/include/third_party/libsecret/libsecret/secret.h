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

#ifndef __SECRET_H__
#define __SECRET_H__

#include <glib.h>

#define __SECRET_INSIDE_HEADER__

#include <libsecret/secret-attributes.h>
#include <libsecret/secret-collection.h>
#include <libsecret/secret-enum-types.h>
#include <libsecret/secret-item.h>
#include <libsecret/secret-password.h>
#include <libsecret/secret-prompt.h>
#include <libsecret/secret-schema.h>
#include <libsecret/secret-schemas.h>
#include <libsecret/secret-service.h>
#include <libsecret/secret-types.h>
#include <libsecret/secret-value.h>

/* SECRET_WITH_UNSTABLE is defined in the secret-unstable.pc pkg-config file */
#if defined(SECRET_WITH_UNSTABLE) || defined(SECRET_API_SUBJECT_TO_CHANGE)

#ifndef SECRET_API_SUBJECT_TO_CHANGE
#warning "Some parts of the libsecret API are unstable. Define SECRET_API_SUBJECT_TO_CHANGE to acknowledge"
#endif

#include <libsecret/secret-paths.h>

#endif /* SECRET_WITH_UNSTABLE || SECRET_API_SUBJECT_TO_CHANGE */

#undef __SECRET_INSIDE_HEADER__

#endif /* __SECRET_H__ */

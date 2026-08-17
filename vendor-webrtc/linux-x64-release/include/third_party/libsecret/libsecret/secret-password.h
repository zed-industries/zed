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

#ifndef __SECRET_PASSWORD_H__
#define __SECRET_PASSWORD_H__

#include <gio/gio.h>

G_BEGIN_DECLS

#include "secret-schema.h"
#include "secret-types.h"

void        secret_password_store                      (const SecretSchema *schema,
                                                        const gchar *collection,
                                                        const gchar *label,
                                                        const gchar *password,
                                                        GCancellable *cancellable,
                                                        GAsyncReadyCallback callback,
                                                        gpointer user_data,
                                                        ...) G_GNUC_NULL_TERMINATED;

void        secret_password_storev                     (const SecretSchema *schema,
                                                        GHashTable *attributes,
                                                        const gchar *collection,
                                                        const gchar *label,
                                                        const gchar *password,
                                                        GCancellable *cancellable,
                                                        GAsyncReadyCallback callback,
                                                        gpointer user_data);

gboolean    secret_password_store_finish               (GAsyncResult *result,
                                                        GError **error);

gboolean    secret_password_store_sync                 (const SecretSchema *schema,
                                                        const gchar *collection,
                                                        const gchar *label,
                                                        const gchar *password,
                                                        GCancellable *cancellable,
                                                        GError **error,
                                                        ...) G_GNUC_NULL_TERMINATED;

gboolean    secret_password_storev_sync                (const SecretSchema *schema,
                                                        GHashTable *attributes,
                                                        const gchar *collection,
                                                        const gchar *label,
                                                        const gchar *password,
                                                        GCancellable *cancellable,
                                                        GError **error);

void        secret_password_lookup                     (const SecretSchema *schema,
                                                        GCancellable *cancellable,
                                                        GAsyncReadyCallback callback,
                                                        gpointer user_data,
                                                        ...) G_GNUC_NULL_TERMINATED;

void        secret_password_lookupv                    (const SecretSchema *schema,
                                                        GHashTable *attributes,
                                                        GCancellable *cancellable,
                                                        GAsyncReadyCallback callback,
                                                        gpointer user_data);

gchar *     secret_password_lookup_finish              (GAsyncResult *result,
                                                        GError **error);

gchar *     secret_password_lookup_nonpageable_finish  (GAsyncResult *result,
                                                        GError **error);

gchar *     secret_password_lookup_sync                (const SecretSchema *schema,
                                                        GCancellable *cancellable,
                                                        GError **error,
                                                        ...) G_GNUC_NULL_TERMINATED;

gchar *     secret_password_lookup_nonpageable_sync    (const SecretSchema *schema,
                                                        GCancellable *cancellable,
                                                        GError **error,
                                                        ...);

gchar *     secret_password_lookupv_sync               (const SecretSchema *schema,
                                                        GHashTable *attributes,
                                                        GCancellable *cancellable,
                                                        GError **error);

gchar *     secret_password_lookupv_nonpageable_sync   (const SecretSchema *schema,
                                                        GHashTable *attributes,
                                                        GCancellable *cancellable,
                                                        GError **error);

void        secret_password_clear                      (const SecretSchema *schema,
                                                        GCancellable *cancellable,
                                                        GAsyncReadyCallback callback,
                                                        gpointer user_data,
                                                        ...) G_GNUC_NULL_TERMINATED;

void        secret_password_clearv                     (const SecretSchema *schema,
                                                        GHashTable *attributes,
                                                        GCancellable *cancellable,
                                                        GAsyncReadyCallback callback,
                                                        gpointer user_data);

gboolean    secret_password_clear_finish               (GAsyncResult *result,
                                                        GError **error);

gboolean    secret_password_clear_sync                 (const SecretSchema* schema,
                                                        GCancellable *cancellable,
                                                        GError **error,
                                                        ...) G_GNUC_NULL_TERMINATED;

gboolean    secret_password_clearv_sync                (const SecretSchema *schema,
                                                        GHashTable *attributes,
                                                        GCancellable *cancellable,
                                                        GError **error);

void        secret_password_free                       (gchar *password);

void        secret_password_wipe                       (gchar *password);

G_END_DECLS

#endif /* __SECRET_PASSWORD_H___ */

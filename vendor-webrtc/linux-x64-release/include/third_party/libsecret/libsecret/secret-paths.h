/* libsecret - GLib wrapper for Secret Service
 *
 * Copyright 2011 Collabora Ltd.
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

#ifndef __SECRET_PATHS_H__
#define __SECRET_PATHS_H__

#include <gio/gio.h>

#include "secret-collection.h"
#include "secret-item.h"
#include "secret-prompt.h"
#include "secret-schema.h"
#include "secret-types.h"
#include "secret-value.h"

G_BEGIN_DECLS


void                secret_collection_new_for_dbus_path                (SecretService *service,
                                                                        const gchar *collection_path,
                                                                        SecretCollectionFlags flags,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

SecretCollection *  secret_collection_new_for_dbus_path_finish         (GAsyncResult *result,
                                                                        GError **error);

SecretCollection *  secret_collection_new_for_dbus_path_sync           (SecretService *service,
                                                                        const gchar *collection_path,
                                                                        SecretCollectionFlags flags,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

void                secret_collection_search_for_dbus_paths            (SecretCollection *collection,
                                                                        const SecretSchema *schema,
                                                                        GHashTable *attributes,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

gchar **            secret_collection_search_for_dbus_paths_finish     (SecretCollection *collection,
                                                                        GAsyncResult *result,
                                                                        GError **error);

gchar **            secret_collection_search_for_dbus_paths_sync       (SecretCollection *collection,
                                                                        const SecretSchema *schema,
                                                                        GHashTable *attributes,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

void                secret_item_new_for_dbus_path                      (SecretService *service,
                                                                        const gchar *item_path,
                                                                        SecretItemFlags flags,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

SecretItem *        secret_item_new_for_dbus_path_finish               (GAsyncResult *result,
                                                                        GError **error);

SecretItem *        secret_item_new_for_dbus_path_sync                 (SecretService *service,
                                                                        const gchar *item_path,
                                                                        SecretItemFlags flags,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

const gchar *       secret_service_get_session_dbus_path               (SecretService *self);

void                secret_service_search_for_dbus_paths               (SecretService *self,
                                                                        const SecretSchema *schema,
                                                                        GHashTable *attributes,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

gboolean            secret_service_search_for_dbus_paths_finish        (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        gchar ***unlocked,
                                                                        gchar ***locked,
                                                                        GError **error);

gboolean            secret_service_search_for_dbus_paths_sync          (SecretService *self,
                                                                        const SecretSchema *schema,
                                                                        GHashTable *attributes,
                                                                        GCancellable *cancellable,
                                                                        gchar ***unlocked,
                                                                        gchar ***locked,
                                                                        GError **error);

void                secret_service_get_secret_for_dbus_path            (SecretService *self,
                                                                        const gchar *item_path,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

SecretValue *       secret_service_get_secret_for_dbus_path_finish     (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        GError **error);

SecretValue *       secret_service_get_secret_for_dbus_path_sync       (SecretService *self,
                                                                        const gchar *item_path,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

void                secret_service_get_secrets_for_dbus_paths          (SecretService *self,
                                                                        const gchar **item_paths,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

GHashTable *        secret_service_get_secrets_for_dbus_paths_finish   (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        GError **error);

GHashTable *        secret_service_get_secrets_for_dbus_paths_sync     (SecretService *self,
                                                                        const gchar **item_paths,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

gint                secret_service_lock_dbus_paths_sync                (SecretService *self,
                                                                        const gchar **paths,
                                                                        GCancellable *cancellable,
                                                                        gchar ***locked,
                                                                        GError **error);

void                secret_service_lock_dbus_paths                     (SecretService *self,
                                                                        const gchar **paths,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

gint                secret_service_lock_dbus_paths_finish              (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        gchar ***locked,
                                                                        GError **error);

gint                secret_service_unlock_dbus_paths_sync              (SecretService *self,
                                                                        const gchar **paths,
                                                                        GCancellable *cancellable,
                                                                        gchar ***unlocked,
                                                                        GError **error);

void                secret_service_unlock_dbus_paths                   (SecretService *self,
                                                                        const gchar **paths,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

gint                secret_service_unlock_dbus_paths_finish            (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        gchar ***unlocked,
                                                                        GError **error);

GVariant *          secret_service_prompt_at_dbus_path_sync            (SecretService *self,
                                                                        const gchar *prompt_path,
                                                                        GCancellable *cancellable,
                                                                        const GVariantType *return_type,
                                                                        GError **error);

void                secret_service_prompt_at_dbus_path                 (SecretService *self,
                                                                        const gchar *prompt_path,
                                                                        const GVariantType *return_type,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

GVariant *          secret_service_prompt_at_dbus_path_finish          (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        GError **error);

void                secret_service_delete_item_dbus_path               (SecretService *self,
                                                                        const gchar *item_path,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

gboolean            secret_service_delete_item_dbus_path_finish        (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        GError **error);

gboolean            secret_service_delete_item_dbus_path_sync          (SecretService *self,
                                                                        const gchar *item_path,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

void                secret_service_create_collection_dbus_path         (SecretService *self,
                                                                        GHashTable *properties,
                                                                        const gchar *alias,
                                                                        SecretCollectionCreateFlags flags,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

gchar *             secret_service_create_collection_dbus_path_finish  (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        GError **error);

gchar *             secret_service_create_collection_dbus_path_sync    (SecretService *self,
                                                                        GHashTable *properties,
                                                                        const gchar *alias,
                                                                        SecretCollectionCreateFlags flags,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

void                secret_service_create_item_dbus_path               (SecretService *self,
                                                                        const gchar *collection_path,
                                                                        GHashTable *properties,
                                                                        SecretValue *value,
                                                                        SecretItemCreateFlags flags,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

gchar *             secret_service_create_item_dbus_path_finish        (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        GError **error);

gchar *             secret_service_create_item_dbus_path_sync          (SecretService *self,
                                                                        const gchar *collection_path,
                                                                        GHashTable *properties,
                                                                        SecretValue *value,
                                                                        SecretItemCreateFlags flags,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

void                secret_service_read_alias_dbus_path                (SecretService *self,
                                                                        const gchar *alias,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

gchar *             secret_service_read_alias_dbus_path_finish         (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        GError **error);

gchar *             secret_service_read_alias_dbus_path_sync           (SecretService *self,
                                                                        const gchar *alias,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

void                secret_service_set_alias_to_dbus_path              (SecretService *self,
                                                                        const gchar *alias,
                                                                        const gchar *collection_path,
                                                                        GCancellable *cancellable,
                                                                        GAsyncReadyCallback callback,
                                                                        gpointer user_data);

gboolean            secret_service_set_alias_to_dbus_path_finish       (SecretService *self,
                                                                        GAsyncResult *result,
                                                                        GError **error);

gboolean            secret_service_set_alias_to_dbus_path_sync         (SecretService *self,
                                                                        const gchar *alias,
                                                                        const gchar *collection_path,
                                                                        GCancellable *cancellable,
                                                                        GError **error);

GVariant *          secret_service_encode_dbus_secret                  (SecretService *service,
                                                                        SecretValue *value);

SecretValue *       secret_service_decode_dbus_secret                  (SecretService *service,
                                                                        GVariant *value);

G_END_DECLS

#endif /* __SECRET_SERVICE_H___ */

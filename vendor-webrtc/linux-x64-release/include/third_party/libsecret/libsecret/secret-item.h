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

#ifndef __SECRET_ITEM_H__
#define __SECRET_ITEM_H__

#include <gio/gio.h>

#include "secret-collection.h"
#include "secret-item.h"
#include "secret-service.h"
#include "secret-value.h"

G_BEGIN_DECLS

typedef enum {
	SECRET_ITEM_NONE,
	SECRET_ITEM_LOAD_SECRET = 1 << 1
} SecretItemFlags;

typedef enum {
	SECRET_ITEM_CREATE_NONE = 0,
	SECRET_ITEM_CREATE_REPLACE = 1 << 1
} SecretItemCreateFlags;

#define SECRET_TYPE_ITEM            (secret_item_get_type ())
#define SECRET_ITEM(inst)           (G_TYPE_CHECK_INSTANCE_CAST ((inst), SECRET_TYPE_ITEM, SecretItem))
#define SECRET_ITEM_CLASS(class)    (G_TYPE_CHECK_CLASS_CAST ((class), SECRET_TYPE_ITEM, SecretItemClass))
#define SECRET_IS_ITEM(inst)        (G_TYPE_CHECK_INSTANCE_TYPE ((inst), SECRET_TYPE_ITEM))
#define SECRET_IS_ITEM_CLASS(class) (G_TYPE_CHECK_CLASS_TYPE ((class), SECRET_TYPE_ITEM))
#define SECRET_ITEM_GET_CLASS(inst) (G_TYPE_INSTANCE_GET_CLASS ((inst), SECRET_TYPE_ITEM, SecretItemClass))

typedef struct _SecretItemClass   SecretItemClass;
typedef struct _SecretItemPrivate   SecretItemPrivate;

struct _SecretItem {
	GDBusProxy parent_instance;

	/*< private >*/
	SecretItemPrivate *pv;
};

struct _SecretItemClass {
	GDBusProxyClass parent_class;

	/*< private >*/
	gpointer padding[4];
};

GType               secret_item_get_type                   (void) G_GNUC_CONST;

void                secret_item_refresh                    (SecretItem *self);

void                secret_item_create                     (SecretCollection *collection,
                                                            const SecretSchema *schema,
                                                            GHashTable *attributes,
                                                            const gchar *label,
                                                            SecretValue *value,
                                                            SecretItemCreateFlags flags,
                                                            GCancellable *cancellable,
                                                            GAsyncReadyCallback callback,
                                                            gpointer user_data);

SecretItem *        secret_item_create_finish              (GAsyncResult *result,
                                                            GError **error);

SecretItem *        secret_item_create_sync                (SecretCollection *collection,
                                                            const SecretSchema *schema,
                                                            GHashTable *attributes,
                                                            const gchar *label,
                                                            SecretValue *value,
                                                            SecretItemCreateFlags flags,
                                                            GCancellable *cancellable,
                                                            GError **error);

void                secret_item_delete                     (SecretItem *self,
                                                            GCancellable *cancellable,
                                                            GAsyncReadyCallback callback,
                                                            gpointer user_data);

gboolean            secret_item_delete_finish              (SecretItem *self,
                                                            GAsyncResult *result,
                                                            GError **error);

gboolean            secret_item_delete_sync                (SecretItem *self,
                                                            GCancellable *cancellable,
                                                            GError **error);

SecretItemFlags     secret_item_get_flags                  (SecretItem *self);

SecretService *     secret_item_get_service                (SecretItem *self);

SecretValue *       secret_item_get_secret                 (SecretItem *self);

void                secret_item_load_secret                (SecretItem *self,
                                                            GCancellable *cancellable,
                                                            GAsyncReadyCallback callback,
                                                            gpointer user_data);

gboolean            secret_item_load_secret_finish         (SecretItem *self,
                                                            GAsyncResult *result,
                                                            GError **error);

gboolean            secret_item_load_secret_sync           (SecretItem *self,
                                                            GCancellable *cancellable,
                                                            GError **error);

void                secret_item_load_secrets               (GList *items,
                                                            GCancellable *cancellable,
                                                            GAsyncReadyCallback callback,
                                                            gpointer user_data);

gboolean            secret_item_load_secrets_finish        (GAsyncResult *result,
                                                            GError **error);

gboolean            secret_item_load_secrets_sync          (GList *items,
                                                            GCancellable *cancellable,
                                                            GError **error);

void                secret_item_set_secret                 (SecretItem *self,
                                                            SecretValue *value,
                                                            GCancellable *cancellable,
                                                            GAsyncReadyCallback callback,
                                                            gpointer user_data);

gboolean            secret_item_set_secret_finish          (SecretItem *self,
                                                            GAsyncResult *result,
                                                            GError **error);

gboolean            secret_item_set_secret_sync            (SecretItem *self,
                                                            SecretValue *value,
                                                            GCancellable *cancellable,
                                                            GError **error);

gchar *             secret_item_get_schema_name            (SecretItem *self);

GHashTable*         secret_item_get_attributes             (SecretItem *self);

void                secret_item_set_attributes             (SecretItem *self,
                                                            const SecretSchema *schema,
                                                            GHashTable *attributes,
                                                            GCancellable *cancellable,
                                                            GAsyncReadyCallback callback,
                                                            gpointer user_data);

gboolean            secret_item_set_attributes_finish      (SecretItem *self,
                                                            GAsyncResult *result,
                                                            GError **error);

gboolean            secret_item_set_attributes_sync        (SecretItem *self,
                                                            const SecretSchema *schema,
                                                            GHashTable *attributes,
                                                            GCancellable *cancellable,
                                                            GError **error);

gchar *             secret_item_get_label                  (SecretItem *self);

void                secret_item_set_label                  (SecretItem *self,
                                                            const gchar *label,
                                                            GCancellable *cancellable,
                                                            GAsyncReadyCallback callback,
                                                            gpointer user_data);

gboolean            secret_item_set_label_finish           (SecretItem *self,
                                                            GAsyncResult *result,
                                                            GError **error);

gboolean            secret_item_set_label_sync             (SecretItem *self,
                                                            const gchar *label,
                                                            GCancellable *cancellable,
                                                            GError **error);

gboolean            secret_item_get_locked                 (SecretItem *self);

guint64             secret_item_get_created                (SecretItem *self);

guint64             secret_item_get_modified               (SecretItem *self);

G_END_DECLS

#endif /* __SECRET_ITEM_H___ */

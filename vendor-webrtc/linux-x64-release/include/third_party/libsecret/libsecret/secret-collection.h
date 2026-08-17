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

#ifndef __SECRET_COLLECTION_H__
#define __SECRET_COLLECTION_H__

#include <gio/gio.h>

#include "secret-schema.h"
#include "secret-service.h"
#include "secret-types.h"

G_BEGIN_DECLS

typedef enum {
	SECRET_COLLECTION_NONE = 0 << 0,
	SECRET_COLLECTION_LOAD_ITEMS = 1 << 1,
} SecretCollectionFlags;

typedef enum {
	SECRET_COLLECTION_CREATE_NONE = 0 << 0,
} SecretCollectionCreateFlags;

#define SECRET_TYPE_COLLECTION            (secret_collection_get_type ())
#define SECRET_COLLECTION(inst)           (G_TYPE_CHECK_INSTANCE_CAST ((inst), SECRET_TYPE_COLLECTION, SecretCollection))
#define SECRET_COLLECTION_CLASS(class)    (G_TYPE_CHECK_CLASS_CAST ((class), SECRET_TYPE_COLLECTION, SecretCollectionClass))
#define SECRET_IS_COLLECTION(inst)        (G_TYPE_CHECK_INSTANCE_TYPE ((inst), SECRET_TYPE_COLLECTION))
#define SECRET_IS_COLLECTION_CLASS(class) (G_TYPE_CHECK_CLASS_TYPE ((class), SECRET_TYPE_COLLECTION))
#define SECRET_COLLECTION_GET_CLASS(inst) (G_TYPE_INSTANCE_GET_CLASS ((inst), SECRET_TYPE_COLLECTION, SecretCollectionClass))

typedef struct _SecretItem              SecretItem;
typedef struct _SecretCollectionClass   SecretCollectionClass;
typedef struct _SecretCollectionPrivate SecretCollectionPrivate;

struct _SecretCollection {
	GDBusProxy parent;

	/*< private >*/
	SecretCollectionPrivate *pv;
};

struct _SecretCollectionClass {
	GDBusProxyClass parent_class;

	/*< private >*/
	gpointer padding[8];
};

GType               secret_collection_get_type                 (void) G_GNUC_CONST;

void                secret_collection_for_alias                (SecretService *service,
                                                                const gchar *alias,
                                                                SecretCollectionFlags flags,
                                                                GCancellable *cancellable,
                                                                GAsyncReadyCallback callback,
                                                                gpointer user_data);

SecretCollection *  secret_collection_for_alias_finish         (GAsyncResult *result,
                                                                GError **error);

SecretCollection *  secret_collection_for_alias_sync           (SecretService *service,
                                                                const gchar *alias,
                                                                SecretCollectionFlags flags,
                                                                GCancellable *cancellable,
                                                                GError **error);

void                secret_collection_load_items               (SecretCollection *self,
                                                                GCancellable *cancellable,
                                                                GAsyncReadyCallback callback,
                                                                gpointer user_data);

gboolean            secret_collection_load_items_finish        (SecretCollection *self,
                                                                GAsyncResult *result,
                                                                GError **error);

gboolean            secret_collection_load_items_sync          (SecretCollection *self,
                                                                GCancellable *cancellable,
                                                                GError **error);

void                secret_collection_refresh                  (SecretCollection *self);

void                secret_collection_create                   (SecretService *service,
                                                                const gchar *label,
                                                                const gchar *alias,
                                                                SecretCollectionCreateFlags flags,
                                                                GCancellable *cancellable,
                                                                GAsyncReadyCallback callback,
                                                                gpointer user_data);

SecretCollection *  secret_collection_create_finish            (GAsyncResult *result,
                                                                GError **error);

SecretCollection *  secret_collection_create_sync              (SecretService *service,
                                                                const gchar *label,
                                                                const gchar *alias,
                                                                SecretCollectionCreateFlags flags,
                                                                GCancellable *cancellable,
                                                                GError **error);

void                secret_collection_search                   (SecretCollection *self,
                                                                const SecretSchema *schema,
                                                                GHashTable *attributes,
                                                                SecretSearchFlags flags,
                                                                GCancellable *cancellable,
                                                                GAsyncReadyCallback callback,
                                                                gpointer user_data);

GList *             secret_collection_search_finish            (SecretCollection *self,
                                                                GAsyncResult *result,
                                                                GError **error);

GList *             secret_collection_search_sync              (SecretCollection *self,
                                                                const SecretSchema *schema,
                                                                GHashTable *attributes,
                                                                SecretSearchFlags flags,
                                                                GCancellable *cancellable,
                                                                GError **error);

void                secret_collection_delete                   (SecretCollection *self,
                                                                GCancellable *cancellable,
                                                                GAsyncReadyCallback callback,
                                                                gpointer user_data);

gboolean            secret_collection_delete_finish            (SecretCollection *self,
                                                                GAsyncResult *result,
                                                                GError **error);

gboolean            secret_collection_delete_sync              (SecretCollection *self,
                                                                GCancellable *cancellable,
                                                                GError **error);

SecretService *     secret_collection_get_service              (SecretCollection *self);

SecretCollectionFlags secret_collection_get_flags              (SecretCollection *self);

GList *             secret_collection_get_items                (SecretCollection *self);

gchar *             secret_collection_get_label                (SecretCollection *self);

void                secret_collection_set_label                (SecretCollection *self,
                                                                const gchar *label,
                                                                GCancellable *cancellable,
                                                                GAsyncReadyCallback callback,
                                                                gpointer user_data);

gboolean            secret_collection_set_label_finish         (SecretCollection *self,
                                                                GAsyncResult *result,
                                                                GError **error);

gboolean            secret_collection_set_label_sync           (SecretCollection *self,
                                                                const gchar *label,
                                                                GCancellable *cancellable,
                                                                GError **error);

gboolean            secret_collection_get_locked               (SecretCollection *self);

guint64             secret_collection_get_created              (SecretCollection *self);

guint64             secret_collection_get_modified             (SecretCollection *self);

G_END_DECLS

#endif /* __SECRET_COLLECTION_H___ */

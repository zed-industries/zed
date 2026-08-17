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

#ifndef __SECRET_SERVICE_H__
#define __SECRET_SERVICE_H__

#include <gio/gio.h>

#include "secret-prompt.h"
#include "secret-schema.h"
#include "secret-types.h"
#include "secret-value.h"

G_BEGIN_DECLS

typedef enum {
	SECRET_SERVICE_NONE = 0,
	SECRET_SERVICE_OPEN_SESSION = 1 << 1,
	SECRET_SERVICE_LOAD_COLLECTIONS = 1 << 2,
} SecretServiceFlags;

typedef enum {
	SECRET_SEARCH_NONE = 0,
	SECRET_SEARCH_ALL = 1 << 1,
	SECRET_SEARCH_UNLOCK = 1 << 2,
	SECRET_SEARCH_LOAD_SECRETS = 1 << 3,
} SecretSearchFlags;

#define SECRET_TYPE_SERVICE            (secret_service_get_type ())
#define SECRET_SERVICE(inst)           (G_TYPE_CHECK_INSTANCE_CAST ((inst), SECRET_TYPE_SERVICE, SecretService))
#define SECRET_SERVICE_CLASS(class)    (G_TYPE_CHECK_CLASS_CAST ((class), SECRET_TYPE_SERVICE, SecretServiceClass))
#define SECRET_IS_SERVICE(inst)        (G_TYPE_CHECK_INSTANCE_TYPE ((inst), SECRET_TYPE_SERVICE))
#define SECRET_IS_SERVICE_CLASS(class) (G_TYPE_CHECK_CLASS_TYPE ((class), SECRET_TYPE_SERVICE))
#define SECRET_SERVICE_GET_CLASS(inst) (G_TYPE_INSTANCE_GET_CLASS ((inst), SECRET_TYPE_SERVICE, SecretServiceClass))

typedef struct _SecretCollection     SecretCollection;
typedef struct _SecretService        SecretService;
typedef struct _SecretServiceClass   SecretServiceClass;
typedef struct _SecretServicePrivate SecretServicePrivate;

struct _SecretService {
	GDBusProxy parent;

	/*< private >*/
	SecretServicePrivate *pv;
};

struct _SecretServiceClass {
	GDBusProxyClass parent_class;

	GType collection_gtype;
	GType item_gtype;

	GVariant *  (* prompt_sync)      (SecretService *self,
	                                  SecretPrompt *prompt,
	                                  GCancellable *cancellable,
	                                  const GVariantType *return_type,
	                                  GError **error);

	void        (* prompt_async)     (SecretService *self,
	                                  SecretPrompt *prompt,
	                                  const GVariantType *return_type,
	                                  GCancellable *cancellable,
	                                  GAsyncReadyCallback callback,
	                                  gpointer user_data);

	GVariant *  (* prompt_finish)    (SecretService *self,
	                                  GAsyncResult *result,
	                                  GError **error);

	GType       (* get_collection_gtype)  (SecretService *self);

	GType       (* get_item_gtype)        (SecretService *self);

	/*< private >*/
	gpointer padding[14];
};

GType                secret_service_get_type                      (void) G_GNUC_CONST;

GType                secret_service_get_collection_gtype          (SecretService *self);

GType                secret_service_get_item_gtype                (SecretService *self);

void                 secret_service_get                           (SecretServiceFlags flags,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

SecretService *      secret_service_get_finish                    (GAsyncResult *result,
                                                                   GError **error);

SecretService *      secret_service_get_sync                      (SecretServiceFlags flags,
                                                                   GCancellable *cancellable,
                                                                   GError **error);

void                 secret_service_disconnect                    (void);

void                 secret_service_open                          (GType service_gtype,
                                                                   const gchar *service_bus_name,
                                                                   SecretServiceFlags flags,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

SecretService *      secret_service_open_finish                   (GAsyncResult *result,
                                                                   GError **error);

SecretService *      secret_service_open_sync                     (GType service_gtype,
                                                                   const gchar *service_bus_name,
                                                                   SecretServiceFlags flags,
                                                                   GCancellable *cancellable,
                                                                   GError **error);

SecretServiceFlags   secret_service_get_flags                     (SecretService *self);

const gchar *        secret_service_get_session_algorithms        (SecretService *self);

GList *              secret_service_get_collections               (SecretService *self);

void                 secret_service_ensure_session                (SecretService *self,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

gboolean             secret_service_ensure_session_finish         (SecretService *self,
                                                                   GAsyncResult *result,
                                                                   GError **error);

gboolean             secret_service_ensure_session_sync           (SecretService *self,
                                                                   GCancellable *cancellable,
                                                                   GError **error);

void                 secret_service_load_collections              (SecretService *self,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

gboolean             secret_service_load_collections_finish       (SecretService *self,
                                                                   GAsyncResult *result,
                                                                   GError **error);

gboolean             secret_service_load_collections_sync         (SecretService *self,
                                                                   GCancellable *cancellable,
                                                                   GError **error);

GVariant *           secret_service_prompt_sync                   (SecretService *self,
                                                                   SecretPrompt *prompt,
                                                                   GCancellable *cancellable,
                                                                   const GVariantType *return_type,
                                                                   GError **error);

void                 secret_service_prompt                        (SecretService *self,
                                                                   SecretPrompt *prompt,
                                                                   const GVariantType *return_type,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

GVariant *           secret_service_prompt_finish                 (SecretService *self,
                                                                   GAsyncResult *result,
                                                                   GError **error);

void                 secret_service_search                        (SecretService *service,
                                                                   const SecretSchema *schema,
                                                                   GHashTable *attributes,
                                                                   SecretSearchFlags flags,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

GList *              secret_service_search_finish                 (SecretService *service,
                                                                   GAsyncResult *result,
                                                                   GError **error);

GList *              secret_service_search_sync                   (SecretService *service,
                                                                   const SecretSchema *schema,
                                                                   GHashTable *attributes,
                                                                   SecretSearchFlags flags,
                                                                   GCancellable *cancellable,
                                                                   GError **error);

void                 secret_service_lock                          (SecretService *service,
                                                                   GList *objects,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

gint                 secret_service_lock_finish                   (SecretService *service,
                                                                   GAsyncResult *result,
                                                                   GList **locked,
                                                                   GError **error);

gint                 secret_service_lock_sync                     (SecretService *service,
                                                                   GList *objects,
                                                                   GCancellable *cancellable,
                                                                   GList **locked,
                                                                   GError **error);

void                 secret_service_unlock                        (SecretService *service,
                                                                   GList *objects,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

gint                 secret_service_unlock_finish                 (SecretService *service,
                                                                   GAsyncResult *result,
                                                                   GList **unlocked,
                                                                   GError **error);

gint                 secret_service_unlock_sync                   (SecretService *service,
                                                                   GList *objects,
                                                                   GCancellable *cancellable,
                                                                   GList **unlocked,
                                                                   GError **error);

void                 secret_service_store                         (SecretService *service,
                                                                   const SecretSchema *schema,
                                                                   GHashTable *attributes,
                                                                   const gchar *collection,
                                                                   const gchar *label,
                                                                   SecretValue *value,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

gboolean             secret_service_store_finish                  (SecretService *service,
                                                                   GAsyncResult *result,
                                                                   GError **error);

gboolean             secret_service_store_sync                    (SecretService *service,
                                                                   const SecretSchema *schema,
                                                                   GHashTable *attributes,
                                                                   const gchar *collection,
                                                                   const gchar *label,
                                                                   SecretValue *value,
                                                                   GCancellable *cancellable,
                                                                   GError **error);

void                 secret_service_lookup                        (SecretService *service,
                                                                   const SecretSchema *schema,
                                                                   GHashTable *attributes,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

SecretValue *        secret_service_lookup_finish                 (SecretService *service,
                                                                   GAsyncResult *result,
                                                                   GError **error);

SecretValue *        secret_service_lookup_sync                   (SecretService *service,
                                                                   const SecretSchema *schema,
                                                                   GHashTable *attributes,
                                                                   GCancellable *cancellable,
                                                                   GError **error);

void                 secret_service_clear                         (SecretService *service,
                                                                   const SecretSchema *schema,
                                                                   GHashTable *attributes,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

gboolean             secret_service_clear_finish                  (SecretService *service,
                                                                   GAsyncResult *result,
                                                                   GError **error);

gboolean             secret_service_clear_sync                    (SecretService *service,
                                                                   const SecretSchema *schema,
                                                                   GHashTable *attributes,
                                                                   GCancellable *cancellable,
                                                                   GError **error);

void                 secret_service_set_alias                     (SecretService *service,
                                                                   const gchar *alias,
                                                                   SecretCollection *collection,
                                                                   GCancellable *cancellable,
                                                                   GAsyncReadyCallback callback,
                                                                   gpointer user_data);

gboolean             secret_service_set_alias_finish              (SecretService *service,
                                                                   GAsyncResult *result,
                                                                   GError **error);

gboolean             secret_service_set_alias_sync                (SecretService *service,
                                                                   const gchar *alias,
                                                                   SecretCollection *collection,
                                                                   GCancellable *cancellable,
                                                                   GError **error);

G_END_DECLS

#endif /* __SECRET_SERVICE_H___ */

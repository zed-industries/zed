/* -*- Mode: C; c-file-style: "gnu"; indent-tabs-mode: nil; -*-
 *
 * Copyright (C) 2008 David Zeuthen <davidz@redhat.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */

#if !defined (_GUDEV_COMPILATION) && !defined(_GUDEV_INSIDE_GUDEV_H)
#error "Only <gudev/gudev.h> can be included directly, this file may disappear or change contents."
#endif

#ifndef __G_UDEV_CLIENT_H__
#define __G_UDEV_CLIENT_H__

#include <gudev/gudevtypes.h>

G_BEGIN_DECLS

#define G_UDEV_TYPE_CLIENT         (g_udev_client_get_type ())
#define G_UDEV_CLIENT(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), G_UDEV_TYPE_CLIENT, GUdevClient))
#define G_UDEV_CLIENT_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST((k), G_UDEV_TYPE_CLIENT, GUdevClientClass))
#define G_UDEV_IS_CLIENT(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), G_UDEV_TYPE_CLIENT))
#define G_UDEV_IS_CLIENT_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), G_UDEV_TYPE_CLIENT))
#define G_UDEV_CLIENT_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), G_UDEV_TYPE_CLIENT, GUdevClientClass))

#ifdef G_DEFINE_AUTOPTR_CLEANUP_FUNC
G_DEFINE_AUTOPTR_CLEANUP_FUNC (GUdevClient, g_object_unref)
#endif

typedef struct _GUdevClientClass   GUdevClientClass;
typedef struct _GUdevClientPrivate GUdevClientPrivate;

/**
 * GUdevClient:
 *
 * The #GUdevClient struct is opaque and should not be accessed directly.
 */
struct _GUdevClient
{
  GObject              parent;

  /*< private >*/
  GUdevClientPrivate *priv;
};

/**
 * GUdevClientClass:
 * @parent_class: Parent class.
 * @uevent: Signal class handler for the #GUdevClient::uevent signal.
 *
 * Class structure for #GUdevClient.
 */
struct _GUdevClientClass
{
  GObjectClass   parent_class;

  /* signals */
  void (*uevent) (GUdevClient  *client,
                  const gchar  *action,
                  GUdevDevice  *device);

  /*< private >*/
  /* Padding for future expansion */
  void (*reserved1) (void);
  void (*reserved2) (void);
  void (*reserved3) (void);
  void (*reserved4) (void);
  void (*reserved5) (void);
  void (*reserved6) (void);
  void (*reserved7) (void);
  void (*reserved8) (void);
};

GType        g_udev_client_get_type                    (void) G_GNUC_CONST;
GUdevClient *g_udev_client_new                         (const gchar* const *subsystems);
GList       *g_udev_client_query_by_subsystem          (GUdevClient        *client,
                                                        const gchar        *subsystem);
GUdevDevice *g_udev_client_query_by_device_number      (GUdevClient        *client,
                                                        GUdevDeviceType     type,
                                                        GUdevDeviceNumber   number);
GUdevDevice *g_udev_client_query_by_device_file        (GUdevClient        *client,
                                                        const gchar        *device_file);
GUdevDevice *g_udev_client_query_by_sysfs_path         (GUdevClient        *client,
                                                        const gchar        *sysfs_path);
GUdevDevice *g_udev_client_query_by_subsystem_and_name (GUdevClient        *client,
                                                        const gchar        *subsystem,
                                                        const gchar        *name);

G_END_DECLS

#endif /* __G_UDEV_CLIENT_H__ */

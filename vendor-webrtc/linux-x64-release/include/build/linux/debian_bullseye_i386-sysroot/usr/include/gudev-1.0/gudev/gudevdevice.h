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

#ifndef __G_UDEV_DEVICE_H__
#define __G_UDEV_DEVICE_H__

#include <gudev/gudevtypes.h>

G_BEGIN_DECLS

#define G_UDEV_TYPE_DEVICE         (g_udev_device_get_type ())
#define G_UDEV_DEVICE(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), G_UDEV_TYPE_DEVICE, GUdevDevice))
#define G_UDEV_DEVICE_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST((k), G_UDEV_TYPE_DEVICE, GUdevDeviceClass))
#define G_UDEV_IS_DEVICE(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), G_UDEV_TYPE_DEVICE))
#define G_UDEV_IS_DEVICE_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), G_UDEV_TYPE_DEVICE))
#define G_UDEV_DEVICE_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), G_UDEV_TYPE_DEVICE, GUdevDeviceClass))

#ifdef G_DEFINE_AUTOPTR_CLEANUP_FUNC
G_DEFINE_AUTOPTR_CLEANUP_FUNC (GUdevDevice, g_object_unref)
#endif

typedef struct _GUdevDeviceClass   GUdevDeviceClass;
typedef struct _GUdevDevicePrivate GUdevDevicePrivate;

/**
 * GUdevDevice:
 *
 * The #GUdevDevice struct is opaque and should not be accessed directly.
 */
struct _GUdevDevice
{
  GObject             parent;

  /*< private >*/
  GUdevDevicePrivate *priv;
};

/**
 * GUdevDeviceClass:
 * @parent_class: Parent class.
 *
 * Class structure for #GUdevDevice.
 */
struct _GUdevDeviceClass
{
  GObjectClass parent_class;

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

GType               g_udev_device_get_type                  (void) G_GNUC_CONST;
gboolean            g_udev_device_get_is_initialized        (GUdevDevice  *device);
guint64             g_udev_device_get_usec_since_initialized (GUdevDevice  *device);
const gchar        *g_udev_device_get_subsystem             (GUdevDevice  *device);
const gchar        *g_udev_device_get_devtype               (GUdevDevice  *device);
const gchar        *g_udev_device_get_name                  (GUdevDevice  *device);
const gchar        *g_udev_device_get_number                (GUdevDevice  *device);
const gchar        *g_udev_device_get_sysfs_path            (GUdevDevice  *device);
const gchar        *g_udev_device_get_driver                (GUdevDevice  *device);
const gchar        *g_udev_device_get_action                (GUdevDevice  *device);
guint64             g_udev_device_get_seqnum                (GUdevDevice  *device);
GUdevDeviceType     g_udev_device_get_device_type           (GUdevDevice  *device);
GUdevDeviceNumber   g_udev_device_get_device_number         (GUdevDevice  *device);
const gchar        *g_udev_device_get_device_file           (GUdevDevice  *device);
const gchar* const *g_udev_device_get_device_file_symlinks  (GUdevDevice  *device);
GUdevDevice        *g_udev_device_get_parent                (GUdevDevice  *device);
GUdevDevice        *g_udev_device_get_parent_with_subsystem (GUdevDevice  *device,
                                                             const gchar  *subsystem,
                                                             const gchar  *devtype);
const gchar* const *g_udev_device_get_property_keys         (GUdevDevice  *device);
gboolean            g_udev_device_has_property              (GUdevDevice  *device,
                                                             const gchar  *key);
const gchar        *g_udev_device_get_property              (GUdevDevice  *device,
                                                             const gchar  *key);
gint                g_udev_device_get_property_as_int       (GUdevDevice  *device,
                                                             const gchar  *key);
guint64             g_udev_device_get_property_as_uint64    (GUdevDevice  *device,
                                                             const gchar  *key);
gdouble             g_udev_device_get_property_as_double    (GUdevDevice  *device,
                                                             const gchar  *key);
gboolean            g_udev_device_get_property_as_boolean   (GUdevDevice  *device,
                                                             const gchar  *key);
const gchar* const *g_udev_device_get_property_as_strv      (GUdevDevice  *device,
                                                             const gchar  *key);

const gchar* const *g_udev_device_get_sysfs_attr_keys       (GUdevDevice  *device);
gboolean            g_udev_device_has_sysfs_attr            (GUdevDevice  *device,
                                                             const gchar  *key);
const gchar        *g_udev_device_get_sysfs_attr            (GUdevDevice  *device,
                                                             const gchar  *name);
gint                g_udev_device_get_sysfs_attr_as_int     (GUdevDevice  *device,
                                                             const gchar  *name);
guint64             g_udev_device_get_sysfs_attr_as_uint64  (GUdevDevice  *device,
                                                             const gchar  *name);
gdouble             g_udev_device_get_sysfs_attr_as_double  (GUdevDevice  *device,
                                                             const gchar  *name);
gboolean            g_udev_device_get_sysfs_attr_as_boolean (GUdevDevice  *device,
                                                             const gchar  *name);
const gchar* const *g_udev_device_get_sysfs_attr_as_strv    (GUdevDevice  *device,
                                                             const gchar  *name);
const gchar* const *g_udev_device_get_tags                  (GUdevDevice  *device);

gboolean            g_udev_device_has_sysfs_attr_uncached            (GUdevDevice  *device,
                                                                      const gchar  *key);
const gchar        *g_udev_device_get_sysfs_attr_uncached            (GUdevDevice  *device,
                                                                      const gchar  *name);
gint                g_udev_device_get_sysfs_attr_as_int_uncached     (GUdevDevice  *device,
                                                                      const gchar  *name);
guint64             g_udev_device_get_sysfs_attr_as_uint64_uncached  (GUdevDevice  *device,
                                                                      const gchar  *name);
gdouble             g_udev_device_get_sysfs_attr_as_double_uncached  (GUdevDevice  *device,
                                                                      const gchar  *name);
gboolean            g_udev_device_get_sysfs_attr_as_boolean_uncached (GUdevDevice  *device,
                                                                      const gchar  *name);
const gchar* const *g_udev_device_get_sysfs_attr_as_strv_uncached    (GUdevDevice  *device,
                                                                      const gchar  *name);

G_END_DECLS

#endif /* __G_UDEV_DEVICE_H__ */

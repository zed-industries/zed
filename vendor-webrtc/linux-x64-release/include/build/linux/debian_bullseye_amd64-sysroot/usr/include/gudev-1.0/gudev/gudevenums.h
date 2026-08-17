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

#ifndef __G_UDEV_ENUMS_H__
#define __G_UDEV_ENUMS_H__

#include <glib-object.h>

G_BEGIN_DECLS

/**
 * GUdevDeviceType:
 * @G_UDEV_DEVICE_TYPE_NONE: Device does not have a device file.
 * @G_UDEV_DEVICE_TYPE_BLOCK: Device is a block device.
 * @G_UDEV_DEVICE_TYPE_CHAR: Device is a character device.
 *
 * Enumeration used to specify a the type of a device.
 */
typedef enum
{
  G_UDEV_DEVICE_TYPE_NONE = 0,
  G_UDEV_DEVICE_TYPE_BLOCK = 'b',
  G_UDEV_DEVICE_TYPE_CHAR = 'c',
} GUdevDeviceType;

G_END_DECLS

#endif /* __G_UDEV_ENUMS_H__ */

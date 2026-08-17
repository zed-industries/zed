/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2016 Red Hat
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Author: Carlos Garnacho <carlosg@gnome.org>
 */

#ifndef __GDK_DEVICE_PAD_H__
#define __GDK_DEVICE_PAD_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>

G_BEGIN_DECLS

#define GDK_TYPE_DEVICE_PAD         (gdk_device_pad_get_type ())
#define GDK_DEVICE_PAD(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_DEVICE_PAD, GdkDevicePad))
#define GDK_IS_DEVICE_PAD(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_DEVICE_PAD))

typedef struct _GdkDevicePad GdkDevicePad;
typedef struct _GdkDevicePadInterface GdkDevicePadInterface;

/**
 * GdkDevicePadFeature:
 * @GDK_DEVICE_PAD_FEATURE_BUTTON: a button
 * @GDK_DEVICE_PAD_FEATURE_RING: a ring-shaped interactive area
 * @GDK_DEVICE_PAD_FEATURE_STRIP: a straight interactive area
 *
 * A pad feature.
 */
typedef enum {
  GDK_DEVICE_PAD_FEATURE_BUTTON,
  GDK_DEVICE_PAD_FEATURE_RING,
  GDK_DEVICE_PAD_FEATURE_STRIP
} GdkDevicePadFeature;

GDK_AVAILABLE_IN_3_22
GType gdk_device_pad_get_type          (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_22
gint  gdk_device_pad_get_n_groups      (GdkDevicePad *pad);

GDK_AVAILABLE_IN_3_22
gint  gdk_device_pad_get_group_n_modes (GdkDevicePad *pad,
                                        gint          group_idx);

GDK_AVAILABLE_IN_3_22
gint  gdk_device_pad_get_n_features    (GdkDevicePad        *pad,
                                        GdkDevicePadFeature  feature);

GDK_AVAILABLE_IN_3_22
gint  gdk_device_pad_get_feature_group (GdkDevicePad        *pad,
                                        GdkDevicePadFeature  feature,
                                        gint                 feature_idx);

G_END_DECLS

#endif /* __GDK_DEVICE_PAD_H__ */

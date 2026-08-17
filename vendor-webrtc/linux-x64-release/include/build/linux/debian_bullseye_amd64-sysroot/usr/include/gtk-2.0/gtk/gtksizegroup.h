/* GTK - The GIMP Toolkit
 * gtksizegroup.h:
 * Copyright (C) 2000 Red Hat Software
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_SIZE_GROUP_H__
#define __GTK_SIZE_GROUP_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_SIZE_GROUP            (gtk_size_group_get_type ())
#define GTK_SIZE_GROUP(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SIZE_GROUP, GtkSizeGroup))
#define GTK_SIZE_GROUP_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SIZE_GROUP, GtkSizeGroupClass))
#define GTK_IS_SIZE_GROUP(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SIZE_GROUP))
#define GTK_IS_SIZE_GROUP_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SIZE_GROUP))
#define GTK_SIZE_GROUP_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SIZE_GROUP, GtkSizeGroupClass))


typedef struct _GtkSizeGroup       GtkSizeGroup;
typedef struct _GtkSizeGroupClass  GtkSizeGroupClass;

struct _GtkSizeGroup
{
  GObject parent_instance;

  /* <private> */
  GSList *GSEAL (widgets);

  guint8 GSEAL (mode);

  guint GSEAL (have_width) : 1;
  guint GSEAL (have_height) : 1;
  guint GSEAL (ignore_hidden) : 1;

  GtkRequisition GSEAL (requisition);
};

struct _GtkSizeGroupClass
{
  GObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

/**
 * GtkSizeGroupMode:
 * @GTK_SIZE_GROUP_NONE: group has no effect
 * @GTK_SIZE_GROUP_HORIZONTAL: group affects horizontal requisition
 * @GTK_SIZE_GROUP_VERTICAL: group affects vertical requisition
 * @GTK_SIZE_GROUP_BOTH: group affects both horizontal and vertical requisition
 *
 * The mode of the size group determines the directions in which the size
 * group affects the requested sizes of its component widgets.
 **/
typedef enum {
  GTK_SIZE_GROUP_NONE,
  GTK_SIZE_GROUP_HORIZONTAL,
  GTK_SIZE_GROUP_VERTICAL,
  GTK_SIZE_GROUP_BOTH
} GtkSizeGroupMode;

GType            gtk_size_group_get_type      (void) G_GNUC_CONST;

GtkSizeGroup *   gtk_size_group_new           (GtkSizeGroupMode  mode);
void             gtk_size_group_set_mode      (GtkSizeGroup     *size_group,
					       GtkSizeGroupMode  mode);
GtkSizeGroupMode gtk_size_group_get_mode      (GtkSizeGroup     *size_group);
void             gtk_size_group_set_ignore_hidden (GtkSizeGroup *size_group,
						   gboolean      ignore_hidden);
gboolean         gtk_size_group_get_ignore_hidden (GtkSizeGroup *size_group);
void             gtk_size_group_add_widget    (GtkSizeGroup     *size_group,
					       GtkWidget        *widget);
void             gtk_size_group_remove_widget (GtkSizeGroup     *size_group,
					       GtkWidget        *widget);
GSList *         gtk_size_group_get_widgets   (GtkSizeGroup     *size_group);


void _gtk_size_group_get_child_requisition (GtkWidget      *widget,
					    GtkRequisition *requisition);
void _gtk_size_group_compute_requisition   (GtkWidget      *widget,
					    GtkRequisition *requisition);
void _gtk_size_group_queue_resize          (GtkWidget      *widget);

G_END_DECLS

#endif /* __GTK_SIZE_GROUP_H__ */

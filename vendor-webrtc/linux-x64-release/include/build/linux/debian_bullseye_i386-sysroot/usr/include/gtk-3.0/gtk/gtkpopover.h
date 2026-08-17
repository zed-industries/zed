/* GTK - The GIMP Toolkit
 * Copyright © 2013 Carlos Garnacho <carlosg@gnome.org>
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
 */

#ifndef __GTK_POPOVER_H__
#define __GTK_POPOVER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwindow.h>

G_BEGIN_DECLS

#define GTK_TYPE_POPOVER           (gtk_popover_get_type ())
#define GTK_POPOVER(o)             (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_POPOVER, GtkPopover))
#define GTK_POPOVER_CLASS(c)       (G_TYPE_CHECK_CLASS_CAST ((c), GTK_TYPE_POPOVER, GtkPopoverClass))
#define GTK_IS_POPOVER(o)          (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_POPOVER))
#define GTK_IS_POPOVER_CLASS(o)    (G_TYPE_CHECK_CLASS_TYPE ((o), GTK_TYPE_POPOVER))
#define GTK_POPOVER_GET_CLASS(o)   (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_POPOVER, GtkPopoverClass))

typedef struct _GtkPopover GtkPopover;
typedef struct _GtkPopoverClass GtkPopoverClass;
typedef struct _GtkPopoverPrivate GtkPopoverPrivate;

struct _GtkPopover
{
  GtkBin parent_instance;

  /*< private >*/

  GtkPopoverPrivate *priv;
};

struct _GtkPopoverClass
{
  GtkBinClass parent_class;

  void (* closed) (GtkPopover *popover);

  /*< private >*/

  /* Padding for future expansion */
  gpointer reserved[10];
};

GDK_AVAILABLE_IN_3_12
GType           gtk_popover_get_type        (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_12
GtkWidget *     gtk_popover_new             (GtkWidget             *relative_to);

GDK_AVAILABLE_IN_3_12
GtkWidget *     gtk_popover_new_from_model  (GtkWidget             *relative_to,
                                             GMenuModel            *model);

GDK_AVAILABLE_IN_3_12
void            gtk_popover_set_relative_to (GtkPopover            *popover,
                                             GtkWidget             *relative_to);
GDK_AVAILABLE_IN_3_12
GtkWidget *     gtk_popover_get_relative_to (GtkPopover            *popover);

GDK_AVAILABLE_IN_3_12
void            gtk_popover_set_pointing_to (GtkPopover            *popover,
                                             const GdkRectangle    *rect);
GDK_AVAILABLE_IN_3_12
gboolean        gtk_popover_get_pointing_to (GtkPopover            *popover,
                                             GdkRectangle          *rect);
GDK_AVAILABLE_IN_3_12
void            gtk_popover_set_position    (GtkPopover            *popover,
                                             GtkPositionType        position);
GDK_AVAILABLE_IN_3_12
GtkPositionType gtk_popover_get_position    (GtkPopover            *popover);

GDK_AVAILABLE_IN_3_12
void            gtk_popover_set_modal       (GtkPopover            *popover,
                                             gboolean               modal);
GDK_AVAILABLE_IN_3_12
gboolean        gtk_popover_get_modal       (GtkPopover            *popover);

GDK_AVAILABLE_IN_3_12
void            gtk_popover_bind_model      (GtkPopover            *popover,
                                             GMenuModel            *model,
                                             const gchar           *action_namespace);

GDK_DEPRECATED_IN_3_22
void            gtk_popover_set_transitions_enabled (GtkPopover *popover,
                                                     gboolean    transitions_enabled);
GDK_DEPRECATED_IN_3_22
gboolean        gtk_popover_get_transitions_enabled (GtkPopover *popover);

GDK_AVAILABLE_IN_3_18
void            gtk_popover_set_default_widget (GtkPopover *popover,
                                                GtkWidget  *widget);
GDK_AVAILABLE_IN_3_18
GtkWidget *     gtk_popover_get_default_widget (GtkPopover *popover);

GDK_AVAILABLE_IN_3_20
void                 gtk_popover_set_constrain_to (GtkPopover           *popover,
                                                   GtkPopoverConstraint  constraint);

GDK_AVAILABLE_IN_3_20
GtkPopoverConstraint gtk_popover_get_constrain_to (GtkPopover           *popover);

GDK_AVAILABLE_IN_3_22
void                 gtk_popover_popup            (GtkPopover *popover);

GDK_AVAILABLE_IN_3_22
void                 gtk_popover_popdown          (GtkPopover *popover);


G_END_DECLS

#endif /* __GTK_POPOVER_H__ */

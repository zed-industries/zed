/* gtkshortcutlabelprivate.h
 *
 * Copyright (C) 2015 Christian Hergert <christian@hergert.me>
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Library General Public License as
 *  published by the Free Software Foundation; either version 2 of the
 *  License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Library General Public License for more details.
 *
 *  You should have received a copy of the GNU Library General Public
 *  License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_SHORTCUT_LABEL_H__
#define __GTK_SHORTCUT_LABEL_H__

#include <gtk/gtkbox.h>

G_BEGIN_DECLS

#define GTK_TYPE_SHORTCUT_LABEL (gtk_shortcut_label_get_type())
#define GTK_SHORTCUT_LABEL(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SHORTCUT_LABEL, GtkShortcutLabel))
#define GTK_SHORTCUT_LABEL_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SHORTCUT_LABEL, GtkShortcutLabelClass))
#define GTK_IS_SHORTCUT_LABEL(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SHORTCUT_LABEL))
#define GTK_IS_SHORTCUT_LABEL_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SHORTCUT_LABEL))
#define GTK_SHORTCUT_LABEL_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SHORTCUT_LABEL, GtkShortcutLabelClass))


typedef struct _GtkShortcutLabel      GtkShortcutLabel;
typedef struct _GtkShortcutLabelClass GtkShortcutLabelClass;

GDK_AVAILABLE_IN_3_22
GType        gtk_shortcut_label_get_type        (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_22
GtkWidget   *gtk_shortcut_label_new             (const gchar      *accelerator);

GDK_AVAILABLE_IN_3_22
const gchar *gtk_shortcut_label_get_accelerator (GtkShortcutLabel *self);

GDK_AVAILABLE_IN_3_22
void         gtk_shortcut_label_set_accelerator (GtkShortcutLabel *self,
                                                 const gchar      *accelerator);

GDK_AVAILABLE_IN_3_22
const gchar *gtk_shortcut_label_get_disabled_text (GtkShortcutLabel *self);

GDK_AVAILABLE_IN_3_22
void         gtk_shortcut_label_set_disabled_text (GtkShortcutLabel *self,
                                                   const gchar      *disabled_text);

G_END_DECLS

#endif /* __GTK_SHORTCUT_LABEL_H__ */

/* GTK - The GIMP Toolkit
 *
 * Copyright (C) 2010 Christian Dywan
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_COMBO_BOX_TEXT_H__
#define __GTK_COMBO_BOX_TEXT_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcombobox.h>

G_BEGIN_DECLS

#define GTK_TYPE_COMBO_BOX_TEXT                 (gtk_combo_box_text_get_type ())
#define GTK_COMBO_BOX_TEXT(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_COMBO_BOX_TEXT, GtkComboBoxText))
#define GTK_COMBO_BOX_TEXT_CLASS(klass)         (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_COMBO_BOX_TEXT, GtkComboBoxTextClass))
#define GTK_IS_COMBO_BOX_TEXT(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_COMBO_BOX_TEXT))
#define GTK_IS_COMBO_BOX_TEXT_CLASS(klass)      (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_COMBO_BOX_TEXT))
#define GTK_COMBO_BOX_TEXT_GET_CLASS(obj)       (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_COMBO_BOX_TEXT, GtkComboBoxTextClass))

typedef struct _GtkComboBoxText             GtkComboBoxText;
typedef struct _GtkComboBoxTextPrivate      GtkComboBoxTextPrivate;
typedef struct _GtkComboBoxTextClass        GtkComboBoxTextClass;

struct _GtkComboBoxText
{
  /*< private >*/
  GtkComboBox parent_instance;

  GtkComboBoxTextPrivate *priv;
};

struct _GtkComboBoxTextClass
{
  GtkComboBoxClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType         gtk_combo_box_text_get_type        (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget*    gtk_combo_box_text_new             (void);
GDK_AVAILABLE_IN_ALL
GtkWidget*    gtk_combo_box_text_new_with_entry  (void);
GDK_AVAILABLE_IN_ALL
void          gtk_combo_box_text_append_text     (GtkComboBoxText     *combo_box,
                                                  const gchar         *text);
GDK_AVAILABLE_IN_ALL
void          gtk_combo_box_text_insert_text     (GtkComboBoxText     *combo_box,
                                                  gint                 position,
                                                  const gchar         *text);
GDK_AVAILABLE_IN_ALL
void          gtk_combo_box_text_prepend_text    (GtkComboBoxText     *combo_box,
                                                  const gchar         *text);
GDK_AVAILABLE_IN_ALL
void          gtk_combo_box_text_remove          (GtkComboBoxText     *combo_box,
                                                  gint                 position);
GDK_AVAILABLE_IN_ALL
void          gtk_combo_box_text_remove_all      (GtkComboBoxText     *combo_box);
GDK_AVAILABLE_IN_ALL
gchar        *gtk_combo_box_text_get_active_text (GtkComboBoxText     *combo_box);

GDK_AVAILABLE_IN_ALL
void          gtk_combo_box_text_insert          (GtkComboBoxText     *combo_box,
                                                  gint                 position,
                                                  const gchar         *id,
                                                  const gchar         *text);
GDK_AVAILABLE_IN_ALL
void          gtk_combo_box_text_append          (GtkComboBoxText     *combo_box,
                                                  const gchar         *id,
                                                  const gchar         *text);
GDK_AVAILABLE_IN_ALL
void          gtk_combo_box_text_prepend         (GtkComboBoxText     *combo_box,
                                                  const gchar         *id,
                                                  const gchar         *text);

G_END_DECLS

#endif /* __GTK_COMBO_BOX_TEXT_H__ */

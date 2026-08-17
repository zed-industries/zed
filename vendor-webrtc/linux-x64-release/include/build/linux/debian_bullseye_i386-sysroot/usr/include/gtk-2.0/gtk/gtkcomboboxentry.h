/* gtkcomboboxentry.h
 * Copyright (C) 2002, 2003  Kristian Rietveld <kris@gtk.org>
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_COMBO_BOX_ENTRY_H__
#define __GTK_COMBO_BOX_ENTRY_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcombobox.h>
#include <gtk/gtktreemodel.h>

G_BEGIN_DECLS

#ifndef GTK_DISABLE_DEPRECATED

#define GTK_TYPE_COMBO_BOX_ENTRY             (gtk_combo_box_entry_get_type ())
#define GTK_COMBO_BOX_ENTRY(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_COMBO_BOX_ENTRY, GtkComboBoxEntry))
#define GTK_COMBO_BOX_ENTRY_CLASS(vtable)    (G_TYPE_CHECK_CLASS_CAST ((vtable), GTK_TYPE_COMBO_BOX_ENTRY, GtkComboBoxEntryClass))
#define GTK_IS_COMBO_BOX_ENTRY(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_COMBO_BOX_ENTRY))
#define GTK_IS_COMBO_BOX_ENTRY_CLASS(vtable) (G_TYPE_CHECK_CLASS_TYPE ((vtable), GTK_TYPE_COMBO_BOX_ENTRY))
#define GTK_COMBO_BOX_ENTRY_GET_CLASS(inst)  (G_TYPE_INSTANCE_GET_CLASS ((inst), GTK_TYPE_COMBO_BOX_ENTRY, GtkComboBoxEntryClass))

typedef struct _GtkComboBoxEntry             GtkComboBoxEntry;
typedef struct _GtkComboBoxEntryClass        GtkComboBoxEntryClass;
typedef struct _GtkComboBoxEntryPrivate      GtkComboBoxEntryPrivate;

struct _GtkComboBoxEntry
{
  GtkComboBox parent_instance;

  /*< private >*/
  GtkComboBoxEntryPrivate *GSEAL (priv);
};

struct _GtkComboBoxEntryClass
{
  GtkComboBoxClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved0) (void);
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
};


GType       gtk_combo_box_entry_get_type        (void) G_GNUC_CONST;
GtkWidget  *gtk_combo_box_entry_new             (void);
GtkWidget  *gtk_combo_box_entry_new_with_model  (GtkTreeModel     *model,
                                                 gint              text_column);

void        gtk_combo_box_entry_set_text_column (GtkComboBoxEntry *entry_box,
                                                 gint              text_column);
gint        gtk_combo_box_entry_get_text_column (GtkComboBoxEntry *entry_box);

/* convenience -- text */
GtkWidget  *gtk_combo_box_entry_new_text        (void);

#endif

G_END_DECLS

#endif /* __GTK_COMBO_BOX_ENTRY_H__ */

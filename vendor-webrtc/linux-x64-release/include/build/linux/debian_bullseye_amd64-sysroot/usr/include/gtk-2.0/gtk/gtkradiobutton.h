/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_RADIO_BUTTON_H__
#define __GTK_RADIO_BUTTON_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcheckbutton.h>


G_BEGIN_DECLS

#define GTK_TYPE_RADIO_BUTTON		       (gtk_radio_button_get_type ())
#define GTK_RADIO_BUTTON(obj)		       (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_RADIO_BUTTON, GtkRadioButton))
#define GTK_RADIO_BUTTON_CLASS(klass)	       (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_RADIO_BUTTON, GtkRadioButtonClass))
#define GTK_IS_RADIO_BUTTON(obj)	       (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_RADIO_BUTTON))
#define GTK_IS_RADIO_BUTTON_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_RADIO_BUTTON))
#define GTK_RADIO_BUTTON_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_RADIO_BUTTON, GtkRadioButtonClass))


typedef struct _GtkRadioButton	     GtkRadioButton;
typedef struct _GtkRadioButtonClass  GtkRadioButtonClass;

struct _GtkRadioButton
{
  GtkCheckButton check_button;

  GSList *GSEAL (group);
};

struct _GtkRadioButtonClass
{
  GtkCheckButtonClass parent_class;

  /* Signals */
  void (*group_changed) (GtkRadioButton *radio_button);

  /* Padding for future expansion */
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType	   gtk_radio_button_get_type	     (void) G_GNUC_CONST;

GtkWidget* gtk_radio_button_new                           (GSList         *group);
GtkWidget* gtk_radio_button_new_from_widget               (GtkRadioButton *radio_group_member);
GtkWidget* gtk_radio_button_new_with_label                (GSList         *group,
                                                           const gchar    *label);
GtkWidget* gtk_radio_button_new_with_label_from_widget    (GtkRadioButton *radio_group_member,
                                                           const gchar    *label);
GtkWidget* gtk_radio_button_new_with_mnemonic             (GSList         *group,
                                                           const gchar    *label);
GtkWidget* gtk_radio_button_new_with_mnemonic_from_widget (GtkRadioButton *radio_group_member,
                                                           const gchar    *label);
GSList*    gtk_radio_button_get_group                     (GtkRadioButton *radio_button);
void       gtk_radio_button_set_group                     (GtkRadioButton *radio_button,
                                                           GSList         *group);

#ifndef GTK_DISABLE_DEPRECATED
#define gtk_radio_button_group gtk_radio_button_get_group
#endif

G_END_DECLS

#endif /* __GTK_RADIO_BUTTON_H__ */

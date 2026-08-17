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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
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

#ifndef __GTK_TOGGLE_BUTTON_H__
#define __GTK_TOGGLE_BUTTON_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbutton.h>


G_BEGIN_DECLS

#define GTK_TYPE_TOGGLE_BUTTON                  (gtk_toggle_button_get_type ())
#define GTK_TOGGLE_BUTTON(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TOGGLE_BUTTON, GtkToggleButton))
#define GTK_TOGGLE_BUTTON_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TOGGLE_BUTTON, GtkToggleButtonClass))
#define GTK_IS_TOGGLE_BUTTON(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TOGGLE_BUTTON))
#define GTK_IS_TOGGLE_BUTTON_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TOGGLE_BUTTON))
#define GTK_TOGGLE_BUTTON_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TOGGLE_BUTTON, GtkToggleButtonClass))


typedef struct _GtkToggleButton       GtkToggleButton;
typedef struct _GtkToggleButtonClass  GtkToggleButtonClass;

struct _GtkToggleButton
{
  GtkButton button;

  guint GSEAL (active) : 1;
  guint GSEAL (draw_indicator) : 1;
  guint GSEAL (inconsistent) : 1;
};

struct _GtkToggleButtonClass
{
  GtkButtonClass parent_class;

  void (* toggled) (GtkToggleButton *toggle_button);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType      gtk_toggle_button_get_type          (void) G_GNUC_CONST;

GtkWidget* gtk_toggle_button_new               (void);
GtkWidget* gtk_toggle_button_new_with_label    (const gchar     *label);
GtkWidget* gtk_toggle_button_new_with_mnemonic (const gchar     *label);
void       gtk_toggle_button_set_mode          (GtkToggleButton *toggle_button,
                                                gboolean         draw_indicator);
gboolean   gtk_toggle_button_get_mode          (GtkToggleButton *toggle_button);
void       gtk_toggle_button_set_active        (GtkToggleButton *toggle_button,
                                                gboolean         is_active);
gboolean   gtk_toggle_button_get_active        (GtkToggleButton *toggle_button);
void       gtk_toggle_button_toggled           (GtkToggleButton *toggle_button);
void       gtk_toggle_button_set_inconsistent  (GtkToggleButton *toggle_button,
                                                gboolean         setting);
gboolean   gtk_toggle_button_get_inconsistent  (GtkToggleButton *toggle_button);


#ifndef GTK_DISABLE_DEPRECATED
#define	gtk_toggle_button_set_state		gtk_toggle_button_set_active
#endif /* GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_TOGGLE_BUTTON_H__ */

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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_TOGGLE_BUTTON_H__
#define __GTK_TOGGLE_BUTTON_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
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

typedef struct _GtkToggleButton              GtkToggleButton;
typedef struct _GtkToggleButtonPrivate       GtkToggleButtonPrivate;
typedef struct _GtkToggleButtonClass         GtkToggleButtonClass;

struct _GtkToggleButton
{
  /*< private >*/
  GtkButton button;

  GtkToggleButtonPrivate *priv;
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


GDK_AVAILABLE_IN_ALL
GType      gtk_toggle_button_get_type          (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_toggle_button_new               (void);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_toggle_button_new_with_label    (const gchar     *label);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_toggle_button_new_with_mnemonic (const gchar     *label);
GDK_AVAILABLE_IN_ALL
void       gtk_toggle_button_set_mode          (GtkToggleButton *toggle_button,
                                                gboolean         draw_indicator);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_toggle_button_get_mode          (GtkToggleButton *toggle_button);
GDK_AVAILABLE_IN_ALL
void       gtk_toggle_button_set_active        (GtkToggleButton *toggle_button,
                                                gboolean         is_active);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_toggle_button_get_active        (GtkToggleButton *toggle_button);
GDK_AVAILABLE_IN_ALL
void       gtk_toggle_button_toggled           (GtkToggleButton *toggle_button);
GDK_AVAILABLE_IN_ALL
void       gtk_toggle_button_set_inconsistent  (GtkToggleButton *toggle_button,
                                                gboolean         setting);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_toggle_button_get_inconsistent  (GtkToggleButton *toggle_button);


G_END_DECLS

#endif /* __GTK_TOGGLE_BUTTON_H__ */

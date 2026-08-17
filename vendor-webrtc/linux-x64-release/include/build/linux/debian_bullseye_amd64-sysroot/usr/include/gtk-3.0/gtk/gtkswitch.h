/* GTK - The GIMP Toolkit
 *
 * Copyright (C) 2010  Intel Corporation
 * Copyright (C) 2010  RedHat, Inc.
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
 * Author:
 *      Emmanuele Bassi <ebassi@linux.intel.com>
 *      Matthias Clasen <mclasen@redhat.com>
 *
 * Based on similar code from Mx.
 */

#ifndef __GTK_SWITCH_H__
#define __GTK_SWITCH_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_SWITCH                 (gtk_switch_get_type ())
#define GTK_SWITCH(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SWITCH, GtkSwitch))
#define GTK_IS_SWITCH(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SWITCH))
#define GTK_SWITCH_CLASS(klass)         (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SWITCH, GtkSwitchClass))
#define GTK_IS_SWITCH_CLASS(klass)      (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SWITCH))
#define GTK_SWITCH_GET_CLASS(obj)       (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SWITCH, GtkSwitchClass))

typedef struct _GtkSwitch               GtkSwitch;
typedef struct _GtkSwitchPrivate        GtkSwitchPrivate;
typedef struct _GtkSwitchClass          GtkSwitchClass;

/**
 * GtkSwitch:
 *
 * The #GtkSwitch-struct contains private
 * data and it should only be accessed using the provided API.
 */
struct _GtkSwitch
{
  /*< private >*/
  GtkWidget parent_instance;

  GtkSwitchPrivate *priv;
};

/**
 * GtkSwitchClass:
 * @parent_class: The parent class.
 * @activate: An action signal and emitting it causes the switch to animate.
 * @state_set: Class handler for the ::state-set signal.
 */
struct _GtkSwitchClass
{
  GtkWidgetClass parent_class;

  /*< public >*/

  void (* activate) (GtkSwitch *sw);

  gboolean (* state_set) (GtkSwitch *sw, gboolean state);
  /*< private >*/

  void (* _switch_padding_1) (void);
  void (* _switch_padding_2) (void);
  void (* _switch_padding_3) (void);
  void (* _switch_padding_4) (void);
  void (* _switch_padding_5) (void);
};

GDK_AVAILABLE_IN_ALL
GType gtk_switch_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *     gtk_switch_new          (void);

GDK_AVAILABLE_IN_ALL
void            gtk_switch_set_active   (GtkSwitch *sw,
                                         gboolean   is_active);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_switch_get_active   (GtkSwitch *sw);

GDK_AVAILABLE_IN_3_14
void            gtk_switch_set_state   (GtkSwitch *sw,
                                        gboolean   state);
GDK_AVAILABLE_IN_3_14
gboolean        gtk_switch_get_state   (GtkSwitch *sw);

G_END_DECLS

#endif /* __GTK_SWITCH_H__ */

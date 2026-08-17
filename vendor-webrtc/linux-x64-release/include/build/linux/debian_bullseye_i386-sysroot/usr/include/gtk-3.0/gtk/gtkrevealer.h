/*
 * Copyright (c) 2013 Red Hat, Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by 
 * the Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public 
 * License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License 
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 * Author: Alexander Larsson <alexl@redhat.com>
 *
 */

#ifndef __GTK_REVEALER_H__
#define __GTK_REVEALER_H__

#include <gtk/gtkbin.h>

G_BEGIN_DECLS


#define GTK_TYPE_REVEALER (gtk_revealer_get_type ())
#define GTK_REVEALER(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_REVEALER, GtkRevealer))
#define GTK_REVEALER_CLASS(klass) (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_REVEALER, GtkRevealerClass))
#define GTK_IS_REVEALER(obj) (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_REVEALER))
#define GTK_IS_REVEALER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_REVEALER))
#define GTK_REVEALER_GET_CLASS(obj) (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_REVEALER, GtkRevealerClass))

typedef struct _GtkRevealer GtkRevealer;
typedef struct _GtkRevealerClass GtkRevealerClass;

typedef enum {
  GTK_REVEALER_TRANSITION_TYPE_NONE,
  GTK_REVEALER_TRANSITION_TYPE_CROSSFADE,
  GTK_REVEALER_TRANSITION_TYPE_SLIDE_RIGHT,
  GTK_REVEALER_TRANSITION_TYPE_SLIDE_LEFT,
  GTK_REVEALER_TRANSITION_TYPE_SLIDE_UP,
  GTK_REVEALER_TRANSITION_TYPE_SLIDE_DOWN
} GtkRevealerTransitionType;

struct _GtkRevealer {
  GtkBin parent_instance;
};

/**
 * GtkRevealerClass:
 * @parent_class: The parent class.
 */
struct _GtkRevealerClass {
  GtkBinClass parent_class;
};

GDK_AVAILABLE_IN_3_10
GType                      gtk_revealer_get_type                (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_3_10
GtkWidget*                 gtk_revealer_new                     (void);
GDK_AVAILABLE_IN_3_10
gboolean                   gtk_revealer_get_reveal_child        (GtkRevealer               *revealer);
GDK_AVAILABLE_IN_3_10
void                       gtk_revealer_set_reveal_child        (GtkRevealer               *revealer,
                                                                 gboolean                   reveal_child);
GDK_AVAILABLE_IN_3_10
gboolean                   gtk_revealer_get_child_revealed      (GtkRevealer               *revealer);
GDK_AVAILABLE_IN_3_10
guint                      gtk_revealer_get_transition_duration (GtkRevealer               *revealer);
GDK_AVAILABLE_IN_3_10
void                       gtk_revealer_set_transition_duration (GtkRevealer               *revealer,
                                                                 guint                      duration);
GDK_AVAILABLE_IN_3_10
void                       gtk_revealer_set_transition_type     (GtkRevealer               *revealer,
                                                                 GtkRevealerTransitionType  transition);
GDK_AVAILABLE_IN_3_10
GtkRevealerTransitionType  gtk_revealer_get_transition_type     (GtkRevealer               *revealer);


G_END_DECLS

#endif

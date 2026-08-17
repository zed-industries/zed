/*
 * Copyright © 2012 Canonical Limited
 *
 * This library is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2 of the
 * licence or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Ryan Lortie <desrt@desrt.ca>
 */

#ifndef __GTK_ACTIONABLE_H__
#define __GTK_ACTIONABLE_H__

#include <glib-object.h>
#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GTK_TYPE_ACTIONABLE                                 (gtk_actionable_get_type ())
#define GTK_ACTIONABLE(inst)                                (G_TYPE_CHECK_INSTANCE_CAST ((inst),                     \
                                                             GTK_TYPE_ACTIONABLE, GtkActionable))
#define GTK_IS_ACTIONABLE(inst)                             (G_TYPE_CHECK_INSTANCE_TYPE ((inst),                     \
                                                             GTK_TYPE_ACTIONABLE))
#define GTK_ACTIONABLE_GET_IFACE(inst)                      (G_TYPE_INSTANCE_GET_INTERFACE ((inst),                  \
                                                             GTK_TYPE_ACTIONABLE, GtkActionableInterface))

typedef struct _GtkActionableInterface                      GtkActionableInterface;
typedef struct _GtkActionable                               GtkActionable;

struct _GtkActionableInterface
{
  /*< private >*/
  GTypeInterface g_iface;

  /*< public >*/

  const gchar * (* get_action_name)             (GtkActionable *actionable);
  void          (* set_action_name)             (GtkActionable *actionable,
                                                 const gchar   *action_name);
  GVariant *    (* get_action_target_value)     (GtkActionable *actionable);
  void          (* set_action_target_value)     (GtkActionable *actionable,
                                                 GVariant      *target_value);
};

GDK_AVAILABLE_IN_3_4
GType                   gtk_actionable_get_type                         (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_4
const gchar *           gtk_actionable_get_action_name                  (GtkActionable *actionable);
GDK_AVAILABLE_IN_3_4
void                    gtk_actionable_set_action_name                  (GtkActionable *actionable,
                                                                         const gchar   *action_name);

GDK_AVAILABLE_IN_3_4
GVariant *              gtk_actionable_get_action_target_value          (GtkActionable *actionable);
GDK_AVAILABLE_IN_3_4
void                    gtk_actionable_set_action_target_value          (GtkActionable *actionable,
                                                                         GVariant      *target_value);

GDK_AVAILABLE_IN_3_4
void                    gtk_actionable_set_action_target                (GtkActionable *actionable,
                                                                         const gchar   *format_string,
                                                                         ...);

GDK_AVAILABLE_IN_3_4
void                    gtk_actionable_set_detailed_action_name         (GtkActionable *actionable,
                                                                         const gchar   *detailed_action_name);

G_END_DECLS

#endif /* __GTK_ACTIONABLE_H__ */

/* GTK - The GIMP Toolkit
 * Copyright (C) 2008 Tristan Van Berkom <tristan.van.berkom@gmail.com>
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

#ifndef __GTK_ACTIVATABLE_H__
#define __GTK_ACTIVATABLE_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkaction.h>
#include <gtk/gtktypeutils.h>

G_BEGIN_DECLS

#define GTK_TYPE_ACTIVATABLE            (gtk_activatable_get_type ())
#define GTK_ACTIVATABLE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ACTIVATABLE, GtkActivatable))
#define GTK_ACTIVATABLE_CLASS(obj)      (G_TYPE_CHECK_CLASS_CAST ((obj), GTK_TYPE_ACTIVATABLE, GtkActivatableIface))
#define GTK_IS_ACTIVATABLE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ACTIVATABLE))
#define GTK_ACTIVATABLE_GET_IFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE ((obj), GTK_TYPE_ACTIVATABLE, GtkActivatableIface))


typedef struct _GtkActivatable      GtkActivatable; /* Dummy typedef */
typedef struct _GtkActivatableIface GtkActivatableIface;


/**
 * GtkActivatableIface:
 * @update: Called to update the activatable when its related action's properties change.
 * You must check the #GtkActivatable:use-action-appearance property only apply action
 * properties that are meant to effect the appearance accordingly.
 * @sync_action_properties: Called to update the activatable completely, this is called internally when
 * #GtkActivatable::related-action property is set or unset and by the implementor when
 * #GtkActivatable::use-action-appearance changes.<note><para>This method can be called
 * with a %NULL action at times</para></note>
 *
 * Since: 2.16
 */

struct _GtkActivatableIface
{
  GTypeInterface g_iface;

  /* virtual table */
  void   (* update)                   (GtkActivatable *activatable,
		                       GtkAction      *action,
		                       const gchar    *property_name);
  void   (* sync_action_properties)   (GtkActivatable *activatable,
		                       GtkAction      *action);
};


GType      gtk_activatable_get_type                   (void) G_GNUC_CONST;

void       gtk_activatable_sync_action_properties     (GtkActivatable *activatable,
						       GtkAction      *action);

void       gtk_activatable_set_related_action         (GtkActivatable *activatable,
						       GtkAction      *action);
GtkAction *gtk_activatable_get_related_action         (GtkActivatable *activatable);

void       gtk_activatable_set_use_action_appearance  (GtkActivatable *activatable,
						       gboolean        use_appearance);
gboolean   gtk_activatable_get_use_action_appearance  (GtkActivatable *activatable);

/* For use in activatable implementations */
void       gtk_activatable_do_set_related_action      (GtkActivatable *activatable,
						       GtkAction      *action);

G_END_DECLS

#endif /* __GTK_ACTIVATABLE_H__ */

/* GTK - The GIMP Toolkit
 * Recent chooser action for GtkUIManager
 *
 * Copyright (C) 2007, Emmanuele Bassi
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

#ifndef __GTK_RECENT_ACTION_H__
#define __GTK_RECENT_ACTION_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkaction.h>
#include <gtk/gtkrecentmanager.h>

G_BEGIN_DECLS

#define GTK_TYPE_RECENT_ACTION                  (gtk_recent_action_get_type ())
#define GTK_RECENT_ACTION(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_RECENT_ACTION, GtkRecentAction))
#define GTK_IS_RECENT_ACTION(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_RECENT_ACTION))
#define GTK_RECENT_ACTION_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_RECENT_ACTION, GtkRecentActionClass))
#define GTK_IS_RECENT_ACTION_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_RECENT_ACTION))
#define GTK_RECENT_ACTION_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_RECENT_ACTION, GtkRecentActionClass))

typedef struct _GtkRecentAction         GtkRecentAction;
typedef struct _GtkRecentActionPrivate  GtkRecentActionPrivate;
typedef struct _GtkRecentActionClass    GtkRecentActionClass;

struct _GtkRecentAction
{
  GtkAction parent_instance;

  /*< private >*/
  GtkRecentActionPrivate *GSEAL (priv);
};

struct _GtkRecentActionClass
{
  GtkActionClass parent_class;
};

GType      gtk_recent_action_get_type         (void) G_GNUC_CONST;
GtkAction *gtk_recent_action_new              (const gchar      *name,
                                               const gchar      *label,
                                               const gchar      *tooltip,
                                               const gchar      *stock_id);
GtkAction *gtk_recent_action_new_for_manager  (const gchar      *name,
                                               const gchar      *label,
                                               const gchar      *tooltip,
                                               const gchar      *stock_id,
                                               GtkRecentManager *manager);
gboolean   gtk_recent_action_get_show_numbers (GtkRecentAction  *action);
void       gtk_recent_action_set_show_numbers (GtkRecentAction  *action,
                                               gboolean          show_numbers);

G_END_DECLS

#endif /* __GTK_RECENT_ACTION_H__ */

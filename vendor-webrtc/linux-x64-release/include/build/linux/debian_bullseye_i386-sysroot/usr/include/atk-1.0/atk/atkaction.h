/* ATK -  Accessibility Toolkit
 * Copyright 2001 Sun Microsystems Inc.
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

#ifndef __ATK_ACTION_H__
#define __ATK_ACTION_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkobject.h>

G_BEGIN_DECLS

/*
 */


#define ATK_TYPE_ACTION                    (atk_action_get_type ())
#define ATK_IS_ACTION(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_ACTION)
#define ATK_ACTION(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_ACTION, AtkAction)
#define ATK_ACTION_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATK_TYPE_ACTION, AtkActionIface))

#ifndef _TYPEDEF_ATK_ACTION_
#define _TYPEDEF_ATK_ACTION_
typedef struct _AtkAction AtkAction;
#endif
typedef struct _AtkActionIface AtkActionIface;

/**
 * AtkActionIface:
 * @do_action:
 * @get_n_actions:
 * @get_description:
 * @get_name:
 * @get_keybinding:
 * @set_description:
 * @get_localized_name:
 *
 * The #AtkAction interface should be supported by any object that can
 * perform one or more actions. The interface provides the standard
 * mechanism for an assistive technology to determine what those actions
 * are as well as tell the object to perform them. Any object that can
 * be manipulated should support this interface.
 */
struct _AtkActionIface
{
  /*< private >*/
  GTypeInterface parent;

  /*< public >*/
  gboolean                (*do_action)         (AtkAction         *action,
                                                gint              i);
  gint                    (*get_n_actions)     (AtkAction         *action);
  const gchar*            (*get_description)   (AtkAction         *action,
                                                gint              i);
  const gchar*            (*get_name)          (AtkAction         *action,
                                                gint              i);
  const gchar*            (*get_keybinding)    (AtkAction         *action,
                                                gint              i);
  gboolean                (*set_description)   (AtkAction         *action,
                                                gint              i,
                                                const gchar       *desc);
  const gchar*            (*get_localized_name)(AtkAction         *action,
						gint              i);
};

ATK_AVAILABLE_IN_ALL
GType atk_action_get_type (void);

/*
 * These are the function which would be called by an application with
 * the argument being a AtkObject object cast to (AtkAction).
 *
 * The function will just check that * the corresponding
 * function pointer is not NULL and will call it.
 *
 * The "real" implementation of the function for accessible will be
 * provided in a support library
 */

ATK_AVAILABLE_IN_ALL
gboolean   atk_action_do_action                (AtkAction         *action,
                                            gint              i);
ATK_AVAILABLE_IN_ALL
gint   atk_action_get_n_actions            (AtkAction *action);
ATK_AVAILABLE_IN_ALL
const gchar*          atk_action_get_description  (AtkAction         *action,
                                                   gint              i);
ATK_AVAILABLE_IN_ALL
const gchar*          atk_action_get_name         (AtkAction         *action,
                                                   gint              i);
ATK_AVAILABLE_IN_ALL
const gchar*          atk_action_get_keybinding   (AtkAction         *action,
                                                   gint              i);
ATK_AVAILABLE_IN_ALL
gboolean              atk_action_set_description  (AtkAction         *action,
                                                   gint              i,
                                                   const gchar       *desc);

/* NEW in ATK 1.1: */
ATK_AVAILABLE_IN_ALL
const gchar* atk_action_get_localized_name (AtkAction       *action,
						     gint            i);

/*
 * Additional GObject properties exported by AtkAction:
 *    "accessible_action"
 *       (an accessible action, or the list of actions, has changed)
 */

G_END_DECLS

#endif /* __ATK_ACTION_H__ */

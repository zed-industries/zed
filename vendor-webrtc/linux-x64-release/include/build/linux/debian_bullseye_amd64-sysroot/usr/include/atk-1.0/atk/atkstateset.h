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

#ifndef __ATK_STATE_SET_H__
#define __ATK_STATE_SET_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <glib-object.h>
#include <atk/atkobject.h>
#include <atk/atkstate.h>

G_BEGIN_DECLS

#define ATK_TYPE_STATE_SET                        (atk_state_set_get_type ())
#define ATK_STATE_SET(obj)                        (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_STATE_SET, AtkStateSet))
#define ATK_STATE_SET_CLASS(klass)                (G_TYPE_CHECK_CLASS_CAST ((klass), ATK_TYPE_STATE_SET, AtkStateSetClass))
#define ATK_IS_STATE_SET(obj)                     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_STATE_SET))
#define ATK_IS_STATE_SET_CLASS(klass)             (G_TYPE_CHECK_CLASS_TYPE ((klass), ATK_TYPE_STATE_SET))
#define ATK_STATE_SET_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), ATK_TYPE_STATE_SET, AtkStateSetClass))

typedef struct _AtkStateSetClass       AtkStateSetClass;


struct _AtkStateSet
{
  GObject parent;

};

struct _AtkStateSetClass
{
  GObjectClass parent;
};

ATK_AVAILABLE_IN_ALL
GType atk_state_set_get_type (void);

ATK_AVAILABLE_IN_ALL
AtkStateSet*    atk_state_set_new               (void);
ATK_AVAILABLE_IN_ALL
gboolean        atk_state_set_is_empty          (AtkStateSet  *set);
ATK_AVAILABLE_IN_ALL
gboolean        atk_state_set_add_state         (AtkStateSet  *set,
                                                 AtkStateType type);
ATK_AVAILABLE_IN_ALL
void            atk_state_set_add_states        (AtkStateSet  *set,
                                                 AtkStateType *types,
                                                 gint         n_types);
ATK_AVAILABLE_IN_ALL
void            atk_state_set_clear_states      (AtkStateSet  *set);
ATK_AVAILABLE_IN_ALL
gboolean        atk_state_set_contains_state    (AtkStateSet  *set,
                                                 AtkStateType type);
ATK_AVAILABLE_IN_ALL
gboolean        atk_state_set_contains_states   (AtkStateSet  *set,
                                                 AtkStateType *types,
                                                 gint         n_types);
ATK_AVAILABLE_IN_ALL
gboolean        atk_state_set_remove_state      (AtkStateSet  *set,
                                                 AtkStateType type);
ATK_AVAILABLE_IN_ALL
AtkStateSet*    atk_state_set_and_sets          (AtkStateSet  *set,
                                                 AtkStateSet  *compare_set);
ATK_AVAILABLE_IN_ALL
AtkStateSet*    atk_state_set_or_sets           (AtkStateSet  *set,
                                                 AtkStateSet  *compare_set);
ATK_AVAILABLE_IN_ALL
AtkStateSet*    atk_state_set_xor_sets          (AtkStateSet  *set,
                                                 AtkStateSet  *compare_set);

G_END_DECLS

#endif /* __ATK_STATE_SET_H__ */

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

#ifndef __ATK_SELECTION_H__
#define __ATK_SELECTION_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkobject.h>

G_BEGIN_DECLS

/*
 * This AtkSelection interface provides the standard mechanism for an 
 * assistive technology to determine what the current selected children are, 
 * as well as modify the selection set. Any object that has children that 
 * can be selected should support the AtkSelection interface.
 */

#define ATK_TYPE_SELECTION                        (atk_selection_get_type ())
#define ATK_IS_SELECTION(obj)                     G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_SELECTION)
#define ATK_SELECTION(obj)                        G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_SELECTION, AtkSelection)
#define ATK_SELECTION_GET_IFACE(obj)              (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATK_TYPE_SELECTION, AtkSelectionIface))

#ifndef _TYPEDEF_ATK_SELECTION_
#define _TYPEDEF_ATK_SELECTION_
typedef struct _AtkSelection AtkSelection;
#endif
typedef struct _AtkSelectionIface AtkSelectionIface;

struct _AtkSelectionIface
{
  GTypeInterface parent;

  gboolean     (* add_selection)        (AtkSelection   *selection,
                                         gint           i);
  gboolean     (* clear_selection)      (AtkSelection   *selection);
  AtkObject*   (* ref_selection)        (AtkSelection   *selection,
                                         gint           i);
  gint         (* get_selection_count)  (AtkSelection   *selection);
  gboolean     (* is_child_selected)    (AtkSelection   *selection,
                                         gint           i);
  gboolean     (* remove_selection)     (AtkSelection   *selection,
                                         gint           i);
  gboolean     (* select_all_selection) (AtkSelection   *selection);

  /* signal handlers */
  
  void         (*selection_changed)     (AtkSelection   *selection);
};

ATK_AVAILABLE_IN_ALL
GType atk_selection_get_type (void);

ATK_AVAILABLE_IN_ALL
gboolean     atk_selection_add_selection        (AtkSelection   *selection,
                                                 gint           i);

ATK_AVAILABLE_IN_ALL
gboolean     atk_selection_clear_selection      (AtkSelection   *selection);

ATK_AVAILABLE_IN_ALL
AtkObject*   atk_selection_ref_selection        (AtkSelection   *selection,
                                                 gint           i);

ATK_AVAILABLE_IN_ALL
gint         atk_selection_get_selection_count  (AtkSelection   *selection);

ATK_AVAILABLE_IN_ALL
gboolean     atk_selection_is_child_selected    (AtkSelection   *selection,
                                                 gint           i);

ATK_AVAILABLE_IN_ALL
gboolean     atk_selection_remove_selection     (AtkSelection   *selection,
                                                 gint           i);

ATK_AVAILABLE_IN_ALL
gboolean     atk_selection_select_all_selection (AtkSelection   *selection);

G_END_DECLS

#endif /* __ATK_SELECTION_H__ */

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

#ifndef __ATK_EDITABLE_TEXT_H__
#define __ATK_EDITABLE_TEXT_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkobject.h>
#include <atk/atktext.h>

G_BEGIN_DECLS

/*
 * AtkEditableText is used to support access in an "accessibility" context
 * to editing features of editable text widgets.
 */

#define ATK_TYPE_EDITABLE_TEXT                    (atk_editable_text_get_type ())
#define ATK_IS_EDITABLE_TEXT(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_EDITABLE_TEXT)
#define ATK_EDITABLE_TEXT(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_EDITABLE_TEXT, AtkEditableText)
#define ATK_EDITABLE_TEXT_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATK_TYPE_EDITABLE_TEXT, AtkEditableTextIface))

#ifndef _TYPEDEF_ATK_EDITABLE_TEXT_
#define _TYPEDEF_ATK_EDITABLE_TEXT_
typedef struct _AtkEditableText AtkEditableText;
#endif
typedef struct _AtkEditableTextIface AtkEditableTextIface;

struct _AtkEditableTextIface
{
  GTypeInterface parent_interface;

  gboolean (* set_run_attributes) (AtkEditableText  *text,
                                   AtkAttributeSet  *attrib_set,
                                   gint		    start_offset,
 				   gint		    end_offset);
  void   (* set_text_contents)    (AtkEditableText  *text,
                                   const gchar      *string);
  void   (* insert_text)          (AtkEditableText  *text,
                                   const gchar      *string,
                                   gint             length,
                                   gint             *position);
  void   (* copy_text)            (AtkEditableText  *text,
                                   gint             start_pos,
                                   gint             end_pos);
  void   (* cut_text)             (AtkEditableText  *text,
                                   gint             start_pos,
                                   gint             end_pos);
  void   (* delete_text)          (AtkEditableText  *text,
                                   gint             start_pos,
                                   gint             end_pos);
  void   (* paste_text)           (AtkEditableText  *text,
                                   gint             position);
};
ATK_AVAILABLE_IN_ALL
GType atk_editable_text_get_type (void);


ATK_AVAILABLE_IN_ALL
gboolean atk_editable_text_set_run_attributes (AtkEditableText          *text,
                                               AtkAttributeSet  *attrib_set,
                                               gint    	        start_offset,
 					       gint	        end_offset);
ATK_AVAILABLE_IN_ALL
void atk_editable_text_set_text_contents    (AtkEditableText  *text,
                                             const gchar      *string);
ATK_AVAILABLE_IN_ALL
void atk_editable_text_insert_text          (AtkEditableText  *text,
                                             const gchar      *string,
                                             gint             length,
                                             gint             *position);
ATK_AVAILABLE_IN_ALL
void atk_editable_text_copy_text            (AtkEditableText  *text,
                                             gint             start_pos,
                                             gint             end_pos);
ATK_AVAILABLE_IN_ALL
void atk_editable_text_cut_text             (AtkEditableText  *text,
                                             gint             start_pos,
                                             gint             end_pos);
ATK_AVAILABLE_IN_ALL
void atk_editable_text_delete_text          (AtkEditableText  *text,
                                             gint             start_pos,
                                             gint             end_pos);
ATK_AVAILABLE_IN_ALL
void atk_editable_text_paste_text           (AtkEditableText  *text,
                                             gint             position);

G_END_DECLS

#endif /* __ATK_EDITABLE_TEXT_H__ */

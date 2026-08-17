/* ATK -  Accessibility Toolkit
 * Copyright 2002 Sun Microsystems Inc.
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

#ifndef __ATK_RELATION_TYPE_H__
#define __ATK_RELATION_TYPE_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <glib.h>

G_BEGIN_DECLS

/**
 *AtkRelationType:
 *@ATK_RELATION_NULL: Not used, represens "no relationship" or an error condition.
 *@ATK_RELATION_CONTROLLED_BY: Indicates an object controlled by one or more target objects.
 *@ATK_RELATION_CONTROLLER_FOR: Indicates an object is an controller for one or more target objects.
 *@ATK_RELATION_LABEL_FOR: Indicates an object is a label for one or more target objects.
 *@ATK_RELATION_LABELLED_BY: Indicates an object is labelled by one or more target objects.
 *@ATK_RELATION_MEMBER_OF: Indicates an object is a member of a group of one or more target objects.
 *@ATK_RELATION_NODE_CHILD_OF: Indicates an object is a cell in a treetable which is displayed because a cell in the same column is expanded and identifies that cell.
 *@ATK_RELATION_FLOWS_TO: Indicates that the object has content that flows logically to another
 *  AtkObject in a sequential way, (for instance text-flow).
 *@ATK_RELATION_FLOWS_FROM: Indicates that the object has content that flows logically from
 *  another AtkObject in a sequential way, (for instance text-flow).
 *@ATK_RELATION_SUBWINDOW_OF: Indicates a subwindow attached to a component but otherwise has no connection in  the UI heirarchy to that component.
 *@ATK_RELATION_EMBEDS: Indicates that the object visually embeds 
 *  another object's content, i.e. this object's content flows around 
 *  another's content.
 *@ATK_RELATION_EMBEDDED_BY: Reciprocal of %ATK_RELATION_EMBEDS, indicates that
 *  this object's content is visualy embedded in another object.
 *@ATK_RELATION_POPUP_FOR: Indicates that an object is a popup for another object.
 *@ATK_RELATION_PARENT_WINDOW_OF: Indicates that an object is a parent window of another object.
 *@ATK_RELATION_DESCRIBED_BY: Reciprocal of %ATK_RELATION_DESCRIPTION_FOR. Indicates that one
 * or more target objects provide descriptive information about this object. This relation
 * type is most appropriate for information that is not essential as its presentation may
 * be user-configurable and/or limited to an on-demand mechanism such as an assistive
 * technology command. For brief, essential information such as can be found in a widget's
 * on-screen label, use %ATK_RELATION_LABELLED_BY. For an on-screen error message, use
 * %ATK_RELATION_ERROR_MESSAGE. For lengthy extended descriptive information contained in
 * an on-screen object, consider using %ATK_RELATION_DETAILS as assistive technologies may
 * provide a means for the user to navigate to objects containing detailed descriptions so
 * that their content can be more closely reviewed.
 *@ATK_RELATION_DESCRIPTION_FOR: Reciprocal of %ATK_RELATION_DESCRIBED_BY. Indicates that this
 * object provides descriptive information about the target object(s). See also
 * %ATK_RELATION_DETAILS_FOR and %ATK_RELATION_ERROR_FOR.
 *@ATK_RELATION_NODE_PARENT_OF: Indicates an object is a cell in a treetable and is expanded to display other cells in the same column.
 *@ATK_RELATION_DETAILS: Reciprocal of %ATK_RELATION_DETAILS_FOR. Indicates that this object
 * has a detailed or extended description, the contents of which can be found in the target
 * object(s). This relation type is most appropriate for information that is sufficiently
 * lengthy as to make navigation to the container of that information desirable. For less
 * verbose information suitable for announcement only, see %ATK_RELATION_DESCRIBED_BY. If
 * the detailed information describes an error condition, %ATK_RELATION_ERROR_FOR should be
 * used instead. @Since: ATK-2.26.
 *@ATK_RELATION_DETAILS_FOR: Reciprocal of %ATK_RELATION_DETAILS. Indicates that this object
 * provides a detailed or extended description about the target object(s). See also
 * %ATK_RELATION_DESCRIPTION_FOR and %ATK_RELATION_ERROR_FOR. @Since: ATK-2.26.
 *@ATK_RELATION_ERROR_MESSAGE: Reciprocal of %ATK_RELATION_ERROR_FOR. Indicates that this object
 * has one or more errors, the nature of which is described in the contents of the target
 * object(s). Objects that have this relation type should also contain %ATK_STATE_INVALID_ENTRY
 * in their #AtkStateSet. @Since: ATK-2.26.
 *@ATK_RELATION_ERROR_FOR: Reciprocal of %ATK_RELATION_ERROR_MESSAGE. Indicates that this object
 * contains an error message describing an invalid condition in the target object(s). @Since:
 * ATK_2.26.
 *@ATK_RELATION_LAST_DEFINED: Not used, this value indicates the end of the enumeration.
 * 
 *Describes the type of the relation
 **/
typedef enum
{
  ATK_RELATION_NULL = 0,
  ATK_RELATION_CONTROLLED_BY,
  ATK_RELATION_CONTROLLER_FOR,
  ATK_RELATION_LABEL_FOR,
  ATK_RELATION_LABELLED_BY,
  ATK_RELATION_MEMBER_OF,
  ATK_RELATION_NODE_CHILD_OF,
  ATK_RELATION_FLOWS_TO,
  ATK_RELATION_FLOWS_FROM,
  ATK_RELATION_SUBWINDOW_OF, 
  ATK_RELATION_EMBEDS, 
  ATK_RELATION_EMBEDDED_BY, 
  ATK_RELATION_POPUP_FOR, 
  ATK_RELATION_PARENT_WINDOW_OF, 
  ATK_RELATION_DESCRIBED_BY,
  ATK_RELATION_DESCRIPTION_FOR,
  ATK_RELATION_NODE_PARENT_OF,
  ATK_RELATION_DETAILS,
  ATK_RELATION_DETAILS_FOR,
  ATK_RELATION_ERROR_MESSAGE,
  ATK_RELATION_ERROR_FOR,
  ATK_RELATION_LAST_DEFINED
} AtkRelationType;

G_END_DECLS

#endif /* __ATK_RELATION_TYPE_H__ */

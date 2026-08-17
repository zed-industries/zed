/* ATK -  Accessibility Toolkit
 * Copyright 2020 GNOME Foundation
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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#if !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#ifndef __GI_SCANNER__

G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkAction, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkComponent, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkDocument, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkEditableText, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkGObjectAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkHyperlink, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkHyperlinkImpl, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkHypertext, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkImage, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkImplementor, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkMisc, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkNoOpObjectFactory, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkNoOpObject, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkObjectFactory, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkObject, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkPlug, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkRegistry, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkRelationSet, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkRelation, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkSelection, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkSocket, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkStateSet, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkStreamableContent, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkTableCell, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkTable, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkText, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkUtil, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkValue, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC (AtkWindow, g_object_unref)

#endif /* __GI_SCANNER__ */

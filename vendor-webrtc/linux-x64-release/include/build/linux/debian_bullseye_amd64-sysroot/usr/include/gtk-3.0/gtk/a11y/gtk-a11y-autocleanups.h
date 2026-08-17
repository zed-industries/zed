/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#if !defined (__GTK_A11Y_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk-a11y.h> can be included directly."
#endif

#ifndef __GI_SCANNER__

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkArrowAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkBooleanCellAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkButtonAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkCellAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkCellAccessibleParent, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkCheckMenuItemAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkComboBoxAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkContainerAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkContainerCellAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkEntryAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkExpanderAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkFlowBoxAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkFlowBoxChildAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkFrameAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkIconViewAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkImageAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkImageCellAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkLabelAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkLevelBarAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkLinkButtonAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkListBoxAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkListBoxRowAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkLockButtonAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkMenuAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkMenuButtonAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkMenuItemAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkMenuShellAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkNotebookAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkNotebookPageAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkPanedAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkPopoverAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkProgressBarAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkRadioButtonAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkRadioMenuItemAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkRangeAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkRendererCellAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkScaleAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkScaleButtonAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkScrolledWindowAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkSpinButtonAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkSpinnerAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkStatusbarAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkSwitchAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkTextCellAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkTextViewAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkToggleButtonAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkToplevelAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkTreeViewAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkWidgetAccessible, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkWindowAccessible, g_object_unref)

#endif

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

#ifndef __ATK_STATE_H__
#define __ATK_STATE_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <glib-object.h>
#include <atk/atkversion.h>

G_BEGIN_DECLS

/**
 *AtkStateType:
 *@ATK_STATE_INVALID: Indicates an invalid state - probably an error condition.
 *@ATK_STATE_ACTIVE: Indicates a window is currently the active window, or an object is the active subelement within a container or table. ATK_STATE_ACTIVE should not be used for objects which have ATK_STATE_FOCUSABLE or ATK_STATE_SELECTABLE: Those objects should use ATK_STATE_FOCUSED and ATK_STATE_SELECTED respectively. ATK_STATE_ACTIVE is a means to indicate that an object which is not focusable and not selectable is the currently-active item within its parent container.
 *@ATK_STATE_ARMED: Indicates that the object is 'armed', i.e. will be activated by if a pointer button-release event occurs within its bounds.  Buttons often enter this state when a pointer click occurs within their bounds, as a precursor to activation. ATK_STATE_ARMED has been deprecated since ATK-2.16 and should not be used in newly-written code.
 *@ATK_STATE_BUSY:  Indicates the current object is busy, i.e. onscreen representation is in the process of changing, or the object is temporarily unavailable for interaction due to activity already in progress.  This state may be used by implementors of Document to indicate that content loading is underway.  It also may indicate other 'pending' conditions; clients may wish to interrogate this object when the ATK_STATE_BUSY flag is removed.
 *@ATK_STATE_CHECKED: Indicates this object is currently checked, for instance a checkbox is 'non-empty'.
 *@ATK_STATE_DEFUNCT: Indicates that this object no longer has a valid backing widget (for instance, if its peer object has been destroyed)
 *@ATK_STATE_EDITABLE: Indicates that this object can contain text, and that the
 * user can change the textual contents of this object by editing those contents
 * directly. For an object which is expected to be editable due to its type, but
 * which cannot be edited due to the application or platform preventing the user
 * from doing so, that object's #AtkStateSet should lack ATK_STATE_EDITABLE and
 * should contain ATK_STATE_READ_ONLY.
 *@ATK_STATE_ENABLED: 	Indicates that this object is enabled, i.e. that it currently reflects some application state. Objects that are "greyed out" may lack this state, and may lack the STATE_SENSITIVE if direct user interaction cannot cause them to acquire STATE_ENABLED. See also: ATK_STATE_SENSITIVE
 *@ATK_STATE_EXPANDABLE: Indicates this object allows progressive disclosure of its children
 *@ATK_STATE_EXPANDED: Indicates this object its expanded - see ATK_STATE_EXPANDABLE above
 *@ATK_STATE_FOCUSABLE: Indicates this object can accept keyboard focus, which means all events resulting from typing on the keyboard will normally be passed to it when it has focus
 *@ATK_STATE_FOCUSED: Indicates this object currently has the keyboard focus
 *@ATK_STATE_HORIZONTAL: Indicates the orientation of this object is horizontal; used, for instance, by objects of ATK_ROLE_SCROLL_BAR.  For objects where vertical/horizontal orientation is especially meaningful.
 *@ATK_STATE_ICONIFIED: Indicates this object is minimized and is represented only by an icon
 *@ATK_STATE_MODAL: Indicates something must be done with this object before the user can interact with an object in a different window
 *@ATK_STATE_MULTI_LINE: Indicates this (text) object can contain multiple lines of text
 *@ATK_STATE_MULTISELECTABLE: Indicates this object allows more than one of its children to be selected at the same time, or in the case of text objects, that the object supports non-contiguous text selections.
 *@ATK_STATE_OPAQUE: Indicates this object paints every pixel within its rectangular region.
 *@ATK_STATE_PRESSED: Indicates this object is currently pressed.
 *@ATK_STATE_RESIZABLE: Indicates the size of this object is not fixed
 *@ATK_STATE_SELECTABLE: Indicates this object is the child of an object that allows its children to be selected and that this child is one of those children that can be selected
 *@ATK_STATE_SELECTED: Indicates this object is the child of an object that allows its children to be selected and that this child is one of those children that has been selected
 *@ATK_STATE_SENSITIVE: Indicates this object is sensitive, e.g. to user interaction. 
 * STATE_SENSITIVE usually accompanies STATE_ENABLED for user-actionable controls,
 * but may be found in the absence of STATE_ENABLED if the current visible state of the 
 * control is "disconnected" from the application state.  In such cases, direct user interaction
 * can often result in the object gaining STATE_SENSITIVE, for instance if a user makes 
 * an explicit selection using an object whose current state is ambiguous or undefined.
 * @see STATE_ENABLED, STATE_INDETERMINATE.
 *@ATK_STATE_SHOWING: Indicates this object, the object's parent, the object's parent's parent, and so on, 
 * are all 'shown' to the end-user, i.e. subject to "exposure" if blocking or obscuring objects do not interpose
 * between this object and the top of the window stack.
 *@ATK_STATE_SINGLE_LINE: Indicates this (text) object can contain only a single line of text
 *@ATK_STATE_STALE: Indicates that the information returned for this object may no longer be
 * synchronized with the application state.  This is implied if the object has STATE_TRANSIENT,
 * and can also occur towards the end of the object peer's lifecycle. It can also be used to indicate that 
 * the index associated with this object has changed since the user accessed the object (in lieu of
 * "index-in-parent-changed" events).
 *@ATK_STATE_TRANSIENT: Indicates this object is transient, i.e. a snapshot which may not emit events when its
 * state changes.  Data from objects with ATK_STATE_TRANSIENT should not be cached, since there may be no
 * notification given when the cached data becomes obsolete.
 *@ATK_STATE_VERTICAL: Indicates the orientation of this object is vertical
 *@ATK_STATE_VISIBLE: Indicates this object is visible, e.g. has been explicitly marked for exposure to the user.
 * **note**: %ATK_STATE_VISIBLE is no guarantee that the object is actually unobscured on the screen, only
 * that it is 'potentially' visible, barring obstruction, being scrolled or clipped out of the 
 * field of view, or having an ancestor container that has not yet made visible.
 * A widget is potentially onscreen if it has both %ATK_STATE_VISIBLE and %ATK_STATE_SHOWING.
 * The absence of %ATK_STATE_VISIBLE and %ATK_STATE_SHOWING is semantically equivalent to saying
 * that an object is 'hidden'.  See also %ATK_STATE_TRUNCATED, which applies if an object with
 * %ATK_STATE_VISIBLE and %ATK_STATE_SHOWING set lies within a viewport which means that its
 * contents are clipped, e.g. a truncated spreadsheet cell or
 * an image within a scrolling viewport.  Mostly useful for screen-review and magnification
 * algorithms.
 *@ATK_STATE_MANAGES_DESCENDANTS: Indicates that "active-descendant-changed" event
 * is sent when children become 'active' (i.e. are selected or navigated to onscreen).
 * Used to prevent need to enumerate all children in very large containers, like tables.
 * The presence of STATE_MANAGES_DESCENDANTS is an indication to the client.
 * that the children should not, and need not, be enumerated by the client.
 * Objects implementing this state are expected to provide relevant state
 * notifications to listening clients, for instance notifications of visibility
 * changes and activation of their contained child objects, without the client 
 * having previously requested references to those children.
 *@ATK_STATE_INDETERMINATE: Indicates that the value, or some other quantifiable
 * property, of this AtkObject cannot be fully determined. In the case of a large
 * data set in which the total number of items in that set is unknown (e.g. 1 of
 * 999+), implementors should expose the currently-known set size (999) along
 * with this state. In the case of a check box, this state should be used to
 * indicate that the check box is a tri-state check box which is currently
 * neither checked nor unchecked.
 *@ATK_STATE_TRUNCATED: Indicates that an object is truncated, e.g. a text value in a speradsheet cell.
 *@ATK_STATE_REQUIRED: Indicates that explicit user interaction with an object is required by the user interface, e.g. a required field in a "web-form" interface.
 *@ATK_STATE_INVALID_ENTRY: Indicates that the object has encountered an error condition due to failure of input validation. For instance, a form control may acquire this state in response to invalid or malformed user input.
 *@ATK_STATE_SUPPORTS_AUTOCOMPLETION:  Indicates that the object in question implements some form of ¨typeahead¨ or 
 * pre-selection behavior whereby entering the first character of one or more sub-elements
 * causes those elements to scroll into view or become selected.  Subsequent character input
 * may narrow the selection further as long as one or more sub-elements match the string.
 * This state is normally only useful and encountered on objects that implement Selection.
 * In some cases the typeahead behavior may result in full or partial ¨completion¨ of 
 * the data in the input field, in which case these input events may trigger text-changed
 * events from the AtkText interface.  This state supplants @ATK_ROLE_AUTOCOMPLETE.
 *@ATK_STATE_SELECTABLE_TEXT:Indicates that the object in question supports text selection. It should only be exposed on objects which implement the Text interface, in order to distinguish this state from @ATK_STATE_SELECTABLE, which infers that the object in question is a selectable child of an object which implements Selection. While similar, text selection and subelement selection are distinct operations.
 *@ATK_STATE_DEFAULT: Indicates that the object is the "default" active component, i.e. the object which is activated by an end-user press of the "Enter" or "Return" key.  Typically a "close" or "submit" button.
 *@ATK_STATE_ANIMATED: Indicates that the object changes its appearance dynamically as an inherent part of its presentation.  This state may come and go if an object is only temporarily animated on the way to a 'final' onscreen presentation.
 * **note**: some applications, notably content viewers, may not be able to detect
 * all kinds of animated content.  Therefore the absence of this state should not
 * be taken as definitive evidence that the object's visual representation is
 * static; this state is advisory.
 *@ATK_STATE_VISITED: Indicates that the object (typically a hyperlink) has already been 'activated', and/or its backing data has already been downloaded, rendered, or otherwise "visited".
 *@ATK_STATE_CHECKABLE: Indicates this object has the potential to be
 *  checked, such as a checkbox or toggle-able table cell. @Since:
 *  ATK-2.12
 *@ATK_STATE_HAS_POPUP: Indicates that the object has a popup context
 * menu or sub-level menu which may or may not be showing. This means
 * that activation renders conditional content.  Note that ordinary
 * tooltips are not considered popups in this context. @Since: ATK-2.12
 *@ATK_STATE_HAS_TOOLTIP: Indicates this object has a tooltip. @Since: ATK-2.16
 *@ATK_STATE_READ_ONLY: Indicates that a widget which is ENABLED and SENSITIVE
 * has a value which can be read, but not modified, by the user. Note that this
 * state should only be applied to widget types whose value is normally directly
 * user modifiable, such as check boxes, radio buttons, spin buttons, text input
 * fields, and combo boxes, as a means to convey that the expected interaction
 * with that widget is not possible. When the expected interaction with a
 * widget does not include modification by the user, as is the case with
 * labels and containers, ATK_STATE_READ_ONLY should not be applied. See also
 * ATK_STATE_EDITABLE. @Since: ATK-2-16
 *@ATK_STATE_COLLAPSED: Indicates this object is collapsed. @Since: ATK-2.38
 *@ATK_STATE_LAST_DEFINED: Not a valid state, used for finding end of enumeration
 *
 *The possible types of states of an object
 **/
typedef enum
{
  ATK_STATE_INVALID,
  ATK_STATE_ACTIVE,
  ATK_STATE_ARMED,
  ATK_STATE_BUSY,
  ATK_STATE_CHECKED,
  ATK_STATE_DEFUNCT,
  ATK_STATE_EDITABLE,
  ATK_STATE_ENABLED,
  ATK_STATE_EXPANDABLE,
  ATK_STATE_EXPANDED,
  ATK_STATE_FOCUSABLE,
  ATK_STATE_FOCUSED,
  ATK_STATE_HORIZONTAL,
  ATK_STATE_ICONIFIED,
  ATK_STATE_MODAL,
  ATK_STATE_MULTI_LINE,
  ATK_STATE_MULTISELECTABLE,
  ATK_STATE_OPAQUE,
  ATK_STATE_PRESSED,
  ATK_STATE_RESIZABLE,
  ATK_STATE_SELECTABLE,
  ATK_STATE_SELECTED,
  ATK_STATE_SENSITIVE,
  ATK_STATE_SHOWING,
  ATK_STATE_SINGLE_LINE,
  ATK_STATE_STALE,
  ATK_STATE_TRANSIENT,
  ATK_STATE_VERTICAL,
  ATK_STATE_VISIBLE,
  ATK_STATE_MANAGES_DESCENDANTS,
  ATK_STATE_INDETERMINATE,
  ATK_STATE_TRUNCATED,
  ATK_STATE_REQUIRED,
  ATK_STATE_INVALID_ENTRY,
  ATK_STATE_SUPPORTS_AUTOCOMPLETION,
  ATK_STATE_SELECTABLE_TEXT,
  ATK_STATE_DEFAULT,
  ATK_STATE_ANIMATED,
  ATK_STATE_VISITED,
  ATK_STATE_CHECKABLE,
  ATK_STATE_HAS_POPUP,
  ATK_STATE_HAS_TOOLTIP,
  ATK_STATE_READ_ONLY,
  ATK_STATE_COLLAPSED,
  ATK_STATE_LAST_DEFINED
} AtkStateType;

typedef guint64      AtkState;

ATK_AVAILABLE_IN_ALL
AtkStateType atk_state_type_register            (const gchar *name);

ATK_AVAILABLE_IN_ALL
const gchar*          atk_state_type_get_name   (AtkStateType type);
ATK_AVAILABLE_IN_ALL
AtkStateType          atk_state_type_for_name   (const gchar  *name);

G_END_DECLS

#endif /* __ATK_STATE_H__ */

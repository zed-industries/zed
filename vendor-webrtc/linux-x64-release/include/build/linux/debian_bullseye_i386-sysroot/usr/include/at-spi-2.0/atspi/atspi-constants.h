/*
 * AT-SPI - Assistive Technology Service Provider Interface
 * (Gnome Accessibility Project; http://developer.gnome.org/projects/gap)
 *
 * Copyright 2010, 2011 Novell, Inc.
 * Copyright (c) 2012 SUSE LINUX Products GmbH, Nuernberg, Germany.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

/* TODO: Auto-generate this file again



 !\mainpage AT-SPI Interfaces and Subinterfaces

  This is the main documentation page for the
  Assistive Technology Service Provider Interface (AT-SPI).

  \section apps Applications and Interface Components
  Namespace Accessibility includes service APIs implemented by
  participating applications and their user interface components:\n\n
  Accessibility::Accessible\n
  Accessibility::Application\n
  Accessibility::Desktop\n
  Accessibility::Collecgtion\n
  Accessibility::Component\n
  Accessibility::Hypertext\n
  Accessibility::Image\n
  Accessibility::Selection\n
  Accessibility::Table\n
  Accessibility::Text\n
  Accessibility::EditableText\n
  Accessibility::Value

  \section types Enumerated Types
  Accessibility defines a number of key enumerated types, including:\n\n
  Accessibility::RelationType\n
  Accessibility::Role\n
  Accessibility::StateType\n
  Accessibility::Event\n
  Accessibility::EventDetails \n
  Accessibility::ScrollType \n
  Accessibility::CoordType \n

  \section Registry
  Accessibility also includes Accessibility::Registry,
  which is the service used by assistive technologies and related
  AT-SPI clients to register interest in certain classes of events,
  enumerate the currently available desktop and application list,
  and to synthesize certain kinds of device events.

  \section listeners Event Listener Interfaces
  Accessibility::EventListener\n
  Accessibility::DeviceEventListener

  \section helpers Helper Interfaces

  The following interfaces may be implemented by assistive technologies
  themselves, in order to export their services in a consistent manner or
  in order to interoperate with other applications or desktop services.\n

  Accessibility::LoginHelper : Implemented by adaptive technologies which
  need to participate in user-authentication or login activities, and which
  therefore may need negotiation with authentication agents or processes.\n

  Accessibility::Selector [NEW]: Implemented by user agents or assistive
  technologies which export lists of choices from which the end-user is
  expected to make selections.  Useful for various types of remote
  activation or intercommunication between multiple ATs.

 */

#ifndef _ATSPI_CONSTANTS_H_
#define _ATSPI_CONSTANTS_H_
#ifdef __cplusplus
extern "C" {
#endif

/**
 * AtspiLocaleType:
 * @ATSPI_LOCALE_TYPE_MESSAGES: For localizable natural-language messages.
 * @ATSPI_LOCALE_TYPE_COLLATE: For regular expression matching and string 
 * collation. 
 * @ATSPI_LOCALE_TYPE_CTYPE: For regular expression matching, character 
 * classification, conversion, case-sensitive comparison, and wide character 
 * functions. 
 * @ATSPI_LOCALE_TYPE_MONETARY: For monetary formatting.
 * @ATSPI_LOCALE_TYPE_NUMERIC: For number formatting (such as the decimal 
 * point and the thousands separator).
 * @ATSPI_LOCALE_TYPE_TIME: For time and date formatting.
 *
 * Used by interfaces #AtspiText and #AtspiDocument, this
 * enumeration corresponds to the POSIX 'setlocale' enum values.
 *
 **/
typedef enum {
    ATSPI_LOCALE_TYPE_MESSAGES,
    ATSPI_LOCALE_TYPE_COLLATE,
    ATSPI_LOCALE_TYPE_CTYPE,
    ATSPI_LOCALE_TYPE_MONETARY,
    ATSPI_LOCALE_TYPE_NUMERIC,
    ATSPI_LOCALE_TYPE_TIME,
} AtspiLocaleType;

/**
 * ATSPI_LOCALE_TYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiLocaleType.
 **/
#define ATSPI_LOCALE_TYPE_COUNT (5+1)

/**
 * AtspiCoordType:
 * @ATSPI_COORD_TYPE_SCREEN: Specifies xy coordinates relative to the screen.
 * @ATSPI_COORD_TYPE_WINDOW: Specifies xy coordinates relative to the widget's
 * top-level window.
 * @ATSPI_COORD_TYPE_PARENT: Specifies xy coordinates relative to the widget's
 * immediate parent.
 *
 * Enumeration used by #AtspiComponent, #AtspiImage, and #AtspiText interfaces
 * to specify whether coordinates are relative to the window or the screen.
 *
 **/
typedef enum {
    ATSPI_COORD_TYPE_SCREEN,
    ATSPI_COORD_TYPE_WINDOW,
    ATSPI_COORD_TYPE_PARENT,
} AtspiCoordType;

/**
 * ATSPI_COORD_TYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiCoordType.
 **/
#define ATSPI_COORD_TYPE_COUNT (2+1)

/**
 * AtspiCollectionSortOrder:
 * @ATSPI_Collection_SORT_ORDER_INVALID: Invalid sort order
 * @ATSPI_Collection_SORT_ORDER_CANONICAL: Canonical sort order
 * @ATSPI_Collection_SORT_ORDER_FLOW: Flow sort order
 * @ATSPI_Collection_SORT_ORDER_TAB: Tab sort order
 * @ATSPI_Collection_SORT_ORDER_REVERSE_CANONICAL: Reverse canonical sort order
 * @ATSPI_Collection_SORT_ORDER_REVERSE_FLOW: Reverse flow sort order
 * @ATSPI_Collection_SORT_ORDER_REVERSE_TAB: Reverse tab sort order
 * @ATSPI_Collection_SORT_ORDER_LAST_DEFINED: Used only to determine the end of the
 * enumeration.

 *
 * Enumeration used by interface #AtspiCollection to specify
 * the way #AtspiAccesible objects should be sorted.
 *
 **/
typedef enum {
    ATSPI_Collection_SORT_ORDER_INVALID,
    ATSPI_Collection_SORT_ORDER_CANONICAL,
    ATSPI_Collection_SORT_ORDER_FLOW,
    ATSPI_Collection_SORT_ORDER_TAB,
    ATSPI_Collection_SORT_ORDER_REVERSE_CANONICAL,
    ATSPI_Collection_SORT_ORDER_REVERSE_FLOW,
    ATSPI_Collection_SORT_ORDER_REVERSE_TAB,
    ATSPI_Collection_SORT_ORDER_LAST_DEFINED,
} AtspiCollectionSortOrder;

/**
 * ATSPI_SORTORDER_COUNT:
 *
 * One higher than the highest valid value of #AtspiCollectionSortOrder.
 */
#define ATSPI_SORTORDER_COUNT (7+1)

/**
 * AtspiCollectionMatchType:
 * @ATSPI_Collection_MATCH_INVALID: Indicates an error condition or
 * uninitialized value.
 * @ATSPI_Collection_MATCH_ALL: #TRUE if all of the criteria are met.
 * @ATSPI_Collection_MATCH_ANY: #TRUE if any of the criteria are met.
 * @ATSPI_Collection_MATCH_NONE: #TRUE if none of the criteria are met.
 * @ATSPI_Collection_MATCH_EMPTY: Same as @ATSPI_Collection_MATCH_ALL if
 * the criteria is non-empty; for empty criteria this rule requires returned
 * value to also have empty set.
 * @ATSPI_Collection_MATCH_LAST_DEFINED: Used only to determine the end of the
 * enumeration.
 *
 * Enumeration used by #AtspiMatchRule to specify
 * how to interpret #AtspiAccessible objects.
 *
 **/
typedef enum {
    ATSPI_Collection_MATCH_INVALID,
    ATSPI_Collection_MATCH_ALL,
    ATSPI_Collection_MATCH_ANY,
    ATSPI_Collection_MATCH_NONE,
    ATSPI_Collection_MATCH_EMPTY,
    ATSPI_Collection_MATCH_LAST_DEFINED,
} AtspiCollectionMatchType;

/**
 * ATSPI_MATCHTYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiCollectionMatchType.
 **/
#define ATSPI_MATCHTYPES_COUNT (5+1)

/**
 * AtspiCollectionTreeTraversalType:
 * @ATSPI_Collection_TREE_RESTRICT_CHILDREN: Restrict children tree traveral
 * @ATSPI_Collection_TREE_RESTRICT_SIBLING: Restrict sibling tree traversal
 * @ATSPI_Collection_TREE_INORDER: In-order tree traversal.
 * @ATSPI_Collection_TREE_LAST_DEFINED: Used only to determine the end of the
 * enumeration.
 *
 * Enumeration used by interface #AtspiCollection to specify
 * restrictions on #AtspiAccesible objects to be traversed.
 *
 **/
typedef enum {
    ATSPI_Collection_TREE_RESTRICT_CHILDREN,
    ATSPI_Collection_TREE_RESTRICT_SIBLING,
    ATSPI_Collection_TREE_INORDER,
    ATSPI_Collection_TREE_LAST_DEFINED,
} AtspiCollectionTreeTraversalType;

/**
 * ATSPI_TREETRAVERSALTYPE_COUNT:
 *
 * One higher than the highest valid value of
 * #AtspiCollection_TreeTraversalType.
 */
#define ATSPI_TREETRAVERSALTYPE_COUNT (3+1)

/**
 * AtspiComponentLayer:
 * @ATSPI_LAYER_INVALID: Indicates an error condition or uninitialized value.
 * @ATSPI_LAYER_BACKGROUND: The bottom-most layer, over which everything else
 * is painted.        The 'desktop background' is generally in this layer.
 * @ATSPI_LAYER_CANVAS: The 'background' layer for most content renderers and
 * UI #AtspiComponent containers.
 * @ATSPI_LAYER_WIDGET: The layer in which the majority of ordinary
 * 'foreground' widgets reside.
 * @ATSPI_LAYER_MDI: A special layer between @ATSPI_LAYER_CANVAS and 
 * @ATSPI_LAYER_WIDGET, in which the 'pseudo windows' (e.g. the MDI frames) 
 * reside. See #atspi_component_get_mdi_z_order. 
 * @ATSPI_LAYER_POPUP: A layer for popup window content, above
 * @ATSPI_LAYER_WIDGET.
 * @ATSPI_LAYER_OVERLAY: The topmost layer.
 * @ATSPI_LAYER_WINDOW: The layer in which a toplevel window background usually
 * resides.
 * @ATSPI_LAYER_LAST_DEFINED: Used only to determine the end of the
 * enumeration.
 *
 * The #AtspiComponentLayer of an #AtspiComponent instance indicates its 
 * relative stacking order with respect to the onscreen visual representation 
 * of the UI. #AtspiComponentLayer, in combination with #AtspiComponent bounds 
 * information, can be used to compute the visibility of all or part of a 
 * component.  This is important in programmatic determination of 
 * region-of-interest for magnification, and in 
 * flat screen review models of the screen, as well as
 * for other uses. Objects residing in two of the #AtspiComponentLayer 
 * categories support further z-ordering information, with respect to their 
 * peers in the same layer: namely, @ATSPI_LAYER_WINDOW and 
 * @ATSPI_LAYER_MDI.  Relative stacking order for other objects within the 
 * same layer is not available; the recommended heuristic is 
 * first child paints first. In other words, assume that the 
 * first siblings in the child list are subject to being overpainted by later 
 * siblings if their bounds intersect. The order of layers, from bottom to top,
 *  is: @ATSPI_LAYER_BACKGROUND, @ATSPI_LAYER_WINDOW, @ATSPI_LAYER_MDI,
 * @ATSPI_LAYER_CANVAS, @ATSPI_LAYER_WIDGET, @ATSPI_LAYER_POPUP, and 	   
 * @ATSPI_LAYER_OVERLAY.
 *
 */
typedef enum {
    ATSPI_LAYER_INVALID,
    ATSPI_LAYER_BACKGROUND,
    ATSPI_LAYER_CANVAS,
    ATSPI_LAYER_WIDGET,
    ATSPI_LAYER_MDI,
    ATSPI_LAYER_POPUP,
    ATSPI_LAYER_OVERLAY,
    ATSPI_LAYER_WINDOW,
    ATSPI_LAYER_LAST_DEFINED,
} AtspiComponentLayer;

/**
 * ATSPI_COMPONENTLAYER_COUNT:
 *
 * One higher than the highest valid value of #AtspiComponentLayer.
 **/
#define ATSPI_COMPONENTLAYER_COUNT (8+1)

/**
 * AtspiTextBoundaryType:
 * @ATSPI_TEXT_BOUNDARY_CHAR: An #AtspiText instance is bounded by this 
 * character only. Start and end offsets differ by one, by definition, 
 * for this value.
 * @ATSPI_TEXT_BOUNDARY_WORD_START: Boundary condition is start of a word; i.e.
 * range is from start of one word to the start of another word.
 * @ATSPI_TEXT_BOUNDARY_WORD_END: Boundary condition is the end of a word; i.e.
 * range is from the end of one word to the end of another. Some locales
 * may not distinguish between words and characters or glyphs. In particular,
 * those locales which use wholly or partially ideographic character sets. 
 * In these cases, characters may be returned in lieu of multi-character
 * substrings.
 * @ATSPI_TEXT_BOUNDARY_SENTENCE_START: Boundary condition is start of a
 * sentence, as determined by the application. Some locales or
 * character sets may not include explicit sentence delimiters, so this
 * boundary type can not always be honored. Some locales will return lines
 * of text instead of grammatical sentences.
 * @ATSPI_TEXT_BOUNDARY_SENTENCE_END: Boundary condition is end of a sentence,
 * as determined by the application, including the sentence-delimiting
 * character, for instance '.' Some locales or character sets may not
 * include explicit sentence delimiters, so this boundary type can not
 * always be honored. Some locales will return lines of text instead of
 * grammatical sentences.
 * @ATSPI_TEXT_BOUNDARY_LINE_START: Boundary condition is the start of a line;
 * i.e. range is from start of one line to the start of another.  This
 * generally means that an end-of-line character will appear at the end of
 * the range.
 * @ATSPI_TEXT_BOUNDARY_LINE_END: Boundary condition is the end of a line; i.e.
 * range is from start of one line to the start of another.  This generally 
 * means that an end-of-line character will be the first character of the
 * range.
 *
 * Specifies the boundary conditions determining a run of text as returned from
 * #atspi_text_get_text_at_offset, #atspi_text_get_text_after_offset, and
 * #atspi_text_get_text_before_offset.
 *
 * This enumerationis deprecated since 2.9.90 and should not be used. Use
 * AtspiTextGranularity with #atspi_text_get_string_at_offset instead.
 **/
typedef enum {
    ATSPI_TEXT_BOUNDARY_CHAR,
    ATSPI_TEXT_BOUNDARY_WORD_START,
    ATSPI_TEXT_BOUNDARY_WORD_END,
    ATSPI_TEXT_BOUNDARY_SENTENCE_START,
    ATSPI_TEXT_BOUNDARY_SENTENCE_END,
    ATSPI_TEXT_BOUNDARY_LINE_START,
    ATSPI_TEXT_BOUNDARY_LINE_END,
} AtspiTextBoundaryType;

/**
 *AtspiTextGranularity:
 *@ATSPI_TEXT_GRANULARITY_CHAR: Granularity is defined by the boundaries between characters
 * (including non-printing characters)
 *@ATSPI_TEXT_GRANULARITY_WORD: Granularity is defined by the boundaries of a word,
 * starting at the beginning of the current word and finishing at the beginning of
 * the following one, if present.
 *@ATSPI_TEXT_GRANULARITY_SENTENCE: Granularity is defined by the boundaries of a sentence,
 * starting at the beginning of the current sentence and finishing at the beginning of
 * the following one, if present.
 *@ATSPI_TEXT_GRANULARITY_LINE: Granularity is defined by the boundaries of a line,
 * starting at the beginning of the current line and finishing at the beginning of
 * the following one, if present.
 *@ATSPI_TEXT_GRANULARITY_PARAGRAPH: Granularity is defined by the boundaries of a paragraph,
 * starting at the beginning of the current paragraph and finishing at the beginning of
 * the following one, if present.
 *
 * Text granularity types used for specifying the granularity of the region of
 * text we are interested in.
 **/
typedef enum {
  ATSPI_TEXT_GRANULARITY_CHAR,
  ATSPI_TEXT_GRANULARITY_WORD,
  ATSPI_TEXT_GRANULARITY_SENTENCE,
  ATSPI_TEXT_GRANULARITY_LINE,
  ATSPI_TEXT_GRANULARITY_PARAGRAPH
} AtspiTextGranularity;

/**
 * ATSPI_TEXT_BOUNDARY_TYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiTextBoundaryType.
 */
#define ATSPI_TEXT_BOUNDARY_TYPE_COUNT (6+1)

/**
 * AtspiTextClipType:
 * @ATSPI_TEXT_CLIP_NONE: No characters/glyphs are omitted.
 * @ATSPI_TEXT_CLIP_MIN: Characters/glyphs clipped by the minimum coordinate
 * are omitted.
 * @ATSPI_TEXT_CLIP_MAX: Characters/glyphs which intersect the maximum
 * coordinate are omitted.
 * @ATSPI_TEXT_CLIP_BOTH: Only glyphs falling entirely within the region
 * bounded by min and max are retained.
 *
 * Enumeration used by interface #AtspiText to indicate
 * how to treat characters intersecting bounding boxes.
 *
 **/
typedef enum {
    ATSPI_TEXT_CLIP_NONE,
    ATSPI_TEXT_CLIP_MIN,
    ATSPI_TEXT_CLIP_MAX,
    ATSPI_TEXT_CLIP_BOTH,
} AtspiTextClipType;

/**
 * ATSPI_TEXT_CLIP_TYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiTextClipType.
 */
#define ATSPI_TEXT_CLIP_TYPE_COUNT (3+1)

/**
 * AtspiStateType:
 * @ATSPI_STATE_INVALID: Indicates an invalid state - probably an error 
 * condition.
 * @ATSPI_STATE_ACTIVE: Indicates a window is currently the active window, or
 * an object is the active subelement within a container or table.
 * @ATSPI_STATE_ACTIVE should not be used for objects which have
 * #ATSPI_STATE_FOCUSABLE or #ATSPI_STATE_SELECTABLE: Those objects should use
 * @ATSPI_STATE_FOCUSED and @ATSPI_STATE_SELECTED respectively.
 * @ATSPI_STATE_ACTIVE is a means to indicate that an object which is not
 * focusable and not selectable is the currently-active item within its
 * parent container.
 * @ATSPI_STATE_ARMED: Indicates that the object is armed.
 * @ATSPI_STATE_BUSY: Indicates the current object is busy, i.e. onscreen
 * representation is in the process of changing, or       the object is
 * temporarily unavailable for interaction due to activity already in progress.
 * @ATSPI_STATE_CHECKED: Indicates this object is currently checked.
 * @ATSPI_STATE_COLLAPSED: Indicates this object is collapsed.
 * @ATSPI_STATE_DEFUNCT: Indicates that this object no longer has a valid
 * backing widget        (for instance, if its peer object has been destroyed).
 * @ATSPI_STATE_EDITABLE: Indicates the user can change the contents of this
 * object.
 * @ATSPI_STATE_ENABLED: Indicates that this object is enabled, i.e. that it
 * currently reflects some application state. Objects that are "greyed out"
 * may lack this state, and may lack the @ATSPI_STATE_SENSITIVE if direct
 * user interaction cannot cause them to acquire @ATSPI_STATE_ENABLED. 
 * See @ATSPI_STATE_SENSITIVE.
 * @ATSPI_STATE_EXPANDABLE: Indicates this object allows progressive
 * disclosure of its children.
 * @ATSPI_STATE_EXPANDED: Indicates this object is expanded.
 * @ATSPI_STATE_FOCUSABLE: Indicates this object can accept keyboard focus,
 * which means all events resulting from typing on the keyboard will
 * normally be passed to it when it has focus.
 * @ATSPI_STATE_FOCUSED: Indicates this object currently has the keyboard
 * focus.
 * @ATSPI_STATE_HAS_TOOLTIP: Indicates that the object has an associated
 * tooltip.
 * @ATSPI_STATE_HORIZONTAL: Indicates the orientation of this object is
 * horizontal.
 * @ATSPI_STATE_ICONIFIED: Indicates this object is minimized and is
 * represented only by an icon.
 * @ATSPI_STATE_MODAL: Indicates something must be done with this object
 * before the user can interact with an object in a different window.
 * @ATSPI_STATE_MULTI_LINE: Indicates this (text) object can contain multiple
 * lines of text.
 * @ATSPI_STATE_MULTISELECTABLE: Indicates this object allows more than one of
 * its children to be selected at the same time, or in the case of text
 * objects, that the object supports non-contiguous text selections.
 * @ATSPI_STATE_OPAQUE: Indicates this object paints every pixel within its
 * rectangular region. It also indicates an alpha value of unity, if it
 * supports alpha blending.
 * @ATSPI_STATE_PRESSED: Indicates this object is currently pressed.
 * @ATSPI_STATE_RESIZABLE: Indicates the size of this object's size is not
 * fixed.
 * @ATSPI_STATE_SELECTABLE: Indicates this object is the child of an object
 * that allows its children to be selected and that this child is one of
 * those children       that can be selected.
 * @ATSPI_STATE_SELECTED: Indicates this object is the child of an object that
 * allows its children to be selected and that this child is one of those
 * children that has been selected.
 * @ATSPI_STATE_SENSITIVE: Indicates this object is sensitive, e.g. to user
 * interaction. @ATSPI_STATE_SENSITIVE usually accompanies.
 * @ATSPI_STATE_ENABLED for user-actionable controls, but may be found in the 
 * absence of @ATSPI_STATE_ENABLED if the current visible state of the control 
 * is "disconnected" from the application state.  In such cases, direct user
 * interaction can often result in the object gaining @ATSPI_STATE_SENSITIVE, 
 * for instance if a user makes an explicit selection using an object whose
 * current state is ambiguous or undefined. See @ATSPI_STATE_ENABLED,
 * @ATSPI_STATE_INDETERMINATE.
 * @ATSPI_STATE_SHOWING: Indicates this object, the object's parent, the
 * object's parent's parent, and so on, are all 'shown' to the end-user,
 * i.e. subject to "exposure" if blocking or obscuring objects do not
 * interpose between this object and the top of the window stack.
 * @ATSPI_STATE_SINGLE_LINE: Indicates this (text) object can contain only a
 * single line of text.
 * @ATSPI_STATE_STALE: Indicates that the information returned for this object
 * may no longer be synchronized with the application state.  This can occur
 * if the object has @ATSPI_STATE_TRANSIENT, and can also occur towards the 
 * end of the object peer's lifecycle.
 * @ATSPI_STATE_TRANSIENT: Indicates this object is transient.
 * @ATSPI_STATE_VERTICAL: Indicates the orientation of this object is vertical;
 * for example this state may appear on such objects as scrollbars, text
 * objects (with vertical text flow), separators, etc.
 * @ATSPI_STATE_VISIBLE: Indicates this object is visible, e.g. has been
 * explicitly marked for exposure to the user. @ATSPI_STATE_VISIBLE is no 
 * guarantee that the object is actually unobscured on the screen, only that 
 * it is 'potentially' visible, barring obstruction, being scrolled or clipped 
 * out of the field of view, or having an ancestor container that has not yet 
 * made visible. A widget is potentially onscreen if it has both 
 * @ATSPI_STATE_VISIBLE and @ATSPI_STATE_SHOWING. The absence of 
 * @ATSPI_STATE_VISIBLE and @ATSPI_STATE_SHOWING is
 * semantically equivalent to saying that an object is 'hidden'.
 * @ATSPI_STATE_MANAGES_DESCENDANTS: Indicates that "active-descendant-changed"
 * event is sent when children become 'active' (i.e. are selected or
 * navigated to onscreen).  Used to prevent need to enumerate all children
 * in very large containers, like tables. The presence of
 * @ATSPI_STATE_MANAGES_DESCENDANTS is an indication to the client that the
 * children should not, and need not, be enumerated by the client.
 * Objects implementing this state are expected to provide relevant state      
 * notifications to listening clients, for instance notifications of 
 * visibility changes and activation of their contained child objects, without 
 * the client having previously requested references to those children.
 * @ATSPI_STATE_INDETERMINATE: Indicates that a check box or other boolean
 * indicator is in a state other than checked or not checked.  This
 * usually means that the boolean value reflected or controlled by the
 * object does not apply consistently to the entire current context.      
 * For example, a checkbox for the "Bold" attribute of text may have
 * @ATSPI_STATE_INDETERMINATE if the currently selected text contains a mixture
 * of weight attributes. In many cases interacting with a
 * @ATSPI_STATE_INDETERMINATE object will cause the context's corresponding
 * boolean attribute to be homogenized, whereupon the object will lose
 * @ATSPI_STATE_INDETERMINATE and a corresponding state-changed event will be
 * fired.
 * @ATSPI_STATE_REQUIRED: Indicates that user interaction with this object is
 * 'required' from the user, for instance before completing the
 * processing of a form.
 * @ATSPI_STATE_TRUNCATED: 	  Indicates that an object's onscreen content
 * is truncated, e.g. a text value in a spreadsheet cell.
 * @ATSPI_STATE_ANIMATED: Indicates this object's visual representation is
 * dynamic, not static. This state may be applied to an object during an
 * animated 'effect' and be removed from the object once its visual
 * representation becomes static. Some applications, notably content viewers,
 * may not be able to detect all kinds of animated content.  Therefore the
 * absence of this state should not be taken as
 * definitive evidence that the object's visual representation is      
 * static; this state is advisory.
 * @ATSPI_STATE_INVALID_ENTRY: This object has indicated an error condition
 * due to failure of input validation.  For instance, a form control may
 * acquire this state in response to invalid or malformed user input.
 * @ATSPI_STATE_SUPPORTS_AUTOCOMPLETION: This state indicates that the object
 * in question implements some form of typeahead or       
 * pre-selection behavior whereby entering the first character of one or more
 * sub-elements causes those elements to scroll into view or become
 * selected. Subsequent character input may narrow the selection further as
 * long as one or more sub-elements match the string. This state is normally
 * only useful and encountered on objects that implement #AtspiSelection.
 * In some cases the typeahead behavior may result in full or partial
 * completion of the data in the input field, in which case
 * these input events may trigger text-changed events from the source.
 * @ATSPI_STATE_SELECTABLE_TEXT: This state indicates that the object in
 * question supports text selection. It should only be exposed on objects
 * which implement the #AtspiText interface, in order to distinguish this state
 * from @ATSPI_STATE_SELECTABLE, which infers that the object in question is a
 * selectable child of an object which implements #AtspiSelection. While 
 * similar, text selection and subelement selection are distinct operations.
 * @ATSPI_STATE_IS_DEFAULT: This state indicates that the object in question is
 * the 'default' interaction object in a dialog, i.e. the one that gets
 * activated if the user presses "Enter" when the dialog is initially
 * posted.
 * @ATSPI_STATE_VISITED: This state indicates that the object (typically a
 * hyperlink) has already been activated or invoked, with the result that
 * some backing data has been downloaded or rendered.
 *@ATSPI_STATE_CHECKABLE: Indicates this object has the potential to
 *  be checked, such as a checkbox or toggle-able table cell. @Since:
 *  2.12
 *@ATSPI_STATE_HAS_POPUP: Indicates that the object has a popup
 * context menu or sub-level menu which may or may not be
 * showing. This means that activation renders conditional content.
 * Note that ordinary tooltips are not considered popups in this
 * context. @Since: 2.12
 * @ATSPI_STATE_READ_ONLY: Indicates that an object which is ENABLED and
 * SENSITIVE has a value which can be read, but not modified, by the
 * user. @Since: 2.16
 * @ATSPI_STATE_LAST_DEFINED: This value of the enumeration should not be used
 * as a parameter, it indicates the number of items in the #AtspiStateType
 * enumeration.
 *
 * 
 * Enumeration used by various interfaces indicating every possible state 
 * an #AtspiAccesible object can assume.
 *
 **/
typedef enum {
    ATSPI_STATE_INVALID,
    ATSPI_STATE_ACTIVE,
    ATSPI_STATE_ARMED,
    ATSPI_STATE_BUSY,
    ATSPI_STATE_CHECKED,
    ATSPI_STATE_COLLAPSED,
    ATSPI_STATE_DEFUNCT,
    ATSPI_STATE_EDITABLE,
    ATSPI_STATE_ENABLED,
    ATSPI_STATE_EXPANDABLE,
    ATSPI_STATE_EXPANDED,
    ATSPI_STATE_FOCUSABLE,
    ATSPI_STATE_FOCUSED,
    ATSPI_STATE_HAS_TOOLTIP,
    ATSPI_STATE_HORIZONTAL,
    ATSPI_STATE_ICONIFIED,
    ATSPI_STATE_MODAL,
    ATSPI_STATE_MULTI_LINE,
    ATSPI_STATE_MULTISELECTABLE,
    ATSPI_STATE_OPAQUE,
    ATSPI_STATE_PRESSED,
    ATSPI_STATE_RESIZABLE,
    ATSPI_STATE_SELECTABLE,
    ATSPI_STATE_SELECTED,
    ATSPI_STATE_SENSITIVE,
    ATSPI_STATE_SHOWING,
    ATSPI_STATE_SINGLE_LINE,
    ATSPI_STATE_STALE,
    ATSPI_STATE_TRANSIENT,
    ATSPI_STATE_VERTICAL,
    ATSPI_STATE_VISIBLE,
    ATSPI_STATE_MANAGES_DESCENDANTS,
    ATSPI_STATE_INDETERMINATE,
    ATSPI_STATE_REQUIRED,
    ATSPI_STATE_TRUNCATED,
    ATSPI_STATE_ANIMATED,
    ATSPI_STATE_INVALID_ENTRY,
    ATSPI_STATE_SUPPORTS_AUTOCOMPLETION,
    ATSPI_STATE_SELECTABLE_TEXT,
    ATSPI_STATE_IS_DEFAULT,
    ATSPI_STATE_VISITED,
    ATSPI_STATE_CHECKABLE,
    ATSPI_STATE_HAS_POPUP,
    ATSPI_STATE_READ_ONLY,
    ATSPI_STATE_LAST_DEFINED,
} AtspiStateType;

/**
 * ATSPI_STATETYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiStateType.
 **/
#define ATSPI_STATETYPE_COUNT (41+1)

/**
 * AtspiKeyEventType:
 * @ATSPI_KEY_PRESSED: Key press event
 * @ATSPI_KEY_RELEASED: Key release event
 *
 * Deprecated. Should not be used.
 *
 **/
typedef enum {
    ATSPI_KEY_PRESSED,
    ATSPI_KEY_RELEASED,
} AtspiKeyEventType;

/**
 * ATSPI_KEYEVENTTYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiKeyEventType.
 **/
#define ATSPI_KEYEVENTTYPE_COUNT (1+1)

/**
 * AtspiEventType:
 * @ATSPI_KEY_PRESSED_EVENT: Indicates that a key on a keyboard device was 
 * pressed.
 * @ATSPI_KEY_RELEASED_EVENT: Indicates that a key on a keyboard device was 
 * released.
 * @ATSPI_BUTTON_PRESSED_EVENT: Indicates that a button on a non-keyboard 
 * human interface device (HID) was pressed.
 * @ATSPI_BUTTON_RELEASED_EVENT: Indicates that a button on a non-keyboard
 * human interface device (HID) was released.
 *
 * Enumeration used to specify the event types of interest to an 
 * #AtspiEventListener, or 
 * to identify the type of an event for which notification has been sent.  
 *
 **/
typedef enum {
    ATSPI_KEY_PRESSED_EVENT,
    ATSPI_KEY_RELEASED_EVENT,
    ATSPI_BUTTON_PRESSED_EVENT,
    ATSPI_BUTTON_RELEASED_EVENT,
} AtspiEventType;

/**
 * ATSPI_EVENTTYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiEventType.
 */
#define ATSPI_EVENTTYPE_COUNT (3+1)

/**
 * AtspiKeySynthType:
 * @ATSPI_KEY_PRESS: Emulates the pressing of a hardware keyboard key.
 * @ATSPI_KEY_RELEASE: Emulates the release of a hardware keyboard key.
 * @ATSPI_KEY_PRESSRELEASE: Emulates the pressing and immediate releasing
 * of a hardware keyboard key.
 * @ATSPI_KEY_SYM: A symbolic key event is generated, without specifying a
 * hardware key. Note: if the keysym is not present in the current keyboard
 * map, the #AtspiDeviceEventController instance has a limited ability to 
 * generate such keysyms on-the-fly. Reliability of GenerateKeyboardEvent 
 * calls using out-of-keymap keysyms will vary from system to system, and on 
 * the number of different out-of-keymap keysyms being generated in quick
 * succession. 
 * In practice this is rarely significant, since the keysyms of interest to 
 * AT clients and keyboard emulators are usually part of the current keymap, 
 * i.e., present on the system keyboard for the current locale (even if a 
 * physical hardware keyboard is not connected).
 * @ATSPI_KEY_STRING: A string is converted to its equivalent keyboard events
 * and emitted. If the string consists of complex characters or composed
 * characters which are not in the current keymap, string emission is
 * subject to the out-of-keymap limitations described for
 * @ATSPI_KEY_SYM. In practice this limitation primarily effects
 * Chinese and Japanese locales.
 * @ATSPI_KEY_LOCKMODIFIERS: Emulates locking a set of modifiers.
 * @ATSPI_KEY_UNLOCKMODIFIERS: Emulates unlocking a set of modifiers.
 *
 * Enumeration used when synthesizing keyboard input via
 * #atspi_generate_keyboard_event.
 *
 **/
typedef enum {
    ATSPI_KEY_PRESS,
    ATSPI_KEY_RELEASE,
    ATSPI_KEY_PRESSRELEASE,
    ATSPI_KEY_SYM,
    ATSPI_KEY_STRING,
    ATSPI_KEY_LOCKMODIFIERS,
    ATSPI_KEY_UNLOCKMODIFIERS,
} AtspiKeySynthType;

/**
 * ATSPI_KEYSYNTHTYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiKeySynthType.
 **/
#define ATSPI_KEYSYNTHTYPE_COUNT (4+1)

/**
 * AtspiModifierType:
 * @ATSPI_MODIFIER_SHIFT: The left or right 'Shift' key.
 * @ATSPI_MODIFIER_SHIFTLOCK: The ShiftLock or CapsLock key.
 * @ATSPI_MODIFIER_CONTROL: 'Control'/'Ctrl'.
 * @ATSPI_MODIFIER_ALT: The Alt key (as opposed to AltGr).
 * @ATSPI_MODIFIER_META: Depending on the platform, this may map to 'Window',
 * 'Function', 'Meta', 'Menu', or 'NumLock'. Such 'Meta keys' will
 * map to one of META, META2, META3. On X Windows platforms these META
 * values map to the modifier masks Mod1Mask, Mod2Mask, Mod3Mask, e.g. an
 * event having @ATSPI_MODIFIER_META2 means that the 'Mod2Mask' bit
 * is set in the corresponding XEvent.
 * @ATSPI_MODIFIER_META2: See @ATSPI_MODIFIER_META.
 * @ATSPI_MODIFIER_META3: See @ATSPI_MODIFIER_META.
 * @ATSPI_MODIFIER_NUMLOCK: A symbolic meta key name that is mapped by AT-SPI
 * to the appropriate META value, for the convenience of the client.
 *
 *
 *
 **/
typedef enum {
    ATSPI_MODIFIER_SHIFT,
    ATSPI_MODIFIER_SHIFTLOCK,
    ATSPI_MODIFIER_CONTROL,
    ATSPI_MODIFIER_ALT,
    ATSPI_MODIFIER_META,
    ATSPI_MODIFIER_META2,
    ATSPI_MODIFIER_META3,
    ATSPI_MODIFIER_NUMLOCK = 14,
} AtspiModifierType;

/**
 * ATSPI_MODIFIERTYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiModifierType.
 **/
#define ATSPI_MODIFIERTYPE_COUNT (7+1)

/**
 * AtspiRelationType:
 * @ATSPI_RELATION_NULL: Not a meaningful relationship; clients should not
 * normally encounter this #AtspiRelationType value.
 * @ATSPI_RELATION_LABEL_FOR: Object is a label for one or more other objects.
 * @ATSPI_RELATION_LABELLED_BY: Object is labelled by one or more other
 * objects.
 * @ATSPI_RELATION_CONTROLLER_FOR: Object is an interactive object which
 * modifies the state, onscreen location, or other attributes of one or more
 * target objects.
 * @ATSPI_RELATION_CONTROLLED_BY: Object state, position, etc. is
 * modified/controlled by user interaction with one or more other objects. 
 * For instance a viewport or scroll pane may be @ATSPI_RELATION_CONTROLLED_BY
 * scrollbars.
 * @ATSPI_RELATION_MEMBER_OF: Object has a grouping relationship (e.g. 'same
 * group as') to one or more other objects.
 * @ATSPI_RELATION_TOOLTIP_FOR: Object is a tooltip associated with another
 * object.
 * @ATSPI_RELATION_NODE_CHILD_OF: Object is a child of the target.
 * @ATSPI_RELATION_NODE_PARENT_OF: Object is a parent of the target.
 * @ATSPI_RELATION_EXTENDED: Used to indicate that a relationship exists, but
 * its type is not specified in the enumeration.
 * @ATSPI_RELATION_FLOWS_TO: Object renders content which flows logically to
 * another object. For instance, text in a paragraph may flow to another
 * object which is not the 'next sibling' in the accessibility hierarchy.
 * @ATSPI_RELATION_FLOWS_FROM: Reciprocal of @ATSPI_RELATION_FLOWS_TO.
 * @ATSPI_RELATION_SUBWINDOW_OF: Object is visually and semantically considered
 * a subwindow of another object, even though it is not the object's child. 
 * Useful when dealing with embedded applications and other cases where the
 * widget hierarchy does not map cleanly to the onscreen presentation.
 * @ATSPI_RELATION_EMBEDS: Similar to @ATSPI_RELATION_SUBWINDOW_OF, but 
 * specifically used for cross-process embedding.
 * @ATSPI_RELATION_EMBEDDED_BY: Reciprocal of @ATSPI_RELATION_EMBEDS. Used to
 * denote content rendered by embedded renderers that live in a separate process
 * space from the embedding context.
 * @ATSPI_RELATION_POPUP_FOR: Denotes that the object is a transient window or
 * frame associated with another onscreen object. Similar to @ATSPI_TOOLTIP_FOR,
 * but more general. Useful for windows which are technically toplevels
 * but which, for one or more reasons, do not explicitly cause their 
 * associated window to lose 'window focus'. Creation of an @ATSPI_ROLE_WINDOW
 * object with the @ATSPI_RELATION_POPUP_FOR relation usually requires 
 * some presentation action on the part
 * of assistive technology clients, even though the previous toplevel
 * @ATSPI_ROLE_FRAME object may still be the active window.
 * @ATSPI_RELATION_PARENT_WINDOW_OF: This is the reciprocal relation to
 * @ATSPI_RELATION_POPUP_FOR.
 * @ATSPI_RELATION_DESCRIBED_BY: Reciprocal of %ATSPI_RELATION_DESCRIPTION_FOR.
 * Indicates that one or more target objects provide descriptive information
 * about this object. This relation type is most appropriate for information
 * that is not essential as its presentation may be user-configurable and/or
 * limited to an on-demand mechanism such as an assistive technology command.
 * For brief, essential information such as can be found in a widget's on-screen
 * label, use %ATSPI_RELATION_LABELLED_BY. For an on-screen error message, use
 * %ATSPI_RELATION_ERROR_MESSAGE. For lengthy extended descriptive information
 * contained in an on-screen object, consider using %ATSPI_RELATION_DETAILS as
 * assistive technologies may provide a means for the user to navigate to
 * objects containing detailed descriptions so that their content can be more
 * closely reviewed.
 * @ATSPI_RELATION_DESCRIPTION_FOR: Reciprocal of %ATSPI_RELATION_DESCRIBED_BY.
 * Indicates that this object provides descriptive information about the target
 * object(s). See also %ATSPI_RELATION_DETAILS_FOR and %ATSPI_RELATION_ERROR_FOR.
 * @ATSPI_RELATION_DETAILS: Reciprocal of %ATSPI_RELATION_DETAILS_FOR. Indicates
 * that this object has a detailed or extended description, the contents of
 * which can be found in the target object(s). This relation type is most
 * appropriate for information that is sufficiently lengthy as to make
 * navigation to the container of that information desirable. For less verbose
 * information suitable for announcement only, see %ATSPI_RELATION_DESCRIBED_BY.
 * If the detailed information describes an error condition,
 * %ATSPI_RELATION_ERROR_FOR should be used instead. @Since: 2.26.
 * @ATSPI_RELATION_DETAILS_FOR: Reciprocal of %ATSPI_RELATION_DETAILS. Indicates
 * that this object provides a detailed or extended description about the target
 * object(s). See also %ATSPI_RELATION_DESCRIPTION_FOR and
 * %ATSPI_RELATION_ERROR_FOR. @Since: 2.26.
 * @ATSPI_RELATION_ERROR_MESSAGE: Reciprocal of %ATSPI_RELATION_ERROR_FOR.
 * Indicates that this object has one or more errors, the nature of which is
 * described in the contents of the target object(s). Objects that have this
 * relation type should also contain %ATSPI_STATE_INVALID_ENTRY in their
 * #AtspiStateSet. @Since: 2.26.
 * @ATSPI_RELATION_ERROR_FOR: Reciprocal of %ATSPI_RELATION_ERROR_MESSAGE.
 * Indicates that this object contains an error message describing an invalid
 * condition in the target object(s). @Since: 2.26.
 * @ATSPI_RELATION_LAST_DEFINED: Do not use as a parameter value, used to
 * determine the size of the enumeration. 
 *
 * #AtspiRelationType specifies a relationship between objects 
 * (possibly one-to-many
 * or many-to-one) outside of the normal parent/child hierarchical
 * relationship. It allows better semantic       identification of how objects
 * are associated with one another.       For instance the 
 * @ATSPI_RELATION_LABELLED_BY
 * relationship may be used to identify labelling information       that should
 * accompany the accessible name property when presenting an object's content or
 * identity       to the end user.  Similarly, 
 * @ATSPI_RELATION_CONTROLLER_FOR can be used
 * to further specify the context in which a valuator is useful, and/or the
 * other UI components which are directly effected by user interactions with
 * the valuator. Common examples include association of scrollbars with the
 * viewport or panel which they control.
 *
 *
 * Enumeration used to specify
 * the type of relation encapsulated in an #AtspiRelation object.
 *
 **/
typedef enum {
    ATSPI_RELATION_NULL,
    ATSPI_RELATION_LABEL_FOR,
    ATSPI_RELATION_LABELLED_BY,
    ATSPI_RELATION_CONTROLLER_FOR,
    ATSPI_RELATION_CONTROLLED_BY,
    ATSPI_RELATION_MEMBER_OF,
    ATSPI_RELATION_TOOLTIP_FOR,
    ATSPI_RELATION_NODE_CHILD_OF,
    ATSPI_RELATION_NODE_PARENT_OF,
    ATSPI_RELATION_EXTENDED,
    ATSPI_RELATION_FLOWS_TO,
    ATSPI_RELATION_FLOWS_FROM,
    ATSPI_RELATION_SUBWINDOW_OF,
    ATSPI_RELATION_EMBEDS,
    ATSPI_RELATION_EMBEDDED_BY,
    ATSPI_RELATION_POPUP_FOR,
    ATSPI_RELATION_PARENT_WINDOW_OF,
    ATSPI_RELATION_DESCRIPTION_FOR,
    ATSPI_RELATION_DESCRIBED_BY,
    ATSPI_RELATION_DETAILS,
    ATSPI_RELATION_DETAILS_FOR,
    ATSPI_RELATION_ERROR_MESSAGE,
    ATSPI_RELATION_ERROR_FOR,
    ATSPI_RELATION_LAST_DEFINED,
} AtspiRelationType;

/**
 * ATSPI_RELATIONTYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiRelationType.
 **/
#define ATSPI_RELATIONTYPE_COUNT (23+1)

/**
 * AtspiRole:
 * @ATSPI_ROLE_INVALID: A role indicating an error condition, such as
 * uninitialized Role data.
 * @ATSPI_ROLE_ACCELERATOR_LABEL: Object is a label indicating the keyboard
 * accelerators for the parent.
 * @ATSPI_ROLE_ALERT: Object is used to alert the user about something.
 * @ATSPI_ROLE_ANIMATION: Object contains a dynamic or moving image of some
 * kind.
 * @ATSPI_ROLE_ARROW: Object is a 2d directional indicator.
 * @ATSPI_ROLE_CALENDAR: Object contains one or more dates, usually arranged
 * into a 2d list.
 * @ATSPI_ROLE_CANVAS: Object that can be drawn into and is used to trap
 * events.
 * @ATSPI_ROLE_CHECK_BOX: A choice that can be checked or unchecked and
 * provides a separate       indicator for the current state.
 * @ATSPI_ROLE_CHECK_MENU_ITEM: A menu item that behaves like a check box. See
 * @ATSPI_ROLE_CHECK_BOX.
 * @ATSPI_ROLE_COLOR_CHOOSER: A specialized dialog that lets the user choose a
 * color.
 * @ATSPI_ROLE_COLUMN_HEADER: The header for a column of data.
 * @ATSPI_ROLE_COMBO_BOX: A list of choices the user can select from.
 * @ATSPI_ROLE_DATE_EDITOR: An object which allows entry of a date.
 * @ATSPI_ROLE_DESKTOP_ICON: An inconifed internal frame within a DESKTOP_PANE.
 * @ATSPI_ROLE_DESKTOP_FRAME: A pane that supports internal frames and
 * iconified versions of those internal frames.
 * @ATSPI_ROLE_DIAL: An object that allows a value to be changed via rotating a
 * visual element, or which displays a value via such a rotating element.
 * @ATSPI_ROLE_DIALOG: A top level window with title bar and a border.
 * @ATSPI_ROLE_DIRECTORY_PANE: A pane that allows the user to navigate through
 * and select the contents of a directory.
 * @ATSPI_ROLE_DRAWING_AREA: A specialized dialog that displays the files in
 * the directory and lets the user select a file, browse a different
 * directory, or specify a filename.
 * @ATSPI_ROLE_FILE_CHOOSER: An object used for drawing custom user interface
 * elements.
 * @ATSPI_ROLE_FILLER: A object that fills up space in a user interface.
 * @ATSPI_ROLE_FOCUS_TRAVERSABLE: Don't use, reserved for future use.
 * @ATSPI_ROLE_FONT_CHOOSER: Allows selection of a display font.
 * @ATSPI_ROLE_FRAME: A top level window with a title bar, border, menubar,
 * etc.
 * @ATSPI_ROLE_GLASS_PANE: A pane that is guaranteed to be painted on top of
 * all panes beneath it.
 * @ATSPI_ROLE_HTML_CONTAINER: A document container for HTML, whose children   
 * represent the document content.
 * @ATSPI_ROLE_ICON: A small fixed size picture, typically used to decorate
 * components.
 * @ATSPI_ROLE_IMAGE: An image, typically static.
 * @ATSPI_ROLE_INTERNAL_FRAME: A frame-like object that is clipped by a desktop
 * pane.
 * @ATSPI_ROLE_LABEL: An object used to present an icon or short string in an
 * interface.
 * @ATSPI_ROLE_LAYERED_PANE: A specialized pane that allows its children to be
 * drawn in layers, providing a form of stacking order.
 * @ATSPI_ROLE_LIST: An object that presents a list of objects to the user and
 * allows the user to select one or more of them.
 * @ATSPI_ROLE_LIST_ITEM: An object that represents an element of a list.
 * @ATSPI_ROLE_MENU: An object usually found inside a menu bar that contains a
 * list of actions the user can choose from.
 * @ATSPI_ROLE_MENU_BAR: An object usually drawn at the top of the primary
 * dialog box of an application that contains a list of menus the user can
 * choose from.
 * @ATSPI_ROLE_MENU_ITEM: An object usually contained in a menu that presents
 * an action the user can choose.
 * @ATSPI_ROLE_OPTION_PANE: A specialized pane whose primary use is inside a
 * dialog.
 * @ATSPI_ROLE_PAGE_TAB: An object that is a child of a page tab list.
 * @ATSPI_ROLE_PAGE_TAB_LIST: An object that presents a series of panels (or
 * page tabs), one at a time,through some mechanism provided by the
 * object.
 * @ATSPI_ROLE_PANEL: A generic container that is often used to group objects.
 * @ATSPI_ROLE_PASSWORD_TEXT: A text object uses for passwords, or other places
 * where the text content is not shown visibly to the user.
 * @ATSPI_ROLE_POPUP_MENU: A temporary window that is usually used to offer the
 * user a list of choices, and then hides when the user selects one of those
 * choices.
 * @ATSPI_ROLE_PROGRESS_BAR: An object used to indicate how much of a task has
 * been completed.
 * @ATSPI_ROLE_PUSH_BUTTON: An object the user can manipulate to tell the
 * application to do something.
 * @ATSPI_ROLE_RADIO_BUTTON: A specialized check box that will cause other
 * radio buttons in the same group to become unchecked when this one is
 * checked.
 * @ATSPI_ROLE_RADIO_MENU_ITEM: Object is both a menu item and a "radio button"
 * . See @ATSPI_ROLE_RADIO_BUTTON.
 * @ATSPI_ROLE_ROOT_PANE: A specialized pane that has a glass pane and a
 * layered pane as its children.
 * @ATSPI_ROLE_ROW_HEADER: The header for a row of data.
 * @ATSPI_ROLE_SCROLL_BAR: An object usually used to allow a user to
 * incrementally view a large amount of data by moving the bounds of a
 * viewport along a one-dimensional axis.
 * @ATSPI_ROLE_SCROLL_PANE: An object that allows a user to incrementally view
 * a large amount of information. @ATSPI_ROLE_SCROLL_PANE objects are usually
 * accompanied by @ATSPI_ROLE_SCROLL_BAR controllers, on which the
 * @ATSPI_RELATION_CONTROLLER_FOR and @ATSPI_RELATION_CONTROLLED_BY 
 * reciprocal relations are set. See  #atspi_get_relation_set.
 * @ATSPI_ROLE_SEPARATOR: An object usually contained in a menu to provide a
 * visible and logical separation of the contents in a menu.
 * @ATSPI_ROLE_SLIDER: An object that allows the user to select from a bounded
 * range.
 * @ATSPI_ROLE_SPIN_BUTTON: An object which allows one of a set of choices to
 * be selected, and which displays the current choice.  Unlike
 * @ATSPI_ROLE_SCROLL_BAR, @ATSPI_ROLE_SLIDER objects need not control 
 * 'viewport'-like objects.
 * @ATSPI_ROLE_SPLIT_PANE: A specialized panel that presents two other panels
 * at the same time.
 * @ATSPI_ROLE_STATUS_BAR: Object displays non-quantitative status information
 * (c.f. @ATSPI_ROLE_PROGRESS_BAR)
 * @ATSPI_ROLE_TABLE: An object used to repesent information in terms of rows
 * and columns.
 * @ATSPI_ROLE_TABLE_CELL: A 'cell' or discrete child within a Table. Note:
 * Table cells need not have @ATSPI_ROLE_TABLE_CELL, other 
 * #AtspiRoleType values are valid as well.
 * @ATSPI_ROLE_TABLE_COLUMN_HEADER: An object which labels a particular column
 * in an #AtspiTable.
 * @ATSPI_ROLE_TABLE_ROW_HEADER: An object which labels a particular row in a
 * #AtspiTable. #AtspiTable rows and columns may also be labelled via the
 * @ATSPI_RELATION_LABEL_FOR/@ATSPI_RELATION_LABELLED_BY relationships.
 * See #atspi_get_relation_set.
 * @ATSPI_ROLE_TEAROFF_MENU_ITEM: Object allows menu to be removed from menubar
 * and shown in its own window.
 * @ATSPI_ROLE_TERMINAL: An object that emulates a terminal.
 * @ATSPI_ROLE_TEXT: An interactive widget that supports multiple lines of text
 * and optionally accepts user input, but whose purpose is not to solicit user
 * input. Thus @ATSPI_ROLE_TEXT is appropriate for the text view in a plain text
 * editor but inappropriate for an input field in a dialog box or web form. For
 * widgets whose purpose is to solicit input from the user, see @ATSPI_ROLE_ENTRY
 * and @ATSPI_ROLE_PASSWORD_TEXT. For generic objects which display a brief amount
 * of textual information, see @ATSPI_ROLE_STATIC.
 * @ATSPI_ROLE_TOGGLE_BUTTON: A specialized push button that can be checked or
 * unchecked, but does not procide a separate indicator for the current
 * state.
 * @ATSPI_ROLE_TOOL_BAR: A bar or palette usually composed of push buttons or
 * toggle buttons.
 * @ATSPI_ROLE_TOOL_TIP: An object that provides information about another
 * object.
 * @ATSPI_ROLE_TREE: An object used to repsent hierarchical information to the
 * user.
 * @ATSPI_ROLE_TREE_TABLE: An object that presents both tabular and
 * hierarchical info to the user.
 * @ATSPI_ROLE_UNKNOWN: The object contains some #AtspiAccessible information, 
 * but its role is not known.
 * @ATSPI_ROLE_VIEWPORT: An object usually used in a scroll pane, or to
 * otherwise clip a larger object or content renderer to a specific
 * onscreen viewport.
 * @ATSPI_ROLE_WINDOW: A top level window with no title or border.
 * @ATSPI_ROLE_EXTENDED: means that the role for this item is known, but not
 * included in the core enumeration. Deprecated since 2.24.
 * @ATSPI_ROLE_HEADER: An object that serves as a document header.
 * @ATSPI_ROLE_FOOTER: An object that serves as a document footer.
 * @ATSPI_ROLE_PARAGRAPH: An object which is contains a single paragraph of
 * text content. See also @ATSPI_ROLE_TEXT.
 * @ATSPI_ROLE_RULER: An object which describes margins and tab stops, etc.    
 *    for text objects which it controls (should have 
 * @ATSPI_RELATION_CONTROLLER_FOR relation to such).
 * @ATSPI_ROLE_APPLICATION: An object corresponding to the toplevel accessible
 * of an application, which may contain @ATSPI_ROLE_FRAME objects or other      
 * accessible objects. Children of #AccessibleDesktop objects  are generally
 * @ATSPI_ROLE_APPLICATION objects.
 * @ATSPI_ROLE_AUTOCOMPLETE: The object is a dialog or list containing items
 * for insertion into an entry widget, for instance a list of words for
 * completion of a text entry.
 * @ATSPI_ROLE_EDITBAR: The object is an editable text object in a toolbar.
 * @ATSPI_ROLE_EMBEDDED: The object is an embedded component container.  This
 * role is a "grouping" hint that the contained objects share a context
 * which is different from the container in which this accessible is
 * embedded. In particular, it is used for some kinds of document embedding,
 * and for embedding of out-of-process component, "panel applets", etc.
 * @ATSPI_ROLE_ENTRY: The object is a component whose textual content may be
 * entered or modified by the user, provided @ATSPI_STATE_EDITABLE is present.
 * A readonly @ATSPI_ROLE_ENTRY object (i.e. where @ATSPI_STATE_EDITABLE is 
 * not present) implies a read-only 'text field' in a form, as opposed to a 
 * title, label, or caption.
 * @ATSPI_ROLE_CHART: The object is a graphical depiction of quantitative data.
 * It may contain multiple subelements whose attributes and/or description
 * may be queried to obtain both the  quantitative data and information about
 * how the data is being presented. The @ATSPI_LABELLED_BY relation is 
 * particularly important in interpreting objects of this type, as is the
 * accessible description property. See @ATSPI_ROLE_CAPTION.
 * @ATSPI_ROLE_CAPTION: The object contains descriptive information, usually
 * textual, about another user interface element such as a table, chart, or
 * image.
 * @ATSPI_ROLE_DOCUMENT_FRAME: The object is a visual frame or container which
 * contains a view of document content. #AtspiDocument frames may occur within
 * another #AtspiDocument instance, in which case the second  document may be
 * said to be embedded in the containing instance.  HTML frames are often
 * ATSPI_ROLE_DOCUMENT_FRAME:  Either this object, or a singleton descendant, 
 * should implement the #AtspiDocument interface.
 * @ATSPI_ROLE_HEADING: The object serves as a heading for content which
 * follows it in a document. The 'heading level' of the heading, if
 * availabe,  may be obtained by       querying the object's attributes.
 * @ATSPI_ROLE_PAGE: The object is a containing instance which encapsulates a
 * page of information. @ATSPI_ROLE_PAGE is used in documents and content which
 * support a paginated navigation model.
 * @ATSPI_ROLE_SECTION: The object is a containing instance of document content
 * which constitutes a particular 'logical' section of the document.  The
 * type of content within a section, and the nature of the section division
 * itself, may be obtained by querying the object's attributes.  Sections
 * may be nested.
 * @ATSPI_ROLE_REDUNDANT_OBJECT: The object is redundant with another object in
 * the hierarchy, and is exposed for purely technical reasons.  Objects of
 * this role should be ignored by clients, if they are encountered at all.
 * @ATSPI_ROLE_FORM: The object is a containing instance of document content
 * which has within it components with which the user can interact in order
 * to input information; i.e. the object is a container for pushbuttons,    
 * comboboxes, text input fields, and other 'GUI' components. @ATSPI_ROLE_FORM
 * should not, in general, be used for toplevel GUI containers or dialogs,
 * but should be reserved for 'GUI' containers which occur within document
 * content, for instance within Web documents, presentations, or text
 * documents.  Unlike other GUI containers and dialogs which occur inside      
 * application instances, @ATSPI_ROLE_FORM containers' components are
 * associated with the current document, rather than the current foreground 
 * application or viewer instance.
 * @ATSPI_ROLE_LINK: The object is a hypertext anchor, i.e. a "link" in a      
 * hypertext document.  Such objects are distinct from 'inline'       content
 * which may also use the #AtspiHypertext/#AtspiHyperlink interfacesto indicate
 * the range/location within a text object where an inline or embedded object
 * lies.
 * @ATSPI_ROLE_INPUT_METHOD_WINDOW: The object is a window or similar viewport
 * which is used to allow composition or input of a 'complex character',    
 * in other words it is an "input method window".
 * @ATSPI_ROLE_TABLE_ROW: A row in a table.
 * @ATSPI_ROLE_TREE_ITEM: An object that represents an element of a tree.
 * @ATSPI_ROLE_DOCUMENT_SPREADSHEET: A document frame which contains a
 * spreadsheet.
 * @ATSPI_ROLE_DOCUMENT_PRESENTATION: A document frame which contains a
 * presentation or slide content.
 * @ATSPI_ROLE_DOCUMENT_TEXT: A document frame which contains textual content,
 * such as found in a word processing
 * application.
 * @ATSPI_ROLE_DOCUMENT_WEB: A document frame which contains HTML or other
 * markup suitable for display in a web browser.
 * @ATSPI_ROLE_DOCUMENT_EMAIL: A document frame which contains email content
 * to be displayed or composed either in plain text or
 * HTML.
 * @ATSPI_ROLE_COMMENT: An object found within a document and designed to
 * present a comment, note, or other annotation. In some cases, this object
 * might not be visible until activated.
 * @ATSPI_ROLE_LIST_BOX: A non-collapsible list of choices the user can
 * select from.
 * @ATSPI_ROLE_GROUPING: A group of related widgets. This group typically has
 * a label.
 * @ATSPI_ROLE_IMAGE_MAP: An image map object. Usually a graphic with multiple
 * hotspots, where each hotspot can be activated resulting in the loading of
 * another document or section of a document.
 * @ATSPI_ROLE_NOTIFICATION: A transitory object designed to present a
 * message to the user, typically at the desktop level rather than inside a
 * particular application.
 * @ATSPI_ROLE_INFO_BAR: An object designed to present a message to the user
 * within an existing window.
 * @ATSPI_ROLE_LEVEL_BAR: A bar that serves as a level indicator to, for
 * instance, show the strength of a password or the state of a battery. @Since: 2.8
 * @ATSPI_ROLE_TITLE_BAR: A bar that serves as the title of a window or a
 *  dialog. @Since: 2.12
 * @ATSPI_ROLE_BLOCK_QUOTE: An object which contains a text section
 *  that is quoted from another source.  @Since: 2.12
 * @ATSPI_ROLE_AUDIO: An object which represents an audio
 *  element. @Since: 2.12
 * @ATSPI_ROLE_VIDEO: An object which represents a video
 *  element. @Since: 2.12
 * @ATSPI_ROLE_DEFINITION: A definition of a term or concept. @Since: 2.12
 * @ATSPI_ROLE_ARTICLE: A section of a page that consists of a
 *  composition that forms an independent part of a document, page, or
 *  site. Examples: A blog entry, a news story, a forum post. @Since:
 *  2.12
 * @ATSPI_ROLE_LANDMARK: A region of a web page intended as a
 *  navigational landmark. This is designed to allow Assistive
 *  Technologies to provide quick navigation among key regions within a
 *  document. @Since: 2.12
 * @ATSPI_ROLE_LOG: A text widget or container holding log content, such
 *  as chat history and error logs. In this role there is a
 *  relationship between the arrival of new items in the log and the
 *  reading order. The log contains a meaningful sequence and new
 *  information is added only to the end of the log, not at arbitrary
 *  points. @Since: 2.12
 * @ATSPI_ROLE_MARQUEE: A container where non-essential information
 *  changes frequently. Common usages of marquee include stock tickers
 *  and ad banners. The primary difference between a marquee and a log
 *  is that logs usually have a meaningful order or sequence of
 *  important content changes. @Since: 2.12
 * @ATSPI_ROLE_MATH: A text widget or container that holds a mathematical
 *  expression. @Since: 2.12
 * @ATSPI_ROLE_RATING: A widget whose purpose is to display a rating,
 *  such as the number of stars associated with a song in a media
 *  player. Objects of this role should also implement
 *  AtspiValue. @Since: 2.12
 * @ATSPI_ROLE_TIMER: An object containing a numerical counter which
 *  indicates an amount of elapsed time from a start point, or the time
 *  remaining until an end point. @Since: 2.12
 * @ATSPI_ROLE_STATIC: A generic non-container object whose purpose is to display
 *  a brief amount of information to the user and whose role is known by the
 *  implementor but lacks semantic value for the user. Examples in which
 *  @ATSPI_ROLE_STATIC is appropriate include the message displayed in a message
 *  box and an image used as an alternative means to display text.
 *  @ATSPI_ROLE_STATIC should not be applied to widgets which are traditionally
 *  interactive, objects which display a significant amount of content, or any
 *  object which has an accessible relation pointing to another object. The
 *  displayed information, as a general rule, should be exposed through the
 *  accessible name of the object. For labels which describe another widget, see
 *  @ATSPI_ROLE_LABEL. For text views, see @ATSPI_ROLE_TEXT. For generic
 *  containers, see @ATSPI_ROLE_PANEL. For objects whose role is not known by the
 *  implementor, see @ATSPI_ROLE_UNKNOWN. @Since: 2.16.
 * @ATSPI_ROLE_MATH_FRACTION: An object that represents a mathematical fraction. @Since: 2.16.
 * @ATSPI_ROLE_MATH_ROOT: An object that represents a mathematical expression
 *  displayed with a radical. @Since: 2.16.
 * @ATSPI_ROLE_SUBSCRIPT: An object that contains text that is displayed as a
 *  subscript. @Since: 2.16.
 * @ATSPI_ROLE_SUPERSCRIPT: An object that contains text that is displayed as a
 *  superscript. @Since: 2.16.
 * @ATSPI_ROLE_DESCRIPTION_LIST: An object that represents a list of term-value
 *  groups. A term-value group represents an individual description and consist
 *  of one or more names (@ATSPI_ROLE_DESCRIPTION_TERM) followed by one or more
 *  values (@ATSPI_ROLE_DESCRIPTION_VALUE). For each list, there should not be
 *  more than one group with the same term name. @Since: 2.26.
 * @ATSPI_ROLE_DESCRIPTION_TERM: An object that represents a term or phrase
 *  with a corresponding definition. @Since: 2.26.
 * @ATSPI_ROLE_DESCRIPTION_VALUE: An object that represents the description,
 *  definition, or value of a term. @Since: 2.26.
 * @ATSPI_ROLE_FOOTNOTE: An object that contains the text of a footnote. @Since: 2.26.
 * @ATSPI_ROLE_CONTENT_DELETION: Content previously deleted or proposed to be
 * deleted, e.g. in revision history or a content view providing suggestions
 * from reviewers. @Since: 2.34.
 * @ATSPI_ROLE_CONTENT_INSERTION: Content previously inserted or proposed to be
 * inserted, e.g. in revision history or a content view providing suggestions
 * from reviewers. @Since: 2.34.
 * @ATSPI_ROLE_MARK: A run of content that is marked or highlighted, such as for
 * reference purposes, or to call it out as having a special purpose. If the
 * marked content has an associated section in the document elaborating on the
 * reason for the mark, then %ATSPI_RELATION_DETAILS should be used on the mark
 * to point to that associated section. In addition, the reciprocal relation
 * %ATSPI_RELATION_DETAILS_FOR should be used on the associated content section
 * to point back to the mark. @Since: 2.36.
 * @ATSPI_ROLE_SUGGESTION: A container for content that is called out as a proposed
 * change from the current version of the document, such as by a reviewer of the
 * content. This role should include either %ATSPI_ROLE_CONTENT_DELETION and/or
 * %ATSPI_ROLE_CONTENT_INSERTION children, in any order, to indicate what the
 * actual change is. @Since: 2.36
 *  @ATSPI_ROLE_LAST_DEFINED: Not a valid role, used for finding end of
 *  enumeration.
 *
 * Enumeration used by interface #AtspiAccessible to specify the role
 * of an #AtspiAccessible object.
 *
 */
typedef enum {
    ATSPI_ROLE_INVALID,
    ATSPI_ROLE_ACCELERATOR_LABEL,
    ATSPI_ROLE_ALERT,
    ATSPI_ROLE_ANIMATION,
    ATSPI_ROLE_ARROW,
    ATSPI_ROLE_CALENDAR,
    ATSPI_ROLE_CANVAS,
    ATSPI_ROLE_CHECK_BOX,
    ATSPI_ROLE_CHECK_MENU_ITEM,
    ATSPI_ROLE_COLOR_CHOOSER,
    ATSPI_ROLE_COLUMN_HEADER,
    ATSPI_ROLE_COMBO_BOX,
    ATSPI_ROLE_DATE_EDITOR,
    ATSPI_ROLE_DESKTOP_ICON,
    ATSPI_ROLE_DESKTOP_FRAME,
    ATSPI_ROLE_DIAL,
    ATSPI_ROLE_DIALOG,
    ATSPI_ROLE_DIRECTORY_PANE,
    ATSPI_ROLE_DRAWING_AREA,
    ATSPI_ROLE_FILE_CHOOSER,
    ATSPI_ROLE_FILLER,
    ATSPI_ROLE_FOCUS_TRAVERSABLE,
    ATSPI_ROLE_FONT_CHOOSER,
    ATSPI_ROLE_FRAME,
    ATSPI_ROLE_GLASS_PANE,
    ATSPI_ROLE_HTML_CONTAINER,
    ATSPI_ROLE_ICON,
    ATSPI_ROLE_IMAGE,
    ATSPI_ROLE_INTERNAL_FRAME,
    ATSPI_ROLE_LABEL,
    ATSPI_ROLE_LAYERED_PANE,
    ATSPI_ROLE_LIST,
    ATSPI_ROLE_LIST_ITEM,
    ATSPI_ROLE_MENU,
    ATSPI_ROLE_MENU_BAR,
    ATSPI_ROLE_MENU_ITEM,
    ATSPI_ROLE_OPTION_PANE,
    ATSPI_ROLE_PAGE_TAB,
    ATSPI_ROLE_PAGE_TAB_LIST,
    ATSPI_ROLE_PANEL,
    ATSPI_ROLE_PASSWORD_TEXT,
    ATSPI_ROLE_POPUP_MENU,
    ATSPI_ROLE_PROGRESS_BAR,
    ATSPI_ROLE_PUSH_BUTTON,
    ATSPI_ROLE_RADIO_BUTTON,
    ATSPI_ROLE_RADIO_MENU_ITEM,
    ATSPI_ROLE_ROOT_PANE,
    ATSPI_ROLE_ROW_HEADER,
    ATSPI_ROLE_SCROLL_BAR,
    ATSPI_ROLE_SCROLL_PANE,
    ATSPI_ROLE_SEPARATOR,
    ATSPI_ROLE_SLIDER,
    ATSPI_ROLE_SPIN_BUTTON,
    ATSPI_ROLE_SPLIT_PANE,
    ATSPI_ROLE_STATUS_BAR,
    ATSPI_ROLE_TABLE,
    ATSPI_ROLE_TABLE_CELL,
    ATSPI_ROLE_TABLE_COLUMN_HEADER,
    ATSPI_ROLE_TABLE_ROW_HEADER,
    ATSPI_ROLE_TEAROFF_MENU_ITEM,
    ATSPI_ROLE_TERMINAL,
    ATSPI_ROLE_TEXT,
    ATSPI_ROLE_TOGGLE_BUTTON,
    ATSPI_ROLE_TOOL_BAR,
    ATSPI_ROLE_TOOL_TIP,
    ATSPI_ROLE_TREE,
    ATSPI_ROLE_TREE_TABLE,
    ATSPI_ROLE_UNKNOWN,
    ATSPI_ROLE_VIEWPORT,
    ATSPI_ROLE_WINDOW,
    ATSPI_ROLE_EXTENDED,
    ATSPI_ROLE_HEADER,
    ATSPI_ROLE_FOOTER,
    ATSPI_ROLE_PARAGRAPH,
    ATSPI_ROLE_RULER,
    ATSPI_ROLE_APPLICATION,
    ATSPI_ROLE_AUTOCOMPLETE,
    ATSPI_ROLE_EDITBAR,
    ATSPI_ROLE_EMBEDDED,
    ATSPI_ROLE_ENTRY,
    ATSPI_ROLE_CHART,
    ATSPI_ROLE_CAPTION,
    ATSPI_ROLE_DOCUMENT_FRAME,
    ATSPI_ROLE_HEADING,
    ATSPI_ROLE_PAGE,
    ATSPI_ROLE_SECTION,
    ATSPI_ROLE_REDUNDANT_OBJECT,
    ATSPI_ROLE_FORM,
    ATSPI_ROLE_LINK,
    ATSPI_ROLE_INPUT_METHOD_WINDOW,
    ATSPI_ROLE_TABLE_ROW,
    ATSPI_ROLE_TREE_ITEM,
    ATSPI_ROLE_DOCUMENT_SPREADSHEET,
    ATSPI_ROLE_DOCUMENT_PRESENTATION,
    ATSPI_ROLE_DOCUMENT_TEXT,
    ATSPI_ROLE_DOCUMENT_WEB,
    ATSPI_ROLE_DOCUMENT_EMAIL,
    ATSPI_ROLE_COMMENT,
    ATSPI_ROLE_LIST_BOX,
    ATSPI_ROLE_GROUPING,
    ATSPI_ROLE_IMAGE_MAP,
    ATSPI_ROLE_NOTIFICATION,
    ATSPI_ROLE_INFO_BAR,
    ATSPI_ROLE_LEVEL_BAR,
    ATSPI_ROLE_TITLE_BAR,
    ATSPI_ROLE_BLOCK_QUOTE,
    ATSPI_ROLE_AUDIO,
    ATSPI_ROLE_VIDEO,
    ATSPI_ROLE_DEFINITION,
    ATSPI_ROLE_ARTICLE,
    ATSPI_ROLE_LANDMARK,
    ATSPI_ROLE_LOG,
    ATSPI_ROLE_MARQUEE,
    ATSPI_ROLE_MATH,
    ATSPI_ROLE_RATING,
    ATSPI_ROLE_TIMER,
    ATSPI_ROLE_STATIC,
    ATSPI_ROLE_MATH_FRACTION,
    ATSPI_ROLE_MATH_ROOT,
    ATSPI_ROLE_SUBSCRIPT,
    ATSPI_ROLE_SUPERSCRIPT,
    ATSPI_ROLE_DESCRIPTION_LIST,
    ATSPI_ROLE_DESCRIPTION_TERM,
    ATSPI_ROLE_DESCRIPTION_VALUE,
    ATSPI_ROLE_FOOTNOTE,
    ATSPI_ROLE_CONTENT_DELETION,
    ATSPI_ROLE_CONTENT_INSERTION,
    ATSPI_ROLE_MARK,
    ATSPI_ROLE_SUGGESTION,
    ATSPI_ROLE_LAST_DEFINED,
} AtspiRole;

/**
 * ATSPI_ROLE_COUNT:
 *
 * One higher than the highest valid value of #AtspiRole.
 */
#define ATSPI_ROLE_COUNT (129+1)

typedef enum
{
     ATSPI_CACHE_NONE     = 0,
     ATSPI_CACHE_PARENT   = 1 << 0,
  ATSPI_CACHE_CHILDREN    = 1 << 1,
  ATSPI_CACHE_NAME        = 1 << 2,
  ATSPI_CACHE_DESCRIPTION = 1 << 3,
  ATSPI_CACHE_STATES      = 1 << 4,
  ATSPI_CACHE_ROLE        = 1 << 5,
  ATSPI_CACHE_INTERFACES  = 1 << 6,
  ATSPI_CACHE_ATTRIBUTES = 1 << 7,
  ATSPI_CACHE_ALL         = 0x3fffffff,
  ATSPI_CACHE_DEFAULT = ATSPI_CACHE_PARENT | ATSPI_CACHE_CHILDREN | ATSPI_CACHE_NAME | ATSPI_CACHE_DESCRIPTION | ATSPI_CACHE_STATES | ATSPI_CACHE_ROLE | ATSPI_CACHE_INTERFACES,
  ATSPI_CACHE_UNDEFINED   = 0x40000000,
} AtspiCache;

/**
 * AtspiScrollType:
 * @ATSPI_SCROLL_TOP_LEFT: Scroll the object to the top left corner of the
 * window.
 * @ATSPI_SCROLL_BOTTOM_RIGHT: Scroll the object to the bottom right corner of
 * the window.
 * @ATSPI_SCROLL_TOP_EDGE: Scroll the object to the top edge of the window.
 * @ATSPI_SCROLL_BOTTOM_EDGE: Scroll the object to the bottom edge of the
 * window.
 * @ATSPI_SCROLL_LEFT_EDGE: Scroll the object to the left edge of the
 * window.
 * @ATSPI_SCROLL_RIGHT_EDGE: Scroll the object to the right edge of the
 * window.
 * @ATSPI_SCROLL_ANYWHERE: Scroll the object to application-dependent position
 * on the window.
 *
 * Enumeration used by interface #AtspiAccessible to specify where an
 * #AtspiAccessible object should be placed on the screen when using scroll_to.
 *
 */
typedef enum {
  ATSPI_SCROLL_TOP_LEFT,
  ATSPI_SCROLL_BOTTOM_RIGHT,
  ATSPI_SCROLL_TOP_EDGE,
  ATSPI_SCROLL_BOTTOM_EDGE,
  ATSPI_SCROLL_LEFT_EDGE,
  ATSPI_SCROLL_RIGHT_EDGE,
  ATSPI_SCROLL_ANYWHERE
} AtspiScrollType;

/**
 * ATSPI_SCROLLTYPE_COUNT:
 *
 * One higher than the highest valid value of #AtspiScrollType.
 */
#define ATSPI_SCROLLTYPE_COUNT (6+1)

#define ATSPI_DBUS_NAME_REGISTRY "org.a11y.atspi.Registry"
#define ATSPI_DBUS_PATH_REGISTRY "/org/a11y/atspi/registry"
#define ATSPI_DBUS_INTERFACE_REGISTRY "org.a11y.atspi.Registry"

#define ATSPI_DBUS_PATH_NULL "/org/a11y/atspi/null"
#define ATSPI_DBUS_PATH_ROOT "/org/a11y/atspi/accessible/root"

#define ATSPI_DBUS_PATH_DEC "/org/a11y/atspi/registry/deviceeventcontroller"
#define ATSPI_DBUS_INTERFACE_DEC "org.a11y.atspi.DeviceEventController"
#define ATSPI_DBUS_INTERFACE_DEVICE_EVENT_LISTENER "org.a11y.atspi.DeviceEventListener"

#define ATSPI_DBUS_INTERFACE_CACHE "org.a11y.atspi.Cache"
#define ATSPI_DBUS_INTERFACE_ACCESSIBLE "org.a11y.atspi.Accessible"
#define ATSPI_DBUS_INTERFACE_ACTION "org.a11y.atspi.Action"
#define ATSPI_DBUS_INTERFACE_APPLICATION "org.a11y.atspi.Application"
#define ATSPI_DBUS_INTERFACE_COLLECTION "org.a11y.atspi.Collection"
#define ATSPI_DBUS_INTERFACE_COMPONENT "org.a11y.atspi.Component"
#define ATSPI_DBUS_INTERFACE_DOCUMENT "org.a11y.atspi.Document"
#define ATSPI_DBUS_INTERFACE_EDITABLE_TEXT "org.a11y.atspi.EditableText"
#define ATSPI_DBUS_INTERFACE_EVENT_KEYBOARD "org.a11y.atspi.Event.Keyboard"
#define ATSPI_DBUS_INTERFACE_EVENT_MOUSE "org.a11y.atspi.Event.Mouse"
#define ATSPI_DBUS_INTERFACE_EVENT_OBJECT "org.a11y.atspi.Event.Object"
#define ATSPI_DBUS_INTERFACE_HYPERLINK "org.a11y.atspi.Hyperlink"
#define ATSPI_DBUS_INTERFACE_HYPERTEXT "org.a11y.atspi.Hypertext"
#define ATSPI_DBUS_INTERFACE_IMAGE "org.a11y.atspi.Image"
#define ATSPI_DBUS_INTERFACE_SELECTION "org.a11y.atspi.Selection"
#define ATSPI_DBUS_INTERFACE_TABLE "org.a11y.atspi.Table"
#define ATSPI_DBUS_INTERFACE_TABLE_CELL "org.a11y.atspi.TableCell"
#define ATSPI_DBUS_INTERFACE_TEXT "org.a11y.atspi.Text"
#define ATSPI_DBUS_INTERFACE_VALUE "org.a11y.atspi.Value"
#define ATSPI_DBUS_INTERFACE_SOCKET "org.a11y.atspi.Socket"

#define ATSPI_DBUS_PATH_SCREEN_READER "/org/a11y/atspi/screenreader"
#define ATSPI_DBUS_INTERFACE_EVENT_SCREEN_READER "org.a11y.atspi.Event.ScreenReader"

#ifdef __cplusplus
}
#endif
#endif	/* _ATSPI_CONSTANTS_H_ */

// SPDX-License-Identifier: MIT OR Apache-2.0 OR Zlib

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(improper_ctypes, improper_ctypes_definitions)]
#![deny(clippy::all)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(clippy, deny(warnings))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! The cross platform cursor icon type.
//!
//! This type is intended to be used as a standard interoperability type between
//! GUI frameworks in order to convey the cursor icon type.
//!
//! # Example
//!
//! ```
//! use cursor_icon::CursorIcon;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Parse a cursor icon from the string that describes it.
//! let cursor_name = "pointer";
//! let cursor_icon: CursorIcon = cursor_name.parse()?;
//! println!("The cursor icon is {:?}", cursor_icon);
//! # Ok(())
//! # }
//! ```

// This file contains a portion of the CSS Basic User Interface Module Level 3
// specification. In particular, the names for the cursor from the #cursor
// section and documentation for some of the variants were taken.
//
// The original document is https://www.w3.org/TR/css-ui-3/#cursor.
// Copyright © 2018 W3C® (MIT, ERCIM, Keio, Beihang)
//
// These documents were used under the terms of the following license. This W3C
// license as well as the W3C short notice apply to the `CursorIcon` enum's
// variants and documentation attached to them.

// --------- BEGINNING OF W3C LICENSE
// --------------------------------------------------------------
//
// License
//
// By obtaining and/or copying this work, you (the licensee) agree that you have
// read, understood, and will comply with the following terms and conditions.
//
// Permission to copy, modify, and distribute this work, with or without
// modification, for any purpose and without fee or royalty is hereby granted,
// provided that you include the following on ALL copies of the work or portions
// thereof, including modifications:
//
// - The full text of this NOTICE in a location viewable to users of the
//   redistributed or derivative work.
// - Any pre-existing intellectual property disclaimers, notices, or terms and
//   conditions. If none exist, the W3C Software and Document Short Notice
//   should be included.
// - Notice of any changes or modifications, through a copyright statement on
//   the new code or document such as "This software or document includes
//   material copied from or derived from [title and URI of the W3C document].
//   Copyright © [YEAR] W3C® (MIT, ERCIM, Keio, Beihang)."
//
// Disclaimers
//
// THIS WORK IS PROVIDED "AS IS," AND COPYRIGHT HOLDERS MAKE NO REPRESENTATIONS
// OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES
// OF MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
// THE SOFTWARE OR DOCUMENT WILL NOT INFRINGE ANY THIRD PARTY PATENTS,
// COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
//
// COPYRIGHT HOLDERS WILL NOT BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL OR
// CONSEQUENTIAL DAMAGES ARISING OUT OF ANY USE OF THE SOFTWARE OR DOCUMENT.
//
// The name and trademarks of copyright holders may NOT be used in advertising
// or publicity pertaining to the work without specific, written prior
// permission. Title to copyright in this work will at all times remain with
// copyright holders.
//
// --------- END OF W3C LICENSE
// --------------------------------------------------------------------

// --------- BEGINNING OF W3C SHORT NOTICE
// ---------------------------------------------------------
//
// winit: https://github.com/rust-windowing/cursor-icon
//
// Copyright © 2023 World Wide Web Consortium, (Massachusetts Institute of
// Technology, European Research Consortium for Informatics and Mathematics,
// Keio University, Beihang). All Rights Reserved. This work is distributed
// under the W3C® Software License [1] in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.
//
// [1] http://www.w3.org/Consortium/Legal/copyright-software
//
// --------- END OF W3C SHORT NOTICE
// --------------------------------------------------------------

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

// XXX for forwards compatibility.
#[cfg(feature = "alloc")]
extern crate alloc as _;

/// Describes the appearance of the (usually mouse) cursor icon.
///
/// The names are taken from the CSS W3C specification:
/// <https://www.w3.org/TR/css-ui-3/#cursor>
///
/// # Examples
///
/// ```
/// use cursor_icon::CursorIcon;
///
/// // Get the cursor icon for the default cursor.
/// let cursor_icon = CursorIcon::Default;
/// ```
#[non_exhaustive]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CursorIcon {
    /// The platform-dependent default cursor. Often rendered as arrow.
    #[default]
    Default,

    /// A context menu is available for the object under the cursor. Often
    /// rendered as an arrow with a small menu-like graphic next to it.
    ContextMenu,

    /// Help is available for the object under the cursor. Often rendered as a
    /// question mark or a balloon.
    Help,

    /// The cursor is a pointer that indicates a link. Often rendered as the
    /// backside of a hand with the index finger extended.
    Pointer,

    /// A progress indicator. The program is performing some processing, but is
    /// different from [`CursorIcon::Wait`] in that the user may still interact
    /// with the program.
    Progress,

    /// Indicates that the program is busy and the user should wait. Often
    /// rendered as a watch or hourglass.
    Wait,

    /// Indicates that a cell or set of cells may be selected. Often rendered as
    /// a thick plus-sign with a dot in the middle.
    Cell,

    /// A simple crosshair (e.g., short line segments resembling a "+" sign).
    /// Often used to indicate a two dimensional bitmap selection mode.
    Crosshair,

    /// Indicates text that may be selected. Often rendered as an I-beam.
    Text,

    /// Indicates vertical-text that may be selected. Often rendered as a
    /// horizontal I-beam.
    VerticalText,

    /// Indicates an alias of/shortcut to something is to be created. Often
    /// rendered as an arrow with a small curved arrow next to it.
    Alias,

    /// Indicates something is to be copied. Often rendered as an arrow with a
    /// small plus sign next to it.
    Copy,

    /// Indicates something is to be moved.
    Move,

    /// Indicates that the dragged item cannot be dropped at the current cursor
    /// location. Often rendered as a hand or pointer with a small circle with a
    /// line through it.
    NoDrop,

    /// Indicates that the requested action will not be carried out. Often
    /// rendered as a circle with a line through it.
    NotAllowed,

    /// Indicates that something can be grabbed (dragged to be moved). Often
    /// rendered as the backside of an open hand.
    Grab,

    /// Indicates that something is being grabbed (dragged to be moved). Often
    /// rendered as the backside of a hand with fingers closed mostly out of
    /// view.
    Grabbing,

    /// The east border to be moved.
    EResize,

    /// The north border to be moved.
    NResize,

    /// The north-east corner to be moved.
    NeResize,

    /// The north-west corner to be moved.
    NwResize,

    /// The south border to be moved.
    SResize,

    /// The south-east corner to be moved.
    SeResize,

    /// The south-west corner to be moved.
    SwResize,

    /// The west border to be moved.
    WResize,

    /// The east and west borders to be moved.
    EwResize,

    /// The south and north borders to be moved.
    NsResize,

    /// The north-east and south-west corners to be moved.
    NeswResize,

    /// The north-west and south-east corners to be moved.
    NwseResize,

    /// Indicates that the item/column can be resized horizontally. Often
    /// rendered as arrows pointing left and right with a vertical bar
    /// separating them.
    ColResize,

    /// Indicates that the item/row can be resized vertically. Often rendered as
    /// arrows pointing up and down with a horizontal bar separating them.
    RowResize,

    /// Indicates that the something can be scrolled in any direction. Often
    /// rendered as arrows pointing up, down, left, and right with a dot in the
    /// middle.
    AllScroll,

    /// Indicates that something can be zoomed in. Often rendered as a
    /// magnifying glass with a "+" in the center of the glass.
    ZoomIn,

    /// Indicates that something can be zoomed in. Often rendered as a
    /// magnifying glass with a "-" in the center of the glass.
    ZoomOut,

    /// Indicates that the user will select the action that will be carried out.
    ///
    /// This is a non-standard extension of the w3c standard used in freedesktop
    /// cursor icon themes.
    DndAsk,

    /// Indicates that something can be moved or resized in any direction.
    ///
    /// This is a non-standard extension of the w3c standard used in freedesktop
    /// cursor icon themes.
    AllResize,
}

impl CursorIcon {
    /// The name of the cursor icon as defined in the w3c standard.
    /// Non-standard cursors such as "DndAsk" and "AllResize" are translated as
    /// "dnd-ask" and "all-resize" respectively.
    ///
    /// This name most of the time could be passed as is to cursor loading
    /// libraries on X11/Wayland and could be used as-is on web.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cursor_icon::CursorIcon;
    /// use wayland_cursor::CursorTheme;
    ///
    /// # use wayland_client::Connection;
    /// # use wayland_client::protocol::wl_shm::WlShm;
    /// # fn test(conn: &Connection, shm: WlShm) -> Result<(), Box<dyn std::error::Error>> {
    /// // Choose a cursor to load.
    /// let cursor = CursorIcon::Help;
    ///
    /// // Load the Wayland cursor theme.
    /// let mut cursor_theme = CursorTheme::load(conn, shm, 32)?;
    ///
    /// // Load the cursor.
    /// let cursor = cursor_theme.get_cursor(cursor.name());
    /// if let Some(cursor) = cursor {
    ///     println!("Total number of images: {}", cursor.image_count());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            CursorIcon::Default => "default",
            CursorIcon::ContextMenu => "context-menu",
            CursorIcon::Help => "help",
            CursorIcon::Pointer => "pointer",
            CursorIcon::Progress => "progress",
            CursorIcon::Wait => "wait",
            CursorIcon::Cell => "cell",
            CursorIcon::Crosshair => "crosshair",
            CursorIcon::Text => "text",
            CursorIcon::VerticalText => "vertical-text",
            CursorIcon::Alias => "alias",
            CursorIcon::Copy => "copy",
            CursorIcon::Move => "move",
            CursorIcon::NoDrop => "no-drop",
            CursorIcon::NotAllowed => "not-allowed",
            CursorIcon::Grab => "grab",
            CursorIcon::Grabbing => "grabbing",
            CursorIcon::EResize => "e-resize",
            CursorIcon::NResize => "n-resize",
            CursorIcon::NeResize => "ne-resize",
            CursorIcon::NwResize => "nw-resize",
            CursorIcon::SResize => "s-resize",
            CursorIcon::SeResize => "se-resize",
            CursorIcon::SwResize => "sw-resize",
            CursorIcon::WResize => "w-resize",
            CursorIcon::EwResize => "ew-resize",
            CursorIcon::NsResize => "ns-resize",
            CursorIcon::NeswResize => "nesw-resize",
            CursorIcon::NwseResize => "nwse-resize",
            CursorIcon::ColResize => "col-resize",
            CursorIcon::RowResize => "row-resize",
            CursorIcon::AllScroll => "all-scroll",
            CursorIcon::ZoomIn => "zoom-in",
            CursorIcon::ZoomOut => "zoom-out",
            CursorIcon::DndAsk => "dnd-ask",
            CursorIcon::AllResize => "all-resize",
        }
    }

    /// A list of alternative names for the cursor icon as commonly found in
    /// legacy Xcursor themes.
    ///
    /// This should only be used as a fallback in case the cursor theme does not
    /// adhere to the w3c standard.
    pub fn alt_names(&self) -> &[&'static str] {
        match self {
            CursorIcon::Default => &["left_ptr", "arrow", "top_left_arrow", "left_arrow"],
            CursorIcon::ContextMenu => &[],
            CursorIcon::Help => &["question_arrow", "whats_this"],
            CursorIcon::Pointer => &["hand2", "hand1", "hand", "pointing_hand"],
            CursorIcon::Progress => &["left_ptr_watch", "half-busy"],
            CursorIcon::Wait => &["watch"],
            CursorIcon::Cell => &["plus"],
            CursorIcon::Crosshair => &["cross"],
            CursorIcon::Text => &["xterm", "ibeam"],
            CursorIcon::VerticalText => &[],
            CursorIcon::Alias => &["link"],
            CursorIcon::Copy => &[],
            CursorIcon::Move => &[],
            CursorIcon::NoDrop => &["circle"],
            CursorIcon::NotAllowed => &["crossed_circle", "forbidden"],
            CursorIcon::Grab => &["openhand", "fleur"],
            CursorIcon::Grabbing => &["closedhand"],
            CursorIcon::EResize => &["right_side"],
            CursorIcon::NResize => &["top_side"],
            CursorIcon::NeResize => &["top_right_corner"],
            CursorIcon::NwResize => &["top_left_corner"],
            CursorIcon::SResize => &["bottom_side"],
            CursorIcon::SeResize => &["bottom_right_corner"],
            CursorIcon::SwResize => &["bottom_left_corner"],
            CursorIcon::WResize => &["left_side"],
            CursorIcon::EwResize => &["h_double_arrow", "size_hor"],
            CursorIcon::NsResize => &["v_double_arrow", "size_ver"],
            CursorIcon::NeswResize => &["fd_double_arrow", "size_bdiag"],
            CursorIcon::NwseResize => &["bd_double_arrow", "size_fdiag"],
            CursorIcon::ColResize => &["split_h", "h_double_arrow", "sb_h_double_arrow"],
            CursorIcon::RowResize => &["split_v", "v_double_arrow", "sb_v_double_arrow"],
            CursorIcon::AllScroll => &["size_all"],
            CursorIcon::ZoomIn => &[],
            CursorIcon::ZoomOut => &[],
            CursorIcon::DndAsk => &["copy"],
            CursorIcon::AllResize => &["move"],
        }
    }
}

impl core::fmt::Display for CursorIcon {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name())
    }
}

impl core::str::FromStr for CursorIcon {
    type Err = ParseError;

    /// Parse a string slice into [`CursorIcon`].
    ///
    /// The `name` is a lower kebab case [`CursorIcon`] variant name, e.g.
    /// `nesw-resize`. The set of possible valid `name` values matches exactly
    /// the set of [`CursorIcon::name`] outputs.
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "default" => Ok(CursorIcon::Default),
            "context-menu" => Ok(CursorIcon::ContextMenu),
            "help" => Ok(CursorIcon::Help),
            "pointer" => Ok(CursorIcon::Pointer),
            "progress" => Ok(CursorIcon::Progress),
            "wait" => Ok(CursorIcon::Wait),
            "cell" => Ok(CursorIcon::Cell),
            "crosshair" => Ok(CursorIcon::Crosshair),
            "text" => Ok(CursorIcon::Text),
            "vertical-text" => Ok(CursorIcon::VerticalText),
            "alias" => Ok(CursorIcon::Alias),
            "copy" => Ok(CursorIcon::Copy),
            "move" => Ok(CursorIcon::Move),
            "no-drop" => Ok(CursorIcon::NoDrop),
            "not-allowed" => Ok(CursorIcon::NotAllowed),
            "grab" => Ok(CursorIcon::Grab),
            "grabbing" => Ok(CursorIcon::Grabbing),
            "e-resize" => Ok(CursorIcon::EResize),
            "n-resize" => Ok(CursorIcon::NResize),
            "ne-resize" => Ok(CursorIcon::NeResize),
            "nw-resize" => Ok(CursorIcon::NwResize),
            "s-resize" => Ok(CursorIcon::SResize),
            "se-resize" => Ok(CursorIcon::SeResize),
            "sw-resize" => Ok(CursorIcon::SwResize),
            "w-resize" => Ok(CursorIcon::WResize),
            "ew-resize" => Ok(CursorIcon::EwResize),
            "ns-resize" => Ok(CursorIcon::NsResize),
            "nesw-resize" => Ok(CursorIcon::NeswResize),
            "nwse-resize" => Ok(CursorIcon::NwseResize),
            "col-resize" => Ok(CursorIcon::ColResize),
            "row-resize" => Ok(CursorIcon::RowResize),
            "all-scroll" => Ok(CursorIcon::AllScroll),
            "zoom-in" => Ok(CursorIcon::ZoomIn),
            "zoom-out" => Ok(CursorIcon::ZoomOut),
            _ => Err(ParseError { _private: () }),
        }
    }
}

/// An error which could be returned when parsing [`CursorIcon`].
///
/// This occurs when the [`FromStr`] implementation of [`CursorIcon`] fails.
///
/// [`FromStr`]: core::str::FromStr
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ParseError {
    _private: (),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("failed to parse cursor icon")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

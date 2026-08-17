#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::*;

const_ordinary! { ADRF: u32;
	/// [`NMTVASYNCDRAW`](crate::NMTVASYNCDRAW) `dwRetFlags` (`u32`).
	///
	/// Don't seem to be defined anywhere; unconfirmed values.
	=>
	=>
	DRAWSYNC 0
	DRAWNOTHING 1
	DRAWFALLBACK 2
	DRAWIMAGE 3
}

const_wm! { BCM;
	/// Button control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-messages)
	/// (`u32`) from comctl.
	=>
	FIRST 0x1600
	=>
	GETIDEALSIZE Self::FIRST.0 + 0x0001
	SETIMAGELIST Self::FIRST.0 + 0x0002
	GETIMAGELIST Self::FIRST.0 + 0x0003
	SETTEXTMARGIN Self::FIRST.0 + 0x0004
	GETTEXTMARGIN Self::FIRST.0 + 0x0005
	SETDROPDOWNSTATE Self::FIRST.0 + 0x0006
	SETSPLITINFO Self::FIRST.0 + 0x0007
	GETSPLITINFO Self::FIRST.0 + 0x0008
	SETNOTE Self::FIRST.0 + 0x0009
	GETNOTE Self::FIRST.0 + 0x000a
	GETNOTELENGTH Self::FIRST.0 + 0x000b
	SETSHIELD Self::FIRST.0 + 0x000c
}

const_nm! { BCN;
	/// Button control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -1250
	=>
	HOTITEMCHANGE Self::FIRST.0 + 0x0001
	DROPDOWN Self::FIRST.0 + 0x0002
}

const_bitflag! { BCSIF: u32;
	/// [`BUTTON_SPLITINFO`](crate::BUTTON_SPLITINFO) `mask` (`u32`).
	=>
	=>
	GLYPH 0x0001
	IMAGE 0x0002
	STYLE 0x0004
	SIZE 0x0008
}

const_bitflag! { BCSS: u32;
	/// [`BUTTON_SPLITINFO`](crate::BUTTON_SPLITINFO) `uSplitStyle` (`u32`).
	=>
	=>
	NOSPLIT 0x0001
	STRETCH 0x0002
	ALIGNLEFT 0x0004
	IMAGE 0x0008
}

const_ordinary! { BIA: u32;
	/// [`BUTTON_IMAGELIST`](crate::BUTTON_IMAGELIST) `uAlign` (`u32`).
	///
	/// Originally has `BUTTON_IMAGELIST_ALIGN_` prefix.
	=>
	=>
	LEFT 0
	RIGHT 1
	TOP 2
	BOTTOM 3
	CENTER 4
}

const_ws! { BTNS: u8;
	/// Toolbar control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/toolbar-control-and-button-styles)
	/// (`u8`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	BUTTON 0x00
	SEP 0x01
	CHECK 0x02
	GROUP 0x04
	CHECKGROUP Self::GROUP.0 | Self::CHECK.0
	DROPDOWN 0x08
	AUTOSIZE 0x10
	NOPREFIX 0x20
	SHOWTEXT 0x40
	WHOLEDROPDOWN 0x80
}

const_ordinary! { CAL: u32;
	/// [`MCM_GETCALID`](https://learn.microsoft.com/en-us/windows/win32/controls/mcm-getcalid)
	/// return type calendar IDs (`u32`).
	=>
	=>
	/// Gregorian (localized) calendar.
	GREGORIAN 1
	/// Gregorian (U.S.) calendar.
	GREGORIAN_US 2
	/// Japanese Emperor Era calendar.
	JAPAN 3
	/// Taiwan calendar.
	TAIWAN 4
	/// Korean Tangun Era calendar.
	KOREA 5
	/// Hijri (Arabic Lunar) calendar.
	HIJRI 6
	/// Thai calendar.
	THAI 7
	/// Hebrew (Lunar) calendar.
	HEBREW 8
	/// Gregorian Middle East French calendar.
	GREGORIAN_ME_FRENCH 9
	/// Gregorian Arabic calendar.
	GREGORIAN_ARABIC 10
	/// Gregorian Transliterated English calendar
	GREGORIAN_XLIT_ENGLISH 11
	/// Gregorian Transliterated French calendar.
	GREGORIAN_XLIT_FRENCH 12
	/// Persian (Solar Hijri) calendar.
	PERSIAN 22
	/// UmAlQura Hijri (Arabic Lunar) calendar.
	UMALQURA 23
}

const_values! { CB;
	FIRST 0x1700
	=>
	SETMINVISIBLE Self::FIRST.raw() + 1
	GETMINVISIBLE Self::FIRST.raw() + 2
	SETCUEBANNER Self::FIRST.raw() + 3
	GETCUEBANNER Self::FIRST.raw() + 4
}

const_wm! { CBEM;
	/// ComboBoxEx control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-comboboxex-control-reference-messages)
	/// (`u32`).
	=>
	=>
	SETIMAGELIST WM::USER.raw() + 2
	GETIMAGELIST WM::USER.raw() + 3
	DELETEITEM CB::DELETESTRING.raw()
	GETCOMBOCONTROL WM::USER.raw() + 6
	GETEDITCONTROL WM::USER.raw() + 7
	SETEXTENDEDSTYLE WM::USER.raw() + 14
	GETEXTENDEDSTYLE WM::USER.raw() + 9
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
	HASEDITCHANGED WM::USER.raw() + 10
	INSERTITEM WM::USER.raw() + 11
	SETITEM WM::USER.raw() + 12
	GETITEM WM::USER.raw() + 13
}

const_wsex! { CBES_EX;
	/// Extended combo box
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/comboboxex-control-extended-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	NOEDITIMAGE 0x0000_0001
	NOEDITIMAGEINDENT 0x0000_0002
	PATHWORDBREAKPROC 0x0000_0004
	NOSIZELIMIT 0x0000_0008
	CASESENSITIVE 0x0000_0010
	TEXTENDELLIPSIS 0x0000_0020
}

const_wm! { CCM;
	/// Generic common controls
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/common-controls-intro)
	/// (`u32`).
	=>
	FIRST 0x2000
	=>
	SETBKCOLOR Self::FIRST.0 + 1
	SETCOLORSCHEME Self::FIRST.0 + 2
	GETCOLORSCHEME Self::FIRST.0 + 3
	GETDROPTARGET Self::FIRST.0 + 4
	SETUNICODEFORMAT Self::FIRST.0 + 5
	GETUNICODEFORMAT Self::FIRST.0 + 6
	SETVERSION Self::FIRST.0 + 0x7
	GETVERSION Self::FIRST.0 + 0x8
	SETNOTIFYWINDOW Self::FIRST.0 + 0x9
	SETWINDOWTHEME Self::FIRST.0 + 0xb
	DPISCALE Self::FIRST.0 + 0xc
}

const_bitflag! { CDDS: u32;
	/// [`NMCUSTOMDRAW`](crate::NMCUSTOMDRAW) `dwDrawStage` (`u32`).
	=>
	=>
	PREPAINT 0x0000_0001
	POSTPAINT 0x0000_0002
	PREERASE 0x0000_0003
	POSTERASE 0x0000_0004
	ITEM 0x0001_0000
	ITEMPREPAINT Self::ITEM.0 | Self::PREPAINT.0
	ITEMPOSTPAINT Self::ITEM.0 | Self::POSTPAINT.0
	ITEMPREERASE Self::ITEM.0 | Self::PREERASE.0
	ITEMPOSTERASE Self::ITEM.0 | Self::POSTERASE.0
	SUBITEM 0x0002_0000
}

const_bitflag! { CDIS: u32;
	/// [`NMCUSTOMDRAW`](crate::NMCUSTOMDRAW) `uItemState` (`u32`).
	=>
	=>
	SELECTED 0x0001
	GRAYED 0x0002
	DISABLED 0x0004
	CHECKED 0x0008
	FOCUS 0x0010
	DEFAULT 0x0020
	HOT 0x0040
	MARKED 0x0080
	INDETERMINATE 0x0100
	SHOWKEYBOARDCUES 0x0200
	NEARHOT 0x0400
	OTHERSIDEHOT 0x0800
	DROPHILITED 0x1000
}

const_ordinary! { CDRF: u32;
	/// [`NM_CUSTOMDRAW`](https://learn.microsoft.com/en-us/windows/win32/controls/nm-customdraw)
	/// return value (`u32`).
	=>
	=>
	DODEFAULT 0x0000_0000
	NEWFONT 0x0000_0002
	SKIPDEFAULT 0x0000_0004
	DOERASE 0x0000_0008
	SKIPPOSTPAINT 0x0000_0100
	NOTIFYPOSTPAINT 0x0000_0010
	NOTIFYITEMDRAW 0x0000_0020
	NOTIFYSUBITEMDRAW 0x0000_0020
	NOTIFYPOSTERASE 0x0000_0040
}

const_wm! { DTM;
	/// Date and time picker control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-messages)
	/// (`u32`).
	=>
	FIRST 0x1000
	=>
	GETSYSTEMTIME Self::FIRST.0 + 1
	SETSYSTEMTIME Self::FIRST.0 + 2
	GETRANGE Self::FIRST.0 + 3
	SETRANGE Self::FIRST.0 + 4
	SETFORMAT Self::FIRST.0 + 50
	SETMCCOLOR Self::FIRST.0 + 6
	GETMCCOLOR Self::FIRST.0 + 7
	GETMONTHCAL Self::FIRST.0 + 8
	SETMCFONT Self::FIRST.0 + 9
	GETMCFONT Self::FIRST.0 + 10
	SETMCSTYLE Self::FIRST.0 + 11
	GETMCSTYLE Self::FIRST.0 + 12
	CLOSEMONTHCAL Self::FIRST.0 + 13
	GETDATETIMEPICKERINFO Self::FIRST.0 + 14
	GETIDEALSIZE Self::FIRST.0 + 15
}

const_nm! { DTN;
	/// Date and time picker control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -740
	FIRST2 -753
	=>
	CLOSEUP Self::FIRST2.0
	DATETIMECHANGE Self::FIRST2.0 - 6
	DROPDOWN Self::FIRST2.0 - 1
	FORMAT Self::FIRST2.0 - 3
	FORMATQUERY Self::FIRST.0 - 3
	USERSTRING Self::FIRST.0 - 5
	WMKEYDOWN Self::FIRST.0 - 4
}

const_ws! { DTS: u32;
	/// Date and time picker control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/date-and-time-picker-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	UPDOWN 0x0001
	SHOWNONE 0x0002
	SHORTDATEFORMAT 0x0000
	LONGDATEFORMAT 0x0004
	SHORTDATECENTURYFORMAT 0x000c
	TIMEFORMAT 0x0009
	APPCANPARSE 0x0010
	RIGHTALIGN 0x0020
}

const_values! { EM;
	FIRST 0x1500
	=>
	SETCUEBANNER Self::FIRST.raw() + 1
	GETCUEBANNER Self::FIRST.raw() + 2
	SHOWBALLOONTIP Self::FIRST.raw() + 3
	HIDEBALLOONTIP Self::FIRST.raw() + 4
	SETHILITE Self::FIRST.raw() + 5
	GETHILITE Self::FIRST.raw() + 6
	NOSETFOCUS Self::FIRST.raw() + 7
	TAKEFOCUS Self::FIRST.raw() + 8
	SETEXTENDEDSTYLE Self::FIRST.raw() + 10
	GETEXTENDEDSTYLE Self::FIRST.raw() + 11
	SETENDOFLINE Self::FIRST.raw() + 12
	GETENDOFLINE Self::FIRST.raw() + 13
	ENABLESEARCHWEB Self::FIRST.raw() + 14
	SEARCHWEB Self::FIRST.raw() + 15
	SETCARETINDEX Self::FIRST.raw() + 17
	GETCARETINDEX Self::FIRST.raw() + 18
	GETZOOM WM::USER.raw() + 224
	SETZOOM WM::USER.raw() + 225
	FILELINEFROMCHAR Self::FIRST.raw() + 19
	FILELINEINDEX Self::FIRST.raw() + 20
	FILELINELENGTH Self::FIRST.raw() + 21
	GETFILELINE Self::FIRST.raw() + 22
	GETFILELINECOUNT Self::FIRST.raw() + 23
}

const_bitflag! { EMF: u32;
	/// [`NMLVEMPTYMARKUP`](crate::NMLVEMPTYMARKUP) `dwFlags` (`u32`).
	=>
	=>
	LEFT 0x0000_0000
	CENTERED 0x0000_0001
}

const_bitflag! { GDT: u32;
	/// [`NMDATETIMECHANGE`](crate::NMDATETIMECHANGE) and
	/// [`NMDATETIMESTRING`](crate::NMDATETIMESTRING) `dwFlags` (`u32`).
	=>
	=>
	VALID 0
	NONE 1
}

const_ordinary! { GDTR: u32;
	/// [`dtm::GetRange`](crate::msg::dtm::GetRange) return value (`u32`).
	=>
	=>
	MIN 0x0001
	MAX 0x0002
}

const_ordinary! { GMR: u32;
	=>
	=>
	VISIBLE 0
	DAYSTATE 1
}

const_bitflag! { HICF: u32;
	/// [NMBCHOTITEM](crate::NMBCHOTITEM) `dwFlags` (`u32`).
	=>
	=>
	OTHER 0x0000_0000
	ARROWKEYS 0x0000_0002
	ACCELERATOR 0x0000_0004
	DUPACCEL 0x0000_0008
	ENTERING 0x0000_0010
	LEAVING 0x0000_0020
	RESELECT 0x0000_0040
	LMOUSE 0x0000_0080
	TOGGLEDROPDOWN 0x0000_0100
}

const_bitflag! { HDF: i32;
	/// [`HDITEM`](crate::HDITEM) `fmt` (`i32`).
	=>
	=>
	LEFT 0x0000
	RIGHT 0x0001
	CENTER 0x0002
	JUSTIFYMASK 0x0003
	RTLREADING 0x0004

	BITMAP 0x2000
	STRING 0x4000
	OWNERDRAW 0x8000
	IMAGE 0x0800
	BITMAP_ON_RIGHT 0x1000

	SORTUP 0x0400
	SORTDOWN 0x0200

	CHECKBOX 0x0040
	CHECKED 0x0080
	FIXEDWIDTH 0x0100
	SPLITBUTTON 0x100_0000
}

const_ordinary! { HDFT: u32;
	/// [`HDITEM`](crate::HDITEM) `typeFilter` (`i32`).
	=>
	=>
	ISSTRING 0x0000
	ISNUMBER 0x0001
	ISDATE 0x0002
	HASNOVALUE 0x8000
}

const_bitflag! { HDI: i32;
	/// [`HDITEM`](crate::HDITEM) `mask` (`i32`).
	=>
	=>
	WIDTH 0x0001
	HEIGHT Self::WIDTH.0
	TEXT 0x0002
	FORMAT 0x0004
	LPARAM 0x0008
	BITMAP 0x0010
	IMAGE 0x0020
	DI_SETITEM 0x0040
	ORDER 0x0080
	FILTER 0x0100
	STATE 0x0200
}

const_ordinary! { HDIS: u32;
	/// [`HDITEM`](crate::HDITEM) `state` (`i32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	FOCUSED 0x0000_0001
}

const_wm! { HDM;
	/// Header control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-messages)
	/// (`u32`).
	=>
	FIRST 0x1200
	=>
	GETITEMCOUNT Self::FIRST.0 + 0
	INSERTITEM Self::FIRST.0 + 10
	DELETEITEM Self::FIRST.0 + 11
	GETITEM Self::FIRST.0 + 11
	SETITEM Self::FIRST.0 + 12
	LAYOUT Self::FIRST.0 + 5
	HITTEST Self::FIRST.0 + 6
	GETITEMRECT Self::FIRST.0 + 7
	SETIMAGELIST Self::FIRST.0 + 8
	GETIMAGELIST Self::FIRST.0 + 9
	ORDERTOINDEX Self::FIRST.0 + 15
	CREATEDRAGIMAGE Self::FIRST.0 + 16
	GETORDERARRAY Self::FIRST.0 + 17
	SETORDERARRAY Self::FIRST.0 + 18
	SETHOTDIVIDER Self::FIRST.0 + 19
	SETBITMAPMARGIN Self::FIRST.0 + 20
	GETBITMAPMARGIN Self::FIRST.0 + 21
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
	SETFILTERCHANGETIMEOUT Self::FIRST.0 + 22
	EDITFILTER Self::FIRST.0 + 23
	CLEARFILTER Self::FIRST.0 + 24
	GETITEMDROPDOWNRECT Self::FIRST.0 + 25
	GETOVERFLOWRECT Self::FIRST.0 + 26
	GETFOCUSEDITEM Self::FIRST.0 + 27
	SETFOCUSEDITEM Self::FIRST.0 + 28
}

const_nm! { HDN;
	/// Header control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -300
	=>
	ITEMCHANGING Self::FIRST.0 - 20
	ITEMCHANGED Self::FIRST.0 - 21
	ITEMCLICK Self::FIRST.0 - 22
	ITEMDBLCLICK Self::FIRST.0 - 23
	DIVIDERDBLCLICK Self::FIRST.0 - 25
	BEGINTRACK Self::FIRST.0 - 26
	ENDTRACK Self::FIRST.0 - 27
	TRACK Self::FIRST.0 - 28
	GETDISPINFO Self::FIRST.0 - 29
	BEGINDRAG Self::FIRST.0 - 10
	ENDDRAG Self::FIRST.0 - 11
	FILTERCHANGE Self::FIRST.0 - 12
	FILTERBTNCLICK Self::FIRST.0 - 13
	BEGINFILTEREDIT Self::FIRST.0 - 14
	ENDFILTEREDIT Self::FIRST.0 - 15
	ITEMSTATEICONCLICK Self::FIRST.0 - 16
	ITEMKEYDOWN Self::FIRST.0 - 17
	DROPDOWN Self::FIRST.0 - 18
	OVERFLOWCLICK Self::FIRST.0 - 19
}

const_ws! { HDS: u32;
	/// Header control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/header-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	HORZ 0x0000
	BUTTONS 0x0002
	HOTTRACK 0x0004
	HIDDEN 0x0008
	DRAGDROP 0x0040
	FULLDRAG 0x0080
	FILTERBAR 0x0100
	FLAT 0x0200
	CHECKBOXES 0x0400
	NOSIZING 0x0800
	OVERFLOW 0x1000
}

const_ordinary! { HDSIL: u8;
	/// [`hdm::GetImageList`](crate::msg::hdm::GetImageList) and
	/// [`hdm::SetImageList`](crate::msg::hdm::SetImageList) `which` (`u8`).
	=>
	=>
	NORMAL 0
	STATE 1
}

const_bitflag! { HHT: u32;
	/// [`HDHITTESTINFO`](crate::HDHITTESTINFO) `flags` (`u32`).
	=>
	=>
	NOWHERE 0x0001
	ONHEADER 0x0002
	ONDIVIDER 0x0004
	ONDIVOPEN 0x0008
	ONFILTER 0x0010
	ONFILTERBUTTON 0x0020
	ABOVE 0x0100
	BELOW 0x0200
	TORIGHT 0x0400
	TOLEFT 0x0800
	ONITEMSTATEICON 0x1000
	ONDROPDOWN 0x2000
	ONOVERFLOW 0x4000
}

const_bitflag! { ICC: u32;
	/// [`INITCOMMONCONTROLSEX`](crate::INITCOMMONCONTROLSEX) `icc` (`u32`).
	=>
	=>
	/// Load animate control class.
	ANIMATE_CLASS 0x0000_0080
	/// Load toolbar, status bar, trackbar, and tooltip control classes.
	BAR_CLASSES 0x0000_0004
	/// Load rebar control class.
	COOL_CLASSES 0x0000_0400
	/// Load date and time picker control class.
	DATE_CLASSES 0x0000_0100
	/// Load hot key control class.
	HOTKEY_CLASS 0x0000_0040
	/// Load IP address class.
	INTERNET_CLASSES 0x0000_0800
	/// Load a hyperlink control class.
	LINK_CLASS 0x0000_8000
	/// Load list-view and header control classes.
	LISTVIEW_CLASSES 0x0000_0001
	/// Load a native font control class.
	NATIVEFNTCTL_CLASS 0x0000_2000
	/// Load pager control class.
	PAGESCROLLER_CLASS 0x0000_1000
	/// Load progress bar control class.
	PROGRESS_CLASS 0x0000_0020
	/// Load one of the intrinsic User32 control classes. The user controls
	/// include button, edit, static, listbox, combobox, and scroll bar.
	STANDARD_CLASSES 0x0000_4000
	/// Load tab and tooltip control classes.
	TAB_CLASSES 0x0000_0008
	/// Load tree-view and tooltip control classes.
	TREEVIEW_CLASSES 0x0000_0002
	/// Load up-down control class.
	UPDOWN_CLASS 0x0000_0010
	/// Load ComboBoxEx class.
	USEREX_CLASSES 0x0000_0200
	/// Load animate control, header, hot key, list-view, progress bar, status
	/// bar, tab, tooltip, toolbar, trackbar, tree-view, and up-down control
	/// classes.
	WIN95_CLASSES 0x0000_00ff
}

const_ordinary! { IDB: usize;
	/// [`TBADDBITMAP`](crate::TBADDBITMAP) `nID` (`usize`).
	=>
	=>
	STD_SMALL_COLOR 0
	STD_LARGE_COLOR 1
	VIEW_SMALL_COLOR 4
	VIEW_LARGE_COLOR 5
	HIST_SMALL_COLOR 8
	HIST_LARGE_COLOR 9
	HIST_NORMAL 12
	HIST_HOT 13
	HIST_DISABLED 14
	HIST_PRESSED 15
}

const_bitflag! { ILC: u32;
	/// [`HIMAGELIST::Create`](crate::prelude::comctl_Himagelist::Create)
	/// `flags` (`u32`).
	=>
	=>
	/// Use a mask. The image list contains two bitmaps one of which is a
	/// monochrome bitmap used as a mask. If this value is not included the
	/// image list contains only one bitmap.
	MASK 0x0000_0001
	/// Use the default behavior if none of the other `ILC::COLORx` flags is
	/// specified. Typically the default is `ILC::COLOR4` but for older
	/// display drivers the default is `ILC::COLORDDB`.
	COLOR 0x0000_0000
	/// Use a device-dependent bitmap.
	COLORDDB 0x0000_00fe
	/// Use a 4-bit (16-color) device-independent bitmap (DIB) section as the
	/// bitmap for the image list.
	COLOR4 0x0000_0004
	/// Use an 8-bit DIB section. The colors used for the color table are the
	/// same colors as the halftone palette.
	COLOR8 0x0000_0008
	/// Use a 16-bit (32/64k-color) DIB section.
	COLOR16 0x0000_0010
	/// Use a 24-bit DIB section.
	COLOR24 0x0000_0018
	/// Use a 32-bit DIB section.
	COLOR32 0x0000_0020
	/// Mirror the icons contained if the process is mirrored.
	MIRROR 0x0000_2000
	/// Causes the mirroring code to mirror each item when inserting a set of
	/// images versus the whole strip.
	PERITEMMIRROR 0x0000_8000
	/// Windows Vista and later. Imagelist should accept smaller than set images
	/// and apply original size based on image added.
	ORIGINALSIZE 0x0001_0000
}

const_bitflag! { ILD: u32;
	/// [`IMAGELISTDRAWFLAGS`](https://learn.microsoft.com/en-us/windows/win32/controls/imagelistdrawflags)
	/// enumeration (`u32`).
	=>
	=>
	NORMAL 0x0000_0000
	TRANSPARENT 0x0000_0001
	MASK 0x0000_0010
	IMAGE 0x0000_0020
	ROP 0x0000_0040
	BLEND25 0x0000_0002
	BLEND50 0x0000_0004
	OVERLAYMASK 0x0000_0f00
	PRESERVEALPHA 0x0000_1000
	SCALE 0x0000_2000
	DPISCALE 0x0000_4000
	ASYNC 0x0000_8000
	SELECTED Self::BLEND50.0
	FOCUS Self::BLEND25.0
	BLEND Self::BLEND50.0
}

const_bitflag! { ILS: u32;
	/// [`IMAGELISTSTATEFLAGS`](https://learn.microsoft.com/en-us/windows/win32/controls/imageliststateflags)
	/// enumeration (`u32`).
	=>
	=>
	NORMAL 0x0000_0000
	GLOW 0x0000_0001
	SHADOW 0x0000_0002
	SATURATE 0x0000_0004
	ALPHA 0x0000_0008
}

const_wm! { IPM;
	/// IP address control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-ip-address-control-reference-messages)
	/// (`u32`).
	=>
	=>
	CLEARADDRESS WM::USER.raw() + 100
	SETADDRESS WM::USER.raw() + 101
	GETADDRESS WM::USER.raw() + 102
	SETRANGE WM::USER.raw() + 103
	SETFOCUS WM::USER.raw() + 104
	ISBLANK WM::USER.raw() + 105
}

const_nm! { IPN;
	/// IP address control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-ip-address-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -860
	=>
	FIELDCHANGED Self::FIRST.0 - 0
}

const_bitflag! { LIF: u32;
	/// [`LITEM`](crate::LITEM) `mask` (`u32`).
	=>
	=>
	ITEMINDEX 0x0000_0001
	STATE 0x0000_0002
	ITEMID 0x0000_0004
	URL 0x0000_0008
}

const_bitflag! { LIS: u32;
	/// [`LITEM`](crate::LITEM) `state` (`u32`).
	=>
	=>
	FOCUSED 0x0000_0001
	ENABLED 0x0000_0002
	VISITED 0x0000_0004
	HOTTRACK 0x0000_0008
	DEFAULTCOLORS 0x0000_0010
}

const_wm! { LM;
	/// SysLink control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-syslink-control-reference-messages)
	/// (`u32`).
	=>
	=>
	HITTEST WM::USER.raw() + 0x300
	GETIDEALHEIGHT WM::USER.raw() + 0x301
	SETITEM WM::USER.raw() + 0x302
	GETITEM WM::USER.raw() + 0x303
	GETIDEALSIZE Self::GETIDEALHEIGHT.0
}

const_bitflag! { LVKF: u32;
	/// [`NMITEMACTIVATE`](crate::NMITEMACTIVATE) `uKeyFlags` (`u32`).
	=>
	=>
	ALT 0x0001
	CONTROL 0x0002
	SHIFT 0x0004
}

const_ordinary! { LV_VIEW: u32;
	/// ListView
	/// [views](https://learn.microsoft.com/en-us/windows/win32/controls/list-view-controls-overview)
	/// (`u32`).
	=>
	=>
	ICON 0x0000
	DETAILS 0x0001
	SMALLICON 0x0002
	LIST 0x0003
	TILE 0x0004
}

const_ordinary! { LVA: u16;
	/// [`lvm::Arrange`](crate::msg::lvm::Arrange) arrangement (`u16`).
	=>
	=>
	DEFAULT 0x0000
	SNAPTOGRID 0x0005
}

const_bitflag! { LVBKIF: u32;
	/// [`LVBKIMAGE`](crate::LVBKIMAGE) `uFlags` (`u32`).
	=>
	=>
	SOURCE_NONE 0x0000_0000
	SOURCE_HBITMAP 0x0000_0001
	SOURCE_URL 0x0000_0002
	SOURCE_MASK 0x0000_0003
	STYLE_NORMAL 0x0000_0000
	STYLE_TILE 0x0000_0010
	STYLE_MASK 0x0000_0010
	FLAG_TILEOFFSET 0x0000_0100
	TYPE_WATERMARK 0x1000_0000
	FLAG_ALPHABLEND 0x2000_0000
}

const_ordinary! { LVCDI: u32;
	/// [`NMLVCUSTOMDRAW`](crate::NMLVCUSTOMDRAW) `dwItemType` (`u32`).
	=>
	=>
	ITEM 0x0000_0000
	GROUP 0x0000_0001
	ITEMSLIST 0x0000_0002
}

const_bitflag! { LVCF: u32;
	/// [`LVCOLUMN`](crate::LVCOLUMN) `mask` (`u32`).
	=>
	=>
	DEFAULTWIDTH 0x0080
	FMT 0x0001
	IDEALWIDTH 0x0100
	IMAGE 0x0010
	MINWIDTH 0x0040
	ORDER 0x0020
	SUBITEM 0x0008
	TEXT 0x0004
	WIDTH 0x0002
}

const_bitflag! { LVCFMT_C: i32;
	/// [`LVCOLUMN`](crate::LVCOLUMN) `mask` (`i32`).
	=>
	=>
	LEFT 0x0000
	RIGHT 0x0001
	CENTER 0x0002
	JUSTIFYMASK 0x0003
	IMAGE 0x0800
	BITMAP_ON_RIGHT 0x1000
	COL_HAS_IMAGES 0x8000
	FIXED_WIDTH 0x0_0100
	NO_DPI_SCALE 0x4_0000
	FIXED_RATIO 0x8_0000
	SPLITBUTTON 0x100_0000
}

const_bitflag! { LVCFMT_I: i32;
	/// [`LVITEM`](crate::LVITEM) `piColFmt` (`i32`).
	=>
	=>
	LINE_BREAK 0x10_0000
	FILL 0x20_0000
	WRAP 0x40_0000
	NO_TITLE 0x80_0000
	TILE_PLACEMENTMASK Self::LINE_BREAK.0 | Self::FILL.0
}

const_ordinary! { LVFF: u32;
	/// [`LVFOOTERINFO`](crate::LVFOOTERINFO) `mask` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	ITEMCOUNT 0x0001
}

const_bitflag! { LVFI: u32;
	/// [`LVFINDINFO`](crate::LVFINDINFO) `flags` (`u32`).
	=>
	=>
	PARAM 0x0001
	STRING 0x0002
	SUBSTRING 0x0004
	PARTIAL 0x0008
	WRAP 0x0020
	NEARESTXY 0x0040
}

const_ordinary! { LVFIF: u32;
	/// [`LVFOOTERITEM`](crate::LVFOOTERITEM) `mask` (`u32`).
	=>
	=>
	TEXT 0x0001
	STATE 0x0002
}

const_ordinary! { LVFIS: u32;
	/// [`LVFOOTERITEM`](crate::LVFOOTERITEM) `state` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	FOCUSED 0x0001
}

const_bitflag! { LVGA_FH: u32;
	/// [`LVGROUP`](crate::LVGROUP) `uAlign` (`u32`).
	///
	/// These constants are composed of both
	/// [`LVGA_HEADER`](crate::co::LVGA_HEADER) and `LVGA_FOOTER`.
	=>
	=>
	FOOTER_LEFT 0x0000_0008
	FOOTER_CENTER 0x0000_0010
	FOOTER_RIGHT 0x0000_0020
	HEADER_LEFT LVGA_HEADER::LEFT.0
	HEADER_CENTER LVGA_HEADER::CENTER.0
	HEADER_RIGHT LVGA_HEADER::RIGHT.0
}

const_ordinary! { LVGA_HEADER: u32;
	/// [`NMLVCUSTOMDRAW`](crate::NMLVCUSTOMDRAW) `uAlign` (`u32`).
	=>
	=>
	LEFT 0x0000_0001
	CENTER 0x0000_0002
	RIGHT 0x0000_0004
}

const_bitflag! { LVGF: u32;
	/// [`LVGROUP`](crate::LVGROUP) `mask` (`u32`).
	=>
	=>
	NONE 0x0000_0000
	HEADER 0x0000_0001
	FOOTER 0x0000_0002
	STATE 0x0000_0004
	ALIGN 0x0000_0008
	GROUPID 0x0000_0010
	SUBTITLE 0x0000_0100
	TASK 0x0000_0200
	DESCRIPTIONTOP 0x0000_0400
	DESCRIPTIONBOTTOM 0x0000_0800
	TITLEIMAGE 0x0000_1000
	EXTENDEDIMAGE 0x0000_2000
	ITEMS 0x0000_4000
	SUBSET 0x0000_8000
	SUBSETITEMS 0x0001_0000
}

const_ordinary! { LVGGR: i32;
	/// [`lvm::GetGroupRect`](crate::msg::lvm::GetGroupRect) `flags` (`i32`).
	=>
	=>
	GROUP 0
	HEADER 1
	LABEL 2
	SUBSETLINK 3
}

const_ordinary! { LVGIT: u32;
	/// [`NMLVGETINFOTIP`](crate::NMLVGETINFOTIP) `dwFlags` (`u32`).
	=>
	=>
	FOLDED 0x0000
	UNFOLDED 0x0001
}

const_bitflag! { LVGMF: u32;
	/// [`LVGROUPMETRICS`](crate::LVGROUPMETRICS) `mask` (`u32`).
	=>
	=>
	NONE 0x0000_0000
	BORDERSIZE 0x0000_0001
	BORDERCOLOR 0x0000_0002
	TEXTCOLOR 0x0000_0004
}

const_bitflag! { LVGS: u32;
	/// [`LVGROUP`](crate::LVGROUP) `state` (`u32`).
	=>
	=>
	NORMAL 0x0000_0000
	COLLAPSED 0x0000_0001
	HIDDEN 0x0000_0002
	NOHEADER 0x0000_0004
	COLLAPSIBLE 0x0000_0008
	FOCUSED 0x0000_0010
	SELECTED 0x0000_0020
	SUBSETED 0x0000_0040
	SUBSETLINKFOCUSED 0x0000_0080
}

const_bitflag! { LVHT: u32;
	/// [`LVHITTESTINFO`](crate::LVHITTESTINFO) `flags` (`u32`).
	=>
	=>
	NOWHERE 0x0000_0001
	ONITEMICON 0x0000_0002
	ONITEMLABEL 0x0000_0004
	ONITEMSTATEICON 0x0000_0008
	ONITEM Self::ONITEMICON.0 | Self::ONITEMLABEL.0 | Self::ONITEMSTATEICON.0
	ABOVE 0x0000_0008
	BELOW 0x0000_0010
	TORIGHT 0x0000_0020
	TOLEFT 0x0000_0040

	EX_GROUP_HEADER 0x1000_0000
	EX_GROUP_FOOTER 0x2000_0000
	EX_GROUP_COLLAPSE 0x4000_0000
	EX_GROUP_BACKGROUND 0x8000_0000
	EX_GROUP_STATEICON 0x0100_0000
	EX_GROUP_SUBSETLINK 0x0200_0000
	EX_GROUP Self::EX_GROUP_BACKGROUND.0 | Self::EX_GROUP_COLLAPSE.0 | Self::EX_GROUP_FOOTER.0 | Self::EX_GROUP_HEADER.0 | Self::EX_GROUP_STATEICON.0 | Self::EX_GROUP_SUBSETLINK.0
	EX_ONCONTENTS 0x0400_0000
	EX_FOOTER 0x0800_0000
}

const_ordinary! { LVI_GROUPID: i32;
	/// [`LVITEM`](crate::LVITEM) `iGroupId` (`i32`).
	=>
	=>
	I_GROUPIDCALLBACK -1
	I_GROUPIDNONE -2
}

const_bitflag! { LVIF: u32;
	/// [`LVITEM`](crate::LVITEM) `mask` (`u32`).
	=>
	=>
	COLFMT 0x0001_0000
	COLUMNS 0x0000_0200
	GROUPID 0x0000_0100
	IMAGE 0x0000_0002
	INDENT 0x0000_0010
	NORECOMPUTE 0x0000_0800
	PARAM 0x0000_0004
	STATE 0x0000_0008
	TEXT 0x0000_0001
}

const_ordinary! { LVIM: u32;
	/// [`LVINSERTMARK`](crate::LVINSERTMARK) `dwFlags` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	AFTER 0x0000_0001
}

const_ordinary! { LVIR: u8;
	/// [`lvm::GetItemRect`](crate::msg::lvm::GetItemRect) `portion` (`u8`).
	=>
	=>
	BOUNDS 0
	ICON 1
	LABEL 2
	SELECTBOUNDS 3
}

const_bitflag! { LVIS: u32;
	/// ListView item
	/// [states](https://learn.microsoft.com/en-us/windows/win32/controls/list-view-item-states)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	FOCUSED 0x0001
	SELECTED 0x0002
	CUT 0x0004
	DROPHILITED 0x0008
	GLOW 0x0010
	ACTIVATING 0x0020
	OVERLAYMASK 0x0f00
	STATEIMAGEMASK 0xf000
}

const_ordinary! { LVSIL: u8;
	/// [`lvm::GetImageList`](crate::msg::lvm::GetImageList) `kind` (`u8`).
	=>
	=>
	NORMAL 0
	SMALL 1
	STATE 2
	GROUPHEADER 3
}

const_wm! { LVM;
	/// List view control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-messages)
	/// (`u32`).
	=>
	FIRST 0x1000
	=>
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
	GETBKCOLOR Self::FIRST.0 + 0
	SETBKCOLOR Self::FIRST.0 + 1
	GETIMAGELIST Self::FIRST.0 + 2
	SETIMAGELIST Self::FIRST.0 + 3
	GETITEMCOUNT Self::FIRST.0 + 4
	DELETEITEM Self::FIRST.0 + 8
	DELETEALLITEMS Self::FIRST.0 + 9
	GETCALLBACKMASK Self::FIRST.0 + 10
	SETCALLBACKMASK Self::FIRST.0 + 11
	GETNEXTITEM Self::FIRST.0 + 12
	GETITEMRECT Self::FIRST.0 + 14
	SETITEMPOSITION Self::FIRST.0 + 15
	GETITEMPOSITION Self::FIRST.0 + 16
	HITTEST Self::FIRST.0 + 18
	ENSUREVISIBLE Self::FIRST.0 + 19
	SCROLL Self::FIRST.0 + 20
	REDRAWITEMS Self::FIRST.0 + 21
	ARRANGE Self::FIRST.0 + 22
	GETEDITCONTROL Self::FIRST.0 + 24
	DELETECOLUMN Self::FIRST.0 + 28
	GETCOLUMNWIDTH Self::FIRST.0 + 29
	SETCOLUMNWIDTH Self::FIRST.0 + 30
	GETHEADER Self::FIRST.0 + 31
	CREATEDRAGIMAGE Self::FIRST.0 + 33
	GETVIEWRECT Self::FIRST.0 + 34
	GETTEXTCOLOR Self::FIRST.0 + 35
	SETTEXTCOLOR Self::FIRST.0 + 36
	GETTEXTBKCOLOR Self::FIRST.0 + 37
	SETTEXTBKCOLOR Self::FIRST.0 + 38
	GETTOPINDEX Self::FIRST.0 + 39
	GETCOUNTPERPAGE Self::FIRST.0 + 40
	GETORIGIN Self::FIRST.0 + 41
	UPDATE Self::FIRST.0 + 42
	SETITEMSTATE Self::FIRST.0 + 43
	GETITEMSTATE Self::FIRST.0 + 44
	SETITEMCOUNT Self::FIRST.0 + 47
	SORTITEMS Self::FIRST.0 + 48
	SETITEMPOSITION32 Self::FIRST.0 + 49
	GETSELECTEDCOUNT Self::FIRST.0 + 50
	GETITEMSPACING Self::FIRST.0 + 51
	SETICONSPACING Self::FIRST.0 + 53
	SETEXTENDEDLISTVIEWSTYLE Self::FIRST.0 + 54
	GETEXTENDEDLISTVIEWSTYLE Self::FIRST.0 + 55
	GETSUBITEMRECT Self::FIRST.0 + 56
	SUBITEMHITTEST Self::FIRST.0 + 57
	SETCOLUMNORDERARRAY Self::FIRST.0 + 58
	GETCOLUMNORDERARRAY Self::FIRST.0 + 59
	SETHOTITEM Self::FIRST.0 + 60
	GETHOTITEM Self::FIRST.0 + 61
	SETHOTCURSOR Self::FIRST.0 + 62
	GETHOTCURSOR Self::FIRST.0 + 63
	APPROXIMATEVIEWRECT Self::FIRST.0 + 64
	SETWORKAREAS Self::FIRST.0 + 65
	GETSELECTIONMARK Self::FIRST.0 + 66
	SETSELECTIONMARK Self::FIRST.0 + 67
	GETWORKAREAS Self::FIRST.0 + 70
	SETHOVERTIME Self::FIRST.0 + 71
	GETHOVERTIME Self::FIRST.0 + 72
	GETNUMBEROFWORKAREAS Self::FIRST.0 + 73
	SETTOOLTIPS Self::FIRST.0 + 74
	GETITEM Self::FIRST.0 + 75
	SETITEM Self::FIRST.0 + 76
	INSERTITEM Self::FIRST.0 + 77
	GETTOOLTIPS Self::FIRST.0 + 78
	SORTITEMSEX Self::FIRST.0 + 81
	FINDITEM Self::FIRST.0 + 83
	GETSTRINGWIDTH Self::FIRST.0 + 87
	GETGROUPSTATE Self::FIRST.0 + 92
	GETFOCUSEDGROUP Self::FIRST.0 + 93
	GETCOLUMN Self::FIRST.0 + 95
	SETCOLUMN Self::FIRST.0 + 96
	INSERTCOLUMN Self::FIRST.0 + 97
	GETGROUPRECT Self::FIRST.0 + 98
	GETITEMTEXT Self::FIRST.0 + 115
	SETITEMTEXT Self::FIRST.0 + 116
	GETISEARCHSTRING Self::FIRST.0 + 117
	EDITLABEL Self::FIRST.0 + 118
	SETBKIMAGE Self::FIRST.0 + 138
	GETBKIMAGE Self::FIRST.0 + 139
	SETSELECTEDCOLUMN Self::FIRST.0 + 140
	SETVIEW Self::FIRST.0 + 142
	GETVIEW Self::FIRST.0 + 143
	INSERTGROUP Self::FIRST.0 + 145
	SETGROUPINFO Self::FIRST.0 + 147
	GETGROUPINFO Self::FIRST.0 + 149
	REMOVEGROUP Self::FIRST.0 + 150
	MOVEGROUP Self::FIRST.0 + 151
	GETGROUPCOUNT Self::FIRST.0 + 152
	GETGROUPINFOBYINDEX Self::FIRST.0 + 153
	MOVEITEMTOGROUP Self::FIRST.0 + 154
	SETGROUPMETRICS Self::FIRST.0 + 155
	GETGROUPMETRICS Self::FIRST.0 + 156
	ENABLEGROUPVIEW Self::FIRST.0 + 157
	SORTGROUPS Self::FIRST.0 + 158
	INSERTGROUPSORTED Self::FIRST.0 + 159
	REMOVEALLGROUPS Self::FIRST.0 + 160
	HASGROUP Self::FIRST.0 + 161
	SETTILEVIEWINFO Self::FIRST.0 + 162
	GETTILEVIEWINFO Self::FIRST.0 + 163
	SETTILEINFO Self::FIRST.0 + 164
	GETTILEINFO Self::FIRST.0 + 165
	SETINSERTMARK Self::FIRST.0 + 166
	GETINSERTMARK Self::FIRST.0 + 167
	INSERTMARKHITTEST Self::FIRST.0 + 168
	GETINSERTMARKRECT Self::FIRST.0 + 169
	SETINSERTMARKCOLOR Self::FIRST.0 + 170
	GETINSERTMARKCOLOR Self::FIRST.0 + 171
	SETINFOTIP Self::FIRST.0 + 173
	GETSELECTEDCOLUMN Self::FIRST.0 + 174
	ISGROUPVIEWENABLED Self::FIRST.0 + 175
	GETOUTLINECOLOR Self::FIRST.0 + 176
	SETOUTLINECOLOR Self::FIRST.0 + 177
	CANCELEDITLABEL Self::FIRST.0 + 179
	MAPINDEXTOID Self::FIRST.0 + 180
	MAPIDTOINDEX Self::FIRST.0 + 181
	ISITEMVISIBLE Self::FIRST.0 + 182
	GETEMPTYTEXT Self::FIRST.0 + 204
	GETFOOTERRECT Self::FIRST.0 + 205
	GETFOOTERINFO Self::FIRST.0 + 206
	GETFOOTERITEMRECT Self::FIRST.0 + 207
	GETFOOTERITEM Self::FIRST.0 + 208
	GETITEMINDEXRECT Self::FIRST.0 + 209
	SETITEMINDEXSTATE Self::FIRST.0 + 210
	GETNEXTITEMINDEX Self::FIRST.0 + 211
}

const_nm! { LVN;
	/// List view control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -100
	=>
	ITEMCHANGING Self::FIRST.0 - 0
	ITEMCHANGED Self::FIRST.0 - 1
	INSERTITEM Self::FIRST.0 - 2
	DELETEITEM Self::FIRST.0 - 3
	DELETEALLITEMS Self::FIRST.0 - 4
	BEGINLABELEDIT Self::FIRST.0 - 75
	ENDLABELEDIT Self::FIRST.0 - 76
	COLUMNCLICK Self::FIRST.0 - 8
	BEGINDRAG Self::FIRST.0 - 9
	BEGINRDRAG Self::FIRST.0 - 11
	ODCACHEHINT Self::FIRST.0 - 13
	ODFINDITEM Self::FIRST.0 - 79
	ITEMACTIVATE Self::FIRST.0 - 14
	ODSTATECHANGED Self::FIRST.0 - 15
	HOTTRACK Self::FIRST.0 - 21
	GETDISPINFO Self::FIRST.0 - 77
	SETDISPINFO Self::FIRST.0 - 78
	KEYDOWN Self::FIRST.0 - 55
	MARQUEEBEGIN Self::FIRST.0 - 56
	GETINFOTIP Self::FIRST.0 - 58
	INCREMENTALSEARCH Self::FIRST.0 - 63
	COLUMNDROPDOWN Self::FIRST.0 - 64
	COLUMNOVERFLOWCLICK Self::FIRST.0 - 66
	BEGINSCROLL Self::FIRST.0 - 80
	ENDSCROLL Self::FIRST.0 - 81
	LINKCLICK Self::FIRST.0 - 84
	GETEMPTYMARKUP Self::FIRST.0 - 87
}

const_bitflag! { LVNI: u32;
	/// [`lvm::GetNextItem`](crate::msg::lvm::GetNextItem) relationship (`u32`).
	=>
	=>
	ALL 0x0000
	FOCUSED 0x0001
	SELECTED 0x0002
	CUT 0x0004
	DROPHILITED 0x0008
	VISIBLEORDER 0x0010
	PREVIOUS 0x0020
	VISIBLEONLY 0x0040
	SAMEGROUPONLY 0x0080
	ABOVE 0x0100
	BELOW 0x0200
	TOLEFT 0x0400
	TORIGHT 0x0800
}

const_ws! { LVS: u32;
	/// List view control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/list-view-window-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	ICON 0x0000
	REPORT 0x0001
	SMALLICON 0x0002
	LIST 0x0003
	TYPEMASK 0x0003
	SINGLESEL 0x0004
	SHOWSELALWAYS 0x0008
	SORTASCENDING 0x0010
	SORTDESCENDING 0x0020
	SHAREIMAGELISTS 0x0040
	NOLABELWRAP 0x0080
	AUTOARRANGE 0x0100
	EDITLABELS 0x0200
	OWNERDATA 0x1000
	NOSCROLL 0x2000
	TYPESTYLEMASK 0xfc00
	ALIGNTOP 0x0000
	ALIGNLEFT 0x0800
	ALIGNMASK 0x0c00
	OWNERDRAWFIXED 0x0400
	NOCOLUMNHEADER 0x4000
	NOSORTHEADER 0x8000
}

const_wsex! { LVS_EX;
	/// Extended list view control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/extended-list-view-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	AUTOAUTOARRANGE 0x0100_0000
	AUTOCHECKSELECT 0x0800_0000
	AUTOSIZECOLUMNS 0x1000_0000
	BORDERSELECT 0x0000_8000
	CHECKBOXES 0x0000_0004
	COLUMNOVERFLOW 0x8000_0000
	COLUMNSNAPPOINTS 0x4000_0000
	DOUBLEBUFFER 0x0001_0000
	FLATSB 0x0000_0100
	FULLROWSELECT 0x0000_0020
	GRIDLINES 0x0000_0001
	HEADERDRAGDROP 0x0000_0010
	HEADERINALLVIEWS 0x0200_0000
	HIDELABELS 0x0002_0000
	INFOTIP 0x0000_0400
	JUSTIFYCOLUMNS 0x0020_0000
	LABELTIP 0x0000_4000
	MULTIWORKAREAS 0x0000_2000
	ONECLICKACTIVATE 0x0000_0040
	REGIONAL 0x0000_0200
	SIMPLESELECT 0x0010_0000
	SINGLEROW 0x0004_0000
	SNAPTOGRID 0x0008_0000
	SUBITEMIMAGES 0x0000_0002
	TRACKSELECT 0x0000_0008
	TRANSPARENTBKGND 0x0040_0000
	TRANSPARENTSHADOWTEXT 0x0080_0000
	TWOCLICKACTIVATE 0x0000_0080
	UNDERLINECOLD 0x000_01000
	UNDERLINEHOT 0x0000_0800
}

const_bitflag! { LVSICF: u32;
	/// [`lvm::SetItemCount`](crate::msg::lvm::SetItemCount) `behavior` (`u32`).
	=>
	=>
	NOINVALIDATEALL 0x0000_0001
	NOSCROLL 0x0000_0002
}

const_bitflag! { LVTVIF: u32;
	/// [`LVTILEVIEWINFO`](crate::LVTILEVIEWINFO) `dwFlags` (`u32`).
	=>
	=>
	AUTOSIZE 0x0000_0000
	FIXEDWIDTH 0x0000_0001
	FIXEDHEIGHT 0x0000_0002
	FIXEDSIZE 0x0000_0003
	EXTENDED 0x0000_0004
}

const_bitflag! { LVTVIM: u32;
	/// [`LVTILEVIEWINFO`](crate::LVTILEVIEWINFO) `dwMask` (`u32`).
	=>
	=>
	TILESIZE 0x0000_0001
	COLUMNS 0x0000_0002
	LABELMARGIN 0x0000_0004
}

const_ws! { LWS: u32;
	/// SysLink control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/syslink-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// The background mix mode is transparent.
	TRANSPARENT 0x0001
	/// When the link has keyboard focus and the user presses Enter the
	/// keystroke is ignored by the control and passed to the host dialog box.
	IGNORERETURN 0x0002
	/// If the text contains an ampersand it is treated as a literal character
	/// rather than the prefix to a shortcut key.
	NOPREFIX 0x0004
	/// The link is displayed in the current visual style.
	USEVISUALSTYLE 0x0008
	/// An `NM_CUSTOMTEXT` notification is sent when the control is drawn so
	/// that the application can supply text dynamically.
	USECUSTOMTEXT 0x0010
	/// The text is right-justified.
	RIGHT 0x0020
}

const_bitflag! { MCGIF: u32;
	=>
	=>
	DATE 0x0000_0001
	RECT 0x0000_0002
	NAME 0x0000_0004
}

const_ordinary! { MCGIP: u32;
	/// [`MCGRIDINFO`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-mcgridinfo)
	/// `dwPart` (`u32`).
	=>
	=>
	CALENDARCONTROL 0
	NEXT 1
	PREV 2
	FOOTER 3
	CALENDAR 4
	CALENDARHEADER 5
	CALENDARBODY 6
	CALENDARROW 7
	CALENDARCELL 8
}

const_bitflag! { MCHT: u32;
	/// [`MCHITTESTINFO`](crate::MCHITTESTINFO) `uHit` (`u32`).
	=>
	=>
	TITLE 0x0001_0000
	CALENDAR 0x0002_0000
	TODAYLINK 0x0003_0000
	CALENDARCONTROL 0x0010_0000
	NEXT 0x0100_0000
	PREV 0x0200_0000
	NOWHERE 0x0000_0000
	TITLEBK MCHT::TITLE.0
	TITLEMONTH MCHT::TITLE.0 | 0x0001
	TITLEYEAR MCHT::TITLE.0 | 0x0002
	TITLEBTNNEXT MCHT::TITLE.0 | MCHT::NEXT.0 | 0x0003
	TITLEBTNPREV MCHT::TITLE.0 | MCHT::PREV.0 | 0x0003
	CALENDARBK MCHT::CALENDAR.0
	CALENDARDATE MCHT::CALENDAR.0 | 0x0001
	CALENDARDATENEXT MCHT::CALENDARDATE.0 | MCHT::NEXT.0
	CALENDARDATEPREV MCHT::CALENDARDATE.0 | MCHT::PREV.0
	CALENDARDAY MCHT::CALENDAR.0 | 0x0002
	CALENDARWEEKNUM MCHT::CALENDAR.0 | 0x0003
	CALENDARDATEMIN MCHT::CALENDAR.0 | 0x0004
	CALENDARDATEMAX MCHT::CALENDAR.0 | 0x0005
}

const_wm! { MCM;
	/// Month calendar control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-messages)
	/// (`u32`).
	=>
	FIRST 0x1000
	=>
	GETCURSEL Self::FIRST.0 + 1
	SETCURSEL Self::FIRST.0 + 2
	GETMAXSELCOUNT Self::FIRST.0 + 3
	SETMAXSELCOUNT Self::FIRST.0 + 4
	GETSELRANGE Self::FIRST.0 + 5
	SETSELRANGE Self::FIRST.0 + 6
	GETMONTHRANGE Self::FIRST.0 + 7
	SETDAYSTATE Self::FIRST.0 + 8
	GETMINREQRECT Self::FIRST.0 + 9
	SETCOLOR Self::FIRST.0 + 10
	GETCOLOR Self::FIRST.0 + 11
	SETTODAY Self::FIRST.0 + 12
	GETTODAY Self::FIRST.0 + 13
	HITTEST Self::FIRST.0 + 14
	SETFIRSTDAYOFWEEK Self::FIRST.0 + 15
	GETFIRSTDAYOFWEEK Self::FIRST.0 + 16
	GETRANGE Self::FIRST.0 + 17
	SETRANGE Self::FIRST.0 + 18
	GETMONTHDELTA Self::FIRST.0 + 19
	SETMONTHDELTA Self::FIRST.0 + 20
	GETMAXTODAYWIDTH Self::FIRST.0 + 21
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
	GETCURRENTVIEW Self::FIRST.0 + 22
	GETCALENDARCOUNT Self::FIRST.0 + 23
	GETCALENDARGRIDINFO Self::FIRST.0 + 24
	GETCALID Self::FIRST.0 + 27
	SETCALID Self::FIRST.0 + 28
	SIZERECTTOMIN Self::FIRST.0 + 29
	SETCALENDARBORDER Self::FIRST.0 + 30
	GETCALENDARBORDER Self::FIRST.0 + 31
	SETCURRENTVIEW Self::FIRST.0 + 32
}

const_ordinary! { MCMV: u32;
	/// [`NMVIEWCHANGE`](crate::NMVIEWCHANGE) `dwOldView` and `dwNewView` (`u32`).
	=>
	=>
	MONTH 0
	YEAR 1
	DECADE 2
	CENTURY 3
}

const_nm! { MCN;
	/// Month calendar control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -746
	=>
	SELECT Self::FIRST.0
	GETDAYSTATE Self::FIRST.0 - 1
	SELCHANGE Self::FIRST.0 - 3
	VIEWCHANGE Self::FIRST.0 - 4
}

const_ws! { MCS: u32;
	/// Month calendar control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/month-calendar-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	DAYSTATE 0x0001
	MULTISELECT 0x0002
	WEEKNUMBERS 0x0004
	NOTODAYCIRCLE 0x0008
	NOTODAY 0x0010
	NOTRAILINGDATES 0x0040
	SHORTDAYSOFWEEK 0x0080
	NOSELCHANGEONNAV 0x0100
}

const_ordinary! { MCSC: u8;
	/// [`dtm::GetMcColor`](crate::msg::dtm::GetMcColor) color (`u8`).
	=>
	=>
	BACKGROUND 0
	TEXT 1
	TITLEBK 2
	TITLETEXT 3
	MONTHBK 4
	TRAILINGTEXT 5
}

const_ordinary! { NM: i32;
	/// [`wm::Notify`](crate::msg::wm::Notify) notification codes (`i32`).
	///
	/// **Note:** Control-specific notification codes have their own types,
	/// which are convertible to `NM`.
	=>
	FIRST 0
	=>
	OUTOFMEMORY Self::FIRST.0 - 1
	CLICK Self::FIRST.0 - 2
	DBLCLK Self::FIRST.0 - 3
	RETURN Self::FIRST.0 - 4
	RCLICK Self::FIRST.0 - 5
	RDBLCLK Self::FIRST.0 - 6
	SETFOCUS Self::FIRST.0 - 7
	KILLFOCUS Self::FIRST.0 - 8
	CUSTOMDRAW Self::FIRST.0 - 12
	HOVER Self::FIRST.0 - 13
	NCHITTEST Self::FIRST.0 - 14
	KEYDOWN Self::FIRST.0 - 15
	RELEASEDCAPTURE Self::FIRST.0 - 16
	SETCURSOR Self::FIRST.0 - 17
	CHAR Self::FIRST.0 - 18
	TOOLTIPSCREATED Self::FIRST.0 - 19
	LDOWN Self::FIRST.0 - 20
	RDOWN Self::FIRST.0 - 21
	THEMECHANGED Self::FIRST.0 - 22
}

const_ws! { PBS: u32;
	/// Progress bar control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/progress-bar-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SMOOTH 0x01
	VERTICAL 0x04
	MARQUEE 0x08
	SMOOTHREVERSE 0x10
}

const_wm! { PBM;
	/// Progress bar control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-progress-bar-control-reference-messages)
	/// (`u32`).
	=>
	=>
	SETRANGE WM::USER.raw() + 1
	SETPOS WM::USER.raw() + 2
	DELTAPOS WM::USER.raw() + 3
	SETSTEP WM::USER.raw() + 4
	STEPIT WM::USER.raw() + 5
	SETRANGE32 WM::USER.raw() + 6
	GETRANGE WM::USER.raw() + 7
	GETPOS WM::USER.raw() + 8
	SETBARCOLOR WM::USER.raw() + 9
	SETBKCOLOR CCM::SETBKCOLOR.0
	SETMARQUEE WM::USER.raw() + 10
	GETSTEP WM::USER.raw() + 13
	GETBKCOLOR WM::USER.raw() + 14
	GETBARCOLOR WM::USER.raw() + 15
	SETSTATE WM::USER.raw() + 16
	GETSTATE WM::USER.raw() + 17
}

const_ordinary! { PBST: u32;
	/// Progress bar
	/// [states](https://learn.microsoft.com/en-us/windows/win32/controls/pbm-setstate)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	NORMAL 0x0001
	ERROR 0x0002
	PAUSED 0x0003
}

const_wm! { RB;
	/// Rebar control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-rebar-control-reference-messages)
	/// (`u32`).
	=>
	=>
	DELETEBAND WM::USER.raw() + 2
	GETBARINFO WM::USER.raw() + 3
	SETBARINFO WM::USER.raw() + 4
	SETPARENT WM::USER.raw() + 7
	HITTEST WM::USER.raw() + 8
	GETRECT WM::USER.raw() + 9
	INSERTBAND WM::USER.raw() + 10
	SETBANDINFO WM::USER.raw() + 11
	GETBANDCOUNT WM::USER.raw() + 12
	GETROWCOUNT WM::USER.raw() + 13
	GETROWHEIGHT WM::USER.raw() + 14
	IDTOINDEX WM::USER.raw() + 16
	GETTOOLTIPS WM::USER.raw() + 17
	SETTOOLTIPS WM::USER.raw() + 18
	SETBKCOLOR WM::USER.raw() + 19
	GETBKCOLOR WM::USER.raw() + 20
	SETTEXTCOLOR WM::USER.raw() + 21
	GETTEXTCOLOR WM::USER.raw() + 22
	SIZETORECT WM::USER.raw() + 23
	SETCOLORSCHEME CCM::SETCOLORSCHEME.0
	GETCOLORSCHEME CCM::GETCOLORSCHEME.0
	BEGINDRAG WM::USER.raw() + 24
	ENDDRAG WM::USER.raw() + 25
	DRAGMOVE WM::USER.raw() + 26
	GETBARHEIGHT WM::USER.raw() + 27
	GETBANDINFO WM::USER.raw() + 28
	MINIMIZEBAND WM::USER.raw() + 30
	MAXIMIZEBAND WM::USER.raw() + 31
	GETDROPTARGET CCM::GETDROPTARGET.0
	GETBANDBORDERS WM::USER.raw() + 34
	SHOWBAND WM::USER.raw() + 35
	SETPALETTE WM::USER.raw() + 37
	GETPALETTE WM::USER.raw() + 38
	MOVEBAND WM::USER.raw() + 39
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
	GETBANDMARGINS WM::USER.raw() + 40
	SETWINDOWTHEME CCM::SETWINDOWTHEME.0
	SETEXTENDEDSTYLE WM::USER.raw() + 41
	GETEXTENDEDSTYLE WM::USER.raw() + 42
	PUSHCHEVRON WM::USER.raw() + 43
	SETBANDWIDTH WM::USER.raw() + 44
}

const_nm! { RBN;
	/// Rebar control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-rebar-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -831
	=>
	HEIGHTCHANGE Self::FIRST.0 - 0
	GETOBJECT Self::FIRST.0 - 1
	LAYOUTCHANGED Self::FIRST.0 - 2
	AUTOSIZE Self::FIRST.0 - 3
	BEGINDRAG Self::FIRST.0 - 4
	ENDDRAG Self::FIRST.0 - 5
	DELETINGBAND Self::FIRST.0 - 6
	DELETEDBAND Self::FIRST.0 - 7
	CHILDSIZE Self::FIRST.0 - 8
	CHEVRONPUSHED Self::FIRST.0 - 10
	SPLITTERDRAG Self::FIRST.0 - 11
	MINMAX Self::FIRST.0 - 21
	AUTOBREAK Self::FIRST.0 - 22
}

const_ws! { RBS: u32;
	/// Rebar control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/rebar-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	TOOLTIPS 0x0000_0100
	VARHEIGHT 0x0000_0200
	BANDBORDERS 0x0000_0400
	FIXEDORDER 0x0000_0800
	REGISTERDROP 0x000_01000
	AUTOSIZE 0x0000_2000
	VERTICALGRIPPER 0x0000_4000
	DBLCLKTOGGLE 0x0000_8000
}

const_wm! { SB;
	/// Status bar control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-messages)
	/// (`u32`).
	=>
	=>
	SETTEXT WM::USER.raw() + 11
	GETTEXT WM::USER.raw() + 13
	GETTEXTLENGTH WM::USER.raw() + 12
	SETPARTS WM::USER.raw() + 4
	GETPARTS WM::USER.raw() + 6
	GETBORDERS WM::USER.raw() + 7
	SETMINHEIGHT WM::USER.raw() + 8
	SIMPLE WM::USER.raw() + 9
	GETRECT WM::USER.raw() + 10
	ISSIMPLE WM::USER.raw() + 14
	SETICON WM::USER.raw() + 15
	SETTIPTEXT WM::USER.raw() + 17
	GETTIPTEXT WM::USER.raw() + 19
	GETICON WM::USER.raw() + 20
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
	SETBKCOLOR CCM::SETBKCOLOR.0
}

const_ws! { SBARS: u32;
	/// Status bar control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/status-bar-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SIZEGRIP 0x0100
	TOOLTIPS 0x0800
}

const_nm! { SBN;
	/// Status bar control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-status-bars-reference-notifications)
	/// (`i32`).
	=>
	FIRST -880
	=>
	SIMPLEMODECHANGE Self::FIRST.0 - 0
}

const_ordinary! { SBT: u16;
	/// [`sb::GetText`](crate::msg::sb::GetText),
	/// [`sb::GetTextLength`](crate::msg::sb::GetTextLength) and
	/// [`sb::SetText`](crate::msg::sb::SetText) drawing operation (`u16`).
	=>
	=>
	BORDER 0
	OWNERDRAW 0x1000
	NOBORDERS 0x0100
	POPOUT 0x0200
	RTLREADING 0x0400
	NOTABPARSING 0x0800
}

const_wm! { STM;
	/// Static control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-messages)
	/// (`u32`).
	=>
	=>
	SETICON 0x0170
	GETICON 0x0171
	SETIMAGE 0x0172
	GETIMAGE 0x0173
}

const_ordinary! { TB: i32;
	/// [`NMTRBTHUMBPOSCHANGING`](crate::NMTRBTHUMBPOSCHANGING) `nReason`
	/// (`i32`).
	=>
	=>
	LINEUP 0
	LINEDOWN 1
	PAGEUP 2
	PAGEDOWN 3
	THUMBPOSITION 4
	THUMBTRACK 5
	TOP 6
	BOTTOM 7
	ENDTRACK 8
}

const_bitflag! { TBBF: u32;
	/// [`tbm::GetBitmapFlags`](crate::msg::tbm::GetBitmapFlags) return value
	/// (`u32`).
	=>
	=>
	LARGE 0x0001
}

const_bitflag! { TBIF: u32;
	/// [`TBBUTTONINFO`](crate::TBBUTTONINFO) `dwFlags` (`u32`).
	=>
	=>
	IMAGE 0x0000_0001
	TEXT 0x0000_0002
	STATE 0x0000_0004
	STYLE 0x0000_0008
	LPARAM 0x0000_0010
	COMMAND 0x0000_0020
	SIZE 0x0000_0040
	BYINDEX 0x8000_0000
}

const_ordinary! { TBIMHT: u32;
	/// [`TBINSERTMARK`](crate::TBINSERTMARK) `dwFlags` (`u32`).
	=>
	=>
	/// Originally just a zero, no actual flag definition.
	BEFORE 0x0000_0000
	AFTER 0x0000_0001
	BACKGROUND 0x0000_0002
}

const_wm! { TBM;
	/// Toolbar control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-messages)
	/// (`u32`).
	///
	/// Originally has `TB` prefix.
	=>
	=>
	ADDBITMAP WM::USER.raw() + 19
	ADDBUTTONS WM::USER.raw() + 68
	ADDSTRING WM::USER.raw() + 77
	AUTOSIZE WM::USER.raw() + 33
	BUTTONCOUNT WM::USER.raw() + 24
	BUTTONSTRUCTSIZE WM::USER.raw() + 30
	CHANGEBITMAP WM::USER.raw() + 43
	CHECKBUTTON WM::USER.raw() + 2
	COMMANDTOINDEX WM::USER.raw() + 25
	CUSTOMIZE WM::USER.raw() + 27
	DELETEBUTTON WM::USER.raw() + 22
	ENABLEBUTTON WM::USER.raw() + 1
	GETANCHORHIGHLIGHT WM::USER.raw() + 74
	GETBITMAP WM::USER.raw() + 44
	GETBITMAPFLAGS WM::USER.raw() + 41
	GETBUTTON WM::USER.raw() + 23
	GETBUTTONINFO WM::USER.raw() + 63
	GETBUTTONSIZE WM::USER.raw() + 58
	GETBUTTONTEXT WM::USER.raw() + 75
	GETCOLORSCHEME CCM::GETCOLORSCHEME.0
	GETDISABLEDIMAGELIST WM::USER.raw() + 55
	GETEXTENDEDSTYLE WM::USER.raw() + 85
	GETHOTIMAGELIST WM::USER.raw() + 53
	GETHOTITEM WM::USER.raw() + 71
	GETIDEALSIZE WM::USER.raw() + 99
	GETIMAGELIST WM::USER.raw() + 49
	GETIMAGELISTCOUNT WM::USER.raw() + 98
	GETINSERTMARK WM::USER.raw() + 79
	GETINSERTMARKCOLOR WM::USER.raw() + 89
	GETITEMDROPDOWNRECT WM::USER.raw() + 103
	GETITEMRECT WM::USER.raw() + 29
	GETMAXSIZE WM::USER.raw() + 83
	GETMETRICS WM::USER.raw() + 101
	GETOBJECT WM::USER.raw() + 62
	GETPADDING WM::USER.raw() + 86
	GETPRESSEDIMAGELIST WM::USER.raw() + 105
	GETRECT WM::USER.raw() + 51
	GETROWS WM::USER.raw() + 40
	GETSTATE WM::USER.raw() + 18
	GETSTRING WM::USER.raw() + 91
	GETSTYLE WM::USER.raw() + 57
	GETTEXTROWS WM::USER.raw() + 61
	GETTOOLTIPS WM::USER.raw() + 35
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
	HASACCELERATOR WM::USER.raw() + 95
	HIDEBUTTON WM::USER.raw() + 4
	HITTEST WM::USER.raw() + 69
	INDETERMINATE WM::USER.raw() + 5
	INSERTBUTTON WM::USER.raw() + 67
	INSERTMARKHITTEST WM::USER.raw() + 81
	ISBUTTONCHECKED WM::USER.raw() + 10
	ISBUTTONENABLED WM::USER.raw() + 9
	ISBUTTONHIDDEN WM::USER.raw() + 12
	ISBUTTONHIGHLIGHTED WM::USER.raw() + 14
	ISBUTTONINDETERMINATE WM::USER.raw() + 13
	ISBUTTONPRESSED WM::USER.raw() + 11
	LOADIMAGES WM::USER.raw() + 50
	MAPACCELERATOR WM::USER.raw() + 90
	MARKBUTTON WM::USER.raw() + 6
	MOVEBUTTON WM::USER.raw() + 82
	PRESSBUTTON WM::USER.raw() + 3
	REPLACEBITMAP WM::USER.raw() + 46
	SAVERESTORE WM::USER.raw() + 76
	SETANCHORHIGHLIGHT WM::USER.raw() + 73
	SETBITMAPSIZE WM::USER.raw() + 32
	SETBOUNDINGSIZE WM::USER.raw() + 93
	SETBUTTONINFO WM::USER.raw() + 64
	SETBUTTONSIZE WM::USER.raw() + 31
	SETBUTTONWIDTH WM::USER.raw() + 59
	SETCMDID WM::USER.raw() + 42
	SETCOLORSCHEME CCM::SETCOLORSCHEME.0
	SETDISABLEDIMAGELIST WM::USER.raw() + 54
	SETDRAWTEXTFLAGS WM::USER.raw() + 70
	SETEXTENDEDSTYLE WM::USER.raw() + 84
	SETHOTIMAGELIST WM::USER.raw() + 52
	SETHOTITEM WM::USER.raw() + 72
	SETHOTITEM2 WM::USER.raw() + 94
	SETIMAGELIST WM::USER.raw() + 48
	SETINDENT WM::USER.raw() + 47
	SETINSERTMARK WM::USER.raw() + 80
	SETINSERTMARKCOLOR WM::USER.raw() + 88
	SETLISTGAP WM::USER.raw() + 96
	SETMAXTEXTROWS WM::USER.raw() + 60
	SETMETRICS WM::USER.raw() + 102
	SETPADDING WM::USER.raw() + 87
	SETPARENT WM::USER.raw() + 37
	SETPRESSEDIMAGELIST WM::USER.raw() + 104
	SETREDRAWTEXTFLAGS WM::USER.raw() + 70
	SETROWS WM::USER.raw() + 39
	SETSTATE WM::USER.raw() + 17
	SETSTYLE WM::USER.raw() + 56
	SETTOOLTIPS WM::USER.raw() + 36
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	SETWINDOWTHEME CCM::SETWINDOWTHEME.0
}

const_bitflag! { TBMF: u32;
	/// [`TBMETRICS`](crate::TBMETRICS) `dwMask` (`u32`).
	=>
	=>
	PAD 0x0000_0001
	BARPAD 0x0000_0002
	BUTTONSPACING 0x0000_0004
}

const_nm! { TBN;
	/// Toolbar control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -700
	=>
	BEGINADJUST Self::FIRST.0 - 3
	BEGINDRAG Self::FIRST.0 - 1
	CUSTHELP Self::FIRST.0 - 9
	DELETINGBUTTON Self::FIRST.0 - 15
	DRAGOUT Self::FIRST.0 - 14
	DRAGOVER Self::FIRST.0 - 27
	DROPDOWN Self::FIRST.0 - 10
	DUPACCELERATOR Self::FIRST.0 - 25
	ENDADJUST Self::FIRST.0 - 4
	ENDDRAG Self::FIRST.0 - 2
	GETBUTTONINFO Self::FIRST.0 - 20
	GETDISPINFO Self::FIRST.0 - 17
	GETINFOTIP Self::FIRST.0 - 19
	GETOBJECT Self::FIRST.0 - 12
	HOTITEMCHANGE Self::FIRST.0 - 13
	INITCUSTOMIZE Self::FIRST.0 - 23
	MAPACCELERATOR Self::FIRST.0 - 28
	QUERYDELETE Self::FIRST.0 - 7
	QUERYINSERT Self::FIRST.0 - 6
	RESET Self::FIRST.0 - 5
	RESTORE Self::FIRST.0 - 21
	SAVE Self::FIRST.0 - 22
	TOOLBARCHANGE Self::FIRST.0 - 8
	WRAPACCELERATOR Self::FIRST.0 - 26
	WRAPHOTITEM Self::FIRST.0 - 24
}

const_ws! { TBS: u32;
	/// Trackbar control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/trackbar-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	AUTOTICKS 0x0001
	VERT 0x0002
	HORZ 0x0000
	TOP 0x0004
	BOTTOM 0x0000
	LEFT 0x0004
	RIGHT 0x0000
	BOTH 0x0008
	NOTICKS 0x0010
	ENABLESELRANGE 0x0020
	FIXEDLENGTH 0x0040
	NOTHUMB 0x0080
	TOOLTIPS 0x0100
	REVERSED 0x0200
	DOWNISLEFT 0x0400
	NOTIFYBEFOREMOVE 0x0800
	TRANSPARENTBKGND 0x1000
}

const_bitflag! { TBSTATE: u8;
	/// Toolbar button
	/// [states](https://learn.microsoft.com/en-us/windows/win32/controls/toolbar-button-states)
	/// (`u8`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	CHECKED 0x01
	PRESSED 0x02
	ENABLED 0x04
	HIDDEN 0x08
	INDETERMINATE 0x10
	WRAP 0x20
	ELLIPSES 0x40
	MARKED 0x80
}

const_wsex! { TBSTYLE_EX;
	/// Extended toolbar control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/toolbar-extended-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	DRAWDDARROWS 0x0000_0001
	MIXEDBUTTONS 0x0000_0008
	HIDECLIPPEDBUTTONS 0x0000_0010
	MULTICOLUMN 0x0000_0002
	VERTICAL 0x0000_0004
	DOUBLEBUFFER 0x0000_0080
}

const_ordinary! { TBTS: u8;
	/// [`trbm::SetTipSide`](crate::msg::trbm::SetTipSide) `location` (`u8`).
	=>
	=>
	TOP 0
	LEFT 1
	BOTTOM 2
	RIGHT 3
}

const_ordinary! { TCHT: u32;
	/// [`TCHITTESTINFO`](crate::TCHITTESTINFO) `flags` (`u32`).
	=>
	=>
	NOWHERE 0x0001
	ONITEMICON 0x0002
	ONITEMLABEL 0x0004
	ONITEM TCHT::ONITEMICON.0 | TCHT::ONITEMLABEL.0
}

const_bitflag! { TCIF: u32;
	/// [`TCITEM`](crate::TCITEM) `mask` (`u32`).
	=>
	=>
	TEXT 0x0001
	IMAGE 0x0002
	RTLREADING 0x0004
	PARAM 0x0008
	STATE 0x0010
}

const_bitflag! { TCIS: u32;
	/// Tab control item
	/// [states](https://learn.microsoft.com/en-us/windows/win32/controls/tab-control-item-states)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	BUTTONPRESSED 0x0001
	HIGHLIGHTED 0x0002
}

const_wm! { TCM;
	/// Tab control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-messages)
	/// (`u32`).
	=>
	FIRST 0x1300
	=>
	GETIMAGELIST Self::FIRST.0 + 2
	SETIMAGELIST Self::FIRST.0 + 3
	GETITEMCOUNT Self::FIRST.0 + 4
	GETITEM Self::FIRST.0 + 60
	SETITEM Self::FIRST.0 + 61
	INSERTITEM Self::FIRST.0 + 62
	DELETEITEM Self::FIRST.0 + 8
	DELETEALLITEMS Self::FIRST.0 + 9
	GETITEMRECT Self::FIRST.0 + 10
	GETCURSEL Self::FIRST.0 + 11
	SETCURSEL Self::FIRST.0 + 12
	HITTEST Self::FIRST.0 + 13
	SETITEMEXTRA Self::FIRST.0 + 14
	ADJUSTRECT Self::FIRST.0 + 40
	SETITEMSIZE Self::FIRST.0 + 41
	REMOVEIMAGE Self::FIRST.0 + 42
	SETPADDING Self::FIRST.0 + 43
	GETROWCOUNT Self::FIRST.0 + 44
	GETTOOLTIPS Self::FIRST.0 + 45
	SETTOOLTIPS Self::FIRST.0 + 46
	GETCURFOCUS Self::FIRST.0 + 47
	SETCURFOCUS Self::FIRST.0 + 48
	SETMINTABWIDTH Self::FIRST.0 + 49
	DESELECTALL Self::FIRST.0 + 50
	HIGHLIGHTITEM Self::FIRST.0 + 51
	SETEXTENDEDSTYLE Self::FIRST.0 + 52
	GETEXTENDEDSTYLE Self::FIRST.0 + 53
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
}

const_nm! { TCN;
	/// Tab control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -550
	=>
	FOCUSCHANGE Self::FIRST.0 - 4
	GETOBJECT Self::FIRST.0 - 3
	KEYDOWN Self::FIRST.0 - 0
	SELCHANGE Self::FIRST.0 - 1
	SELCHANGING Self::FIRST.0 - 2
}

const_ws! { TCS: u32;
	/// Tab control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/tab-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SCROLLOPPOSITE 0x0001
	BOTTOM 0x0002
	RIGHT 0x0002
	MULTISELECT 0x0004
	FLATBUTTONS 0x0008
	FORCEICONLEFT 0x0010
	FORCELABELLEFT 0x0020
	HOTTRACK 0x0040
	VERTICAL 0x0080
	TABS 0x0000
	BUTTONS 0x0100
	SINGLELINE 0x0000
	MULTILINE 0x0200
	RIGHTJUSTIFY 0x0000
	FIXEDWIDTH 0x0400
	RAGGEDRIGHT 0x0800
	FOCUSONBUTTONDOWN 0x1000
	OWNERDRAWFIXED 0x2000
	TOOLTIPS 0x4000
	FOCUSNEVER 0x8000
}

const_wsex! { TCS_EX;
	/// Extended tab control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/tab-control-extended-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	FLATSEPARATORS 0x0000_0001
	REGISTERDROP 0x0000_0002
}

const_ordinary! { TD_ICON: u16;
	/// [`HWND::TaskDialog`](crate::prelude::comctl_Hwnd::TaskDialog)
	/// `pszIcon` and [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG)
	/// `pszMainIcon` (`u16`).
	///
	/// Originally has `TD` prefix and `ICON` suffix.
	=>
	=>
	/// An exclamation-point icon appears in the task dialog.
	WARNING 0xffff
	/// A stop-sign icon appears in the task dialog.
	ERROR 0xfffe
	/// An icon consisting of a lowercase letter i in a circle appears in the
	/// task dialog.
	INFORMATION 0xfffd
	/// A security shield icon appears in the task dialog.
	SHIELD 0xfffc
}

const_bitflag! { TDCBF: i32;
	/// [`HWND::TaskDialog`](crate::prelude::comctl_Hwnd::TaskDialog) and
	/// [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `dwCommonButtons` (`i32`).
	///
	/// Originally has `TDCBF` prefix and `BUTTON` suffix.
	=>
	=>
	OK 0x0001
	YES 0x0002
	NO 0x0004
	CANCEL 0x0008
	RETRY 0x0010
	CLOSE 0x0020
}

const_bitflag! { TDF: i32;
	/// [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `dwFlags` (`i32`).
	=>
	=>
	ENABLE_HYPERLINKS 0x0001
	USE_HICON_MAIN 0x0002
	USE_HICON_FOOTER 0x0004
	ALLOW_DIALOG_CANCELLATION 0x0008
	USE_COMMAND_LINKS 0x0010
	USE_COMMAND_LINKS_NO_ICON 0x0020
	EXPAND_FOOTER_AREA 0x0040
	EXPANDED_BY_DEFAULT 0x0080
	VERIFICATION_FLAG_CHECKED 0x0100
	SHOW_PROGRESS_BAR 0x0200
	SHOW_MARQUEE_PROGRESS_BAR 0x0400
	CALLBACK_TIMER 0x0800
	POSITION_RELATIVE_TO_WINDOW 0x1000
	RTL_LAYOUT 0x2000
	NO_DEFAULT_RADIO_BUTTON 0x4000
	CAN_BE_MINIMIZED 0x8000
	NO_SET_FOREGROUND 0x0001_0000
	SIZE_TO_CONTENT 0x0100_0000
}

const_ordinary! { TDN: u32;
	/// [`PFTASKDIALOGCALLBACK`](crate::PFTASKDIALOGCALLBACK) `msg` (`u32`).
	=>
	=>
    CREATED 0
    NAVIGATED 1
    BUTTON_CLICKED 2           	// wParam = Button ID
    HYPERLINK_CLICKED 3         // lParam = (LPCWSTR)pszHREF
    TIMER 4            			// wParam = Milliseconds since dialog created or timer reset
    DESTROYED 5
    RADIO_BUTTON_CLICKED 6      // wParam = Radio Button ID
    DIALOG_CONSTRUCTED 7
    VERIFICATION_CLICKED 8      // wParam = 1 if checkbox checked, 0 if not, lParam is unused and always 0
    HELP 9
    EXPANDO_BUTTON_CLICKED 10   // wParam = 0 (dialog is now collapsed), wParam != 0 (dialog is now expanded)
}

const_wm! { TRBM;
	/// Trackbar control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-messages)
	/// (`u32`).
	///
	/// Originally has `TBM` prefix.
	=>
	=>
	GETPOS WM::USER.raw()
	GETRANGEMIN WM::USER.raw() + 1
	GETRANGEMAX WM::USER.raw() + 2
	GETTIC WM::USER.raw() + 3
	SETTIC WM::USER.raw() + 4
	SETPOS WM::USER.raw() + 5
	SETRANGE WM::USER.raw() + 6
	SETRANGEMIN WM::USER.raw() + 7
	SETRANGEMAX WM::USER.raw() + 8
	CLEARTICS WM::USER.raw() + 9
	SETSEL WM::USER.raw() + 10
	SETSELSTART WM::USER.raw() + 11
	SETSELEND WM::USER.raw() + 12
	GETPTICS WM::USER.raw() + 14
	GETTICPOS WM::USER.raw() + 15
	GETNUMTICS WM::USER.raw() + 16
	GETSELSTART WM::USER.raw() + 17
	GETSELEND WM::USER.raw() + 18
	CLEARSEL WM::USER.raw() + 19
	SETTICFREQ WM::USER.raw() + 20
	SETPAGESIZE WM::USER.raw() + 21
	GETPAGESIZE WM::USER.raw() + 22
	SETLINESIZE WM::USER.raw() + 23
	GETLINESIZE WM::USER.raw() + 24
	GETTHUMBRECT WM::USER.raw() + 25
	GETCHANNELRECT WM::USER.raw() + 26
	SETTHUMBLENGTH WM::USER.raw() + 27
	GETTHUMBLENGTH WM::USER.raw() + 28
	SETTOOLTIPS WM::USER.raw() + 29
	GETTOOLTIPS WM::USER.raw() + 30
	SETTIPSIDE WM::USER.raw() + 31
	SETBUDDY WM::USER.raw() + 32
	GETBUDDY WM::USER.raw() + 33
	SETPOSNOTIFY WM::USER.raw() + 34
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
}

const_nm! { TRBN;
	/// Trackbar control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -1501
	=>
	THUMBPOSCHANGING Self::FIRST.0 - 1
}

const_ordinary! { TTI: i32;
	/// [`EDITBALLOONTIP`](crate::EDITBALLOONTIP) ttiIcon (`i32`).
	=>
	=>
	ERROR 3
	INFO 1
	NONE 0
	WARNING 2
	INFO_LARGE 4
	WARNING_LARGE 5
	ERROR_LARGE 6
}

const_ordinary! { TVC: u32;
	/// [`NMTREEVIEW`](crate::NMTREEVIEW) `action` (`u32`).
	=>
	=>
	UNKNOWN 0x0000
	BYMOUSE 0x0001
	BYKEYBOARD 0x0002
}

const_ordinary! { TVE: u32;
	/// [`tvm::Expand`](crate::msg::tvm::Expand) `action` (`u32`).
	=>
	=>
	COLLAPSE 0x0001
	EXPAND 0x0002
	TOGGLE 0x0003
	EXPANDPARTIAL 0x4000
	COLLAPSERESET 0x8000
}

const_ordinary! { TVGN: u32;
	/// [`tvm::GetNextItem`](crate::msg::tvm::GetNextItem) `which` (`u32`).
	=>
	=>
	ROOT 0x0000
	NEXT 0x0001
	PREVIOUS 0x0002
	PARENT 0x0003
	CHILD 0x0004
	FIRSTVISIBLE 0x0005
	NEXTVISIBLE 0x0006
	PREVIOUSVISIBLE 0x0007
	DROPHILITE 0x0008
	CARET 0x0009
	LASTVISIBLE 0x000a
	NEXTSELECTED 0x000b
	/// Originally has no `TVGN` prefix.
	TVSI_NOSINGLEEXPAND 0x8000
}

const_bitflag! { TVHT: u32;
	/// [`TVHITTESTINFO`](crate::TVHITTESTINFO) `flags` (`u32`).
	=>
	=>
	NOWHERE 0x0001
	ONITEMICON 0x0002
	ONITEMLABEL 0x0004
	ONITEM TVHT::ONITEMICON.0 | TVHT::ONITEMLABEL.0 | TVHT::ONITEMSTATEICON.0
	ONITEMINDENT 0x0008
	ONITEMBUTTON 0x0010
	ONITEMRIGHT 0x0020
	ONITEMSTATEICON 0x0040
	ABOVE 0x0100
	BELOW 0x0200
	TORIGHT 0x0400
	TOLEFT 0x0800
}

const_ordinary! { TVI: isize;
	/// [`TVINSERTSTRUCT`](crate::TVINSERTSTRUCT) `hInsertAfter` (`isize`).
	=>
	=>
	ROOT -0x10000
	FIRST -0x0ffff
	LAST -0x0fffe
	SORT -0x0fffd
}

const_bitflag! { TVIF: u32;
	/// [`TVITEM`](crate::TVITEM) `mask` (`u32`).
	=>
	=>
	TEXT 0x0001
	IMAGE 0x0002
	PARAM 0x0004
	STATE 0x0008
	HANDLE 0x0010
	SELECTEDIMAGE 0x0020
	CHILDREN 0x0040
	INTEGRAL 0x0080
	STATEEX 0x0100
	EXPANDEDIMAGE 0x0200
}

const_bitflag! { TVIS: u32;
	/// Tree view item
	/// [states](https://learn.microsoft.com/en-us/windows/win32/controls/tree-view-control-item-states)
	/// (`u32`)
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SELECTED 0x0002
	CUT 0x0004
	DROPHILITED 0x0008
	BOLD 0x0010
	EXPANDED 0x0020
	EXPANDEDONCE 0x0040
	EXPANDPARTIAL 0x0080
	OVERLAYMASK 0x0f00
	STATEIMAGEMASK 0xf000
	USERMASK 0xf000
}

const_bitflag! { TVIS_EX: u32;
	/// [`TVITEMEX`](crate::TVITEMEX) `uStateEx` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	DISABLED 0x0002
	FLAT 0x0001
	/// This value is not declared in any header, it may not be accurate.
	HWND 0x0000
}

const_bitflag! { TVSBF: u32;
	/// [`tvm::SetBorder`](crate::msg::tvm::SetBorder) `action` (`u32`).
	=>
	=>
	XBORDER 0x0000_0001
	YBORDER 0x0000_0002
}

const_wm! { TVM;
	/// Tree view control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-messages)
	/// (`u32`).
	=>
	FIRST 0x1100
	=>
	INSERTITEM Self::FIRST.0 + 50
	DELETEITEM Self::FIRST.0 + 1
	EXPAND Self::FIRST.0 + 2
	GETITEMRECT Self::FIRST.0 + 4
	GETCOUNT Self::FIRST.0 + 5
	GETINDENT Self::FIRST.0 + 6
	SETINDENT Self::FIRST.0 + 7
	GETIMAGELIST Self::FIRST.0 + 8
	SETIMAGELIST Self::FIRST.0 + 9
	GETNEXTITEM Self::FIRST.0 + 10
	SELECTITEM Self::FIRST.0 + 11
	GETITEM Self::FIRST.0 + 62
	SETITEM Self::FIRST.0 + 63
	EDITLABEL Self::FIRST.0 + 65
	GETEDITCONTROL Self::FIRST.0 + 15
	GETVISIBLECOUNT Self::FIRST.0 + 16
	HITTEST Self::FIRST.0 + 17
	CREATEDRAGIMAGE Self::FIRST.0 + 18
	SORTCHILDREN Self::FIRST.0 + 19
	ENSUREVISIBLE Self::FIRST.0 + 20
	SORTCHILDRENCB Self::FIRST.0 + 21
	ENDEDITLABELNOW Self::FIRST.0 + 22
	GETISEARCHSTRING Self::FIRST.0 + 64
	SETTOOLTIPS Self::FIRST.0 + 24
	GETTOOLTIPS Self::FIRST.0 + 25
	SETINSERTMARK Self::FIRST.0 + 26
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
	SETITEMHEIGHT Self::FIRST.0 + 27
	GETITEMHEIGHT Self::FIRST.0 + 28
	SETBKCOLOR Self::FIRST.0 + 29
	SETTEXTCOLOR Self::FIRST.0 + 30
	GETBKCOLOR Self::FIRST.0 + 31
	GETTEXTCOLOR Self::FIRST.0 + 32
	SETSCROLLTIME Self::FIRST.0 + 33
	GETSCROLLTIME Self::FIRST.0 + 34
	SETINSERTMARKCOLOR Self::FIRST.0 + 37
	GETINSERTMARKCOLOR Self::FIRST.0 + 38
	SETBORDER Self::FIRST.0 + 35
	GETITEMSTATE Self::FIRST.0 + 39
	SETLINECOLOR Self::FIRST.0 + 40
	GETLINECOLOR Self::FIRST.0 + 41
	MAPACCIDTOHTREEITEM Self::FIRST.0 + 42
	MAPHTREEITEMTOACCID Self::FIRST.0 + 43
	SETEXTENDEDSTYLE Self::FIRST.0 + 44
	GETEXTENDEDSTYLE Self::FIRST.0 + 45
	SETAUTOSCROLLINFO Self::FIRST.0 + 59
	SETHOT Self::FIRST.0 + 58
	GETSELECTEDCOUNT Self::FIRST.0 + 70
	SHOWINFOTIP Self::FIRST.0 + 71
	GETITEMPARTRECT Self::FIRST.0 + 72
}

const_nm! { TVN;
	/// Tree view control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -400
	=>
	SELCHANGING Self::FIRST.0 - 50
	SELCHANGED Self::FIRST.0 - 51
	GETDISPINFO Self::FIRST.0 - 52
	SETDISPINFO Self::FIRST.0 - 53
	ITEMEXPANDING Self::FIRST.0 - 54
	ITEMEXPANDED Self::FIRST.0 - 55
	BEGINDRAG Self::FIRST.0 - 56
	BEGINRDRAG Self::FIRST.0 - 57
	DELETEITEM Self::FIRST.0 - 58
	BEGINLABELEDIT Self::FIRST.0 - 59
	ENDLABELEDIT Self::FIRST.0 - 60
	KEYDOWN Self::FIRST.0 - 12
	GETINFOTIP Self::FIRST.0 - 14
	SINGLEEXPAND Self::FIRST.0 - 15
	ITEMCHANGING Self::FIRST.0 - 17
	ITEMCHANGED Self::FIRST.0 - 19
	ASYNCDRAW Self::FIRST.0 - 20
}

const_ws! { TVS: u32;
	/// Tree view control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/tree-view-control-window-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	HASBUTTONS 0x0001
	HASLINES 0x0002
	LINESATROOT 0x0004
	EDITLABELS 0x0008
	DISABLEDRAGDROP 0x0010
	SHOWSELALWAYS 0x0020
	RTLREADING 0x0040
	NOTOOLTIPS 0x0080
	CHECKBOXES 0x0100
	TRACKSELECT 0x0200
	SINGLEEXPAND 0x0400
	INFOTIP 0x0800
	FULLROWSELECT 0x1000
	NOSCROLL 0x2000
	NONEVENHEIGHT 0x4000
	NOHSCROLL 0x8000
}

const_wsex! { TVS_EX;
	/// Extended tree view control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/tree-view-control-window-extended-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	NOSINGLECOLLAPSE 0x0001
	MULTISELECT 0x0002
	DOUBLEBUFFER 0x0004
	NOINDENTSTATE 0x0008
	RICHTOOLTIP 0x0010
	AUTOHSCROLL 0x0020
	FADEINOUTEXPANDOS 0x0040
	PARTIALCHECKBOXES 0x0080
	EXCLUSIONCHECKBOXES 0x0100
	DIMMEDCHECKBOXES 0x0200
	DRAWIMAGEASYNC 0x0400
}

const_ordinary! { TVSIL: u8;
	/// [`tvm::GetImageList`](crate::msg::tvm::GetImageList) and
	/// [`tvm::SetImageList`](crate::msg::tvm::SetImageList) `kind`.
	=>
	=>
	NORMAL 0
	STATE 2
}

const_wm! { UDM;
	/// Up-down control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-up-down-control-reference-messages)
	/// (`u32`).
	=>
	=>
	SETRANGE WM::USER.raw() + 101
	GETRANGE WM::USER.raw() + 102
	SETPOS WM::USER.raw() + 103
	GETPOS WM::USER.raw() + 104
	SETBUDDY WM::USER.raw() + 105
	GETBUDDY WM::USER.raw() + 106
	SETACCEL WM::USER.raw() + 107
	GETACCEL WM::USER.raw() + 108
	SETBASE WM::USER.raw() + 109
	GETBASE WM::USER.raw() + 110
	SETRANGE32 WM::USER.raw() + 111
	GETRANGE32 WM::USER.raw() + 112
	SETUNICODEFORMAT CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT CCM::GETUNICODEFORMAT.0
	SETPOS32 WM::USER.raw() + 113
	GETPOS32 WM::USER.raw() + 114
}

const_nm! { UDN;
	/// Up-down control `WM_NOTIFY`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-up-down-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -721
	=>
	DELTAPOS Self::FIRST.0 - 1
}

const_ws! { UDS: u32;
	/// Up-down control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/up-down-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Causes the position to "wrap" if it is incremented or decremented beyond
	/// the ending or beginning of the range.
	WRAP 0x0001
	/// Causes the up-down control to set the text of the buddy window (using
	/// the [`WM_SETTEXT`](crate::msg::wm::SetText) message) when the position
	/// changes. The text consists of the position formatted as a decimal or
	/// hexadecimal string.
	SETBUDDYINT 0x0002
	/// Positions the up-down control next to the right edge of the buddy
	/// window. The width of the buddy window is decreased to accommodate the
	/// width of the up-down control.
	ALIGNRIGHT 0x0004
	/// Positions the up-down control next to the left edge of the buddy window.
	/// The buddy window is moved to the right, and its width is decreased to
	/// accommodate the width of the up-down control.
	ALIGNLEFT 0x0008
	/// Automatically selects the previous window in the z-order as the up-down
	/// control's buddy window.
	AUTOBUDDY 0x0010
	/// Causes the up-down control to increment and decrement the position when
	/// the UP ARROW and DOWN ARROW keys are pressed.
	ARROWKEYS 0x0020
	/// Causes the up-down control's arrows to point left and right instead of
	/// up and down.
	HORZ 0x0040
	/// Does not insert a thousands separator between every three decimal
	/// digits.
	NOTHOUSANDS 0x0080
	/// Causes the control to exhibit "hot tracking" behavior. That is, it
	/// highlights the UP ARROW and DOWN ARROW on the control as the pointer
	/// passes over them.
	HOTTRACK 0x0100
}

const_ordinary! { VK_DIR: u16;
	/// [`LVFINDINFO`](crate::LVFINDINFO) `vkDirection` (`u16`).
	=>
	=>
	PRIOR 0x21
	NEXT 0x22
	END 0x23
	HOME 0x24
	LEFT 0x25
	UP 0x26
	RIGHT 0x27
	DOWN 0x28
}

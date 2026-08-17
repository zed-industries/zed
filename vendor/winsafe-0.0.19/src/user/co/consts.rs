#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::*;

const_bitflag! { ACCELF: u8;
	/// [`ACCELL`](crate::ACCEL) `fVirt` (`u8`).
	///
	/// Originally has `F` prefix.
	=>
	=>
	/// The `key` member specifies a virtual-key code. If this flag is not
	/// specified key is assumed to specify a character code.
	VIRTKEY 1
	/// The SHIFT key must be held down when the accelerator key is pressed.
	SHIFT 0x04
	/// The CTRL key must be held down when the accelerator key is pressed.
	CONTROL 0x08
	/// The ALT key must be held down when the accelerator key is pressed.
	ALT 0x10
}

const_ordinary! { APPCOMMAND: u16;
	/// [`wm::AppCommand`](crate::msg::wm::AppCommand) commands (`u16`).
	=>
	=>
	BROWSER_BACKWARD 1
	BROWSER_FORWARD 2
	BROWSER_REFRESH 3
	BROWSER_STOP 4
	BROWSER_SEARCH 5
	BROWSER_FAVORITES 6
	BROWSER_HOME 7
	VOLUME_MUTE 8
	VOLUME_DOWN 9
	VOLUME_UP 10
	MEDIA_NEXTTRACK 11
	MEDIA_PREVIOUSTRACK 12
	MEDIA_STOP 13
	MEDIA_PLAY_PAUSE 14
	LAUNCH_MAIL 15
	LAUNCH_MEDIA_SELECT 16
	LAUNCH_APP1 17
	LAUNCH_APP2 18
	BASS_DOWN 19
	BASS_BOOST 20
	BASS_UP 21
	TREBLE_DOWN 22
	TREBLE_UP 23
	MICROPHONE_VOLUME_MUTE 24
	MICROPHONE_VOLUME_DOWN 25
	MICROPHONE_VOLUME_UP 26
	HELP 27
	FIND 28
	NEW 29
	OPEN 30
	CLOSE 31
	SAVE 32
	PRINT 33
	UNDO 34
	REDO 35
	COPY 36
	CUT 37
	PASTE 38
	REPLY_TO_MAIL 39
	FORWARD_MAIL 40
	SEND_MAIL 41
	SPELL_CHECK 42
	DICTATE_OR_COMMAND_CONTROL_TOGGLE 43
	MIC_ON_OFF_TOGGLE 44
	CORRECTION_LIST 45
	MEDIA_PLAY 46
	MEDIA_PAUSE 47
	MEDIA_RECORD 48
	MEDIA_FAST_FORWARD 49
	MEDIA_REWIND 50
	MEDIA_CHANNEL_UP 51
	MEDIA_CHANNEL_DOWN 52
	DELETE 53
	DWM_FLIP3D 54
}

const_wm! { BM;
	/// Button control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-messages)
	/// (`u32`).
	=>
	=>
	GETCHECK 0x00f0
	SETCHECK 0x00f1
	GETSTATE 0x00f2
	SETSTATE 0x00f3
	SETSTYLE 0x00f4
	CLICK 0x00f5
	GETIMAGE 0x00f6
	SETIMAGE 0x00f7
	SETDONTCLICK 0x00f8
}

const_cmd! { BN;
	/// Button control `WM_COMMAND`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications)
	/// (`u16`).
	=>
	=>
	CLICKED 0
	PAINT 1
	HILITE 2
	UNHILITE 3
	DISABLE 4
	DOUBLECLICKED 5
	PUSHED Self::HILITE.0
	UNPUSHED Self::UNHILITE.0
	DBLCLK Self::DOUBLECLICKED.0
	SETFOCUS 6
	KILLFOCUS 7
}

const_ws! { BS: u32;
	/// Button control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/button-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	PUSHBUTTON 0x0000_0000
	DEFPUSHBUTTON 0x0000_0001
	CHECKBOX 0x0000_0002
	AUTOCHECKBOX 0x0000_0003
	RADIOBUTTON 0x0000_0004
	R3STATE 0x0000_0005
	AUTO3STATE 0x0000_0006
	GROUPBOX 0x0000_0007
	USERBUTTON 0x0000_0008
	AUTORADIOBUTTON 0x0000_0009
	PUSHBOX 0x0000_000a
	OWNERDRAW 0x0000_000b
	TYPEMASK 0x0000_000f
	LEFTTEXT 0x0000_0020
	TEXT 0x0000_0000
	ICON 0x0000_0040
	BITMAP 0x0000_0080
	LEFT 0x0000_0100
	RIGHT 0x0000_0200
	CENTER 0x0000_0300
	TOP 0x0000_0400
	BOTTOM 0x0000_0800
	VCENTER 0x0000_0c00
	PUSHLIKE 0x0000_1000
	MULTILINE 0x0000_2000
	NOTIFY 0x0000_4000
	FLAT 0x0000_8000
	RIGHTBUTTON Self::LEFTTEXT.0
}

const_bitflag! { BSM: u32;
	/// [`BroadcastSystemMessage`](crate::BroadcastSystemMessage) `info` and
	/// return value (`u32`).
	=>
	=>
	ALLCOMPONENTS 0x0000_0000
	VXDS 0x0000_0001
	NETDRIVER 0x0000_0002
	INSTALLABLEDRIVERS 0x0000_0004
	APPLICATIONS 0x0000_0008
	ALLDESKTOPS 0x0000_0010
}

const_bitflag! { BSF: u32;
	/// [`BroadcastSystemMessage`](crate::BroadcastSystemMessage) `flags`
	/// (`u32`).
	=>
	=>
	QUERY 0x0000_0001
	IGNORECURRENTTASK 0x0000_0002
	FLUSHDISK 0x0000_0004
	NOHANG 0x0000_0008
	POSTMESSAGE 0x0000_0010
	FORCEIFHUNG 0x0000_0020
	NOTIMEOUTIFNOTHUNG 0x0000_0040
	ALLOWSFW 0x0000_0080
	SENDNOTIFYMESSAGE 0x0000_0100
	RETURNHDESK 0x0000_0200
	LUID 0x0000_0400
}

const_ordinary! { BST: u32;
	/// [`bm::GetCheck`](crate::msg::bm::GetCheck) return value (`u32`).
	=>
	=>
	UNCHECKED 0x0000
	CHECKED 0x0001
	INDETERMINATE 0x0002
	PUSHED 0x0004
	FOCUS 0x0008
}

const_wm! { CB;
	/// Combo box control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-messages)
	/// (`u32`).
	=>
	=>
	GETEDITSEL 0x0140
	LIMITTEXT 0x0141
	SETEDITSEL 0x0142
	ADDSTRING 0x0143
	DELETESTRING 0x0144
	DIR 0x0145
	GETCOUNT 0x0146
	GETCURSEL 0x0147
	GETLBTEXT 0x0148
	GETLBTEXTLEN 0x0149
	INSERTSTRING 0x014a
	RESETCONTENT 0x014b
	FINDSTRING 0x014c
	SELECTSTRING 0x014d
	SETCURSEL 0x014e
	SHOWDROPDOWN 0x014f
	GETITEMDATA 0x0150
	SETITEMDATA 0x0151
	GETDROPPEDCONTROLRECT 0x0152
	SETITEMHEIGHT 0x0153
	GETITEMHEIGHT 0x0154
	SETEXTENDEDUI 0x0155
	GETEXTENDEDUI 0x0156
	GETDROPPEDSTATE 0x0157
	FINDSTRINGEXACT 0x0158
	SETLOCALE 0x0159
	GETLOCALE 0x015a
	GETTOPINDEX 0x015b
	SETTOPINDEX 0x015c
	GETHORIZONTALEXTENT 0x015d
	SETHORIZONTALEXTENT 0x015e
	GETDROPPEDWIDTH 0x015f
	SETDROPPEDWIDTH 0x0160
	INITSTORAGE 0x0161
	GETCOMBOBOXINFO 0x0164
}

const_cmd! { CBN;
	/// Combo box control `WM_COMMAND`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-notifications)
	/// (`u16`).
	=>
	=>
	ERRSPACE -1i16 as _
	SELCHANGE 1
	DBLCLK 2
	SETFOCUS 3
	KILLFOCUS 4
	EDITCHANGE 5
	EDITUPDATE 6
	DROPDOWN 7
	CLOSEUP 8
	SELENDOK 9
	SELENDCANCEL 10
}

const_ws! { CBS: u32;
	/// Combo box control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/combo-box-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SIMPLE 0x0001
	DROPDOWN 0x0002
	DROPDOWNLIST 0x0003
	OWNERDRAWFIXED 0x0010
	OWNERDRAWVARIABLE 0x0020
	AUTOHSCROLL 0x0040
	OEMCONVERT 0x0080
	SORT 0x0100
	HASSTRINGS 0x0200
	NOINTEGRALHEIGHT 0x0400
	DISABLENOSCROLL 0x0800
	UPPERCASE 0x2000
	LOWERCASE 0x4000
}

const_bitflag! { CC: u32;
	/// [`CHOOSECOLOR`](crate::CHOOSECOLOR) `Flags` (`u32`).
	=>
	=>
	/// Causes the dialog box to use the color specified in the `rgbResult`
	/// member as the initial color selection.
	RGBINIT 0x0000_0001
	/// Causes the dialog box to display the additional controls that allow the
	/// user to create custom colors. If this flag is not set the user must
	/// click the Define Custom Color button to display the custom color
	/// controls.
	FULLOPEN 0x0000_0002
	/// Disables the Define Custom Color button.
	PREVENTFULLOPEN 0x0000_0004
	/// Causes the dialog box to display the Help button. The `hwndOwner` member
	/// must specify the window to receive the `HELPMSGSTRING` registered
	/// messages that the dialog box sends when the user clicks the Help button.
	SHOWHELP 0x0000_0008
	/// Enables the hook procedure specified in the `lpfnHook` member of this
	/// structure. This flag is used only to initialize the dialog box.
	ENABLEHOOK 0x0000_0010
	/// The `hInstance` and `lpTemplateName` members specify a dialog box
	/// template to use in place of the default template. This flag is used only
	/// to initialize the dialog box.
	ENABLETEMPLATE 0x0000_0020
	/// The `hInstance` member identifies a data block that contains a preloaded
	/// dialog box template. The system ignores the `lpTemplateName` member if
	/// this flag is specified. This flag is used only to initialize the dialog
	/// box.
	ENABLETEMPLATEHANDLE 0x0000_0040
	/// Causes the dialog box to display only solid colors in the set of basic
	/// colors.
	SOLIDCOLOR 0x0000_0080
	/// Causes the dialog box to display all available colors in the set of
	/// basic colors.
	ANYCOLOR 0x0000_0100
}

const_bitflag! { CDS: u32;
	/// [`ChangeDisplaySettings`](crate::ChangeDisplaySettings) `flags` (`u32`).
	=>
	=>
	DISABLE_UNSAFE_MODES 0x0000_0200
	DYNAMICALLY 0
	ENABLE_UNSAFE_MODES 0x0000_0100
	FULLSCREEN 0x0000_0004
	GLOBAL 0x0000_0008
	NORESET 0x1000_0000
	RESET 0x40000_000
	SET_PRIMARY 0x0000_0010
	TEST 0x0000_0002
	UPDATEREGISTRY 0x0000_0001
	VIDEOPARAMETERS 0x0000_0020
}

const_ordinary! { CF: u32;
	/// Standard clipboard
	/// [formats](https://learn.microsoft.com/en-us/windows/win32/dataxchg/standard-clipboard-formats)
	/// (`u32`).
	=>
	=>
	TEXT 1
	BITMAP 2
	METAFILEPICT 3
	SYLK 4
	DIF 5
	TIFF 6
	OEMTEXT 7
	DIB 8
	PALETTE 9
	PENDATA 10
	RIFF 11
	WAVE 12
	UNICODETEXT 13
	ENHMETAFILE 14
	HDROP 15
	LOCALE 16
	DIBV5 17
	OWNERDISPLAY 0x0080
	DSPTEXT 0x0081
	DSPBITMAP 0x0082
	DSPMETAFILEPICT 0x0083
	DSPENHMETAFILE 0x008e
	PRIVATEFIRST 0x0200
	PRIVATELAST 0x02ff
	GDIOBJFIRST 0x0300
	GDIOBJLAST 0x03ff
}

const_ordinary! { COLOR: i32;
	/// System
	/// [colors](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
	/// (`i32`).
	=>
	=>
	SCROLLBAR 0
	BACKGROUND 1
	ACTIVECAPTION 2
	INACTIVECAPTION 3
	MENU 4
	WINDOW 5
	WINDOWFRAME 6
	MENUTEXT 7
	WINDOWTEXT 8
	CAPTIONTEXT 9
	ACTIVEBORDER 10
	INACTIVEBORDER 11
	APPWORKSPACE 12
	HIGHLIGHT 13
	HIGHLIGHTTEXT 14
	BTNFACE 15
	BTNSHADOW 16
	GRAYTEXT 17
	BTNTEXT 18
	INACTIVECAPTIONTEXT 19
	BTNHIGHLIGHT 20
	C3DDKSHADOW 21
	C3DLIGHT 22
	INFOTEXT 23
	INFOBK 24
	HOTLIGHT 26
	GRADIENTACTIVECAPTION 27
	GRADIENTINACTIVECAPTION 28
	MENUHILIGHT 29
	MENUBAR 30
	DESKTOP Self::BACKGROUND.0
	C3DFACE Self::BTNFACE.0
	C3DSHADOW Self::BTNSHADOW.0
	C3DHIGHLIGHT Self::BTNHIGHLIGHT.0
	C3DHILIGHT Self::BTNHIGHLIGHT.0
	BTNHILIGHT Self::BTNHIGHLIGHT.0
}

const_ordinary! { CLR: u32;
	/// [`IMAGELISTDRAWPARAMS`](crate::IMAGELISTDRAWPARAMS) `rgbFg` (`u32`).
	=>
	=>
	NONE 0xffff_ffff
	DEFAULT 0xff00_0000
}

const_ordinary! { CMD: u16;
	/// [`wm::Command`](crate::msg::wm::Command) notification codes (`u16`).
	///
	/// **Note:** Control-specific notification codes have their own types,
	/// which are convertible to `CMD`.
	=>
	=>
	Menu 0
	Accelerator 1
}

const_bitflag! { CS: u32;
	/// Window class
	/// [`styles`](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-class-styles)
	/// (`u32`).
	=>
	=>
	VREDRAW 0x0001
	HREDRAW 0x0002
	DBLCLKS 0x0008
	OWNDC 0x0020
	CLASSDC 0x0040
	PARENTDC 0x0080
	NOCLOSE 0x0200
	SAVEBITS 0x0800
	BYTEALIGNCLIENT 0x1000
	BYTEALIGNWINDOW 0x2000
	GLOBALCLASS 0x4000
	IME 0x00010000
	DROPSHADOW 0x00020000
}

const_bitflag! { DC: u32;
	/// [`HWND::DrawCaption`](crate::prelude::user_Hwnd::DrawCaption) `flags`
	/// (`u32`).
	=>
	=>
	ACTIVE 0x0001
	SMALLCAP 0x0002
	ICON 0x0004
	TEXT 0x0008
	INBUTTON 0x0010
	GRADIENT 0x0020
	BUTTONS 0x1000
}

const_bitflag! { DDL: u16;
	/// [`cb::Dir`](crate::msg::cb::Dir) and [`lb::Dir`](crate::msg::lb::Dir)
	/// attributes (`u16`).
	=>
	=>
	READWRITE 0x0000
	READONLY 0x0001
	HIDDEN 0x0002
	SYSTEM 0x0004
	DIRECTORY 0x0010
	ARCHIVE 0x0020
	POSTMSGS 0x2000
	DRIVES 0x4000
	EXCLUSIVE 0x8000
}

const_bitflag! { DESKTOP_RIGHTS: u32;
	/// Desktop security and access rights
	/// [flags](https://learn.microsoft.com/en-us/windows/win32/winstation/desktop-security-and-access-rights)
	/// (`u32`).
	///
	/// Originally aglutinates [`co::ACCESS_RIGHTS`](crate::co::ACCESS_RIGHTS)
	/// and specific constants with `DESKTOP` prefix.
	=>
	=>
	DELETE ACCESS_RIGHTS::DELETE.raw()
	READ_CONTROL ACCESS_RIGHTS::READ_CONTROL.raw()
	WRITE_DAC ACCESS_RIGHTS::WRITE_DAC.raw()
	WRITE_OWNER ACCESS_RIGHTS::WRITE_OWNER.raw()
	SYNCHRONIZE ACCESS_RIGHTS::SYNCHRONIZE.raw()

	READOBJECTS 0x0001
	CREATEWINDOW 0x0002
	CREATEMENU 0x0004
	HOOKCONTROL 0x0008
	JOURNALRECORD 0x0010
	JOURNALPLAYBACK 0x0020
	ENUMERATE 0x0040
	WRITEOBJECTS 0x0080
	SWITCHDESKTOP 0x0100

	GENERIC_READ Self::ENUMERATE.0 | Self::READOBJECTS.0 | STANDARD_RIGHTS::READ.raw()
	GENERIC_WRITE Self::CREATEMENU.0 | Self::CREATEWINDOW.0 | Self::HOOKCONTROL.0 | Self::JOURNALPLAYBACK.0 | Self::JOURNALRECORD.0 | Self::WRITEOBJECTS.0 | STANDARD_RIGHTS::WRITE.raw()
	GENERICE_EXECUTE Self::SWITCHDESKTOP.0 | STANDARD_RIGHTS::EXECUTE.raw()
	GENERIC_ALL Self::CREATEMENU.0 | Self::CREATEWINDOW.0 | Self::ENUMERATE.0 | Self::HOOKCONTROL.0 | Self::JOURNALPLAYBACK.0 | Self::JOURNALRECORD.0 | Self::READOBJECTS.0 | Self::SWITCHDESKTOP.0 | Self::WRITEOBJECTS.0 | STANDARD_RIGHTS::REQUIRED.raw()
}

const_ordinary! { DF: u32;
	/// [`HDESK::OpenDesktop`](crate::prelude::user_Hdesk::OpenDesktop) `flags`
	/// (`u32`).
	=>
	=>
	ALLOWOTHERACCOUNTHOOK 0x0001
}

const_ordinary! { DISP_CHANGE: i32;
	/// [`ChangeDisplaySettings`](crate::ChangeDisplaySettings) return value
	/// (`u32`).
	=>
	=>
	SUCCESSFUL 0
	RESTART 1
	FAILED -1
	BADMODE -2
	NOTUPDATED -3
	BADFLAGS -4
	BADPARAM -5
	BADDUALVIEW -6
}

const_bitflag! { DISPLAY_DEVICE: u32;
	/// [`DISPLAY_DEVICE`](crate::DISPLAY_DEVICE) `StateFlags` (`u32`).
	=>
	=>
	ATTACHED_TO_DESKTOP 0x0000_0001
	MULTI_DRIVER 0x0000_0002
	PRIMARY_DEVICE 0x0000_0004
	MIRRORING_DRIVER 0x0000_0008
	VGA_COMPATIBLE 0x0000_0010
	REMOVABLE 0x0000_0020
	ACC_DRIVER 0x0000_0040
	MODESPRUNED 0x0800_0000
	RDPUDD 0x0100_0000
	REMOTE 0x0400_0000
	DISCONNECT 0x0200_0000
	TS_COMPATIBLE 0x0020_0000
	UNSAFE_MODES_ON 0x0008_0000
}

const_bitflag! { DM: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmFields` (`u32`).
	=>
	=>
	ORIENTATION 0x0000_0001
	PAPERSIZE 0x0000_0002
	PAPERLENGTH 0x0000_0004
	PAPERWIDTH 0x0000_0008
	SCALE 0x0000_0010
	POSITION 0x0000_0020
	NUP 0x0000_0040
	DISPLAYORIENTATION 0x0000_0080
	COPIES 0x0000_0100
	DEFAULTSOURCE 0x0000_0200
	PRINTQUALITY 0x0000_0400
	COLOR 0x0000_0800
	DUPLEX 0x0000_1000
	YRESOLUTION 0x0000_2000
	TTOPTION 0x0000_4000
	COLLATE 0x0000_8000
	FORMNAME 0x0001_0000
	LOGPIXELS 0x0002_0000
	BITSPERPEL 0x0004_0000
	PELSWIDTH 0x0008_0000
	PELSHEIGHT 0x0010_0000
	DISPLAYFLAGS 0x0020_0000
	DISPLAYFREQUENCY 0x0040_0000
	ICMMETHOD 0x0080_0000
	ICMINTENT 0x0100_0000
	MEDIATYPE 0x0200_0000
	DITHERTYPE 0x0400_0000
	PANNINGWIDTH 0x0800_0000
	PANNINGHEIGHT 0x1000_0000
	DISPLAYFIXEDOUTPUT 0x2000_0000
}

const_ordinary! { DMBIN: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmDefaultSource` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	UPPER 1
	ONLYONE 1
	LOWER 2
	MIDDLE 3
	MANUAL 4
	ENVELOPE 5
	ENVMANUAL 6
	AUTO 7
	TRACTOR 8
	SMALLFMT 9
	LARGEFMT 10
	LARGECAPACITY 11
	CASSETTE 14
	FORMSOURCE 15
	LAST Self::FORMSOURCE.0
	/// Device-specific bins start here.
	USER 256
}

const_ordinary! { DMCOLOR: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmColor` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	MONOCHROME 1
	COLOR 2
}

const_ordinary! { DMDFO: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmDisplayFixedOutput` (`u32`).
	=>
	=>
	DEFAULT 0
	STRETCH 1
	CENTER 2
}

const_bitflag! { DMDISPLAYFLAGS: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmDisplayFlags` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	INTERLACED 0x0000_0002
	TEXTMODE 0x0000_0004
}

const_ordinary! { DMDITHER: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmDitherType` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// No dithering.
	NONE 1
	/// Dither with a coarse brush.
	COARSE 2
	/// Dither with a fine brush.
	FINE 3
	/// LineArt dithering.
	LINEART 4
	/// LineArt dithering.
	ERRORDIFFUSION 5
	/// LineArt dithering.
	RESERVED6 6
	/// LineArt dithering.
	RESERVED7 7
	/// LineArt dithering.
	RESERVED8 8
	/// LineArt dithering.
	RESERVED9 9
	/// Device does grayscaling.
	GRAYSCALE 10
	/// Device-specific dithers start here.
	USER 256
}

const_ordinary! { DMDO: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmDisplayOrientation` (`u32`).
	=>
	=>
	DEFAULT 0
	D90 1
	D180 2
	D270 3
}

const_ordinary! { DMDUP: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmDuplex` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SIMPLEX 1
	VERTICAL 2
	HORIZONTAL 3
}

const_ordinary! { DMICM: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmICMIntent` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Maximize color saturation.
	SATURATE 1
	/// Maximize color contrast.
	CONTRAST 2
	/// Use specific color metric.
	COLORIMETRIC 3
	/// Use specific color metric.
	ABS_COLORIMETRIC 4
	/// Device-specific intents start here.
	USER 256
}

const_ordinary! { DMICMMETHOD: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmICMMethod` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// ICM disabled.
	NONE 1
	/// ICM handled by system.
	SYSTEM 2
	/// ICM handled by driver.
	DRIVER 3
	/// ICM handled by device.
	DEVICE 4
	/// Device-specific intents start here.
	USER 256
}

const_ordinary! { DMMEDIA: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmMediaType` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Standard paper.
	STANDARD 1
	/// Transparency.
	TRANSPARENCY 2
	/// Glossy paper.
	GLOSSY 3
	/// Device-specific media start here.
	USER 256
}

const_ordinary! { DMNUP: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmNup` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SYSTEM 1
	ONEUP 2
}

const_ordinary! { DMORIENT: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmOrientation` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	PORTRAIT 1
	LANDSCAPE 2
}

const_ordinary! { DMPAPER: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmPaperSize` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Letter 8 1/2 x 11 in.
	LETTER 1
	/// Letter Small 8 1/2 x 11 in.
	LETTERSMALL 2
	/// Tabloid 11 x 17 in.
	TABLOID 3
	/// Ledger 17 x 11 in.
	LEDGER 4
	/// Legal 8 1/2 x 14 in.
	LEGAL 5
	/// Statement 5 1/2 x 8 1/2 in.
	STATEMENT 6
	/// Executive 7 1/4 x 10 1/2 in.
	EXECUTIVE 7
	/// A3 297 x 420 mm.
	A3 8
	/// A4 210 x 297 mm.
	A4 9
	/// A4 Small 210 x 297 mm.
	A4SMALL 10
	/// A5 148 x 210 mm.
	A5 11
	/// B4 (JIS) 250 x 354.
	B4 12
	/// B5 (JIS) 182 x 257 mm.
	B5 13
	/// Folio 8 1/2 x 13 in.
	FOLIO 14
	/// Quarto 215 x 275 mm.
	QUARTO 15
	/// 10x14 in.
	P10X14 16
	/// 11x17 in.
	P11X17 17
	/// Note 8 1/2 x 11 in.
	NOTE 18
	/// Envelope #9 3 7/8 x 8 7/8.
	ENV_9 19
	/// Envelope #10 4 1/8 x 9 1/2.
	ENV_10 20
	/// Envelope #11 4 1/2 x 10 3/8.
	ENV_11 21
	/// Envelope #12 4 \276 x 11.
	ENV_12 22
	/// Envelope #14 5 x 11 1/2.
	ENV_14 23
	/// C size sheet.
	CSHEET 24
	/// D size sheet.
	DSHEET 25
	/// E size sheet.
	ESHEET 26
	/// Envelope DL 110 x 220mm.
	ENV_DL 27
	/// Envelope C5 162 x 229 mm.
	ENV_C5 28
	/// Envelope C3 324 x 458 mm.
	ENV_C3 29
	/// Envelope C4 229 x 324 mm.
	ENV_C4 30
	/// Envelope C6 114 x 162 mm.
	ENV_C6 31
	/// Envelope C65 114 x 229 mm.
	ENV_C65 32
	/// Envelope B4 250 x 353 mm.
	ENV_B4 33
	/// Envelope B5 176 x 250 mm.
	ENV_B5 34
	/// Envelope B6 176 x 125 mm.
	ENV_B6 35
	/// Envelope 110 x 230 mm.
	ENV_ITALY 36
	/// Envelope Monarch 3.875 x 7.5 in.
	ENV_MONARCH 37
	/// 6 3/4 Envelope 3 5/8 x 6 1/2 in.
	ENV_PERSONAL 38
	/// US Std Fanfold 14 7/8 x 11 in.
	FANFOLD_US 39
	/// German Std Fanfold 8 1/2 x 12 in.
	FANFOLD_STD_GERMAN 40
	/// German Legal Fanfold 8 1/2 x 13 in.
	FANFOLD_LGL_GERMAN 41
	/// B4 (ISO) 250 x 353 mm.
	ISO_B4 42
	/// Japanese Postcard 100 x 148 mm.
	JAPANESE_POSTCARD 43
	/// 9 x 11 in.
	P9X11 44
	/// 10 x 11 in.
	P10X11 45
	/// 15 x 11 in.
	P15X11 46
	/// Envelope Invite 220 x 220 mm.
	ENV_INVITE 47
	/// Letter Extra 9 275 x 12 in.
	LETTER_EXTRA 50
	/// Legal Extra 9 275 x 15 in.
	LEGAL_EXTRA 51
	/// Tabloid Extra 11.69 x 18 in.
	TABLOID_EXTRA 52
	/// A4 Extra 9.27 x 12.69 in.
	A4_EXTRA 53
	/// Letter Transverse 8 275 x 11 in.
	LETTER_TRANSVERSE 54
	/// A4 Transverse 210 x 297 mm.
	A4_TRANSVERSE 55
	/// Letter Extra Transverse 9\275 x 12 in.
	LETTER_EXTRA_TRANSVERSE 56
	/// SuperA/SuperA/A4 227 x 356 mm.
	A_PLUS 57
	/// SuperB/SuperB/A3 305 x 487 mm.
	B_PLUS 58
	/// Letter Plus 8.5 x 12.69 in.
	ETTER_PLUS 59
	/// A4 Plus 210 x 330 mm.
	A4_PLUS 60
	/// A5 Transverse 148 x 210 mm.
	A5_TRANSVERSE 61
	/// B5 (JIS) Transverse 182 x 257 mm.
	B5_TRANSVERSE 62
	/// A3 Extra 322 x 445 mm.
	A3_EXTRA 63
	/// A5 Extra 174 x 235 mm.
	A5_EXTRA 64
	/// B5 (ISO) Extra 201 x 276 mm.
	B5_EXTRA 65
	/// A2 420 x 594 mm.
	A2 66
	/// A3 Transverse 297 x 420 mm.
	A3_TRANSVERSE 67
	/// A3 Extra Transverse 322 x 445 mm.
	A3_EXTRA_TRANSVERSE 68
	/// Japanese Double Postcard 200 x 148 mm.
	DBL_JAPANESE_POSTCARD 69
	/// A6 105 x 148 mm.
	A6 70
	/// Japanese Envelope Kaku #2.
	JENV_KAKU2 71
	/// Japanese Envelope Kaku #3.
	JENV_KAKU3 72
	/// Japanese Envelope Chou #3.
	JENV_CHOU3 73
	/// Japanese Envelope Chou #4.
	JENV_CHOU4 74
	/// Letter Rotated 11 x 8 1/2 11 in.
	LETTER_ROTATED 75
	/// A3 Rotated 420 x 297 mm.
	A3_ROTATED 76
	/// A4 Rotated 297 x 210 mm.
	A4_ROTATED 77
	/// A5 Rotated 210 x 148 mm.
	A5_ROTATED 78
	/// B4 (JIS) Rotated 364 x 257 mm.
	B4_JIS_ROTATED 79
	/// B5 (JIS) Rotated 257 x 182 mm.
	B5_JIS_ROTATED 80
	/// Japanese Postcard Rotated 148 x 100 mm.
	JAPANESE_POSTCARD_ROTATED 81
	/// Double Japanese Postcard Rotated 148 x 200 mm.
	DBL_JAPANESE_POSTCARD_ROTATED 82
	/// A6 Rotated 148 x 105 mm.
	A6_ROTATED 83
	/// Japanese Envelope Kaku #2 Rotated.
	JENV_KAKU2_ROTATED 84
	/// Japanese Envelope Kaku #3 Rotated.
	JENV_KAKU3_ROTATED 85
	/// Japanese Envelope Chou #3 Rotated.
	JENV_CHOU3_ROTATED 86
	/// Japanese Envelope Chou #4 Rotated.
	JENV_CHOU4_ROTATED 87
	/// B6 (JIS) 128 x 182 mm.
	B6_JIS 88
	/// B6 (JIS) Rotated 182 x 128 mm.
	B6_JIS_ROTATED 89
	/// 12 x 11 in.
	P12X11 90
	/// Japanese Envelope You #4.
	JENV_YOU4 91
	/// Japanese Envelope You #4 Rotated.
	JENV_YOU4_ROTATED 92
	/// PRC 16K 146 x 215 mm.
	P16K 93
	/// PRC 32K 97 x 151 mm.
	P32K 94
	/// PRC 32K (Big) 97 x 151 mm.
	P32KBIG 95
	/// PRC Envelope #1 102 x 165 mm.
	PENV_1 96
	/// PRC Envelope #2 102 x 176 mm.
	PENV_2 97
	/// PRC Envelope #3 125 x 176 mm.
	PENV_3 98
	/// PRC Envelope #4 110 x 208 mm.
	PENV_4 99
	/// PRC Envelope #5 110 x 220 mm.
	PENV_5 100
	/// PRC Envelope #6 120 x 230 mm.
	PENV_6 101
	/// PRC Envelope #7 160 x 230 mm.
	PENV_7 102
	/// PRC Envelope #8 120 x 309 mm.
	PENV_8 103
	/// PRC Envelope #9 229 x 324 mm.
	PENV_9 104
	/// PRC Envelope #10 324 x 458 mm.
	PENV_10 105
	/// PRC 16K Rotated.
	P16K_ROTATED 106
	/// PRC 32K Rotated.
	P32K_ROTATED 107
	/// PRC 32K(Big) Rotated.
	P32KBIG_ROTATED 108
	/// PRC Envelope #1 Rotated 165 x 102 mm.
	PENV_1_ROTATED 109
	/// PRC Envelope #2 Rotated 176 x 102 mm.
	PENV_2_ROTATED 110
	/// PRC Envelope #3 Rotated 176 x 125 mm.
	PENV_3_ROTATED 111
	/// PRC Envelope #4 Rotated 208 x 110 mm.
	PENV_4_ROTATED 112
	/// PRC Envelope #5 Rotated 220 x 110 mm.
	PENV_5_ROTATED 113
	/// PRC Envelope #6 Rotated 230 x 120 mm.
	PENV_6_ROTATED 114
	/// PRC Envelope #7 Rotated 230 x 160 mm.
	PENV_7_ROTATED 115
	/// PRC Envelope #8 Rotated 309 x 120 mm.
	PENV_8_ROTATED 116
	/// PRC Envelope #9 Rotated 324 x 229 mm.
	PENV_9_ROTATED 117
	/// PRC Envelope #10 Rotated 458 x 324 mm.
	PENV_10_ROTATED 118
	/// Other papers start here.
	USER 256
}

const_ordinary! { DMRES: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmPrintQuality` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	DRAFT -1
	LOW -2
	MEDIUM -3
	HIGH -4
}

const_ordinary! { DMTT: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmTTOption` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Print TT fonts as graphics.
	BITMAP 1
	/// Download TT fonts as soft fonts.
	DOWNLOAD 2
	/// Substitude device fonts for TT fonts.
	SUBDEV 3
	/// Download TT fonts as outline soft fonts.
	DOWNLOAD_OUTLINE 4
}

const_ordinary! { DLGC: u16;
	/// [`wm::GetDlgCode`](crate::msg::wm::GetDlgCode) return value (`u16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	BUTTON 0x2000
	DEFPUSHBUTTON 0x0010
	HASSETSEL 0x0008
	RADIOBUTTON 0x0040
	STATIC 0x0100
	UNDEFPUSHBUTTON 0x0020
	WANTALLKEYS 0x0004
	WANTARROWS 0x0001
	WANTCHARS 0x0080
	WANTMESSAGE 0x0004
	WANTTAB 0x0002
}

const_ordinary! { DLGID: u16;
	/// Dialog built-in IDs (`u16`). These are also returned from
	/// [`HWND::MessageBox`](crate::prelude::user_Hwnd::MessageBox) and
	/// [`HWND::TaskDialog`](crate::prelude::comctl_Hwnd::TaskDialog).
	=>
	=>
	OK 1
	CANCEL 2
	ABORT 3
	RETRY 4
	IGNORE 5
	YES 6
	NO 7
	TRYAGAIN 10
	CONTINUE 11
}

const_bitflag! { DT: u32;
	/// [`HDC::DrawText`](crate::prelude::user_Hdc::DrawText) `format` (`u32`).
	=>
	=>
	TOP 0x0000_0000
	LEFT 0x0000_0000
	CENTER 0x0000_0001
	RIGHT 0x0000_0002
	VCENTER 0x0000_0004
	BOTTOM 0x0000_0008
	WORDBREAK 0x0000_0010
	SINGLELINE 0x0000_0020
	EXPANDTABS 0x0000_0040
	TABSTOP 0x0000_0080
	NOCLIP 0x0000_0100
	EXTERNALLEADING 0x0000_0200
	CALCRECT 0x0000_0400
	NOPREFIX 0x0000_0800
	INTERNAL 0x0000_1000
	EDITCONTROL 0x0000_2000
	PATH_ELLIPSIS 0x0000_4000
	END_ELLIPSIS 0x0000_8000
	MODIFYSTRING 0x0001_0000
	RTLREADING 0x0002_0000
	WORD_ELLIPSIS 0x0004_0000
	NOFULLWIDTHCHARBREAK 0x0008_0000
	HIDEPREFIX 0x0010_0000
	PREFIXONLY 0x0020_0000
}

const_bitflag! { EC: u16;
	/// [`em::GetImeStatus`](crate::msg::em::SetMargins) margins to set (`u16`).
	=>
	=>
	LEFTMARGIN 0x0001
	RIGHTMARGIN 0x0002
	USEFONTINFO 0xffff
}

const_bitflag! { EDD: u32;
	/// [`EnumDisplayDevices`](crate::EnumDisplayDevices) `flags` (`u32`).
	=>
	=>
	GET_DEVICE_INTERFACE_NAME 0x0000_0001
}

const_bitflag! { EDS: u32;
	/// [`EnumDisplaySettingsEx`](crate::EnumDisplaySettingsEx) `flags` (`u32`).
	=>
	=>
	RAWMODE 0x0000_0002
	ROTATEDMODE 0x0000_0004
}

const_bitflag! { EIMES: u16;
	/// [`em::GetImeStatus`](crate::msg::em::GetImeStatus) and
	/// [`em::SetImeStatus`](crate::msg::em::SetImeStatus) status (`u16`).
	=>
	=>
	GETCOMPSTRATONCE 0x0001
	CANCELCOMPSTRINFOCUS 0x0002
	COMPLETECOMPSTRKILLFOCUS 0x0004
}

const_wm! { EM;
	/// Edit control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-messages)
	/// (`u32`).
	=>
	=>
	GETSEL 0x00b0
	SETSEL 0x00b1
	GETRECT 0x00b2
	SETRECT 0x00b3
	SETRECTNP 0x00b4
	SCROLL 0x00b5
	LINESCROLL 0x00b6
	SCROLLCARET 0x00b7
	GETMODIFY 0x00b8
	SETMODIFY 0x00b9
	GETLINECOUNT 0x00ba
	LINEINDEX 0x00bb
	SETHANDLE 0x00bc
	GETHANDLE 0x00bd
	GETTHUMB 0x00be
	LINELENGTH 0x00c1
	REPLACESEL 0x00c2
	GETLINE 0x00c4
	LIMITTEXT 0x00c5
	CANUNDO 0x00c6
	UNDO 0x00c7
	FMTLINES 0x00c8
	LINEFROMCHAR 0x00c9
	SETTABSTOPS 0x00cb
	SETPASSWORDCHAR 0x00cc
	EMPTYUNDOBUFFER 0x00cd
	GETFIRSTVISIBLELINE 0x00ce
	SETREADONLY 0x00cf
	SETWORDBREAKPROC 0x00d0
	GETWORDBREAKPROC 0x00d1
	GETPASSWORDCHAR 0x00d2
	SETMARGINS 0x00d3
	GETMARGINS 0x00d4
	SETLIMITTEXT Self::LIMITTEXT.0
	GETLIMITTEXT 0x00d5
	POSFROMCHAR 0x00d6
	CHARFROMPOS 0x00d7
	SETIMESTATUS 0x00d8
	GETIMESTATUS 0x00d9
	ENABLEFEATURE 0x00da
}

const_cmd! { EN;
	/// Edit control `WM_COMMAND`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications)
	/// (`u16`).
	=>
	=>
	SETFOCUS 0x0100
	KILLFOCUS 0x0200
	CHANGE 0x0300
	UPDATE 0x0400
	ERRSPACE 0x0500
	MAXTEXT 0x0501
	HSCROLL 0x0601
	VSCROLL 0x0602
	ALIGN_LTR_EC 0x0700
	ALIGN_RTL_EC 0x0701
	BEFORE_PASTE 0x0800
	AFTER_PASTE 0x0801
}

const_bitflag! { ENDSESSION: u32;
	/// [`wm::EndSession`](crate::msg::wm::EndSession) event (`u32`).
	=>
	=>
	RESTARTORSHUTDOWN 0
	CLOSEAPP 0x0000_0001
	CRITICAL 0x4000_0000
	LOGOFF 0x8000_0000
}

const_ordinary! { ENUM_SETTINGS: u32;
	/// [`EnumDisplaySettingsEx`](crate::EnumDisplaySettingsEx) `mode_num`
	/// (`u32`).
	///
	/// Originally with `ENUM` prefix and `SETTINGS` suffix.
	=>
	=>
	CURRENT -1i32 as u32
	REGISTRY -2i32 as u32
}

const_ws! { ES: u32;
	/// Edit control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/edit-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Aligns text with the left margin.
	LEFT 0x0000
	/// Centers text in a single-line or multiline edit control.
	CENTER 0x0001
	/// Right-aligns text in a single-line or multiline edit control.
	RIGHT 0x0002
	/// Designates a multiline edit control.
	MULTILINE 0x0004
	/// Converts all characters to uppercase as they are typed into the edit
	/// control.
	UPPERCASE 0x0008
	/// Converts all characters to lowercase as they are typed into the edit
	/// control.
	LOWERCASE 0x0010
	/// Displays an asterisk (*) for each character typed into the edit control.
	/// This style is valid only for single-line edit controls. To change the
	/// characters that is displayed, or set or clear this style, use the
	/// [`EM_SETPASSWORDCHAR`](crate::msg::em::SetPasswordChar) message.
	PASSWORD 0x0020
	/// Automatically scrolls text up one page when the user presses the ENTER
	/// key on the last line.
	AUTOVSCROLL 0x0040
	/// Automatically scrolls text to the right by 10 characters when the user
	/// types a character at the end of the line. When the user presses the
	/// ENTER key, the control scrolls all text back to position zero.
	AUTOHSCROLL 0x0080
	/// Negates the default behavior for an edit control. The default behavior
	/// hides the selection when the control loses the input focus and inverts
	/// the selection when the control receives the input focus.
	NOHIDESEL 0x0100
	/// Converts text entered in the edit control. The text is converted from
	/// the Windows character set to the OEM character set and then back to the
	/// Windows character set. This style is most useful for edit controls that
	/// contain file names that will be used on file systems that do not support
	/// Unicode.
	OEMCONVERT 0x0400
	/// Prevents the user from typing or editing text in the edit control.
	READONLY 0x0800
	/// Specifies that a carriage return be inserted when the user presses the
	/// ENTER key while entering text into a multiline edit control in a dialog
	/// box. If you do not specify this style, pressing the ENTER key has the
	/// same effect as pressing the dialog box's default push button. This style
	/// has no effect on a single-line edit control.
	WANTRETURN 0x1000
	/// Allows only digits to be entered into the edit control. Note that, even
	/// with this set, it is still possible to paste non-digits into the edit
	/// control.
	NUMBER 0x2000
}

const_ordinary! { ESB: u32;
	/// [`HWND::EnableScrollBar`](crate::prelude::user_Hwnd::EnableScrollBar)
	/// `arrows` (`u32`).
	=>
	=>
	ENABLE_BOTH 0x0000
	DISABLE_BOTH 0x0003
	DISABLE_LEFT 0x0001
	DISABLE_RIGHT 0x0002
	DISABLE_UP 0x0001
	DISABLE_DOWN 0x0002
	DISABLE_LTUP Self::DISABLE_LEFT.0
	DISABLE_RTDN Self::DISABLE_RIGHT.0
}

const_bitflag! { EWX: u32;
	/// [`ExitWindowsEx`](crate::ExitWindowsEx) `flags` (`u32`).
	=>
	=>
	HYBRID_SHUTDOWN 0x0040_0000
	LOGOFF 0
	POWEROFF 0x0000_0008
	REBOOT 0x0000_0002
	RESTARTAPPS 0x0000_0040
	SHUTDOWN 0x0000_0001

	FORCE 0x0000_0004
	FORCEIFHUNG 0x0000_0010
}

const_ordinary! { FAPPCOMMAND: u16;
	/// [`wm::AppCommand`](crate::msg::wm::AppCommand) input event (`u16`).
	=>
	=>
	MOUSE 0x8000
	KEY 0
	OEM 0x1000
}

const_ordinary! { GA: u32;
	/// [`HWND::GetAncestor`](crate::prelude::user_Hwnd::GetAncestor) `flags`
	/// (`u32`).
	=>
	=>
	/// Retrieves the parent window. This does not include the owner as it does
	/// with the [`HWND::GetParent`](crate::prelude::user_Hwnd::GetParent)
	/// function.
	PARENT 1
	/// Retrieves the root window by walking the chain of parent windows.
	///
	/// Returns the
	/// [closest](https://groups.google.com/a/chromium.org/g/chromium-dev/c/Hirr_DkuZdw/m/N0pSoJBhAAAJ)
	/// parent with [`WS::OVERLAPPED`](crate::co::WS::OVERLAPPED) or
	/// [`WS::POPUP`](crate::co::WS::POPUP).
	ROOT 2
	/// Retrieves the owned root window by walking the chain of parent and owner
	/// windows returned by
	/// [`HWND::GetParent`](crate::prelude::user_Hwnd::GetParent).
	///
	/// Returns the
	/// [furthest](https://groups.google.com/a/chromium.org/g/chromium-dev/c/Hirr_DkuZdw/m/N0pSoJBhAAAJ)
	/// parent with [`WS::OVERLAPPED`](crate::co::WS::OVERLAPPED) or
	/// [`WS::POPUP`](crate::co::WS::POPUP) which usually is the main
	/// application window.
	ROOTOWNER 3
}

const_ordinary! { GCLP: i32;
	/// [`HWND::GetClassLongPtr`](crate::prelude::user_Hwnd::GetClassLongPtr)
	/// `index` (`i32`).
	///
	/// Originally has `GCW` and `GCL` prefixes.
	=>
	=>
	ATOM -32
	CBWNDEXTRA -18
	CBCLSEXTRA -20
	MENUNAME -8
	HBRBACKGROUND -10
	HCURSOR -12
	HICON -14
	HMODULE -16
	WNDPROC -24
	HICONSM -34
}

const_bitflag! { GMDI: u32;
	/// [`HMENU::GetMenuDefaultItem`](crate::prelude::user_Hmenu::GetMenuDefaultItem)
	/// `flags` (`u32`).
	=>
	=>
	USEDISABLED 0x0001
	GOINTOPOPUPS 0x0002
}

const_bitflag! { GUI: u32;
	/// [`GUITHREADINFO`](crate::GUITHREADINFO) `flags` (`u32`).
	=>
	=>
	CARETBLINKING 0x0000_0001
	INMENUMODE 0x0000_0004
	INMOVESIZE 0x0000_0002
	POPUPMENUMODE 0x0000_00010
	SYSTEMMENUMODE 0x0000_0008
}

const_ordinary! { GW: u32;
	/// [`HWND::GetWindow`](crate::prelude::user_Hwnd::GetWindow) `cmd` (`u32`).
	=>
	=>
	HWNDFIRST 0
	HWNDLAST 1
	HWNDNEXT 2
	HWNDPREV 3
	OWNER 4
	CHILD 5
	ENABLEDPOPUP 6
	MAX 6
}

const_ordinary! { GWL_C: i8;
	/// [`wm::StyleChanged`](crate::msg::wm::StyleChanged) and
	/// [`wm::StyleChanging`](crate::msg::wm::StyleChanging) change (`i8`).
	///
	/// Originally has `GWL` prefix.
	=>
	=>
	EXSTYLE -20
	STYLE -16
}

const_ordinary! { GWLP: i32;
	/// [`HWND::GetWindowLongPtr`](crate::prelude::user_Hwnd::GetWindowLongPtr)
	/// and
	/// [`HWND::SetWindowLongPtr`](crate::prelude::user_Hwnd::SetWindowLongPtr)
	/// `index` (`i32`).
	///
	/// Originally has prefixes `GWL`, `GWLP`, `DWL` and `DWLP`.
	=>
	=>
	WNDPROC -4
	HINSTANCE -6
	HWNDPARENT -8
	ID -12
	STYLE -16
	EXSTYLE -20
	USERDATA -21

	DWLP_MSGRESULT 0
	DWLP_DLGPROC Self::DWLP_MSGRESULT.0 + std::mem::size_of::<isize>() as i32
	DWLP_USER Self::DWLP_DLGPROC.0 + std::mem::size_of::<isize>() as i32
}

const_ordinary! { HELPINFO: i32;
	/// [`HELPINFO`](crate::HELPINFO) `iContextType` (`i32`).
	=>
	=>
	WINDOW 0x0001
	MENUITEM 0x0002
}

const_ordinary! { HELPW: u32;
	/// [`HWND::WinHelp`](crate::prelude::user_Hwnd::WinHelp) `uCommand`
	/// (`u32`).
	///
	/// Originally has `HELP` prefix.
	=>
	=>
	CONTEXT 0x0001
	QUIT 0x0002
	INDEX 0x0003
	CONTENTS 0x0003
	HELPONHELP 0x0004
	SETINDEX 0x0005
	SETCONTENTS 0x0005
	CONTEXTPOPUP 0x0008
	FORCEFILE 0x0009
	KEY 0x0101
	COMMAND 0x0102
	PARTIALKEY 0x0105
	MULTIKEY 0x0201
	SETWINPOS 0x0203
	CONTEXTMENU 0x000a
	FINDER 0x000b
	WM_HELP 0x000c
	SETPOPUP_POS 0x000d
	TCARD 0x8000
	TCARD_DATA 0x0010
	TCARD_OTHER_CALLER 0x0011
}

const_ordinary! { HT: u16;
	/// [`wm::NcHitTest`](crate::msg::wm::NcHitTest),
	/// [`wm::SetCursor`](crate::msg::wm::SetCursor) `hit_test` (`u16`).
	=>
	=>
	BORDER 18
	BOTTOM 15
	BOTTOMLEFT 16
	BOTTOMRIGHT 17
	CAPTION 2
	CLIENT 1
	CLOSE 20
	ERROR -2i16 as u16
	GROWBOX 4
	HELP 21
	HSCROLL 6
	LEFT 10
	MENU 5
	MAXBUTTON 9
	MINBUTTON 8
	NOWHERE 0
	REDUCE 8
	RIGHT 11
	SIZE 4
	SYSMENU 3
	TOP 12
	TOPLEFT 13
	TOPRIGHT 14
	TRANSPARENT 1i16 as u16
	VSCROLL 7
	ZOOM 9
}

const_ordinary! { HWND_PLACE: isize;
	/// [`HWND::SetWindowPos`](crate::prelude::user_Hwnd::SetWindowPos)
	/// `hWndInsertAfter` (`isize`).
	=>
	=>
	TOP 0
	BOTTOM 1
	TOPMOST -1
	NOTOPMOST -2
}

const_ordinary! { ICON_SZ: u8;
	/// [`wm::SetIcon`](crate::msg::wm::SetIcon) icon size (`u8`).
	///
	/// Originally has `ICON` prefix.
	=>
	=>
	SMALL 0
	BIG 1
}

const_ordinary! { IDC: u32;
	/// [`HINSTANCE::LoadCursor`](crate::prelude::user_Hinstance::LoadCursor)
	/// `lpCursorName` (`u32`).
	=>
	=>
	ARROW 32512
	IBEAM 32513
	WAIT 32514
	CROSS 32515
	UPARROW 32516
	SIZENWSE 32642
	SIZENESW 32643
	SIZEWE 32644
	SIZENS 32645
	SIZEALL 32646
	NO 32648
	HAND 32649
	APPSTARTING 32650
	HELP 32651
	PIN 32671
	PERSON 32672
}

const_ordinary! { IDI: u32;
	/// [`HINSTANCE::LoadIcon`](crate::prelude::user_Hinstance::LoadIcon)
	/// `lpIconName` (`u32`).
	=>
	=>
	APPLICATION 32512
	HAND 32513
	QUESTION 32514
	EXCLAMATION 32515
	ASTERISK 32516
	WINLOGO 32517
	SHIELD 32518
	WARNING Self::EXCLAMATION.0
	ERROR Self::HAND.0
	INFORMATION Self::ASTERISK.0
}

const_ordinary! { IMAGE_TYPE: u8;
	/// [`bm::GetImage`](crate::msg::bm::GetImage) `img_type`;
	/// [`stm::GetImage`](crate::msg::stm::GetImage) `img_type` (`u8`).
	///
	/// Originally has `IMAGE` prefix.
	=>
	=>
	BITMAP 0
	ICON 1
	CURSOR 2
	ENHMETAFILE 3
}

const_ordinary! { INPUT: u32;
	/// [`INPUT`](crate::INPUT) `type` (`u32`).
	=>
	=>
	MOUSE 0
	KEYBOARD 1
	HARDWARE 2
}

#[cfg(target_pointer_width = "64")]
const_bitflag! { ISMEX: u32;
	/// [`InSendMessageEx`](crate::InSendMessageEx) return value (`u32`).
	///
	/// **Note:** This constant doesn't exist in x32.
	=>
	=>
	NOSEND 0x0000_0000
	CALLBACK 0x0000_0004
	NOTIFY 0x0000_0002
	REPLIED 0x0000_0008
	SEND 0x0000_0001
}

const_bitflag! { KEYEVENTF: u32;
	/// [`KEYBDINPUT`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-keybdinput)
	/// `dwFlags` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	EXTENDEDKEY 0x0001
	KEYUP 0x0002
	UNICODE 0x0004
	SCANCODE 0x0008
}

const_wm! { LB;
	/// List box control
	/// [messages](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-messages)
	/// (`u32`).
	=>
	=>
	ADDSTRING 0x0180
	INSERTSTRING 0x0181
	DELETESTRING 0x0182
	SELITEMRANGEEX 0x0183
	RESETCONTENT 0x0184
	SETSEL 0x0185
	SETCURSEL 0x0186
	GETSEL 0x0187
	GETCURSEL 0x0188
	GETTEXT 0x0189
	GETTEXTLEN 0x018a
	GETCOUNT 0x018b
	SELECTSTRING 0x018c
	DIR 0x018d
	GETTOPINDEX 0x018e
	FINDSTRING 0x018f
	GETSELCOUNT 0x0190
	GETSELITEMS 0x0191
	SETTABSTOPS 0x0192
	GETHORIZONTALEXTENT 0x0193
	SETHORIZONTALEXTENT 0x0194
	SETCOLUMNWIDTH 0x0195
	ADDFILE 0x0196
	SETTOPINDEX 0x0197
	GETITEMRECT 0x0198
	GETITEMDATA 0x0199
	SETITEMDATA 0x019a
	SELITEMRANGE 0x019b
	SETANCHORINDEX 0x019c
	GETANCHORINDEX 0x019d
	SETCARETINDEX 0x019e
	GETCARETINDEX 0x019f
	SETITEMHEIGHT 0x01a0
	GETITEMHEIGHT 0x01a1
	FINDSTRINGEXACT 0x01a2
	SETLOCALE 0x01a5
	GETLOCALE 0x01a6
	SETCOUNT 0x01a7
	INITSTORAGE 0x01a8
	ITEMFROMPOINT 0x01a9
	GETLISTBOXINFO 0x01b2
}

const_cmd! { LBN;
	/// List box control `WM_COMMAND`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-notifications)
	/// (`u16`).
	=>
	=>
	ERRSPACE -2i16 as _
	SELCHANGE 1
	DBLCLK 2
	SELCANCEL 3
	SETFOCUS 4
	KILLFOCUS 5
}

const_ws! { LBS: u32;
	/// List box control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/list-box-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	NOTIFY 0x0001
	SORT 0x0002
	NOREDRAW 0x0004
	MULTIPLESEL 0x0008
	OWNERDRAWFIXED 0x0010
	OWNERDRAWVARIABLE 0x0020
	HASSTRINGS 0x0040
	USETABSTOPS 0x0080
	NOINTEGRALHEIGHT 0x0100
	MULTICOLUMN 0x0200
	WANTKEYBOARDINPUT 0x0400
	EXTENDEDSEL 0x0800
	DISABLENOSCROLL 0x1000
	NODATA 0x2000
	NOSEL 0x4000
	COMBOBOX 0x8000
	STANDARD Self::NOTIFY.0 | Self::SORT.0 | WS::VSCROLL.0 | WS::BORDER.0
}

const_ordinary! { LSFW: u32;
	/// [`LockSetForegroundWindow`](crate::LockSetForegroundWindow) `lock_code`
	/// (`u32`).
	=>
	=>
	LOCK 1
	UNLOCK 2
}

const_bitflag! { LWA: u32;
	/// [`HWND::SetLayeredWindowAttributes`](crate::prelude::user_Hwnd::SetLayeredWindowAttributes)
	/// `flags` (`u32`).
	=>
	=>
	ALPHA 0x0000_0002
	COLORKEY 0x0000_0001
}

const_bitflag! { MB: u32;
	/// [`HWND::MessageBox`](crate::prelude::user_Hwnd::MessageBox) `flags`
	/// (`u32`).
	=>
	=>
	/// The message box contains three push buttons: Abort Retry and Ignore.
	ABORTRETRYIGNORE 0x0000_0002
	/// The message box contains three push buttons: Cancel Try Again,
	/// Continue. Use this message box type instead of
	/// [`MB::ABORTRETRYIGNORE`](crate::co::MB::ABORTRETRYIGNORE).
	CANCELTRYCONTINUE 0x0000_0006
	/// Adds a Help button to the message box. When the user clicks the Help
	/// button or presses F1 the system sends a
	/// [`wm::Help`](crate::msg::wm::Help) message to the owner.
	HELP 0x0000_4000
	/// The message box contains one push button: OK. This is the default.
	OK 0x0000_0000
	/// The message box contains two push buttons: OK and Cancel.
	OKCANCEL 0x0000_0001
	/// The message box contains two push buttons: Retry and Cancel.
	RETRYCANCEL 0x0000_0005
	/// The message box contains two push buttons: Yes and No.
	YESNO 0x0000_0004
	/// The message box contains three push buttons: Yes No and Cancel.
	YESNOCANCEL 0x0000_0003

	/// An exclamation-point icon appears in the message box.
	ICONEXCLAMATION 0x0000_0030
	/// An exclamation-point icon appears in the message box.
	ICONWARNING Self::ICONEXCLAMATION.0
	/// An icon consisting of a lowercase letter i in a circle appears in the
	/// message box.
	ICONINFORMATION 0x0000_0040
	/// An icon consisting of a lowercase letter i in a circle appears in the
	/// message box.
	ICONASTERISK Self::ICONINFORMATION.0
	/// A question-mark icon appears in the message box. The question-mark
	/// message icon is no longer recommended because it does not clearly
	/// represent a specific type of message and because the phrasing of a
	/// message as a question could apply to any message type. In addition,
	/// users can confuse the message symbol question mark with Help
	/// information. Therefore do not use this question mark message symbol in
	/// your message boxes. The system continues to support its inclusion only
	/// for backward compatibility.
	ICONQUESTION 0x0000_0020
	/// A stop-sign icon appears in the message box.
	ICONSTOP Self::ICONERROR.0
	/// A stop-sign icon appears in the message box.
	ICONERROR 0x0000_0010
	/// A stop-sign icon appears in the message box.
	ICONHAND Self::ICONERROR.0

	/// The first button is the default button. `MB::DEFBUTTON1` is the default
	/// unless [`MB::DEFBUTTON2`](crate::co::MB::DEFBUTTON2),
	/// [`MB::DEFBUTTON3`](crate::co::MB::DEFBUTTON3) or
	/// [`MB::DEFBUTTON4`](crate::co::MB::DEFBUTTON4) is specified.
	DEFBUTTON1 0x0000_0000
	/// The second button is the default button.
	DEFBUTTON2 0x0000_0100
	/// The third button is the default button.
	DEFBUTTON3 0x0000_0200
	/// The fourth button is the default button.
	DEFBUTTON4 0x0000_0300

	/// The user must respond to the message box before continuing work in the
	/// window identified by the hWnd parameter. However the user can move to
	/// the windows of other threads and work in those windows.
	///
	/// Depending on the hierarchy of windows in the application the user may
	/// be able to move to other windows within the thread. All child windows of
	/// the parent of the message box are automatically disabled but pop-up
	/// windows are not.
	///
	/// `MB::APPLMODAL` is the default if neither
	/// [`MB::SYSTEMMODAL`](crate::co::MB::SYSTEMMODAL) nor
	/// [`MB::TASKMODAL`](crate::co::MB::TASKMODAL) is specified.
	APPLMODAL 0x0000_0000
	/// Same as [`MB::APPLMODAL`](crate::co::MB::APPLMODAL) except that the
	/// message box has the [`WS_EX::TOPMOST`](crate::co::WS_EX::TOPMOST) style.
	/// Use system-modal message boxes to notify the user of serious,
	/// potentially damaging errors that require immediate attention (for
	/// example running out of memory). This flag has no effect on the user's
	/// ability to interact with windows other than those associated with hWnd.
	SYSTEMMODAL 0x0000_1000
	/// Same as [`MB::APPLMODAL`](crate::co::MB::APPLMODAL) except that all the
	/// top-level windows belonging to the current thread are disabled if the
	/// hWnd parameter is NULL. Use this flag when the calling application or
	/// library does not have a window handle available but still needs to
	/// prevent input to other windows in the calling thread without suspending
	/// other threads.
	TASKMODAL 0x0000_2000

	/// Same as desktop of the interactive window station. For more information,
	/// see
	/// [Window Stations](https://learn.microsoft.com/en-us/windows/win32/winstation/window-stations).
	///
	/// If the current input desktop is not the default desktop,
	/// [`HWND::MessageBox`](crate::prelude::user_Hwnd::MessageBox) does not
	/// return until the user switches to the default desktop.
	DEFAULT_DESKTOP_ONLY 0x0002_0000
	/// The text is right-justified.
	RIGHT 0x0008_0000
	/// Displays message and caption text using right-to-left reading order on
	/// Hebrew and Arabic systems.
	RTLREADING 0x0010_0000
	/// The message box becomes the foreground window. Internally the system
	/// calls the
	/// [`HWND::SetForegroundWindow`](crate::prelude::user_Hwnd::SetForegroundWindow)
	/// function for the message box.
	SETFOREGROUND 0x0001_0000
	/// The message box is created with the
	/// [`WS_EX::TOPMOST`](crate::co::WS_EX::TOPMOST) window style.
	TOPMOST 0x0004_0000
	/// The caller is a service notifying the user of an event. The function
	/// displays a message box on the current active desktop even if there is
	/// no user logged on to the computer.
	///
	/// Terminal Services: If the calling thread has an impersonation token the
	/// function directs the message box to the session specified in the
	/// impersonation token.
	///
	/// If this flag is set the `hWnd` parameter must be NULL. This is so that
	/// the message box can appear on a desktop other than the desktop
	/// corresponding to the `hWnd`.
	///
	/// For information on security considerations in regard to using this flag,
	/// see
	/// [Interactive Services](https://learn.microsoft.com/en-us/windows/win32/services/interactive-services).
	/// In particular be aware that this flag can produce interactive content
	/// on a locked desktop and should therefore be used for only a very limited
	/// set of scenarios such as resource exhaustion.
	SERVICE_NOTIFICATION 0x0020_0000
}

const_ordinary! { MDITILE: u32;
	/// [`HWND::TileWindows`](crate::prelude::user_Hwnd::TileWindows) `how`
	/// (`u32`).
	=>
	=>
	MDITILE_VERTICAL 0x0000
	HORIZONTAL 0x0001
	SKIPDISABLED 0x0002
}

const_bitflag! { MIIM: u32;
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fMask` (`u32`).
	=>
	=>
	BITMAP 0x0000_0080
	CHECKMARKS 0x0000_0008
	DATA 0x0000_0020
	FTYPE 0x0000_0100
	ID 0x0000_0002
	STATE 0x0000_0001
	STRING 0x0000_0040
	SUBMENU 0x0000_0004
	TYPE 0x0000_0010
}

const_bitflag! { MIM: u32;
	/// [`MENUINFO`](crate::MENUINFO) `fMask` (`u32`).
	=>
	=>
	MAXHEIGHT 0x0000_0001
	BACKGROUND 0x0000_0002
	HELPID 0x0000_0004
	MENUDATA 0x0000_0008
	STYLE 0x0000_0010
	APPLYTOSUBMENUS 0x8000_0000
}

const_bitflag! { MK: u16;
	/// [`wm::LButtonDown`](crate::msg::wm::LButtonDown) (and similar) virtual
	/// keys (`u16`).
	=>
	=>
	LBUTTON 0x0001
	RBUTTON 0x0002
	SHIFT 0x0004
	CONTROL 0x0008
	MBUTTON 0x0010
	XBUTTON1 0x0020
	XBUTTON2 0x0040
	ALT 0x20 // from oleidl.h
}

const_bitflag! { MF: u32;
	/// [`HMENU::AppendMenu`](crate::prelude::user_Hmenu::AppendMenu) `flags`,
	/// [`HMENU::GetMenuState`](crate::prelude::user_Hmenu::GetMenuState) return
	/// value,
	/// [`HWND::HiliteMenuItem`](crate::prelude::user_Hwnd::HiliteMenuItem)
	/// `hilite` (`u32`).
	=>
	=>
	INSERT 0x0000_0000
	CHANGE 0x0000_0080
	APPEND 0x0000_0100
	DELETE 0x0000_0200
	REMOVE 0x0000_1000
	BYCOMMAND 0x0000_0000
	BYPOSITION 0x0000_0400
	SEPARATOR 0x0000_0800
	ENABLED 0x0000_0000
	GRAYED 0x0000_0001
	DISABLED 0x0000_0002
	UNCHECKED 0x0000_0000
	CHECKED 0x0000_0008
	USECHECKBITMAPS 0x0000_0200
	STRING 0x0000_0000
	BITMAP 0x0000_0004
	OWNERDRAW 0x0000_0100
	POPUP 0x0000_0010
	MENUBARBREAK 0x0000_0020
	MENUBREAK 0x0000_0040
	UNHILITE 0x0000_0000
	HILITE 0x0000_0080
	DEFAULT 0x0000_1000
	SYSMENU 0x0000_2000
	HELP 0x0000_4000
	RIGHTJUSTIFY 0x0000_4000
	MOUSESELECT 0x0000_8000
}

const_bitflag! { MFS: u32;
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fState` (`u32`).
	=>
	=>
	GRAYED 0x0000_0003
	DISABLED MFS::GRAYED.0
	CHECKED MF::CHECKED.0
	HILITE MF::HILITE.0
	ENABLED MF::ENABLED.0
	UNCHECKED MF::UNCHECKED.0
	UNHILITE MF::UNHILITE.0
	DEFAULT MF::DEFAULT.0
}

const_bitflag! { MFT: u32;
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fType` (`u32`).
	=>
	=>
	STRING MF::STRING.0
	BITMAP MF::BITMAP.0
	MENUBARBREAK MF::MENUBARBREAK.0
	MENUBREAK MF::MENUBREAK.0
	OWNERDRAW MF::OWNERDRAW.0
	RADIOCHECK 0x0000_0200
	SEPARATOR MF::SEPARATOR.0
	RIGHTORDER 0x0000_2000
	RIGHTJUSTIFY MF::RIGHTJUSTIFY.0
}

const_ordinary! { MND: u8;
	/// [`wm::MenuDrag`](crate::msg::wm::MenuDrag) return value (`u8`).
	=>
	=>
	CONTINUE 0
	ENDMENU 1
}

const_bitflag! { MNS: u32;
	/// [`MENUINFO`](crate::MENUINFO) `dwStyle` (`u32`).
	=>
	=>
	NOCHECK 0x8000_0000
	MODELESS 0x4000_0000
	DRAGDROP 0x2000_0000
	AUTODISMISS 0x1000_0000
	NOTIFYBYPOS 0x0800_0000
	CHECKORBMP 0x0400_0000
}

const_ordinary! { MONITOR: u32;
	/// [`HMONITOR::MonitorFromPoint`](crate::prelude::user_Hmonitor::MonitorFromPoint),
	/// [`HMONITOR::MonitorFromRect`](crate::prelude::user_Hmonitor::MonitorFromRect),
	/// [`HWND::MonitorFromWindow`](crate::prelude::user_Hwnd::MonitorFromWindow)
	/// flags (`u32`).
	=>
	=>
	DEFAULTTONULL 0x0000_0000
	DEFAULTTOPRIMARY 0x0000_0001
	DEFAULTTONEAREST 0x0000_0002
}

const_ordinary! { MONITORINFOF: u32;
	/// [`MONITORINFOEX`](crate::MONITORINFOEX) `dwFlags` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	PRIMARY 0x0000_0001
}

const_bitflag! { MOUSEEVENTF: u32;
	/// [`MOUSEINPUT`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-mouseinput)
	/// `dwFlags` (`u32`).
	=>
	=>
	MOVE 0x0001
	LEFTDOWN 0x0002
	LEFTUP 0x0004
	RIGHTDOWN 0x0008
	RIGHTUP 0x0010
	MIDDLEDOWN 0x0020
	MIDDLEUP 0x0040
	XDOWN 0x0080
	XUP 0x0100
	WHEEL 0x0800
	HWHEEL 0x01000
	MOVE_NOCOALESCE 0x2000
	VIRTUALDESK 0x4000
	ABSOLUTE 0x8000
}

const_ordinary! { MSGF: u8;
	/// [`wm::EnterIdle`](crate::msg::wm::EnterIdle) reason (`u8`).
	=>
	=>
	DIALOGBOX 0
	MENU 2
}

const_ordinary! { OBJID: u32;
	/// [`HWND::GetMenuBarInfo`](crate::prelude::user_Hwnd::GetMenuBarInfo)
	/// `idObject` (`i32`).
	=>
	=>
	CLIENT 0xffff_fffc
	MENU 0xffff_fffd
	SYSMENU 0xffff_ffff
}

const_ordinary! { OBM: u32;
	/// [`HINSTANCE::LoadImageBitmap`](crate::prelude::gdi_Hinstance::LoadImageBitmap)
	/// OEM image identifier (`u32`).
	=>
	=>
	CLOSE 32754
	UPARROW 32753
	DNARROW 32752
	RGARROW 32751
	LFARROW 32750
	REDUCE 32749
	ZOOM 32748
	RESTORE 32747
	REDUCED 32746
	ZOOMD 32745
	RESTORED 32744
	UPARROWD 32743
	DNARROWD 32742
	RGARROWD 32741
	LFARROWD 32740
	MNARROW 32739
	COMBO 32738
	UPARROWI 32737
	DNARROWI 32736
	RGARROWI 32735
	LFARROWI 32734

	OLD_CLOSE 32767
	SIZE 32766
	OLD_UPARROW 32765
	OLD_DNARROW 32764
	OLD_RGARROW 32763
	OLD_LFARROW 32762
	BTSIZE 32761
	CHECK 32760
	CHECKBOXES 32759
	BTNCORNERS 32758
	OLD_REDUCE 32757
	OLD_ZOOM 32756
	OLD_RESTORE 32755
}

const_ordinary! { OCR: u32;
	/// [`HINSTANCE::LoadImageCursor`](crate::prelude::gdi_Hinstance::LoadImageCursor)
	/// and
	/// [`HCURSOR::SetSystemCursor`](crate::prelude::user_Hcursor::SetSystemCursor)
	/// OEM cursor identifier (`u32`).
	=>
	=>
	NORMAL 32512
	IBEAM 32513
	WAIT 32514
	CROSS 32515
	UP 32516
	SIZENWSE 32642
	SIZENESW 32643
	SIZEWE 32644
	SIZENS 32645
	SIZEALL 32646
	WINLOGO 32517
	NO 32648
	HAND 32649
	APPSTARTING 32650
	HELP 32651
}

const_bitflag! { ODA: u32;
	/// [`DRAWITEMSTRUCT`](crate::DRAWITEMSTRUCT) `itemAction` (`u32`).
	=>
	=>
	DRAWENTIRE 0x0001
	SELECT 0x0002
	FOCUS 0x0004
}

const_bitflag! { ODS: u32;
	/// [`DRAWITEMSTRUCT`](crate::DRAWITEMSTRUCT) `itemState` (`u32`).
	=>
	=>
	SELECTED 0x0001
	GRAYED 0x0002
	DISABLED 0x0004
	CHECKED 0x0008
	FOCUS 0x0010
	DEFAULT 0x0020
	COMBOBOXEDIT 0x1000
	HOTLIGHT 0x0040
	INACTIVE 0x0080
	NOACCEL 0x0100
	NOFOCUSRECT 0x0200
}

const_ordinary! { ODT: u32;
	/// [`DRAWITEMSTRUCT`](crate::DRAWITEMSTRUCT) `CtlType` (`u32`).
	=>
	=>
	MENU 1
	LISTBOX 2
	COMBOBOX 3
	BUTTON 4
	STATIC 5
	TAB 101
	LISTVIEW 102
}

const_ordinary! { ODT_C: u32;
	/// [`COMPAREITEMSTRUCT`](crate::COMPAREITEMSTRUCT) and
	/// [`DELETEITEMSTRUCT`](crate::DELETEITEMSTRUCT) `CtlType` (`u32`).
	///
	/// Originally has `ODT` prefix.
	=>
	=>
	LISTBOX ODT::LISTBOX.0
	COMBOBOX ODT::COMBOBOX.0
}

const_ordinary! { OIC: u32;
	/// [`HINSTANCE::LoadImageIcon`](crate::prelude::gdi_Hinstance::LoadImageIcon)
	/// OEM icon identifier (`u32`).
	=>
	=>
	SAMPLE 32512
	HAND 32513
	QUES 32514
	BANG 32515
	NOTE 32516
	WINLOGO 32517
	WARNING Self::BANG.0
	ERROR Self::HAND.0
	INFORMATION Self::NOTE.0
	SHIELD 32518
}

const_bitflag! { PM: u32;
	/// [`PeekMessage`](crate::PeekMessage) `remove_msg` (`u32`).
	=>
	=>
	NOREMOVE 0x0000
	REMOVE 0x0001
	NOYIELD 0x0002

	QS_INPUT QS::INPUT.0 << 16
	QS_POSTMESSAGE (QS::POSTMESSAGE.0 | QS::HOTKEY.0 | QS::TIMER.0) << 16
	QS_PAINT QS::PAINT.0 << 16
	QS_SENDMESSAGE QS::SENDMESSAGE.0 << 16
}

const_bitflag! { QS: u32;
	/// [`GetQueueStatus`](crate::GetQueueStatus) `flags` (`u32`).
	=>
	=>
	KEY 0x0001
	MOUSEMOVE 0x0002
	MOUSEBUTTON 0x0004
	POSTMESSAGE 0x0008
	TIMER 0x0010
	PAINT 0x0020
	SENDMESSAGE 0x0040
	HOTKEY 0x0080
	ALLPOSTMESSAGE 0x0100
	RAWINPUT 0x0400
	TOUCH 0x0800
	POINTER 0x1000
	MOUSE Self::MOUSEMOVE.0 | Self::MOUSEBUTTON.0
	INPUT Self::MOUSE.0 | Self::KEY.0 | Self::RAWINPUT.0 | Self::TOUCH.0 | Self::POINTER.0
	ALLINPUT Self::INPUT.0 | Self::POSTMESSAGE.0 | Self::TIMER.0 | Self::PAINT.0 | Self::HOTKEY.0 | Self::SENDMESSAGE.0
}

const_bitflag! { RDW: u32;
	/// [`HWND::RedrawWindow`](crate::prelude::user_Hwnd::RedrawWindow) `flags`
	/// (`u32`).
	=>
	=>
	INVALIDATE 0x0001
	INTERNALPAINT 0x0002
	ERASE 0x0004
	VALIDATE 0x0008
	NOINTERNALPAINT 0x0010
	NOERASE 0x0020
	NOCHILDREN 0x0040
	ALLCHILDREN 0x0080
	UPDATENOW 0x0100
	ERASENOW 0x0200
	FRAME 0x0400
	NOFRAME 0x0800
}

const_ordinary! { REGION: i32;
	/// [`HWND::GetUpdateRgn`](crate::prelude::user_Hwnd::GetUpdateRgn),
	/// [`HWND::GetWindowRgn`](crate::prelude::user_Hwnd::GetWindowRgn) and
	/// [`HDC::SelectObject`](crate::prelude::gdi_Hdc::SelectObject) return
	/// value (`i32`).
	=>
	=>
	NULL 1
	SIMPLE 2
	COMPLEX 3
}

const_ordinary! { SB_EM: u16;
	/// [`em::Scroll`](crate::msg::em::Scroll) action.
	///
	/// Originally has `SB` prefix.
	=>
	=>
	LINEUP 0
	LINEDOWN 1
	PAGEUP 2
	PAGEDOWN 3
}

const_ordinary! { SB_REQ: u16;
	/// [`wm::HScroll`](crate::msg::wm::HScroll) and
	/// [`wm::VScroll`](crate::msg::wm::VScroll) request (`u16`).
	///
	/// Originally has `SB` prefix.
	=>
	=>
	LINEUP 0
	LINELEFT 0
	LINEDOWN 1
	LINERIGHT 1
	PAGEUP 2
	PAGELEFT 2
	PAGEDOWN 3
	PAGERIGHT 3
	THUMBPOSITION 4
	THUMBTRACK 5
	TOP 6
	LEFT 6
	BOTTOM 7
	RIGHT 7
	ENDSCROLL 8
}

const_ordinary! { SBB: i32;
	/// [`HWND::EnableScrollBar`](crate::prelude::user_Hwnd::EnableScrollBar),
	/// [`HWND::GetScrollInfo`](crate::prelude::user_Hwnd::GetScrollInfo),
	/// [`HWND::SetScrollInfo`](crate::prelude::user_Hwnd::SetScrollInfo) and
	/// [`HWND::SetScrollRange`](crate::prelude::user_Hwnd::SetScrollRange)
	/// `bar` (`i32`).
	///
	/// Originally has `SB` prefix.
	=>
	=>
	HORZ 0
	VERT 1
	CTL 2
	BOTH 3
}

const_ordinary! { SC: u32;
	/// [`wm::SysCommand`](crate::msg::wm::SysCommand) type of system command
	/// requested (`u32`).
	=>
	=>
	CLOSE 0xf060
	CONTEXTHELP 0xf180
	DEFAULT 0xf160
	HOTKEY 0xf150
	HSCROLL 0xf080
	ISSECURE 0x0000_0001
	KEYMENU 0xf100
	MAXIMIZE 0xf030
	MINIMIZE 0xf020
	MONITORPOWER 0xf170
	MOUSEMENU 0xf090
	MOVE 0xf010
	NEXTWINDOW 0xf040
	PREVWINDOW 0xf050
	RESTORE 0xf120
	SCREENSAVE 0xf140
	SIZE 0xf000
	TASKLIST 0xf130
	VSCROLL 0xf070
}

const_bitflag! { SCROLLW: u32;
	/// [`ScrollWindowEx`](crate::prelude::user_Hwnd::ScrollWindowEx) `flags`
	/// (`u32`).
	///
	/// Originally has `SW` prefix.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SCROLLCHILDREN 0x0001
	INVALIDATE 0x0002
	ERASE 0x0004
	SMOOTHSCROLL 0x0010
}

const_bitflag! { SIF: u32;
	/// [`SCROLLINFO`](crate::SCROLLINFO) `fMask` (`u32`).
	=>
	=>
	RANGE 0x0001
	PAGE 0x0002
	POS 0x0004
	DISABLENOSCROLL 0x0008
	TRACKPOS 0x0010
	ALL Self::RANGE.0 | Self::PAGE.0 | Self::POS.0 | Self::TRACKPOS.0
}

const_ordinary! { SIZE_R: u8;
	/// [`wm::Size`](crate::msg::wm::Size) request (`u8`).
	=>
	=>
	/// The window has been resized but neither the `SIZE_R::MINIMIZED` nor
	/// `SIZE_R::MAXIMIZED` value applies.
	RESTORED 0
	/// The window has been minimized.
	MINIMIZED 1
	/// The window has been maximized.
	MAXIMIZED 2
	/// Message is sent to all pop-up windows when some other window has been
	/// restored to its former size.
	MAXSHOW 3
	/// Message is sent to all pop-up windows when some other window is
	/// maximized.
	MAXHIDE 4
}

const_ordinary! { SM: i32;
	/// [`GetSystemMetrics`](crate::GetSystemMetrics) `index` (`i32`).
	=>
	=>
	CXSCREEN 0
	CYSCREEN 1
	CXVSCROLL 2
	CYHSCROLL 3
	CYCAPTION 4
	CXBORDER 5
	CYBORDER 6
	CXDLGFRAME 7
	CYDLGFRAME 8
	CYVTHUMB 9
	CXHTHUMB 10
	CXICON 11
	CYICON 12
	CXCURSOR 13
	CYCURSOR 14
	CYMENU 15
	CXFULLSCREEN 16
	CYFULLSCREEN 17
	CYKANJIWINDOW 18
	MOUSEPRESENT 19
	CYVSCROLL 20
	CXHSCROLL 21
	DEBUG 22
	SWAPBUTTON 23
	RESERVED1 24
	RESERVED2 25
	RESERVED3 26
	RESERVED4 27
	/// The minimum width of a window in pixels.
	CXMIN 28
	/// The minimum height of a window in pixels.
	CYMIN 29
	/// The width of a button in a window caption or title bar in pixels.
	CXSIZE 30
	/// The height of a button in a window caption or title bar in pixels.
	CYSIZE 31
	CXFRAME 32
	CYFRAME 33
	/// The minimum tracking width of a window in pixels. The user cannot drag
	/// the window frame to a size smaller than these dimensions. A window can
	/// override this value by processing the
	/// [`wm::GetMinMaxInfo`](crate::msg::wm::GetMinMaxInfo) message.
	CXMINTRACK 34
	/// The minimum tracking height of a window in pixels. The user cannot drag
	/// the window frame to a size smaller than these dimensions. A window can
	/// override this value by processing the
	/// [`wm::GetMinMaxInfo`](crate::msg::wm::GetMinMaxInfo) message.
	CYMINTRACK 35
	CXDOUBLECLK 36
	CYDOUBLECLK 37
	CXICONSPACING 38
	CYICONSPACING 39
	MENUDROPALIGNMENT 40
	PENWINDOWS 41
	DBCSENABLED 42
	CMOUSEBUTTONS 43
	CXFIXEDFRAME Self::CXDLGFRAME.0
	CYFIXEDFRAME Self::CYDLGFRAME.0
	CXSIZEFRAME Self::CXFRAME.0
	CYSIZEFRAME Self::CYFRAME.0
	SECURE 44
	CXEDGE 45
	CYEDGE 46
	CXMINSPACING 47
	CYMINSPACING 48
	CXSMICON 49
	CYSMICON 50
	CYSMCAPTION 51
	CXSMSIZE 52
	CYSMSIZE 53
	CXMENUSIZE 54
	CYMENUSIZE 55
	ARRANGE 56
	CXMINIMIZED 57
	CYMINIMIZED 58
	CXMAXTRACK 59
	CYMAXTRACK 60
	CXMAXIMIZED 61
	CYMAXIMIZED 62
	NETWORK 63
	CLEANBOOT 67
	CXDRAG 68
	CYDRAG 69
	SHOWSOUNDS 70
	CXMENUCHECK 71
	CYMENUCHECK 72
	SLOWMACHINE 73
	MIDEASTENABLED 74
	MOUSEWHEELPRESENT 75
	XVIRTUALSCREEN 76
	YVIRTUALSCREEN 77
	CXVIRTUALSCREEN 78
	CYVIRTUALSCREEN 79
	CMONITORS 80
	SAMEDISPLAYFORMAT 81
	IMMENABLED 82
	CXFOCUSBORDER 83
	CYFOCUSBORDER 84
	TABLETPC 86
	MEDIACENTER 87
	STARTER 88
	SERVERR2 89
	MOUSEHORIZONTALWHEELPRESENT 91
	CXPADDEDBORDER 92
	DIGITIZER 94
	MAXIMUMTOUCHES 95
	CMETRICS 97
	REMOTESESSION 0x1000
	SHUTTINGDOWN 0x2000
	REMOTECONTROL 0x2001
	CARETBLINKINGENABLED 0x2002
	CONVERTIBLESLATEMODE 0x2003
	SYSTEMDOCKED 0x2004
}

const_bitflag! { SMTO: u32;
	/// [`SendMessageTimeout`](crate::prelude::user_Hwnd::SendMessageTimeout)
	/// `flags` (`u32`).
	=>
	=>
	ABORTIFHUNG 0x0002
	BLOCK 0x0001
	NORMAL 0x0000
	NOTIMEOUTIFNOTHUNG 0x0008
	ERRORONEXIT 0x0020
}

const_ordinary! { SPI: u32;
	/// [`SystemParametersInfo`](crate::SystemParametersInfo) `action` (`u32`).
	=>
	=>
	GETBEEP 0x0001
	SETBEEP 0x0002
	GETMOUSE 0x0003
	SETMOUSE 0x0004
	GETBORDER 0x0005
	SETBORDER 0x0006
	GETKEYBOARDSPEED 0x000a
	SETKEYBOARDSPEED 0x000b
	LANGDRIVER 0x000c
	ICONHORIZONTALSPACING 0x000d
	GETSCREENSAVETIMEOUT 0x000e
	SETSCREENSAVETIMEOUT 0x000f
	GETSCREENSAVEACTIVE 0x0010
	SETSCREENSAVEACTIVE 0x0011
	GETGRIDGRANULARITY 0x0012
	SETGRIDGRANULARITY 0x0013
	SETDESKWALLPAPER 0x0014
	SETDESKPATTERN 0x0015
	GETKEYBOARDDELAY 0x0016
	SETKEYBOARDDELAY 0x0017
	ICONVERTICALSPACING 0x0018
	GETICONTITLEWRAP 0x0019
	SETICONTITLEWRAP 0x001a
	GETMENUDROPALIGNMENT 0x001b
	SETMENUDROPALIGNMENT 0x001c
	SETDOUBLECLKWIDTH 0x001d
	SETDOUBLECLKHEIGHT 0x001e
	GETICONTITLELOGFONT 0x001f
	SETDOUBLECLICKTIME 0x0020
	SETMOUSEBUTTONSWAP 0x0021
	SETICONTITLELOGFONT 0x0022
	GETFASTTASKSWITCH 0x0023
	SETFASTTASKSWITCH 0x0024
	SETDRAGFULLWINDOWS 0x0025
	GETDRAGFULLWINDOWS 0x0026
	GETNONCLIENTMETRICS 0x0029
	SETNONCLIENTMETRICS 0x002a
	GETMINIMIZEDMETRICS 0x002b
	SETMINIMIZEDMETRICS 0x002c
	GETICONMETRICS 0x002d
	SETICONMETRICS 0x002e
	SETWORKAREA 0x002f
	GETWORKAREA 0x0030
	SETPENWINDOWS 0x0031
	GETHIGHCONTRAST 0x0042
	SETHIGHCONTRAST 0x0043
	GETKEYBOARDPREF 0x0044
	SETKEYBOARDPREF 0x0045
	GETSCREENREADER 0x0046
	SETSCREENREADER 0x0047
	GETANIMATION 0x0048
	SETANIMATION 0x0049
	GETFONTSMOOTHING 0x004a
	SETFONTSMOOTHING 0x004b
	SETDRAGWIDTH 0x004c
	SETDRAGHEIGHT 0x004d
	SETHANDHELD 0x004e
	GETLOWPOWERTIMEOUT 0x004f
	GETPOWEROFFTIMEOUT 0x0050
	SETLOWPOWERTIMEOUT 0x0051
	SETPOWEROFFTIMEOUT 0x0052
	GETLOWPOWERACTIVE 0x0053
	GETPOWEROFFACTIVE 0x0054
	SETLOWPOWERACTIVE 0x0055
	SETPOWEROFFACTIVE 0x0056
	SETCURSORS 0x0057
	SETICONS 0x0058
	GETDEFAULTINPUTLANG 0x0059
	SETDEFAULTINPUTLANG 0x005a
	SETLANGTOGGLE 0x005b
	GETWINDOWSEXTENSION 0x005c
	SETMOUSETRAILS 0x005d
	GETMOUSETRAILS 0x005e
	SETSCREENSAVERRUNNING 0x0061
	SCREENSAVERRUNNING Self::SETSCREENSAVERRUNNING.0
	GETFILTERKEYS 0x0032
	SETFILTERKEYS 0x0033
	GETTOGGLEKEYS 0x0034
	SETTOGGLEKEYS 0x0035
	GETMOUSEKEYS 0x0036
	SETMOUSEKEYS 0x0037
	GETSHOWSOUNDS 0x0038
	SETSHOWSOUNDS 0x0039
	GETSTICKYKEYS 0x003a
	SETSTICKYKEYS 0x003b
	GETACCESSTIMEOUT 0x003c
	SETACCESSTIMEOUT 0x003d
	GETSERIALKEYS 0x003e
	SETSERIALKEYS 0x003f
	GETSOUNDSENTRY 0x0040
	SETSOUNDSENTRY 0x0041
	GETSNAPTODEFBUTTON 0x005f
	SETSNAPTODEFBUTTON 0x0060
	GETMOUSEHOVERWIDTH 0x0062
	SETMOUSEHOVERWIDTH 0x0063
	GETMOUSEHOVERHEIGHT 0x0064
	SETMOUSEHOVERHEIGHT 0x0065
	GETMOUSEHOVERTIME 0x0066
	SETMOUSEHOVERTIME 0x0067
	GETWHEELSCROLLLINES 0x0068
	SETWHEELSCROLLLINES 0x0069
	GETMENUSHOWDELAY 0x006a
	SETMENUSHOWDELAY 0x006b
	GETWHEELSCROLLCHARS 0x006c
	SETWHEELSCROLLCHARS 0x006d
	GETSHOWIMEUI 0x006e
	SETSHOWIMEUI 0x006f
	GETMOUSESPEED 0x0070
	SETMOUSESPEED 0x0071
	GETSCREENSAVERRUNNING 0x0072
	GETDESKWALLPAPER 0x0073
	GETAUDIODESCRIPTION 0x0074
	SETAUDIODESCRIPTION 0x0075
	GETSCREENSAVESECURE 0x0076
	SETSCREENSAVESECURE 0x0077
	GETHUNGAPPTIMEOUT 0x0078
	SETHUNGAPPTIMEOUT 0x0079
	GETWAITTOKILLTIMEOUT 0x007a
	SETWAITTOKILLTIMEOUT 0x007b
	GETWAITTOKILLSERVICETIMEOUT 0x007c
	SETWAITTOKILLSERVICETIMEOUT 0x007d
	GETMOUSEDOCKTHRESHOLD 0x007e
	SETMOUSEDOCKTHRESHOLD 0x007f
	GETPENDOCKTHRESHOLD 0x0080
	SETPENDOCKTHRESHOLD 0x0081
	GETWINARRANGING 0x0082
	SETWINARRANGING 0x0083
	GETMOUSEDRAGOUTTHRESHOLD 0x0084
	SETMOUSEDRAGOUTTHRESHOLD 0x0085
	GETPENDRAGOUTTHRESHOLD 0x0086
	SETPENDRAGOUTTHRESHOLD 0x0087
	GETMOUSESIDEMOVETHRESHOLD 0x0088
	SETMOUSESIDEMOVETHRESHOLD 0x0089
	GETPENSIDEMOVETHRESHOLD 0x008a
	SETPENSIDEMOVETHRESHOLD 0x008b
	GETDRAGFROMMAXIMIZE 0x008c
	SETDRAGFROMMAXIMIZE 0x008d
	GETSNAPSIZING 0x008e
	SETSNAPSIZING 0x008f
	GETDOCKMOVING 0x0090
	SETDOCKMOVING 0x0091
}

const_bitflag! { SPIF: u32;
	/// [`SystemParametersInfo`](crate::SystemParametersInfo) `win_ini` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	UPDATEINIFILE 0x0001
	SENDWININICHANGE 0x0002
	SENDCHANGE Self::SENDWININICHANGE.0
}

const_ws! { SS: u32;
	/// Label control
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/controls/static-control-styles)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	LEFT 0x0000_0000
	CENTER 0x0000_0001
	RIGHT 0x0000_0002
	ICON 0x0000_0003
	BLACKRECT 0x0000_0004
	GRAYRECT 0x0000_0005
	WHITERECT 0x0000_0006
	BLACKFRAME 0x0000_0007
	GRAYFRAME 0x0000_0008
	WHITEFRAME 0x0000_0009
	USERITEM 0x0000_000a
	SIMPLE 0x0000_000b
	LEFTNOWORDWRAP 0x0000_000c
	OWNERDRAW 0x0000_000d
	BITMAP 0x0000_000e
	ENHMETAFILE 0x0000_000f
	ETCHEDHORZ 0x0000_0010
	ETCHEDVERT 0x0000_0011
	ETCHEDFRAME 0x0000_0012
	TYPEMASK 0x0000_001f
	REALSIZECONTROL 0x0000_0040
	NOPREFIX 0x0000_0080
	NOTIFY 0x0000_0100
	CENTERIMAGE 0x0000_0200
	RIGHTJUST 0x0000_0400
	REALSIZEIMAGE 0x0000_0800
	SUNKEN 0x0000_1000
	EDITCONTROL 0x0000_2000
	ENDELLIPSIS 0x0000_4000
	PATHELLIPSIS 0x0000_8000
	WORDELLIPSIS 0x0000_c000
}

const_bitflag! { STATE_SYSTEM: u32;
	/// [`DATETIMEPICKERINFO`](crate::DATETIMEPICKERINFO) `stateCheck` and
	/// `stateButton`, [`TITLEBARINFOEX`](crate::TITLEBARINFOEX) `rgstate`,
	/// [`COMBOBOXINFO`](crate::COMBOBOXINFO) `stateButton` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	UNAVAILABLE 0x0000_0001
	SELECTED 0x0000_0002
	FOCUSED 0x0000_0004
	PRESSED 0x0000_0008
	CHECKED 0x0000_0010
	MIXED 0x0000_0020
	INDETERMINATE Self::MIXED.0
	READONLY 0x0000_0040
	HOTTRACKED 0x0000_0080
	DEFAULT 0x0000_0100
	EXPANDED 0x0000_0200
	COLLAPSED 0x0000_0400
	BUSY 0x0000_0800
	FLOATING 0x0000_1000
	MARQUEED 0x0000_2000
	ANIMATED 0x0000_4000
	INVISIBLE 0x0000_8000
	OFFSCREEN 0x0001_0000
	SIZEABLE 0x0002_0000
	MOVEABLE 0x0004_0000
	SELFVOICING 0x0008_0000
	FOCUSABLE 0x0010_0000
	SELECTABLE 0x0020_0000
	LINKED 0x0040_0000
	TRAVERSED 0x0080_0000
	MULTISELECTABLE 0x0100_0000
	EXTSELECTABLE 0x0200_0000
	ALERT_LOW 0x0400_0000
	ALERT_MEDIUM 0x0800_0000
	ALERT_HIGH 0x1000_0000
	PROTECTED 0x2000_0000
	VALID 0x3fff_ffff
}

const_cmd! { STN;
	/// Static control `WM_COMMAND`
	/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-static-control-reference-notifications)
	/// (`u16`).
	=>
	=>
	CLICKED 0
	DBLCLK 1
	ENABLE 2
	DISABLE 3
}

const_bitflag! { SWP: u32;
	/// [`HWND::SetWindowPos`](crate::prelude::user_Hwnd::SetWindowPos) `flags`
	/// (`u32`).
	=>
	=>
	NOSIZE 0x0001
	NOMOVE 0x0002
	NOZORDER 0x0004
	NOREDRAW 0x0008
	NOACTIVATE 0x0010
	FRAMECHANGED 0x0020
	SHOWWINDOW 0x0040
	HIDEWINDOW 0x0080
	NOCOPYBITS 0x0100
	NOOWNERZORDER 0x0200
	NOSENDCHANGING 0x0400
	DRAWFRAME Self::FRAMECHANGED.0
	NOREPOSITION Self::NOOWNERZORDER.0
	DEFERERASE 0x2000
	ASYNCWINDOWPOS 0x4000
}

const_ordinary! { SW_S: u8;
	/// [`wm::ShowWindow`](crate::msg::wm::ShowWindow) status (`u8`).
	///
	/// Originally has `SW` prefix.
	=>
	=>
	PARENTCLOSING 1
	OTHERZOOM 2
	PARENTOPENING 3
	OTHERUNZOOM 4
}

const_bitflag! { TME: u32;
	/// [`TrackMouseEvent`](crate::TrackMouseEvent) `dwFlags` (`u32`).
	=>
	=>
	CANCEL 0x8000_0000
	HOVER 0x0000_0001
	LEAVE 0x0000_0002
	NONCLIENT 0x0000_0010
	QUERY 0x4000_0000
}

const_bitflag! { TPM: u32;
	/// [`TrackPopupMenu`](crate::prelude::user_Hmenu::TrackPopupMenu) `flags`
	/// (`u32`).
	=>
	=>
	LEFTBUTTON 0x0000
	RIGHTBUTTON 0x0002
	LEFTALIGN 0x0000
	CENTERALIGN 0x0004
	RIGHTALIGN 0x0008
	TOPALIGN 0x0000
	VCENTERALIGN 0x0010
	BOTTOMALIGN 0x0020
	HORIZONTAL 0x0000
	VERTICAL 0x0040
	NONOTIFY 0x0080
	RETURNCMD 0x0100
	RECURSE 0x0001
	HORPOSANIMATION 0x0400
	HORNEGANIMATION 0x0800
	VERPOSANIMATION 0x1000
	VERNEGANIMATION 0x2000
	NOANIMATION 0x4000
	LAYOUTRTL 0x8000
	WORKAREA 0x10000
}

const_ordinary! { UOI: i32;
	/// [`HPROCESS::SetUserObjectInformation`](crate::prelude::user_Hprocess::SetUserObjectInformation)
	/// `index` (`i32`).
	=>
	=>
	FLAGS 1
	TIMERPROC_EXCEPTION_SUPPRESSION 7
}

const_ordinary! { VK: u16;
	/// [Virtual key codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
	/// (`u16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Left mouse button.
	LBUTTON 0x01
	/// Right mouse button.
	RBUTTON 0x02
	/// Control-break processing.
	CANCEL 0x03
	/// Middle mouse button (three-button mouse).
	MBUTTON 0x04
	/// X1 mouse button.
	XBUTTON1 0x05
	/// X2 mouse button.
	XBUTTON2 0x06
	/// BACKSPACE key.
	BACK 0x08
	/// TAB key.
	TAB 0x09
	/// CLEAR key.
	CLEAR 0x0c
	/// ENTER key.
	RETURN 0x0d
	/// SHIFT key.
	SHIFT 0x10
	/// CTRL key.
	CONTROL 0x11
	/// ALT key.
	MENU 0x12
	/// PAUSE key.
	PAUSE 0x13
	/// CAPS LOCK key.
	CAPITAL 0x14
	/// IME Kana mode.
	KANA 0x15
	/// IME Hangul mode.
	HANGUL 0x15
	/// IME On.
	IME_ON 0x16
	/// IME Junja mode.
	JUNJA 0x17
	/// IME final mode.
	FINAL 0x18
	/// IME Hanja mode.
	HANJA 0x19
	/// IME Kanji mode.
	KANJI 0x19
	/// ESC key.
	ESCAPE 0x1b
	/// IME convert.
	CONVERT 0x1c
	/// IME nonconvert.
	NONCONVERT 0x1d
	/// IME accept.
	ACCEPT 0x1e
	/// IME mode change request.
	MODECHANGE 0x1f
	/// SPACEBAR key.
	SPACE 0x20
	/// PAGE UP key.
	PRIOR 0x21
	/// PAGE DOWN key.
	NEXT 0x22
	/// END key.
	END 0x23
	/// HOME key.
	HOME 0x24
	/// LEFT ARROW key.
	LEFT 0x25
	/// UP ARROW key.
	UP 0x26
	/// RIGHT ARROW key.
	RIGHT 0x27
	/// DOWN ARROW key.
	DOWN 0x28
	/// SELECT key.
	SELECT 0x29
	/// PRINT key.
	PRINT 0x2a
	/// EXECUTE key.
	EXECUTE 0x2b
	/// PRINT SCREEN key.
	SNAPSHOT 0x2c
	/// INS key.
	INSERT 0x2d
	/// DEL key.
	DELETE 0x2e
	/// HELP key.
	HELP 0x2f

	/// Number 0 key.
	CHAR_0 0x30
	/// Number 1 key.
	CHAR_1 0x31
	/// Number 2 key.
	CHAR_2 0x32
	/// Number 3 key.
	CHAR_3 0x33
	/// Number 4 key.
	CHAR_4 0x34
	/// Number 5 key.
	CHAR_5 0x35
	/// Number 6 key.
	CHAR_6 0x36
	/// Number 7 key.
	CHAR_7 0x37
	/// Number 8 key.
	CHAR_8 0x38
	/// Number 9 key.
	CHAR_9 0x39
	/// Character A key.
	CHAR_A 0x41
	/// Character B key.
	CHAR_B 0x42
	/// Character C key.
	CHAR_C 0x43
	/// Character D key.
	CHAR_D 0x44
	/// Character E key.
	CHAR_E 0x45
	/// Character F key.
	CHAR_F 0x46
	/// Character G key.
	CHAR_G 0x47
	/// Character H key.
	CHAR_H 0x48
	/// Character I key.
	CHAR_I 0x49
	/// Character J key.
	CHAR_J 0x4a
	/// Character K key.
	CHAR_K 0x4b
	/// Character L key.
	CHAR_L 0x4c
	/// Character M key.
	CHAR_M 0x4d
	/// Character N key.
	CHAR_N 0x4e
	/// Character O key.
	CHAR_O 0x4f
	/// Character P key.
	CHAR_P 0x50
	/// Character Q key.
	CHAR_Q 0x51
	/// Character R key.
	CHAR_R 0x52
	/// Character S key.
	CHAR_S 0x53
	/// Character T key.
	CHAR_T 0x54
	/// Character U key.
	CHAR_U 0x55
	/// Character V key.
	CHAR_V 0x56
	/// Character W key.
	CHAR_W 0x57
	/// Character X key.
	CHAR_X 0x58
	/// Character Y key.
	CHAR_Y 0x59
	/// Character Z key.
	CHAR_Z 0x5a

	/// Left Windows key (Natural keyboard).
	LWIN 0x5b
	/// Right Windows key (Natural keyboard).
	RWIN 0x5c
	/// Applications key context menu (Natural keyboard).
	APPS 0x5d
	/// Computer Sleep key.
	SLEEP 0x5f
	/// Numeric keypad 0 key.
	NUMPAD0 0x60
	/// Numeric keypad 1 key.
	NUMPAD1 0x61
	/// Numeric keypad 2 key.
	NUMPAD2 0x62
	/// Numeric keypad 3 key.
	NUMPAD3 0x63
	/// Numeric keypad 4 key.
	NUMPAD4 0x64
	/// Numeric keypad 5 key.
	NUMPAD5 0x65
	/// Numeric keypad 6 key.
	NUMPAD6 0x66
	/// Numeric keypad 7 key.
	NUMPAD7 0x67
	/// Numeric keypad 8 key.
	NUMPAD8 0x68
	/// Numeric keypad 9 key.
	NUMPAD9 0x69
	/// Numeric keypad multiply key.
	MULTIPLY 0x6a
	/// Numeric keypad add key.
	ADD 0x6b
	/// Numeric keypad separator key.
	SEPARATOR 0x6c
	/// Numeric keypad subtract key.
	SUBTRACT 0x6d
	/// Numeric keypad decimal key.
	DECIMAL 0x6e
	/// Numeric keypad divide key.
	DIVIDE 0x6f
	F1 0x70
	F2 0x71
	F3 0x72
	F4 0x73
	F5 0x74
	F6 0x75
	F7 0x76
	F8 0x77
	F9 0x78
	F10 0x79
	F11 0x7a
	F12 0x7b
	F13 0x7c
	F14 0x7d
	F15 0x7e
	F16 0x7f
	F17 0x80
	F18 0x81
	F19 0x82
	F20 0x83
	F21 0x84
	F22 0x85
	F23 0x86
	F24 0x87
	/// NUM LOCK key.
	NUMLOCK 0x90
	/// SCROLL LOCK key.
	SCROLL 0x91
	OEM_NEC_EQUAL 0x92
	OEM_FJ_JISHO 0x92
	OEM_FJ_MASSHOU 0x93
	OEM_FJ_TOUROKU 0x94
	OEM_FJ_LOYA 0x95
	OEM_FJ_ROYA 0x96
	/// Left SHIFT key.
	LSHIFT 0xa0
	/// Right SHIFT key.
	RSHIFT 0xa1
	/// Left CONTROL key.
	LCONTROL 0xa2
	/// Right CONTROL key.
	RCONTROL 0xa3
	/// Left MENU key.
	LMENU 0xa4
	/// Right MENU key.
	RMENU 0xa5
	BROWSER_BACK 0xa6
	BROWSER_FORWARD 0xa7
	BROWSER_REFRESH 0xa8
	BROWSER_STOP 0xa9
	BROWSER_SEARCH 0xaa
	BROWSER_FAVORITES 0xab
	BROWSER_HOME 0xac
	VOLUME_MUTE 0xad
	VOLUME_DOWN 0xae
	VOLUME_UP 0xaf
	MEDIA_NEXT_TRACK 0xb0
	MEDIA_PREV_TRACK 0xb1
	MEDIA_STOP 0xb2
	MEDIA_PLAY_PAUSE 0xb3
	LAUNCH_MAIL 0xb4
	LAUNCH_MEDIA_SELECT 0xb5
	LAUNCH_APP1 0xb6
	LAUNCH_APP2 0xb7
	OEM_1 0xba
	OEM_PLUS 0xbb
	OEM_COMMA 0xbc
	OEM_MINUS 0xbd
	OEM_PERIOD 0xbe
	OEM_2 0xbf
	OEM_3 0xc0
	OEM_4 0xdb
	OEM_5 0xdc
	OEM_6 0xdd
	OEM_7 0xde
	OEM_8 0xdf
	OEM_AX 0xe1
	OEM_102 0xe2
	ICO_HELP 0xe3
	ICO_00 0xe4
	PROCESSKEY 0xe5
	ICO_CLEAR 0xe6
	PACKET 0xe7
	OEM_RESET 0xe9
	OEM_JUMP 0xea
	OEM_PA1 0xeb
	OEM_PA2 0xec
	OEM_PA3 0xed
	OEM_WSCTRL 0xee
	OEM_CUSEL 0xef
	OEM_ATTN 0xf0
	OEM_FINISH 0xf1
	OEM_COPY 0xf2
	OEM_AUTO 0xf3
	OEM_ENLW 0xf4
	OEM_BACKTAB 0xf5
	ATTN 0xf6
	CRSEL 0xf7
	EXSEL 0xf8
	EREOF 0xf9
	PLAY 0xfa
	ZOOM 0xfb
	NONAME 0xfc
	PA1 0xfd
	OEM_CLEAR 0xfe
}

const_ordinary! { WA: u16;
	/// [`wm::Activate`](crate::msg::wm::Activate) activation state (`u16`).
	=>
	=>
	INACTIVE 0
	ACTIVE 1
	CLICKACTIVE 2
}

const_ordinary! { WDA: u32;
	/// [`HWND::GetWindowDisplayAffinity`](crate::prelude::user_Hwnd::GetWindowDisplayAffinity)
	/// and
	/// [`HWND::SetWindowDisplayAffinity`](crate::prelude::user_Hwnd::SetWindowDisplayAffinity)
	/// `dwAffinity` (`u32`).
	=>
	=>
	NONE 0x0000_0000
	MONITOR 0x0000_0001
	EXCLUDEFROMCAPTURE 0x0000_0011
}

const_ordinary! { WH: i32;
	/// [`HHOOK::CallNextHookEx`](crate::prelude::user_Hhook::CallNextHookEx)
	/// `code` and
	/// [`HHOOK::SetWindowsHookEx`](crate::prelude::user_Hhook::SetWindowsHookEx)
	/// `hook_id` (`i32`).
	=>
	=>
	MSGFILTER -1
	JOURNALRECORD 0
	JOURNALPLAYBACK 1
	KEYBOARD 2
	GETMESSAGE 3
	CALLWNDPROC 4
	CBT 5
	SYSMSGFILTER 6
	MOUSE 7
	DEBUG 9
	SHELL 10
	FOREGROUNDIDLE 11
	CALLWNDPROCRET 12
	KEYBOARD_LL 13
	MOUSE_LL 14
}

const_bitflag! { WPF: u32;
	/// [`WINDOWPLACEMENT`](crate::WINDOWPLACEMENT) `flags` (`u32`).
	=>
	=>
	SETMINPOSITION 0x0001
	RESTORETOMAXIMIZED 0x0002
	ASYNCWINDOWPLACEMENT 0x0004
}

const_ordinary! { WM: u32;
	/// Window message codes (`u32`).
	///
	/// **Note:** Control-specific messages have their own types, which are
	/// convertible to `WM`.
	=>
	=>
	NULL 0x0000
	CREATE 0x0001
	DESTROY 0x0002
	MOVE 0x0003
	SIZE 0x0005
	ACTIVATE 0x0006
	SETFOCUS 0x0007
	KILLFOCUS 0x0008
	ENABLE 0x000a
	SETREDRAW 0x000b
	SETTEXT 0x000c
	GETTEXT 0x000d
	GETTEXTLENGTH 0x000e
	PAINT 0x000f
	CLOSE 0x0010
	QUERYENDSESSION 0x0011
	QUERYOPEN 0x0013
	ENDSESSION 0x0016
	QUIT 0x0012
	ERASEBKGND 0x0014
	SYSCOLORCHANGE 0x0015
	SHOWWINDOW 0x0018
	WININICHANGE 0x001a
	DEVMODECHANGE 0x001b
	ACTIVATEAPP 0x001c
	FONTCHANGE 0x001d
	TIMECHANGE 0x001e
	CANCELMODE 0x001f
	SETCURSOR 0x0020
	MOUSEACTIVATE 0x0021
	CHILDACTIVATE 0x0022
	QUEUESYNC 0x0023
	GETMINMAXINFO 0x0024
	PAINTICON 0x0026
	ICONERASEBKGND 0x0027
	NEXTDLGCTL 0x0028
	SPOOLERSTATUS 0x002a
	DRAWITEM 0x002b
	MEASUREITEM 0x002c
	DELETEITEM 0x002d
	VKEYTOITEM 0x002e
	CHARTOITEM 0x002f
	SETFONT 0x0030
	GETFONT 0x0031
	SETHOTKEY 0x0032
	GETHOTKEY 0x0033
	QUERYDRAGICON 0x0037
	COMPAREITEM 0x0039
	GETOBJECT 0x003d
	COPYDATA 0x004a
	COMPACTING 0x0041
	COMMNOTIFY 0x0044
	WINDOWPOSCHANGING 0x0046
	WINDOWPOSCHANGED 0x0047
	POWER 0x0048
	NOTIFY 0x004e
	INPUTLANGCHANGEREQUEST 0x0050
	INPUTLANGCHANGE 0x0051
	TCARD 0x0052
	HELP 0x0053
	USERCHANGED 0x0054
	NOTIFYFORMAT 0x0055
	CONTEXTMENU 0x007b
	STYLECHANGING 0x007c
	STYLECHANGED 0x007d
	DISPLAYCHANGE 0x007e
	GETICON 0x007f
	SETICON 0x0080
	NCCREATE 0x0081
	NCDESTROY 0x0082
	NCCALCSIZE 0x0083
	NCHITTEST 0x0084
	NCPAINT 0x0085
	NCACTIVATE 0x0086
	GETDLGCODE 0x0087
	SYNCPAINT 0x0088
	NCMOUSEMOVE 0x00a0
	NCLBUTTONDOWN 0x00a1
	NCLBUTTONUP 0x00a2
	NCLBUTTONDBLCLK 0x00a3
	NCRBUTTONDOWN 0x00a4
	NCRBUTTONUP 0x00a5
	NCRBUTTONDBLCLK 0x00a6
	NCMBUTTONDOWN 0x00a7
	NCMBUTTONUP 0x00a8
	NCMBUTTONDBLCLK 0x00a9
	NCXBUTTONDOWN 0x00ab
	NCXBUTTONUP 0x00ac
	NCXBUTTONDBLCLK 0x00ad
	INPUT_DEVICE_CHANGE 0x00fe
	INPUT 0x00ff
	KEYFIRST 0x0100
	KEYDOWN 0x0100
	KEYUP 0x0101
	CHAR 0x0102
	DEADCHAR 0x0103
	SYSKEYDOWN 0x0104
	SYSKEYUP 0x0105
	SYSCHAR 0x0106
	SYSDEADCHAR 0x0107
	UNICHAR 0x0109
	KEYLAST 0x0109
	IME_STARTCOMPOSITION 0x010d
	IME_ENDCOMPOSITION 0x010e
	IME_COMPOSITION 0x010f
	IME_KEYLAST 0x010f
	INITDIALOG 0x0110
	COMMAND 0x0111
	SYSCOMMAND 0x0112
	TIMER 0x0113
	HSCROLL 0x0114
	VSCROLL 0x0115
	INITMENU 0x0116
	INITMENUPOPUP 0x0117
	GESTURE 0x0119
	GESTURENOTIFY 0x011a
	MENUSELECT 0x011f
	MENUCHAR 0x0120
	ENTERIDLE 0x0121
	MENURBUTTONUP 0x0122
	MENUDRAG 0x0123
	MENUGETOBJECT 0x0124
	UNINITMENUPOPUP 0x0125
	MENUCOMMAND 0x0126
	CHANGEUISTATE 0x0127
	UPDATEUISTATE 0x0128
	QUERYUISTATE 0x0129
	CTLCOLORMSGBOX 0x0132
	CTLCOLOREDIT 0x0133
	CTLCOLORLISTBOX 0x0134
	CTLCOLORBTN 0x0135
	CTLCOLORDLG 0x0136
	CTLCOLORSCROLLBAR 0x0137
	CTLCOLORSTATIC 0x0138
	/// Originally has no `WM` prefix.
	MN_GETHMENU 0x01e1
	MOUSEFIRST 0x0200
	MOUSEMOVE 0x0200
	LBUTTONDOWN 0x0201
	LBUTTONUP 0x0202
	LBUTTONDBLCLK 0x0203
	RBUTTONDOWN 0x0204
	RBUTTONUP 0x0205
	RBUTTONDBLCLK 0x0206
	MBUTTONDOWN 0x0207
	MBUTTONUP 0x0208
	MBUTTONDBLCLK 0x0209
	MOUSEHWHEEL 0x020e
	XBUTTONDOWN 0x020b
	XBUTTONUP 0x020c
	XBUTTONDBLCLK 0x020d
	MOUSELAST 0x020e
	PARENTNOTIFY 0x0210
	ENTERMENULOOP 0x0211
	EXITMENULOOP 0x0212
	NEXTMENU 0x0213
	SIZING 0x0214
	CAPTURECHANGED 0x0215
	MOVING 0x0216
	POWERBROADCAST 0x0218
	DEVICECHANGE 0x0219
	MDICREATE 0x0220
	MDIDESTROY 0x0221
	MDIACTIVATE 0x0222
	MDIRESTORE 0x0223
	MDINEXT 0x0224
	MDIMAXIMIZE 0x0225
	MDITILE 0x0226
	MDICASCADE 0x0227
	MDIICONARRANGE 0x0228
	MDIGETACTIVE 0x0229
	MDISETMENU 0x0230
	ENTERSIZEMOVE 0x0231
	EXITSIZEMOVE 0x0232
	DROPFILES 0x0233
	MDIREFRESHMENU 0x0234
	POINTERDEVICECHANGE 0x0238
	POINTERDEVICEINRANGE 0x0239
	POINTERDEVICEOUTOFRANGE 0x023a
	TOUCH 0x0240
	NCPOINTERUPDATE 0x0241
	NCPOINTERDOWN 0x0242
	NCPOINTERUP 0x0243
	POINTERUPDATE 0x0245
	POINTERDOWN 0x0246
	POINTERUP 0x0247
	POINTERENTER 0x0249
	POINTERLEAVE 0x024a
	POINTERACTIVATE 0x024b
	POINTERCAPTURECHANGED 0x024c
	TOUCHHITTESTING 0x024d
	POINTERWHEEL 0x024e
	POINTERHWHEEL 0x024f
	/// Originally has no `WM` prefix.
	DM_POINTERHITTEST 0x0250
	POINTERROUTEDTO 0x0251
	POINTERROUTEDAWAY 0x0252
	POINTERROUTEDRELEASED 0x0253
	IME_SETCONTEXT 0x0281
	IME_NOTIFY 0x0282
	IME_CONTROL 0x0283
	IME_COMPOSITIONFULL 0x0284
	IME_SELECT 0x0285
	IME_CHAR 0x0286
	IME_REQUEST 0x0288
	IME_KEYDOWN 0x0290
	IME_KEYUP 0x0291
	MOUSEHOVER 0x02a1
	MOUSELEAVE 0x02a3
	NCMOUSEHOVER 0x02a0
	NCMOUSELEAVE 0x02a2
	WTSSESSION_CHANGE 0x02b1
	TABLET_FIRST 0x02c0
	TABLET_LAST 0x02df
	DPICHANGED 0x02e0
	DPICHANGED_BEFOREPARENT 0x02e2
	DPICHANGED_AFTERPARENT 0x02e3
	GETDPISCALEDSIZE 0x02e4
	CUT 0x0300
	COPY 0x0301
	PASTE 0x0302
	CLEAR 0x0303
	UNDO 0x0304
	RENDERFORMAT 0x0305
	RENDERALLFORMATS 0x0306
	DESTROYCLIPBOARD 0x0307
	DRAWCLIPBOARD 0x0308
	PAINTCLIPBOARD 0x0309
	VSCROLLCLIPBOARD 0x030a
	SIZECLIPBOARD 0x030b
	ASKCBFORMATNAME 0x030c
	CHANGECBCHAIN 0x030d
	HSCROLLCLIPBOARD 0x030e
	QUERYNEWPALETTE 0x030f
	PALETTEISCHANGING 0x0310
	PALETTECHANGED 0x0311
	HOTKEY 0x0312
	PRINT 0x0317
	PRINTCLIENT 0x0318
	APPCOMMAND 0x0319
	THEMECHANGED 0x031a
	CLIPBOARDUPDATE 0x031d
	DWMCOMPOSITIONCHANGED 0x031e
	DWMNCRENDERINGCHANGED 0x031f
	DWMCOLORIZATIONCOLORCHANGED 0x0320
	DWMWINDOWMAXIMIZEDCHANGE 0x0321
	DWMSENDICONICTHUMBNAIL 0x0323
	DWMSENDICONICLIVEPREVIEWBITMAP 0x0326
	GETTITLEBARINFOEX 0x033f
	HANDHELDFIRST 0x0358
	HANDHELDLAST 0x035f
	AFXFIRST 0x0360
	AFXLAST 0x037f
	PENWINFIRST 0x0380
	PENWINLAST 0x038f
	APP 0x8000
	USER 0x0400
}

const_ordinary! { WMPN: u16;
	/// [`wm::ParentNotify`](crate::msg::wm::ParentNotify) event (`u16`).
	=>
	=>
	CREATE WM::CREATE.0 as u16
	DESTROY WM::DESTROY.0 as u16
	LBUTTONDOWN WM::LBUTTONDOWN.0 as u16
	MBUTTONDOWN WM::MBUTTONDOWN.0 as u16
	RBUTTONDOWN WM::RBUTTONDOWN.0 as u16
	XBUTTONDOWN WM::XBUTTONDOWN.0 as u16
	POINTERDOWN WM::POINTERDOWN.0 as u16
}

const_ordinary! { WMSZ: u8;
	/// [`wm::Sizing`](crate::msg::wm::Sizing) window edge (`u8`).
	=>
	=>
	LEFT 1
	RIGHT 2
	TOP 3
	TOPLEFT 4
	TOPRIGHT 5
	BOTTOM 6
	BOTTOMLEFT 7
	BOTTOMRIGHT 8
}

const_bitflag! { WS: u32;
	/// Window
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles)
	/// (`u32`).
	///
	/// **Note:** Control-specific styles have their own types, which are
	/// convertible to `WS`.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// The window is an overlapped window. An overlapped window has a title bar
	/// and a border. Same as the `WS::TILED` style.
	OVERLAPPED 0x0000_0000
	/// The window is a pop-up window. This style cannot be used with the
	/// `WS_CHILD` style.
	POPUP 0x8000_0000
	/// The window is a child window. A window with this style cannot have a menu
	/// bar. This style cannot be used with the `WS::POPUP` style.
	CHILD 0x4000_0000
	/// The window is initially minimized. Same as the `WS::ICONIC` style.
	MINIMIZE 0x2000_0000
	/// The window is initially visible. This style can be turned on and off by
	/// using the [`HWND::ShowWindow`](crate::prelude::user_Hwnd::ShowWindow) or
	/// [`HWND::SetWindowPos`](crate::prelude::user_Hwnd::SetWindowPos)
	/// function.
	VISIBLE 0x1000_0000
	/// The window is initially disabled. A disabled window cannot receive input
	/// from the user. To change this after a window has been created use the
	/// [`HWND::EnableWindow`](crate::prelude::user_Hwnd::EnableWindow)
	/// function.
	DISABLED 0x0800_0000
	/// Clips child windows relative to each other; that is when a particular
	/// child window receives a [`wm::Paint`](crate::msg::wm::Paint) message,
	/// the `WS::CLIPSIBLINGS` style clips all other overlapping child windows
	/// out of the region of the child window to be updated. If
	/// `WS::CLIPSIBLINGS` is not specified and child windows overlap it is
	/// possible when drawing within the client area of a child window to draw
	/// within the client area of a neighboring child window.
	CLIPSIBLINGS 0x0400_0000
	/// Excludes the area occupied by child windows when drawing occurs within
	/// the parent window. This style is used when creating the parent window.
	CLIPCHILDREN 0x0200_0000
	/// The window is initially maximized.
	MAXIMIZE 0x0100_0000
	/// The window has a title bar (includes the `WS::BORDER` style).
	CAPTION 0x00c0_0000
	/// The window has a thin-line border.
	BORDER 0x0080_0000
	/// The window has a border of a style typically used with dialog boxes. A
	/// window with this style cannot have a title bar.
	DLGFRAME 0x0040_0000
	/// The window has a vertical scroll bar.
	VSCROLL 0x0020_0000
	/// The window has a horizontal scroll bar.
	HSCROLL 0x0010_0000
	/// The window has a window menu on its title bar. The `WS::CAPTION` style
	/// must also be specified.
	SYSMENU 0x0008_0000
	/// The window has a sizing border. Same as the `WS::SIZEBOX` style.
	THICKFRAME 0x0004_0000
	/// The window is the first control of a group of controls. The group
	/// consists of this first control and all controls defined after it up to
	/// the next control with the `WS::GROUP` style. The first control in each
	/// group usually has the `WS::TABSTOP` style so that the user can move from
	/// group to group. The user can subsequently change the keyboard focus from
	/// one control in the group to the next control in the group by using the
	/// direction keys.
	///
	/// You can turn this style on and off to change dialog box navigation. To
	/// change this style after a window has been created use the
	/// [`HWND::SetWindowLongPtr`](crate::prelude::user_Hwnd::SetWindowLongPtr)
	/// function.
	GROUP 0x0002_0000
	/// The window is a control that can receive the keyboard focus when the user
	/// presses the TAB key. Pressing the TAB key changes the keyboard focus to
	/// the next control with the `WS::TABSTOP` style.
	///
	/// You can turn this style on and off to change dialog box navigation. To
	/// change this style after a window has been created use the
	/// [`HWND::SetWindowLongPtr`](crate::prelude::user_Hwnd::SetWindowLongPtr)
	/// function. For user-created windows and modeless dialogs to work with tab
	/// stops alter the message loop to call the
	/// [`HWND::IsDialogMessage`](crate::prelude::user_Hwnd::IsDialogMessage)
	/// function.
	TABSTOP 0x0001_0000
	/// The window has a minimize button. Cannot be combined with the
	/// [`WS_EX::CONTEXTHELP`](crate::co::WS_EX::CONTEXTHELP) style. The
	/// `WS::SYSMENU` style must also be specified.
	MINIMIZEBOX 0x0002_0000
	/// The window has a maximize button. Cannot be combined with the
	/// [`WS_EX::CONTEXTHELP`](crate::co::WS_EX::CONTEXTHELP) style. The
	/// `WS::SYSMENU` style must also be specified.
	MAXIMIZEBOX 0x0001_0000
	/// The window is an overlapped window. An overlapped window has a title bar
	/// and a border. Same as the `WS::OVERLAPPED` style.
	TILED Self::OVERLAPPED.0
	/// The window is initially minimized. Same as the `WS::MINIMIZE` style.
	ICONIC Self::MINIMIZE.0
	/// The window has a sizing border. Same as the `WS::THICKFRAME` style.
	SIZEBOX Self::THICKFRAME.0
	/// The window is an overlapped window. Same as the `WS::OVERLAPPEDWINDOW`
	/// style.
	TILEDWINDOW Self::OVERLAPPEDWINDOW.0
	/// The window is an overlapped window. Same as the `WS::TILEDWINDOW` style.
	OVERLAPPEDWINDOW Self::OVERLAPPED.0 | Self::CAPTION.0 | Self::SYSMENU.0 | Self::THICKFRAME.0 | Self::MINIMIZEBOX.0 | Self::MAXIMIZEBOX.0
	/// The window is a pop-up window. This style cannot be used with the
	/// `WS::CHILD` style.
	POPUPWINDOW Self::POPUP.0 | Self::BORDER.0 | Self::SYSMENU.0
	/// Same as the `WS::CHILD` style.
	CHILDWINDOW Self::CHILD.0
}

const_bitflag! { WS_EX: u32;
	/// Extended window
	/// [styles](https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)
	/// (`u32`).
	///
	/// **Note:** Control-specific extended styles have their own types, which
	/// are convertible to `WS`.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// The window has a double border; the window can optionally be created
	/// with a title bar by specifying the
	/// [`WS::CAPTION`](crate::co::WS::CAPTION) style in the dwStyle parameter.
	DLGMODALFRAME 0x0000_0001
	/// The child window created with this style does not send the
	/// [`wm::ParentNotify`](crate::msg::wm::ParentNotify) message to its parent
	/// window when it is created or destroyed.
	NOPARENTNOTIFY 0x0000_0004
	/// The window should be placed above all non-topmost windows and should
	/// stay above them even when the window is deactivated. To add or remove
	/// this style use the
	/// [`HWND::SetWindowPos`](crate::prelude::user_Hwnd::SetWindowPos)
	/// function.
	TOPMOST 0x0000_0008
	/// The window accepts drag-drop files.
	ACCEPTFILES 0x0000_0010
	/// The window should not be painted until siblings beneath the window (that
	/// were created by the same thread) have been painted. The window appears
	/// transparent because the bits of underlying sibling windows have already
	/// been painted.
	///
	/// To achieve transparency without these restrictions use the
	/// [`HWND::SetWindowRgn`](crate::prelude::user_Hwnd::SetWindowRgn)
	/// function.
	TRANSPARENT 0x0000_0020
	/// The window is a MDI child window.
	MDICHILD 0x0000_0040
	/// The window is intended to be used as a floating toolbar. A tool window
	/// has a title bar that is shorter than a normal title bar and the window
	/// title is drawn using a smaller font. A tool window does not appear in the
	/// taskbar or in the dialog that appears when the user presses ALT+TAB. If a
	/// tool window has a system menu its icon is not displayed on the title
	/// bar. However you can display the system menu by right-clicking or by
	/// typing ALT+SPACE.
	TOOLWINDOW 0x0000_0080
	/// The window has a border with a raised edge.
	WINDOWEDGE 0x0000_0100
	/// The window has a border with a sunken edge.
	CLIENTEDGE 0x0000_0200
	/// The title bar of the window includes a question mark. When the user
	/// clicks the question mark the cursor changes to a question mark with a
	/// pointer. If the user then clicks a child window the child receives a
	/// [`wm::Help`](crate::msg::wm::Help) message. The child window should pass
	/// the message to the parent window procedure which should call the
	/// [`WinHelp`](crate::prelude::user_Hwnd::WinHelp) function using the
	/// `HELP_WM_HELP` command. The Help application displays a pop-up window
	/// that typically contains help for the child window.
	///
	/// `WS_EX::CONTEXTHELP` cannot be used with the
	/// [`WS::MAXIMIZEBOX`](crate::co::WS::MAXIMIZEBOX) or
	/// [`WS::MINIMIZEBOX`](crate::co::WS::MINIMIZEBOX) styles.
	CONTEXTHELP 0x0000_0400
	/// The window has generic "right-aligned" properties. This depends on the
	/// window class. This style has an effect only if the shell language is
	/// Hebrew Arabic or another language that supports reading-order
	/// alignment; otherwise the style is ignored.
	///
	/// Using the `WS_EX::RIGHT` style for static or edit controls has the same
	/// effect as using the [`SS::RIGHT`](crate::co::SS::RIGHT) or
	/// [`ES::RIGHT`](crate::co::ES::RIGHT) style respectively. Using this
	/// style with button controls has the same effect as using
	/// [`BS::RIGHT`](crate::co::BS::RIGHT) and
	/// [`BS::RIGHTBUTTON`](crate::co::BS::RIGHTBUTTON) styles.
	RIGHT 0x0000_1000
	/// The window has generic left-aligned properties. This is the default.
	LEFT 0x0000_0000
	/// If the shell language is Hebrew Arabic or another language that
	/// supports reading-order alignment the window text is displayed using
	/// right-to-left reading-order properties. For other languages the style is
	/// ignored.
	RTLREADING 0x0000_2000
	/// The window text is displayed using left-to-right reading-order
	/// properties. This is the default.
	LTRREADING 0x0000_0000
	/// If the shell language is Hebrew Arabic or another language that
	/// supports reading order alignment the vertical scroll bar (if present) is
	/// to the left of the client area. For other languages the style is ignored.
	LEFTSCROLLBAR 0x0000_4000
	/// The vertical scroll bar (if present) is to the right of the client area.
	/// This is the default.
	RIGHTSCROLLBAR 0x0000_0000
	/// The window itself contains child windows that should take part in dialog
	/// box navigation. If this style is specified the dialog manager recurses
	/// into children of this window when performing navigation operations such
	/// as handling the TAB key an arrow key or a keyboard mnemonic.
	CONTROLPARENT 0x0001_0000
	/// The window has a three-dimensional border style intended to be used for
	/// items that do not accept user input.
	STATICEDGE 0x0002_0000
	/// Forces a top-level window onto the taskbar when the window is visible.
	APPWINDOW 0x0004_0000
	/// The window is an overlapped window.
	OVERLAPPEDWINDOW Self::WINDOWEDGE.0 | Self::CLIENTEDGE.0
	/// The window is palette window which is a modeless dialog box that
	/// presents an array of commands.
	PALETTEWINDOW Self::WINDOWEDGE.0 | Self::TOOLWINDOW.0 | Self::TOPMOST.0
	/// The window is a layered window. This style cannot be used if the window
	/// has a class style of either [`CS::OWNDC`](crate::co::CS::OWNDC) or
	/// [`CS::CLASSDC`](crate::co::CS::CLASSDC).
	///
	/// Windows 8: The `WS_EX::LAYERED` style is supported for top-level windows
	/// and child windows. Previous Windows versions support `WS_EX::LAYERED`
	/// only for top-level windows.
	LAYERED 0x0008_0000
	/// The window does not pass its window layout to its child windows.
	NOINHERITLAYOUT 0x0010_0000
	/// The window does not render to a redirection surface. This is for windows
	/// that do not have visible content or that use mechanisms other than
	/// surfaces to provide their visual.
	NOREDIRECTIONBITMAP 0x0020_0000
	/// If the shell language is Hebrew Arabic or another language that
	/// supports reading order alignment the horizontal origin of the window is
	/// on the right edge. Increasing horizontal values advance to the left.
	LAYOUTRTL 0x0040_0000
	/// Paints all descendants of a window in bottom-to-top painting order using
	/// double-buffering. Bottom-to-top painting order allows a descendent
	/// window to have translucency (alpha) and transparency (color-key)
	/// effects but only if the descendent window also has the
	/// `WS_EX::TRANSPARENT` bit set. Double-buffering allows the window and its
	/// descendents to be painted without flicker. This cannot be used if the
	/// window has a class style of either [`CS::OWNDC`](crate::co::CS::OWNDC)
	/// or [`CS::CLASSDC`](crate::co::CS::CLASSDC).
	///
	/// Windows 2000: This style is not supported.
	COMPOSITED 0x0200_0000
	/// A top-level window created with this style does not become the foreground
	/// window when the user clicks it. The system does not bring this window to
	/// the foreground when the user minimizes or closes the foreground window.
	///
	/// The window should not be activated through programmatic access or via
	/// keyboard navigation by accessible technology such as Narrator.
	///
	/// To activate the window use the SetActiveWindow or
	/// [`HWND::SetForegroundWindow`](crate::prelude::user_Hwnd::SetForegroundWindow)
	/// function.
	///
	/// The window does not appear on the taskbar by default. To force the
	/// window to appear on the taskbar use the `WS_EX::APPWINDOW` style.
	NOACTIVATE 0x0800_0000
}

const_bitflag! { WVR: u32;
	/// [`wm::NcCalcSize`](crate::msg::wm::NcCalcSize) return flags (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	ALIGNTOP 0x0010
	ALIGNLEFT 0x0020
	ALIGNBOTTOM 0x0040
	ALIGNRIGHT 0x0080
	HREDRAW 0x0100
	VREDRAW 0x0200
	REDRAW Self::HREDRAW.0 | Self::VREDRAW.0
	VALIDRECTS 0x0400
}

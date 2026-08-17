#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::*;

const_ordinary! { FO: u32;
	/// [`SHFILEOPSTRUCT`](crate::SHFILEOPSTRUCT) `wFunc` (`u32`).
	=>
	=>
	MOVE 0x0001
	COPY 0x0002
	DELETE 0x0003
	RENAME 0x0004
}

const_bitflag! { FOF: u16;
	/// [`SHFILEOPSTRUCT`](crate::SHFILEOPSTRUCT) `fFlags` (`u16`).
	=>
	=>
	MULTIDESTFILES 0x0001
	CONFIRMMOUSE 0x0002
	SILENT 0x0004
	RENAMEONCOLLISION 0x0008
	NOCONFIRMATION 0x0010
	WANTMAPPINGHANDLE 0x0020
	ALLOWUNDO 0x0040
	FILESONLY 0x0080
	SIMPLEPROGRESS 0x0100
	NOCONFIRMMKDIR 0x0200
	NOERRORUI 0x0400
	NOCOPYSECURITYATTRIBS 0x0800
	NORECURSION 0x1000
	NO_CONNECTED_ELEMENTS 0x2000
	WANTNUKEWARNING 0x4000
	NORECURSEREPARSE 0x8000
	NO_UI Self::SILENT.0 | Self::NOCONFIRMATION.0 | Self::NOERRORUI.0 | Self::NOCONFIRMMKDIR.0
}

const_bitflag! { FOS: u32;
	/// [`_FILEOPENDIALOGOPTIONS`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-_fileopendialogoptions)
	/// enumeration (`u32`).
	=>
	=>
	/// When saving a file prompt before overwriting an existing file of the
	/// same name. This is a default value for the Save dialog.
	OVERWRITEPROMPT 0x2
	/// In the Save dialog only allow the user to choose a file that has one of
	/// the file name extensions specified through
	/// [`IFileDialog::SetFileTypes`](crate::prelude::shell_IFileDialog::SetFileTypes).
	STRICTFILETYPES 0x4
	/// Don't change the current working directory.
	NOCHANGEDIR 0x8
	/// Present an Open dialog that offers a choice of folders rather than
	/// files.
	PICKFOLDERS 0x20
	/// Ensures that returned items are file system items
	/// ([`SFGAO::FILESYSTEM`](crate::co::SFGAO::FILESYSTEM)). Note that this
	/// does not apply to items returned by
	/// [`IFileDialog::GetCurrentSelection`](crate::prelude::shell_IFileDialog::GetCurrentSelection).
	FORCEFILESYSTEM 0x40
	/// Enables the user to choose any item in the Shell namespace not just
	/// those with [`SFGAO::STREAM`](crate::co::SFGAO::STREAM) or
	/// [`SFAGO::FILESYSTEM`](crate::co::SFGAO::FILESYSTEM) attributes. This
	/// flag cannot be combined with
	/// [`FOS::FORCEFILESYSTEM`](crate::co::FOS::FORCEFILESYSTEM).
	ALLNONSTORAGEITEMS 0x80
	/// Do not check for situations that would prevent an application from
	/// opening the selected file such as sharing violations or access denied
	/// errors.
	NOVALIDATE 0x100
	/// Enables the user to select multiple items in the open dialog. Note that
	/// when this flag is set the [`IFileOpenDialog`](crate::IFileOpenDialog)
	/// interface must be used to retrieve those items.
	ALLOWMULTISELECT 0x200
	/// The item returned must be in an existing folder. This is a default
	/// value.
	PATHMUSTEXIST 0x800
	/// The item returned must exist. This is a default value for the Open
	/// dialog.
	FILEMUSTEXIST 0x1000
	/// Prompt for creation if the item returned in the save dialog does not
	/// exist. Note that this does not actually create the item.
	CREATEPROMPT 0x2000
	/// In the case of a sharing violation when an application is opening a
	/// file call the application back through
	/// [`OnShareViolation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialogevents-onshareviolation)
	/// for guidance. This flag is overridden by
	/// [`FOS::NOVALIDATE`](crate::co::FOS::NOVALIDATE).
	SHAREAWARE 0x4000
	/// Do not return read-only items. This is a default value for the Save
	/// dialog.
	NOREADONLYRETURN 0x8000
	/// Do not test whether creation of the item as specified in the Save dialog
	/// will be successful. If this flag is not set the calling application
	/// must handle errors such as denial of access discovered when the item
	/// is created.
	NOTESTFILECREATE 0x1_0000
	/// Hide the list of places from which the user has recently opened or saved
	/// items. This value is not supported as of Windows 7.
	HIDEMRUPLACES 0x2_0000
	/// Hide items shown by default in the view's navigation pane. This flag is
	/// often used in conjunction with the
	/// [`IFileDialog::AddPlace`](crate::prelude::shell_IFileDialog::AddPlace)
	/// method, to hide standard locations and replace them with custom
	/// locations.
	///
	/// Windows 7 and later. Hide all of the standard namespace locations (such
	/// as Favorites Libraries Computer and Network) shown in the navigation
	/// pane.
	///
	/// Windows Vista. Hide the contents of the Favorite Links tree in the
	/// navigation pane. Note that the category itself is still displayed but
	/// shown as empty.
	HIDEPINNEDPLACES 0x4_0000
	/// Shortcuts should not be treated as their target items. This allows an
	/// application to open a .lnk file rather than what that file is a shortcut
	/// to.
	NODEREFERENCELINKS 0x10_0000
	/// (This constant has no official documentation.)
	OKBUTTONNEEDSINTERACTION 0x20_0000
	/// Do not add the item being opened or saved to the recent documents list
	/// ([`SHAddToRecentDocs`](crate::SHAddToRecentDocs)).
	DONTADDTORECENT 0x200_0000
	/// Include hidden and system items.
	FORCESHOWHIDDEN 0x1000_0000
	/// Indicates to the Save As dialog box that it should open in expanded
	/// mode. Expanded mode is the mode that is set and unset by clicking the
	/// button in the lower-left corner of the Save As dialog box that switches
	/// between Browse Folders and Hide Folders when clicked. This value is not
	/// supported as of Windows 7.
	DEFAULTNOMINIMODE 0x2000_0000
	/// Indicates to the Open dialog box that the preview pane should always be
	/// displayed.
	FORCEPREVIEWPANEON 0x4000_0000
	/// Indicates that the caller is opening a file as a stream
	/// ([`BHID_Stream`](crate::prelude::shell_IShellItem::BindToHandler)) so
	/// there is no need to download that file.
	SUPPORTSTREAMABLEITEMS 0x8000_0000
}

const_ordinary! { FDAP: u32;
	/// [`FDAP`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-fdap)
	/// enumeration (`u32`).
	=>
	=>
	BOTTOM 0
	TOP 1
}

const_bitflag! { GPS: u32;
	/// [`GETPROPERTYSTOREFLAGS`](https://learn.microsoft.com/en-us/windows/win32/api/propsys/ne-propsys-getpropertystoreflags)
	/// enumeration (`u32`).
	=>
	=>
	DEFAULT 0
	HANDLERPROPERTIESONLY 0x1
	READWRITE 0x2
	TEMPORARY 0x4
	FASTPROPERTIESONLY 0x8
	OPENSLOWITEM 0x10
	DELAYCREATION 0x20
	BESTEFFORT 0x40
	NO_OPLOCK 0x80
	PREFERQUERYPROPERTIES 0x100
	EXTRINSICPROPERTIES 0x200
	EXTRINSICPROPERTIESONLY 0x400
	VOLATILEPROPERTIES 0x800
	VOLATILEPROPERTIESONLY 0x1000
	MASK_VALID 0x1fff
}

const_bitflag! { KF: u32;
	/// [`KNOWN_FOLDER_FLAG`](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/ne-shlobj_core-known_folder_flag)
	/// enumeration (`u32`).
	=>
	=>
	DEFAULT 0x0000_0000
   FORCE_APP_DATA_REDIRECTION 0x0008_0000
	RETURN_FILTER_REDIRECTION_TARGET 0x0004_0000
	FORCE_PACKAGE_REDIRECTION 0x0002_0000
	NO_PACKAGE_REDIRECTION 0x0001_0000
	FORCE_APPCONTAINER_REDIRECTION 0x0002_0000
	NO_APPCONTAINER_REDIRECTION 0x0001_0000
	CREATE 0x0000_8000
	DONT_VERIFY 0x0000_4000
	DONT_UNEXPAND 0x0000_2000
	NO_ALIAS 0x0000_1000
	INIT 0x0000_0800
	DEFAULT_PATH 0x0000_0400
	NOT_PARENT_RELATIVE 0x0000_0200
	SIMPLE_IDLIST 0x0000_0100
	ALIAS_ONLY 0x8000_0000
}

const_bitflag! { NIF: u32;
	/// [`NOTIFYICONDATA`](crate::NOTIFYICONDATA) `uFlags` (`u32`).
	=>
	=>
	MESSAGE 0x0000_0001
	ICON 0x0000_0002
	TIP 0x0000_0004
	STATE 0x0000_0008
	INFO 0x0000_0010
	GUID 0x0000_0020
	REALTIME 0x0000_0040
	SHOWTIP 0x0000_0080
}

const_bitflag! { NIIF: u32;
	/// [`NOTIFYICONDATA`](crate::NOTIFYICONDATA) `dwInfoFlags` (`u32`).
	=>
	=>
	NONE 0x0000_0000
	INFO 0x0000_0001
	WARNING 0x0000_0002
	ERROR 0x0000_0003
	USER 0x0000_0004
	NOSOUND 0x0000_0010
	LARGE_ICON 0x0000_0020
	RESPECT_QUIET_TIME 0x0000_0080
}

const_ordinary! { NIM: u32;
	/// [`Shell_NotifyIcon`](crate::Shell_NotifyIcon) `message` (`u32`).
	=>
	=>
	ADD 0x0000_0000
	MODIFY 0x0000_0001
	DELETE 0x0000_0002
	SETFOCUS 0x0000_0003
	SETVERSION 0x0000_0004
}

const_bitflag! { NIS: u32;
	/// [`NOTIFYICONDATA`](crate::NOTIFYICONDATA) `dwState` and `dwStateFlags`
	/// (`u32`).
	=>
	=>
	HIDDEN 0x0000_0001
	SHAREDICON 0x0000_0002
}

const_ordinary! { SE_ERR: u32;
	/// [`HWND::ShellExecute`](crate::prelude::shell_Hwnd::ShellExecute) return
	/// value (`u32`).
	=>
	=>
	FILE_NOT_FOUND 2
	PATH_NOT_FOUND 3
	BAD_FORMAT 11

	ACCESSDENIED 5
	OOM 8
	DLLNOTFOUND 32

	SHARE 26
	ASSOCINCOMPLETE 27
	DDETIMEOUT 28
	DDEFAIL 29
	DDEBUSY 30
	NOASSOC 31
}

const_bitflag! { SFGAO: u32;
	/// [`SFGAO`](https://learn.microsoft.com/en-us/windows/win32/shell/sfgao)
	/// constants (`u32`).
	=>
	=>
	CANCOPY DROPEFFECT::COPY.raw()
	CANMOVE DROPEFFECT::MOVE.raw()
	CANLINK DROPEFFECT::LINK.raw()
	STORAGE 0x0000_0008
	CANRENAME 0x0000_0010
	CANDELETE 0x0000_0020
	HASPROPSHEET 0x0000_0040
	DROPTARGET 0x0000_0100
	CAPABILITYMASK 0x0000_0177
	SYSTEM 0x0000_1000
	ENCRYPTED 0x0000_2000
	ISSLOW 0x0000_4000
	GHOSTED 0x0000_8000
	LINK 0x0001_0000
	SHARE 0x0002_0000
	READONLY 0x0004_0000
	HIDDEN 0x0008_0000
	FILESYSANCESTOR 0x1000_0000
	FOLDER 0x2000_0000
	FILESYSTEM 0x4000_0000
	HASSUBFOLDER 0x8000_0000
	CONTENTSMASK 0x8000_0000
	VALIDATE 0x0100_0000
	REMOVABLE 0x0200_0000
	COMPRESSED 0x0400_0000
	BROWSABLE 0x0800_0000
	NONENUMERATED 0x0010_0000
	NEWCONTENT 0x0020_0000
	CANMONIKER 0x0040_0000
	HASSTORAGE 0x0040_0000
	STREAM 0x0040_0000
	STORAGEANCESTOR 0x0080_0000
	STORAGECAPMASK 0x70c5_0008
	PKEYSFGAOMASK 0x8104_4000
}

const_ordinary! { SHARD: u32;
	/// [`SHARD`](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/ne-shlobj_core-shard)
	/// enumeration (`u32`).
	=>
	=>
	PIDL 0x0000_0001
	PATHA 0x0000_0002
	PATHW 0x0000_0003
	APPIDINFO 0x0000_0004
	APPIDINFOIDLIST 0x0000_0005
	LINK 0x0000_0006
	APPIDINFOLINK 0x0000_0007
	SHELLITEM 0x0000_0008
}

const_bitflag! { SHGFI: u32;
	/// [`SHGetFileInfo`](crate::SHGetFileInfo) `flags` (`u32`).
	=>
	=>
	ICON 0x0000_0100
	DISPLAYNAME 0x0000_0200
	TYPENAME 0x0000_0400
	ATTRIBUTES 0x0000_0800
	ICONLOCATION 0x0000_1000
	EXETYPE 0x0000_2000
	SYSICONINDEX 0x0000_4000
	LINKOVERLAY 0x0000_8000
	SELECTED 0x0001_0000
	ATTR_SPECIFIED 0x0002_0000
	LARGEICON 0x0000_0000
	SMALLICON 0x0000_0001
	OPENICON 0x0000_0002
	SHELLICONSIZE 0x0000_0004
	PIDL 0x0000_0008
	USEFILEATTRIBUTES 0x0000_0010
	ADDOVERLAYS 0x0000_0020
	OVERLAYINDEX 0x0000_0040
}

const_bitflag! { SHGSI: u32;
	/// [`SHGetStockIconInfo`](crate::SHGetStockIconInfo) `flags` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	ICONLOCATION 0
	ICON SHGFI::ICON.0
	SYSICONINDEX SHGFI::SYSICONINDEX.0
	LINKOVERLAY SHGFI::LINKOVERLAY.0
	SELECTED SHGFI::SELECTED.0
	LARGEICON SHGFI::LARGEICON.0
	SMALLICON SHGFI::SMALLICON.0
	SHELLICONSIZE SHGFI::SHELLICONSIZE.0
}

const_ordinary! { SICHINTF: u32;
	/// [`SICHINTF`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-_sichintf)
	/// enumeration (`u32`).
	=>
	=>
	DISPLAY 0
	ALLFIELDS 0x8000_0000
	CANONICAL 0x1000_0000
	TEST_FILESYSPATH_IF_NOT_EQUAL 0x2000_0000
}

const_ordinary! { SIGDN: u32;
	/// [`SIGDN`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-sigdn)
	/// enumeration (`u32`).
	=>
	=>
	/// Returns the display name relative to the parent folder. In UI this name
	/// is generally ideal for display to the user.
	NORMALDISPLAY 0
	/// Returns the parsing name relative to the parent folder. This name is not
	/// suitable for use in UI.
	PARENTRELATIVEPARSING 0x8001_8001
	/// Returns the parsing name relative to the desktop. This name is not
	/// suitable for use in UI.
	DESKTOPABSOLUTEPARSING 0x8002_8000
	/// Returns the editing name relative to the parent folder. In UI this name
	/// is suitable for display to the user.
	PARENTRELATIVEEDITING 0x8003_1001
	/// Returns the editing name relative to the desktop. In UI this name is
	/// suitable for display to the user.
	DESKTOPABSOLUTEEDITING 0x8004_c000
	/// Returns the item's file system path if it has one. Only items that
	/// report [`SFGAO::FILESYSTEM`](crate::co::SFGAO::FILESYSTEM) have a file
	/// system path. When an item does not have a file system path a call to
	/// [`IShellItem::GetDisplayName`](crate::prelude::shell_IShellItem::GetDisplayName)
	/// on that item will fail. In UI this name is suitable for display to the
	/// user in some cases but note that it might not be specified for all
	/// items.
	FILESYSPATH 0x8005_8000
	/// Returns the item's URL if it has one. Some items do not have a URL and
	/// in those cases a call to
	/// [`IShellItem::GetDisplayName`](crate::prelude::shell_IShellItem::GetDisplayName)
	/// will fail. This name is suitable for display to the user in some cases,
	/// but note that it might not be specified for all items.
	URL 0x8006_8000
	/// Returns the path relative to the parent folder in a friendly format as
	/// displayed in an address bar. This name is suitable for display to the
	/// user.
	PARENTRELATIVEFORADDRESSBAR 0x8007_c001
	/// Returns the path relative to the parent folder.
	PARENTRELATIVE 0x8008_0001
	/// Introduced in Windows 8.
	PARENTRELATIVEFORUI 0x8009_4001
}

const_ordinary! { SIID: u32;
	/// [`SHSTOCKICONID`](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ne-shellapi-shstockiconid)
	/// enumeration, [`SHGetStockIconInfo`](crate::SHGetStockIconInfo) `siid`
	/// (`u32`).
	=>
	=>
	DOCNOASSOC 0
	DOCASSOC 1
	APPLICATION 2
	FOLDER 3
	FOLDEROPEN 4
	DRIVE525 5
	DRIVE35 6
	DRIVEREMOVE 7
	DRIVEFIXED 8
	DRIVENET 9
	DRIVENETDISABLED 10
	DRIVECD 11
	DRIVERAM 12
	WORLD 13
	SERVER 15
	PRINTER 16
	MYNETWORK 17
	FIND 22
	HELP 23
	SHARE 28
	LINK 29
	SLOWFILE 30
	RECYCLER 31
	RECYCLERFULL 32
	MEDIACDAUDIO 40
	LOCK 47
	AUTOLIST 49
	PRINTERNET 50
	SERVERSHARE 51
	PRINTERFAX 52
	PRINTERFAXNET 53
	PRINTERFILE 54
	STACK 55
	MEDIASVCD 56
	STUFFEDFOLDER 57
	DRIVEUNKNOWN 58
	DRIVEDVD 59
	MEDIADVD 60
	MEDIADVDRAM 61
	MEDIADVDRW 62
	MEDIADVDR 63
	MEDIADVDROM 64
	MEDIACDAUDIOPLUS 65
	MEDIACDRW 66
	MEDIACDR 67
	MEDIACDBURN 68
	MEDIABLANKCD 69
	MEDIACDROM 70
	AUDIOFILES 71
	IMAGEFILES 72
	VIDEOFILES 73
	MIXEDFILES 74
	FOLDERBACK 75
	FOLDERFRONT 76
	SHIELD 77
	WARNING 78
	INFO 79
	ERROR 80
	KEY 81
	SOFTWARE 82
	RENAME 83
	DELETE 84
	MEDIAAUDIODVD 85
	MEDIAMOVIEDVD 86
	MEDIAENHANCEDCD 87
	MEDIAENHANCEDDVD 88
	MEDIAHDDVD 89
	MEDIABLURAY 90
	MEDIAVCD 91
	MEDIADVDPLUSR 92
	MEDIADVDPLUSRW 93
	DESKTOPPC 94
	MOBILEPC 95
	USERS 96
	MEDIASMARTMEDIA 97
	MEDIACOMPACTFLASH 98
	DEVICECELLPHONE 99
	DEVICECAMERA 100
	DEVICEVIDEOCAMERA 101
	DEVICEAUDIOPLAYER 102
	NETWORKCONNECT 103
	INTERNET 104
	ZIPFILE 105
	SETTINGS 106
	DRIVEHDDVD 132
	DRIVEBD 133
	MEDIAHDDVDROM 134
	MEDIAHDDVDR 135
	MEDIAHDDVDRAM 136
	MEDIABDROM 137
	MEDIABDR 138
	MEDIABDRE 139
	CLUSTEREDDRIVE 140
	MAX_ICONS 181
}

const_bitflag! { SLGP: u32;
	/// [`IShellLink::GetPath`](crate::prelude::shell_IShellLink::GetPath)
	/// `flags` (`u32`).
	=>
	=>
	SHORTPATH 0x1
	UNCPRIORITY 0x2
	RAWPATH 0x4
	RELATIVEPRIORITY 0x8
}

const_bitflag! { SLR: u32;
	/// [`IShellLink::Resolve`](crate::prelude::shell_IShellLink::GetPath)
	/// `flags` (`u32`).
	=>
	=>
	NONE 0
	NO_UI 0x1
	ANY_MATCH 0x2
	UPDATE 0x4
	NOUPDATE 0x8
	NOSEARCH 0x10
	NOTRACK 0x20
	NOLINKINFO 0x40
	INVOKE_MSI 0x80
	NO_UI_WITH_MSG_PUMP 0x101
	OFFER_DELETE_WITHOUT_FILE 0x200
	KNOWNFOLDER 0x400
	MACHINE_IN_LOCAL_TARGET 0x800
	UPDATE_MACHINE_AND_SID 0x1000
	NO_OBJECT_ID 0x2000
}

const_ordinary! { STPFLAG: u32;
	/// [`STPFLAG`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/ne-shobjidl_core-stpflag)
	/// enumeration (`u32`).
	=>
	=>
	NONE 0
	USEAPPTHUMBNAILALWAYS 0x1
	USEAPPTHUMBNAILWHENACTIVE 0x2
	USEAPPPEEKALWAYS 0x4
	USEAPPPEEKWHENACTIVE 0x8
}

const_ordinary! { TBPF: u32;
	/// [`ITaskbarList3::SetProgressState`](crate::prelude::shell_ITaskbarList3::SetProgressState)
	/// `tbpFlags` (`u32`).
	=>
	=>
	/// Stops displaying progress and returns the button to its normal state.
	/// Call this method with this flag to dismiss the progress bar when the
	/// operation is complete or canceled.
	NOPROGRESS 0
	/// The progress indicator does not grow in size but cycles repeatedly
	/// along the length of the taskbar button. This indicates activity without
	/// specifying what proportion of the progress is complete. Progress is
	/// taking place but there is no prediction as to how long the operation
	/// will take.
	INDETERMINATE 0x1
	/// The progress indicator grows in size from left to right in proportion to
	/// the estimated amount of the operation completed. This is a determinate
	/// progress indicator; a prediction is being made as to the duration of the
	/// operation.
	NORMAL 0x2
	/// The progress indicator turns red to show that an error has occurred in
	/// one of the windows that is broadcasting progress. This is a determinate
	/// state. If the progress indicator is in the indeterminate state it
	/// switches to a red determinate display of a generic percentage not
	/// indicative of actual progress.
	ERROR 0x4
	/// The progress indicator turns yellow to show that progress is currently
	/// stopped in one of the windows but can be resumed by the user. No error
	/// condition exists and nothing is preventing the progress from continuing.
	/// This is a determinate state. If the progress indicator is in the
	/// indeterminate state it switches to a yellow determinate display of a
	/// generic percentage not indicative of actual progress.
	PAUSED 0x8
}

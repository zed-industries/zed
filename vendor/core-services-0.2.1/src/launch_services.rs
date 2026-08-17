#![allow(non_upper_case_globals)]
#![allow(bad_style)]

pub use core_foundation::array::CFArray;
pub use core_foundation::array::CFArrayRef;
/// The functions you want are probably missing - please make a [pull request](https://github.com/kornelski/core-services) adding whatever you need!
///
/// [The original Apple docs are here](https://developer.apple.com/documentation/coreservices?language=objc)
use core_foundation::array::CFIndex;
use core_foundation::base::CFAllocatorRef;
use core_foundation::base::CFTypeID;
use core_foundation::base::CFTypeRef;
use core_foundation::base::OSStatus;
pub use core_foundation::base::TCFType;
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::error::CFErrorRef;
use core_foundation::runloop::CFRunLoopRef;
pub use core_foundation::string::CFString;
pub use core_foundation::string::CFStringRef;
use core_foundation::url::CFURLRef;
use std::os::raw::c_void;

/// Requests the role None (the application cannot open the item, but provides an icon and a kind string for it).
pub const kLSRolesNone: LSRolesMask = 0x00000001;

/// Requests the role Viewer (the application can read and present the item, but cannot manipulate or save it).
pub const kLSRolesViewer: LSRolesMask = 0x00000002;

/// Requests the role Editor (the application can read, present, manipulate, and save the item).
pub const kLSRolesEditor: LSRolesMask = 0x00000004;

/// Requests the role Shell (the application can execute the item).
pub const kLSRolesShell: LSRolesMask = 0x00000008;

/// Accepts any role with respect to the item.
pub const kLSRolesAll: LSRolesMask = 0xFFFFFFFF;

/// Requests the default behavior (as opposed to the behavior specified by the kLSAcceptAllowLoginUI flag).
pub const kLSAcceptDefault: LSAcceptanceFlags = 0x00000001;

/// Requests that the user interface to log in be presented if necessary. If LSCanRefAcceptItem or
/// LSCanURLAcceptURL is called during a drag-and-drop operation, showing a server login dialog would
/// be an inappropriate user experience. If the target designated in the function call is an alias to
/// an application, Launch Services needs to resolve the alias to ascertain what file types the
/// application can open; however, if the application is on a server that needs to be authenticated,
/// Launch Services will by default fail to resolve the alias, to avoid having to present the login interface.
/// To override this default behavior by allowing the server login interface, set the kLSAcceptAllowLoginUI flag.
pub const kLSAcceptAllowLoginUI: LSAcceptanceFlags = 0x00000002;

pub type FourCharCode = u32;
pub type OSType = FourCharCode;
pub type ResType = FourCharCode;

pub type DescType = ResType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAEDataStorageType {
    _unused: [u8; 0],
}
pub type AEDataStorageType = *mut OpaqueAEDataStorageType;
pub type AEDataStorage = *mut AEDataStorageType;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct AEDesc {
    pub descriptorType: DescType,
    pub dataHandle: AEDataStorage,
}
pub type OptionBits = u32;
pub type AEDescList = AEDesc;
pub type AERecord = AEDescList;
pub type AppleEvent = AERecord;
pub type LSRolesMask = OptionBits;
/// Specify the desired role or roles for an application to claim with respect to an item or family of items.
pub type LSAcceptanceFlags = OptionBits;
pub type LSRequestedInfo = OptionBits;
pub type LSItemInfoFlags = OptionBits;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LSItemInfoRecord {
    pub flags: LSItemInfoFlags,
    pub filetype: OSType,
    pub creator: OSType,
    pub extension: CFStringRef,
}
pub type LSHandlerOptions = OptionBits;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSRef {
    pub hidden: [u8; 80usize],
}

pub type ItemCount = ::std::os::raw::c_ulong;

#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LSLaunchFSRefSpec {
    pub appRef: *const FSRef,
    pub numDocs: ItemCount,
    pub itemRefs: *const FSRef,
    pub passThruParams: *const AEDesc,
    pub launchFlags: LSLaunchFlags,
    pub asyncRefCon: *mut ::std::os::raw::c_void,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct LSApplicationParameters {
    pub version: CFIndex,
    pub flags: LSLaunchFlags,
    pub application: *const FSRef,
    pub asyncLaunchRefCon: *mut ::std::os::raw::c_void,
    pub environment: CFDictionaryRef,
    pub argv: CFArrayRef,
    pub initialEvent: *mut AppleEvent,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueLSSharedFileListItemRef {
    _unused: [u8; 0],
}
pub type LSSharedFileListItemRef = *mut OpaqueLSSharedFileListItemRef;
pub type LSSharedFileListResolutionFlags = u32;
pub type LSSharedFileListChangedProcPtr = ::std::option::Option<
    unsafe extern "C" fn(inList: LSSharedFileListRef, context: *mut ::std::os::raw::c_void),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueIconRef {
    _unused: [u8; 0],
}
pub type IconRef = *mut OpaqueIconRef;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationOpaqueRef {
    _unused: [u8; 0],
}
pub type AuthorizationRef = *const AuthorizationOpaqueRef;

#[repr(C, packed(2))]
pub struct LSLaunchURLSpec {
    /// A Core Foundation URL reference designating the application to launch.
    /// The URL must have scheme file and contain a valid path to an application file or application bundle.
    /// Set this field to NULL to request that each item in the itemURLs array be opened in its own preferred application.
    pub appURL: CFURLRef,
    /// items to open/print
    pub itemURLs: CFArrayRef,
    /// passed untouched to application as optional parameter
    pub passThruParams: *const AEDesc,
    pub launchFlags: LSLaunchFlags,
    /// used if you register for app birth/death notification
    pub asyncRefCon: *mut c_void,
}

pub type LSLaunchFlags = u32;

/// Defaults = open, async
pub const kLSLaunchDefaults: LSLaunchFlags = 0x00000001;
/// Print items instead of open them
pub const kLSLaunchAndPrint: LSLaunchFlags = 0x00000002;
/// Report launch/open failures in the UI
pub const kLSLaunchAndDisplayErrors: LSLaunchFlags = 0x00000040;
/// Do not add app or documents to recents menus.
pub const kLSLaunchDontAddToRecents: LSLaunchFlags = 0x00000100;
/// Do not bring new app to the foreground.
pub const kLSLaunchDontSwitch: LSLaunchFlags = 0x00000200;
/// Asynchronous launch; return as soon as the app starts launching.
pub const kLSLaunchAsync: LSLaunchFlags = 0x00010000;
/// Instantiate app even if it is already running.
pub const kLSLaunchNewInstance: LSLaunchFlags = 0x00080000;
/// Send child a "hide" request as soon as it checks in.
pub const kLSLaunchAndHide: LSLaunchFlags = 0x00100000;
/// Hide all other apps when the app checks in.
pub const kLSLaunchAndHideOthers: LSLaunchFlags = 0x00200000;

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    /// Indicates that the tag is a filename extension.
    pub static kUTTagClassFilenameExtension: CFStringRef;
    /// Indicates that the tag is a MIME type.
    pub static kUTTagClassMIMEType: CFStringRef;
    /// Indicates that the tag is an NSPasteBoard type.
    pub static kUTTagClassNSPboardType: CFStringRef;
    /// Indicates that the tag is an OSType.
    pub static kUTTagClassOSType: CFStringRef;

    pub fn UTTypeCreatePreferredIdentifierForTag(
        inTagClass: CFStringRef,
        inTag: CFStringRef,
        inConformingToUTI: CFStringRef,
    ) -> CFStringRef;

    pub fn LSCopyDefaultRoleHandlerForContentType(
        inContentType: CFStringRef,
        inRole: LSRolesMask,
    ) -> CFStringRef;
    /// Locates an array of application bundle identifiers for applications capable of handling a specified content type with the specified roles.
    pub fn LSCopyAllRoleHandlersForContentType(
        inContentType: CFStringRef,
        inRole: LSRolesMask,
    ) -> CFArrayRef;

    /// Locates all URLs for applications that correspond to the specified bundle identifier.
    pub fn LSCopyApplicationURLsForBundleIdentifier(
        inBundleIdentifier: CFStringRef,
        outError: *mut CFErrorRef,
    ) -> CFArrayRef;

    /// Tests whether an application can accept (open) an item designated by URL.
    pub fn LSCanURLAcceptURL(
        inItemURL: CFURLRef,
        inTargetURL: CFURLRef,
        inRoleMask: LSRolesMask,
        inFlags: LSAcceptanceFlags,
        outAcceptsItem: *mut bool,
    ) -> OSStatus;

    /// Opens one or more items designated by URL, in either their preferred applications or a designated application.
    pub fn LSOpenFromURLSpec(
        inLaunchSpec: *const LSLaunchURLSpec,
        outLaunchedURL: *mut CFURLRef,
    ) -> OSStatus;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueLSSharedFileListRef {
    _unused: [u8; 0],
}
pub type LSSharedFileListRef = *mut OpaqueLSSharedFileListRef;

extern "C" {
    pub fn LSSharedFileListGetTypeID() -> CFTypeID;

    pub fn LSSharedFileListItemGetTypeID() -> CFTypeID;

    pub fn LSSharedFileListCreate(
        inAllocator: CFAllocatorRef,
        inListType: CFStringRef,
        listOptions: CFTypeRef,
    ) -> LSSharedFileListRef;

    pub fn LSSharedFileListSetAuthorization(
        inList: LSSharedFileListRef,
        inAuthorization: AuthorizationRef,
    ) -> OSStatus;

    pub fn LSSharedFileListAddObserver(
        inList: LSSharedFileListRef,
        inRunloop: CFRunLoopRef,
        inRunloopMode: CFStringRef,
        callback: LSSharedFileListChangedProcPtr,
        context: *mut ::std::os::raw::c_void,
    );

    pub fn LSSharedFileListRemoveObserver(
        inList: LSSharedFileListRef,
        inRunloop: CFRunLoopRef,
        inRunloopMode: CFStringRef,
        callback: LSSharedFileListChangedProcPtr,
        context: *mut ::std::os::raw::c_void,
    );

    pub fn LSSharedFileListGetSeedValue(inList: LSSharedFileListRef) -> u32;

    pub fn LSSharedFileListCopyProperty(
        inList: LSSharedFileListRef,
        inPropertyName: CFStringRef,
    ) -> CFTypeRef;

    pub fn LSSharedFileListSetProperty(
        inList: LSSharedFileListRef,
        inPropertyName: CFStringRef,
        inPropertyData: CFTypeRef,
    ) -> OSStatus;

    pub fn LSSharedFileListCopySnapshot(
        inList: LSSharedFileListRef,
        outSnapshotSeed: *mut u32,
    ) -> CFArrayRef;

    pub fn LSSharedFileListInsertItemURL(
        inList: LSSharedFileListRef,
        insertAfterThisItem: LSSharedFileListItemRef,
        inDisplayName: CFStringRef,
        inIconRef: IconRef,
        inURL: CFURLRef,
        inPropertiesToSet: CFDictionaryRef,
        inPropertiesToClear: CFArrayRef,
    ) -> LSSharedFileListItemRef;

    pub fn LSSharedFileListInsertItemFSRef(
        inList: LSSharedFileListRef,
        insertAfterThisItem: LSSharedFileListItemRef,
        inDisplayName: CFStringRef,
        inIconRef: IconRef,
        inFSRef: *const FSRef,
        inPropertiesToSet: CFDictionaryRef,
        inPropertiesToClear: CFArrayRef,
    ) -> LSSharedFileListItemRef;

    pub fn LSSharedFileListItemMove(
        inList: LSSharedFileListRef,
        inItem: LSSharedFileListItemRef,
        inMoveAfterItem: LSSharedFileListItemRef,
    ) -> OSStatus;

    pub fn LSSharedFileListItemRemove(
        inList: LSSharedFileListRef,
        inItem: LSSharedFileListItemRef,
    ) -> OSStatus;

    pub fn LSSharedFileListRemoveAllItems(inList: LSSharedFileListRef) -> OSStatus;

    pub fn LSSharedFileListItemGetID(inItem: LSSharedFileListItemRef) -> u32;

    pub fn LSSharedFileListItemCopyIconRef(inItem: LSSharedFileListItemRef) -> IconRef;

    pub fn LSSharedFileListItemCopyDisplayName(inItem: LSSharedFileListItemRef) -> CFStringRef;

    pub fn LSSharedFileListItemResolve(
        inItem: LSSharedFileListItemRef,
        inFlags: LSSharedFileListResolutionFlags,
        outURL: *mut CFURLRef,
        outRef: *mut FSRef,
    ) -> OSStatus;

    pub fn LSSharedFileListItemCopyResolvedURL(
        inItem: LSSharedFileListItemRef,
        inFlags: LSSharedFileListResolutionFlags,
        outError: *mut CFErrorRef,
    ) -> CFURLRef;

    pub fn LSSharedFileListItemCopyProperty(
        inItem: LSSharedFileListItemRef,
        inPropertyName: CFStringRef,
    ) -> CFTypeRef;

    pub fn LSSharedFileListItemSetProperty(
        inItem: LSSharedFileListItemRef,
        inPropertyName: CFStringRef,
        inPropertyData: CFTypeRef,
    ) -> OSStatus;
}

pub const kLSNo32BitEnvironmentErr: no_overview_available_1191 = -10386;
pub const kLSAppInTrashErr: no_overview_available_1191 = -10660;
pub const kLSExecutableIncorrectFormat: no_overview_available_1191 = -10661;
pub const kLSAttributeNotFoundErr: no_overview_available_1191 = -10662;
pub const kLSAttributeNotSettableErr: no_overview_available_1191 = -10663;
pub const kLSIncompatibleApplicationVersionErr: no_overview_available_1191 = -10664;
pub const kLSNoRosettaEnvironmentErr: no_overview_available_1191 = -10665;
pub const kLSGarbageCollectionUnsupportedErr: no_overview_available_1191 = -10666;
pub const kLSUnknownErr: no_overview_available_1191 = -10810;
pub const kLSNotAnApplicationErr: no_overview_available_1191 = -10811;
pub const kLSNotInitializedErr: no_overview_available_1191 = -10812;
pub const kLSDataUnavailableErr: no_overview_available_1191 = -10813;
pub const kLSApplicationNotFoundErr: no_overview_available_1191 = -10814;
pub const kLSUnknownTypeErr: no_overview_available_1191 = -10815;
pub const kLSDataTooOldErr: no_overview_available_1191 = -10816;
pub const kLSDataErr: no_overview_available_1191 = -10817;
pub const kLSLaunchInProgressErr: no_overview_available_1191 = -10818;
pub const kLSNotRegisteredErr: no_overview_available_1191 = -10819;
pub const kLSAppDoesNotClaimTypeErr: no_overview_available_1191 = -10820;
pub const kLSAppDoesNotSupportSchemeWarning: no_overview_available_1191 = -10821;
pub const kLSServerCommunicationErr: no_overview_available_1191 = -10822;
pub const kLSCannotSetInfoErr: no_overview_available_1191 = -10823;
pub const kLSNoRegistrationInfoErr: no_overview_available_1191 = -10824;
pub const kLSIncompatibleSystemVersionErr: no_overview_available_1191 = -10825;
pub const kLSNoLaunchPermissionErr: no_overview_available_1191 = -10826;
pub const kLSNoExecutableErr: no_overview_available_1191 = -10827;
pub const kLSNoClassicEnvironmentErr: no_overview_available_1191 = -10828;
pub const kLSMultipleSessionsNotSupportedErr: no_overview_available_1191 = -10829;
pub type no_overview_available_1191 = i32;
pub const kLSUnknownType: no_overview_available_1193 = 0;
pub const kLSUnknownCreator: no_overview_available_1193 = 0;
pub type no_overview_available_1193 = u32;

pub const kLSRequestExtension: LSRequestedInfo = 1;
pub const kLSRequestTypeCreator: LSRequestedInfo = 2;
pub const kLSRequestBasicFlagsOnly: LSRequestedInfo = 4;
pub const kLSRequestAppTypeFlags: LSRequestedInfo = 8;
pub const kLSRequestAllFlags: LSRequestedInfo = 16;
pub const kLSRequestIconAndKind: LSRequestedInfo = 32;
pub const kLSRequestExtensionFlagsOnly: LSRequestedInfo = 64;
pub const kLSRequestAllInfo: LSRequestedInfo = 4294967295;

pub const kLSItemInfoIsPlainFile: LSItemInfoFlags = 1;
pub const kLSItemInfoIsPackage: LSItemInfoFlags = 2;
pub const kLSItemInfoIsApplication: LSItemInfoFlags = 4;
pub const kLSItemInfoIsContainer: LSItemInfoFlags = 8;
pub const kLSItemInfoIsAliasFile: LSItemInfoFlags = 16;
pub const kLSItemInfoIsSymlink: LSItemInfoFlags = 32;
pub const kLSItemInfoIsInvisible: LSItemInfoFlags = 64;
pub const kLSItemInfoIsNativeApp: LSItemInfoFlags = 128;
pub const kLSItemInfoIsClassicApp: LSItemInfoFlags = 256;
pub const kLSItemInfoAppPrefersNative: LSItemInfoFlags = 512;
pub const kLSItemInfoAppPrefersClassic: LSItemInfoFlags = 1024;
pub const kLSItemInfoAppIsScriptable: LSItemInfoFlags = 2048;
pub const kLSItemInfoIsVolume: LSItemInfoFlags = 4096;
pub const kLSItemInfoExtensionIsHidden: LSItemInfoFlags = 1048576;

extern "C" {
    pub static mut kLSSharedFileListFavoriteVolumes: CFStringRef;
    pub static mut kLSSharedFileListFavoriteItems: CFStringRef;
    pub static mut kLSSharedFileListRecentApplicationItems: CFStringRef;
    pub static mut kLSSharedFileListRecentDocumentItems: CFStringRef;
    pub static mut kLSSharedFileListRecentServerItems: CFStringRef;
    pub static mut kLSSharedFileListSessionLoginItems: CFStringRef;
    pub static mut kLSSharedFileListGlobalLoginItems: CFStringRef;
    pub static mut kLSSharedFileListRecentItemsMaxAmount: CFStringRef;
    pub static mut kLSSharedFileListVolumesComputerVisible: CFStringRef;
    pub static mut kLSSharedFileListVolumesIDiskVisible: CFStringRef;
    pub static mut kLSSharedFileListVolumesNetworkVisible: CFStringRef;
    pub static mut kLSSharedFileListItemBeforeFirst: LSSharedFileListItemRef;
    pub static mut kLSSharedFileListItemLast: LSSharedFileListItemRef;
    pub static mut kLSSharedFileListItemHidden: CFStringRef;
    pub static mut kLSSharedFileListLoginItemHidden: CFStringRef;
}
extern "C" {
    pub static kLSItemContentType: CFStringRef;
    pub static kLSItemFileType: CFStringRef;
    pub static kLSItemFileCreator: CFStringRef;
    pub static kLSItemExtension: CFStringRef;
    pub static kLSItemDisplayName: CFStringRef;
    pub static kLSItemDisplayKind: CFStringRef;
    pub static kLSItemRoleHandlerDisplayName: CFStringRef;
    pub static kLSItemIsInvisible: CFStringRef;
    pub static kLSItemExtensionIsHidden: CFStringRef;
    pub static kLSItemQuarantineProperties: CFStringRef;
}

extern "C" {
    pub static kLSQuarantineAgentNameKey: CFStringRef;
    pub static kLSQuarantineAgentBundleIdentifierKey: CFStringRef;
    pub static kLSQuarantineTimeStampKey: CFStringRef;
    pub static kLSQuarantineTypeKey: CFStringRef;
    pub static kLSQuarantineTypeWebDownload: CFStringRef;
    pub static kLSQuarantineTypeOtherDownload: CFStringRef;
    pub static kLSQuarantineTypeEmailAttachment: CFStringRef;
    pub static kLSQuarantineTypeInstantMessageAttachment: CFStringRef;
    pub static kLSQuarantineTypeCalendarEventAttachment: CFStringRef;
    pub static kLSQuarantineTypeOtherAttachment: CFStringRef;
    pub static kLSQuarantineOriginURLKey: CFStringRef;
    pub static kLSQuarantineDataURLKey: CFStringRef;
}

pub const kLSLaunchInhibitBGOnly: no_overview_available_1199 = 128;
pub const kLSLaunchNoParams: no_overview_available_1199 = 2048;
pub const kLSLaunchStartClassic: no_overview_available_1199 = 131072;
pub const kLSLaunchInClassic: no_overview_available_1199 = 262144;
pub const kLSLaunchHasUntrustedContents: no_overview_available_1199 = 4194304;
pub type no_overview_available_1199 = u32;

pub const kLSSharedFileListNoUserInteraction: LSSharedFileListResolutionFlags = 1;
pub const kLSSharedFileListDoNotMountVolumes: LSSharedFileListResolutionFlags = 2;

// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Declaration of Writer
use shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, UINT};
use shared::wtypes::BSTR;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::vss::{VSS_ID, VSS_ROLLFORWARD_TYPE};
use um::winnt::{HRESULT, LPCWSTR, VOID};
ENUM!{enum VSS_USAGE_TYPE {
    VSS_UT_UNDEFINED = 0,
    VSS_UT_BOOTABLESYSTEMSTATE = 1,
    VSS_UT_SYSTEMSERVICE = 2,
    VSS_UT_USERDATA = 3,
    VSS_UT_OTHER = 4,
}}
ENUM!{enum VSS_SOURCE_TYPE {
    VSS_ST_UNDEFINED = 0,
    VSS_ST_TRANSACTEDDB = 1,
    VSS_ST_NONTRANSACTEDDB = 2,
    VSS_ST_OTHER = 3,
}}
ENUM!{enum VSS_RESTOREMETHOD_ENUM {
    VSS_RME_UNDEFINED = 0,
    VSS_RME_RESTORE_IF_NOT_THERE = 1,
    VSS_RME_RESTORE_IF_CAN_REPLACE = 2,
    VSS_RME_STOP_RESTORE_START = 3,
    VSS_RME_RESTORE_TO_ALTERNATE_LOCATION = 4,
    VSS_RME_RESTORE_AT_REBOOT = 5,
    VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE = 6,
    VSS_RME_CUSTOM = 7,
    VSS_RME_RESTORE_STOP_START = 8,
}}
ENUM!{enum VSS_WRITERRESTORE_ENUM {
    VSS_WRE_UNDEFINED = 0,
    VSS_WRE_NEVER = 1,
    VSS_WRE_IF_REPLACE_FAILS = 2,
    VSS_WRE_ALWAYS = 3,
}}
ENUM!{enum VSS_COMPONENT_TYPE {
    VSS_CT_UNDEFINED = 0,
    VSS_CT_DATABASE = 1,
    VSS_CT_FILEGROUP = 2,
}}
ENUM!{enum VSS_ALTERNATE_WRITER_STATE {
    VSS_AWS_UNDEFINED = 0,
    VSS_AWS_NO_ALTERNATE_WRITER = 1,
    VSS_AWS_ALTERNATE_WRITER_EXISTS = 2,
    VSS_AWS_THIS_IS_ALTERNATE_WRITER = 3,
}}
ENUM!{enum VSS_SUBSCRIBE_MASK {
    VSS_SM_POST_SNAPSHOT_FLAG = 0x00000001,
    VSS_SM_BACKUP_EVENTS_FLAG = 0x00000002,
    VSS_SM_RESTORE_EVENTS_FLAG = 0x00000004,
    VSS_SM_IO_THROTTLING_FLAG = 0x00000008,
    VSS_SM_ALL_FLAGS = 0xffffffff,
}}
ENUM!{enum VSS_RESTORE_TARGET {
    VSS_RT_UNDEFINED = 0,
    VSS_RT_ORIGINAL = 1,
    VSS_RT_ALTERNATE = 2,
    VSS_RT_DIRECTED = 3,
    VSS_RT_ORIGINAL_LOCATION = 4,
}}
ENUM!{enum VSS_FILE_RESTORE_STATUS {
    VSS_RS_UNDEFINED = 0,
    VSS_RS_NONE = 1,
    VSS_RS_ALL = 2,
    VSS_RS_FAILED = 3,
}}
ENUM!{enum VSS_COMPONENT_FLAGS {
    VSS_CF_BACKUP_RECOVERY = 0x00000001,
    VSS_CF_APP_ROLLBACK_RECOVERY = 0x00000002,
    VSS_CF_NOT_SYSTEM_STATE = 0x00000004,
}}
RIDL!{#[uuid(0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)]
interface IVssWMFiledesc(IVssWMFiledescVtbl): IUnknown(IUnknownVtbl) {
    fn GetPath(
        pbstrPath: *mut BSTR,
    ) -> HRESULT,
    fn GetFilespec(
        pbstrFilespec: *mut BSTR,
    ) -> HRESULT,
    fn GetRecursive(
        pbRecursive: *mut bool,
    ) -> HRESULT,
    fn GetAlternateLocation(
        pbstrAlternateLocation: *mut BSTR,
    ) -> HRESULT,
    fn GetBackupTypeMask(
        pdwTypeMask: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)]
interface IVssWMDependency(IVssWMDependencyVtbl): IUnknown(IUnknownVtbl) {
    fn GetWriterId(
        pWriterId: *mut VSS_ID,
    ) -> HRESULT,
    fn GetLogicalPath(
        pbstrLogicalPath: *mut BSTR,
    ) -> HRESULT,
    fn GetComponentName(
        pbstrComponentName: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd2c72c96, 0xc121, 0x4518, 0xb6, 0x27, 0xe5, 0xa9, 0x3d, 0x01, 0x0e, 0xad)]
interface IVssComponent(IVssComponentVtbl): IUnknown(IUnknownVtbl) {
    fn GetLogicalPath(
        pbstrPath: *mut BSTR,
    ) -> HRESULT,
    fn GetComponentType(
        pct: *mut VSS_COMPONENT_TYPE,
    ) -> HRESULT,
    fn GetComponentName(
        pbstrName: *mut BSTR,
    ) -> HRESULT,
    fn GetBackupSucceeded(
        pbSucceeded: *mut bool,
    ) -> HRESULT,
    fn GetAlternateLocationMappingCount(
        pcMappings: *mut UINT,
    ) -> HRESULT,
    fn GetAlternateLocationMapping(
        iMapping: UINT,
        ppFiledesc: *mut *mut IVssWMFiledesc,
    ) -> HRESULT,
    fn SetBackupMetadata(
        wszData: LPCWSTR,
    ) -> HRESULT,
    fn GetBackupMetadata(
        pbstrData: *mut BSTR,
    ) -> HRESULT,
    fn AddPartialFile(
        wszPath: LPCWSTR,
        wszFilename: LPCWSTR,
        wszRanges: LPCWSTR,
        wszMetadata: LPCWSTR,
    ) -> HRESULT,
    fn GetPartialFileCount(
        pcPartialFiles: *mut UINT,
    ) -> HRESULT,
    fn GetPartialFile(
        iPartialFile: UINT,
        pbstrPath: *mut BSTR,
        pbstrFilename: *mut BSTR,
        pbstrRange: *mut BSTR,
        pbstrMetadata: *mut BSTR,
    ) -> HRESULT,
    fn IsSelectedForRestore(
        pbSelectedForRestore: *mut bool,
    ) -> HRESULT,
    fn GetAdditionalRestores(
        pbAdditionalRestores: *mut bool,
    ) -> HRESULT,
    fn GetNewTargetCount(
        pcNewTarget: *mut UINT,
    ) -> HRESULT,
    fn GetNewTarget(
        iNewTarget: UINT,
        ppFiledesc: *mut *mut IVssWMFiledesc,
    ) -> HRESULT,
    fn AddDirectedTarget(
        wszSourcePath: LPCWSTR,
        wszSourceFilename: LPCWSTR,
        wszSourceRangeList: LPCWSTR,
        wszDestinationPath: LPCWSTR,
        wszDestinationFilename: LPCWSTR,
        wszDestinationRangeList: LPCWSTR,
    ) -> HRESULT,
    fn GetDirectedTargetCount(
        pcDirectedTarget: *mut UINT,
    ) -> HRESULT,
    fn GetDirectedTarget(
        iDirectedTarget: UINT,
        pbstrSourcePath: *mut BSTR,
        pbstrSourceFileName: *mut BSTR,
        pbstrSourceRangeList: *mut BSTR,
        pbstrDestinationPath: *mut BSTR,
        pbstrDestinationFilename: *mut BSTR,
        pbstrDestinationRangeList: *mut BSTR,
    ) -> HRESULT,
    fn SetRestoreMetadata(
        wszRestoreMetadata: LPCWSTR,
    ) -> HRESULT,
    fn GetRestoreMetadata(
        pbstrRestoreMetadata: *mut BSTR,
    ) -> HRESULT,
    fn SetRestoreTarget(
        target: VSS_RESTORE_TARGET,
    ) -> HRESULT,
    fn GetRestoreTarget(
        pTarget: *mut VSS_RESTORE_TARGET,
    ) -> HRESULT,
    fn SetPreRestoreFailureMsg(
        wszPreRestoreFailureMsg: LPCWSTR,
    ) -> HRESULT,
    fn GetPreRestoreFailureMsg(
        pbstrPreRestoreFailureMsg: *mut BSTR,
    ) -> HRESULT,
    fn SetPostRestoreFailureMsg(
        wszPostRestoreFailureMsg: LPCWSTR,
    ) -> HRESULT,
    fn GetPostRestoreFailureMsg(
        pbstrPostRestoreFailureMsg: *mut BSTR,
    ) -> HRESULT,
    fn SetBackupStamp(
        wszBackupStamp: LPCWSTR,
    ) -> HRESULT,
    fn GetBackupStamp(
        pbstrBackupStamp: *mut BSTR,
    ) -> HRESULT,
    fn GetPreviousBackupStamp(
        pbstrBackupStamp: *mut BSTR,
    ) -> HRESULT,
    fn GetBackupOptions(
        pbstrBackupOptions: *mut BSTR,
    ) -> HRESULT,
    fn GetRestoreOptions(
        pbstrRestoreOptions: *mut BSTR,
    ) -> HRESULT,
    fn GetRestoreSubcomponentCount(
        pcRestoreSubcomponent: *mut UINT,
    ) -> HRESULT,
    fn GetRestoreSubcomponent(
        iComponent: UINT,
        pbstrLogicalPath: *mut BSTR,
        pbstrComponentName: *mut BSTR,
        pbRepair: *mut bool,
    ) -> HRESULT,
    fn GetFileRestoreStatus(
        pStatus: *mut VSS_FILE_RESTORE_STATUS,
    ) -> HRESULT,
    fn AddDifferencedFilesByLastModifyTime(
        wszPath: LPCWSTR,
        wszFilespec: LPCWSTR,
        bRecursive: BOOL,
        ftLastModifyTime: FILETIME,
    ) -> HRESULT,
    fn AddDifferencedFilesByLastModifyLSN(
        wszPath: LPCWSTR,
        wszFilespec: LPCWSTR,
        bRecursive: BOOL,
        bstrLsnString: BSTR,
    ) -> HRESULT,
    fn GetDifferencedFilesCount(
        pcDifferencedFiles: *mut UINT,
    ) -> HRESULT,
    fn GetDifferencedFile(
        iDifferencedFile: UINT,
        pbstrPath: *mut BSTR,
        pbstrFilespec: *mut BSTR,
        pbRecursive: *mut BOOL,
        pbstrLsnString: *mut BSTR,
        pftLastModifyTime: *mut FILETIME,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)]
interface IVssWriterComponents(IVssWriterComponentsVtbl) {
    fn GetComponentCount(
        pcComponents: *mut UINT,
    ) -> HRESULT,
    fn GetWriterInfo(
        pidInstance: *mut VSS_ID,
        pidWriter: *mut VSS_ID,
    ) -> HRESULT,
    fn GetComponent(
        iComponent: UINT,
        ppComponent: *mut *mut IVssComponent,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x156c8b5e, 0xf131, 0x4bd7, 0x9c, 0x97, 0xd1, 0x92, 0x3b, 0xe7, 0xe1, 0xfa)]
interface IVssComponentEx(IVssComponentExVtbl): IVssComponent(IVssComponentVtbl) {
    fn SetPrepareForBackupFailureMsg(
        wszFailureMsg: LPCWSTR,
    ) -> HRESULT,
    fn SetPostSnapshotFailureMsg(
        wszFailureMsg: LPCWSTR,
    ) -> HRESULT,
    fn GetPrepareForBackupFailureMsg(
        pbstrFailureMsg: *mut BSTR,
    ) -> HRESULT,
    fn GetPostSnapshotFailureMsg(
        pbstrFailureMsg: *mut BSTR,
    ) -> HRESULT,
    fn GetAuthoritativeRestore(
        pbAuth: *mut bool,
    ) -> HRESULT,
    fn GetRollForward(
        pRollType: *mut VSS_ROLLFORWARD_TYPE,
        pbstrPoint: *mut BSTR,
    ) -> HRESULT,
    fn GetRestoreName(
        pbstrName: *mut BSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3b5be0f2, 0x07a9, 0x4e4b, 0xbd, 0xd3, 0xcf, 0xdc, 0x8e, 0x2c, 0x0d, 0x2d)]
interface IVssComponentEx2(IVssComponentEx2Vtbl): IVssComponentEx(IVssComponentExVtbl) {
    fn SetFailure(
        hr: HRESULT,
        hrApplication: HRESULT,
        wszApplicationMessage: LPCWSTR,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn GetFailure(
        phr: *mut HRESULT,
        phrApplication: *mut HRESULT,
        pbstrApplicationMessage: *mut BSTR,
        pdwReserved: *mut DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)]
interface IVssCreateWriterMetadata(IVssCreateWriterMetadataVtbl) {
    fn AddIncludeFiles(
        wszPath: LPCWSTR,
        wszFilespec: LPCWSTR,
        bRecursive: bool,
        wszAlternateLocation: LPCWSTR,
    ) -> HRESULT,
    fn AddExcludeFiles(
        wszPath: LPCWSTR,
        wszFilespec: LPCWSTR,
        bRecursive: bool,
    ) -> HRESULT,
    fn AddComponent(
        ct: VSS_COMPONENT_TYPE,
        wszLogicalPath: LPCWSTR,
        wszComponentName: LPCWSTR,
        wszCaption: LPCWSTR,
        pbIcon: *const BYTE,
        cbIcon: UINT,
        bRestoreMetadata: bool,
        bNotifyOnBackupComplete: bool,
        bSelectableForRestore: bool,
        dwComponentFlags: DWORD,
    ) -> HRESULT,
    fn AddDatabaseFiles(
        wszLogicalPath: LPCWSTR,
        wszDatabaseName: LPCWSTR,
        wszPath: LPCWSTR,
        wszFilespec: LPCWSTR,
        dwBackupTypeMask: DWORD,
    ) -> HRESULT,
    fn AddDatabaseLogFiles(
        wszLogicalPath: LPCWSTR,
        wszDatabaseName: LPCWSTR,
        wszPath: LPCWSTR,
        wszFilespec: LPCWSTR,
        dwBackupTypeMask: DWORD,
    ) -> HRESULT,
    fn AddFilesToFileGroup(
        wszLogicalPath: LPCWSTR,
        wszGroupName: LPCWSTR,
        wszPath: LPCWSTR,
        wszFilespec: LPCWSTR,
        bRecursive: bool,
        wszAlternateLocation: LPCWSTR,
        dwBackupTypeMask: DWORD,
    ) -> HRESULT,
    fn SetRestoreMethod(
        method: VSS_RESTOREMETHOD_ENUM,
        wszService: LPCWSTR,
        wszUserProcedure: LPCWSTR,
        writerRestore: VSS_WRITERRESTORE_ENUM,
        bRebootRequired: bool,
    ) -> HRESULT,
    fn AddAlternateLocationMapping(
        wszSourcePath: LPCWSTR,
        wszSourceFilespec: LPCWSTR,
        bRecursive: bool,
        wszDestination: LPCWSTR,
    ) -> HRESULT,
    fn AddComponentDependency(
        wszForLogicalPath: LPCWSTR,
        wszForComponentName: LPCWSTR,
        onWriterId: VSS_ID,
        wszOnLogicalPath: LPCWSTR,
        wszOnComponentName: LPCWSTR,
    ) -> HRESULT,
    fn SetBackupSchema(
        dwSchemaMask: DWORD,
    ) -> HRESULT,
    fn GetDocument(
        pDoc: *mut *mut VOID,
    ) -> HRESULT, //TODO IXMLDOMDocument,
    fn SaveAsXML(
        pbstrXML: *mut BSTR,
    ) -> HRESULT,
}}
//IVssCreateWriterMetadataEx
//IVssWriterImpl
//IVssCreateExpressWriterMetadata
//IVssExpressWriter
//CVssWriter
//CVssWriterEx
//CVssWriterEx2

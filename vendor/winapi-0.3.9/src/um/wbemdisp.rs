// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_long;
use shared::winerror::HRESULT;
use shared::wtypes::{BSTR, DATE, VARIANT_BOOL};
use um::oaidl::{DISPID, IDispatch, IDispatchVtbl, VARIANT};
use um::unknwnbase::IUnknown;
// extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0000_v0_0_s_ifspec;
ENUM!{enum WbemChangeFlagEnum {
    wbemChangeFlagCreateOrUpdate = 0,
    wbemChangeFlagUpdateOnly = 0x1,
    wbemChangeFlagCreateOnly = 0x2,
    wbemChangeFlagUpdateCompatible = 0,
    wbemChangeFlagUpdateSafeMode = 0x20,
    wbemChangeFlagUpdateForceMode = 0x40,
    wbemChangeFlagStrongValidation = 0x80,
    wbemChangeFlagAdvisory = 0x10000,
}}
ENUM!{enum WbemFlagEnum {
    wbemFlagReturnImmediately = 0x10,
    wbemFlagReturnWhenComplete = 0,
    wbemFlagBidirectional = 0,
    wbemFlagForwardOnly = 0x20,
    wbemFlagNoErrorObject = 0x40,
    wbemFlagReturnErrorObject = 0,
    wbemFlagSendStatus = 0x80,
    wbemFlagDontSendStatus = 0,
    wbemFlagEnsureLocatable = 0x100,
    wbemFlagDirectRead = 0x200,
    wbemFlagSendOnlySelected = 0,
    wbemFlagUseAmendedQualifiers = 0x20000,
    wbemFlagGetDefault = 0,
    wbemFlagSpawnInstance = 0x1,
    wbemFlagUseCurrentTime = 0x1,
}}
ENUM!{enum WbemQueryFlagEnum {
    wbemQueryFlagDeep = 0,
    wbemQueryFlagShallow = 1,
    wbemQueryFlagPrototype = 2,
}}
ENUM!{enum WbemTextFlagEnum {
    wbemTextFlagNoFlavors = 0x1,
}}
ENUM!{enum WbemTimeout {
    wbemTimeoutInfinite = 0xffffffff,
}}
ENUM!{enum WbemComparisonFlagEnum {
    wbemComparisonFlagIncludeAll = 0,
    wbemComparisonFlagIgnoreQualifiers = 0x1,
    wbemComparisonFlagIgnoreObjectSource = 0x2,
    wbemComparisonFlagIgnoreDefaultValues = 0x4,
    wbemComparisonFlagIgnoreClass = 0x8,
    wbemComparisonFlagIgnoreCase = 0x10,
    wbemComparisonFlagIgnoreFlavor = 0x20,
}}
ENUM!{enum WbemCimtypeEnum {
    wbemCimtypeSint8 = 16,
    wbemCimtypeUint8 = 17,
    wbemCimtypeSint16 = 2,
    wbemCimtypeUint16 = 18,
    wbemCimtypeSint32 = 3,
    wbemCimtypeUint32 = 19,
    wbemCimtypeSint64 = 20,
    wbemCimtypeUint64 = 21,
    wbemCimtypeReal32 = 4,
    wbemCimtypeReal64 = 5,
    wbemCimtypeBoolean = 11,
    wbemCimtypeString = 8,
    wbemCimtypeDatetime = 101,
    wbemCimtypeReference = 102,
    wbemCimtypeChar16 = 103,
    wbemCimtypeObject = 13,
}}
ENUM!{enum WbemErrorEnum {
    wbemNoErr = 0,
    wbemErrFailed = 0x80041001,
    wbemErrNotFound = 0x80041002,
    wbemErrAccessDenied = 0x80041003,
    wbemErrProviderFailure = 0x80041004,
    wbemErrTypeMismatch = 0x80041005,
    wbemErrOutOfMemory = 0x80041006,
    wbemErrInvalidContext = 0x80041007,
    wbemErrInvalidParameter = 0x80041008,
    wbemErrNotAvailable = 0x80041009,
    wbemErrCriticalError = 0x8004100a,
    wbemErrInvalidStream = 0x8004100b,
    wbemErrNotSupported = 0x8004100c,
    wbemErrInvalidSuperclass = 0x8004100d,
    wbemErrInvalidNamespace = 0x8004100e,
    wbemErrInvalidObject = 0x8004100f,
    wbemErrInvalidClass = 0x80041010,
    wbemErrProviderNotFound = 0x80041011,
    wbemErrInvalidProviderRegistration = 0x80041012,
    wbemErrProviderLoadFailure = 0x80041013,
    wbemErrInitializationFailure = 0x80041014,
    wbemErrTransportFailure = 0x80041015,
    wbemErrInvalidOperation = 0x80041016,
    wbemErrInvalidQuery = 0x80041017,
    wbemErrInvalidQueryType = 0x80041018,
    wbemErrAlreadyExists = 0x80041019,
    wbemErrOverrideNotAllowed = 0x8004101a,
    wbemErrPropagatedQualifier = 0x8004101b,
    wbemErrPropagatedProperty = 0x8004101c,
    wbemErrUnexpected = 0x8004101d,
    wbemErrIllegalOperation = 0x8004101e,
    wbemErrCannotBeKey = 0x8004101f,
    wbemErrIncompleteClass = 0x80041020,
    wbemErrInvalidSyntax = 0x80041021,
    wbemErrNondecoratedObject = 0x80041022,
    wbemErrReadOnly = 0x80041023,
    wbemErrProviderNotCapable = 0x80041024,
    wbemErrClassHasChildren = 0x80041025,
    wbemErrClassHasInstances = 0x80041026,
    wbemErrQueryNotImplemented = 0x80041027,
    wbemErrIllegalNull = 0x80041028,
    wbemErrInvalidQualifierType = 0x80041029,
    wbemErrInvalidPropertyType = 0x8004102a,
    wbemErrValueOutOfRange = 0x8004102b,
    wbemErrCannotBeSingleton = 0x8004102c,
    wbemErrInvalidCimType = 0x8004102d,
    wbemErrInvalidMethod = 0x8004102e,
    wbemErrInvalidMethodParameters = 0x8004102f,
    wbemErrSystemProperty = 0x80041030,
    wbemErrInvalidProperty = 0x80041031,
    wbemErrCallCancelled = 0x80041032,
    wbemErrShuttingDown = 0x80041033,
    wbemErrPropagatedMethod = 0x80041034,
    wbemErrUnsupportedParameter = 0x80041035,
    wbemErrMissingParameter = 0x80041036,
    wbemErrInvalidParameterId = 0x80041037,
    wbemErrNonConsecutiveParameterIds = 0x80041038,
    wbemErrParameterIdOnRetval = 0x80041039,
    wbemErrInvalidObjectPath = 0x8004103a,
    wbemErrOutOfDiskSpace = 0x8004103b,
    wbemErrBufferTooSmall = 0x8004103c,
    wbemErrUnsupportedPutExtension = 0x8004103d,
    wbemErrUnknownObjectType = 0x8004103e,
    wbemErrUnknownPacketType = 0x8004103f,
    wbemErrMarshalVersionMismatch = 0x80041040,
    wbemErrMarshalInvalidSignature = 0x80041041,
    wbemErrInvalidQualifier = 0x80041042,
    wbemErrInvalidDuplicateParameter = 0x80041043,
    wbemErrTooMuchData = 0x80041044,
    wbemErrServerTooBusy = 0x80041045,
    wbemErrInvalidFlavor = 0x80041046,
    wbemErrCircularReference = 0x80041047,
    wbemErrUnsupportedClassUpdate = 0x80041048,
    wbemErrCannotChangeKeyInheritance = 0x80041049,
    wbemErrCannotChangeIndexInheritance = 0x80041050,
    wbemErrTooManyProperties = 0x80041051,
    wbemErrUpdateTypeMismatch = 0x80041052,
    wbemErrUpdateOverrideNotAllowed = 0x80041053,
    wbemErrUpdatePropagatedMethod = 0x80041054,
    wbemErrMethodNotImplemented = 0x80041055,
    wbemErrMethodDisabled = 0x80041056,
    wbemErrRefresherBusy = 0x80041057,
    wbemErrUnparsableQuery = 0x80041058,
    wbemErrNotEventClass = 0x80041059,
    wbemErrMissingGroupWithin = 0x8004105a,
    wbemErrMissingAggregationList = 0x8004105b,
    wbemErrPropertyNotAnObject = 0x8004105c,
    wbemErrAggregatingByObject = 0x8004105d,
    wbemErrUninterpretableProviderQuery = 0x8004105f,
    wbemErrBackupRestoreWinmgmtRunning = 0x80041060,
    wbemErrQueueOverflow = 0x80041061,
    wbemErrPrivilegeNotHeld = 0x80041062,
    wbemErrInvalidOperator = 0x80041063,
    wbemErrLocalCredentials = 0x80041064,
    wbemErrCannotBeAbstract = 0x80041065,
    wbemErrAmendedObject = 0x80041066,
    wbemErrClientTooSlow = 0x80041067,
    wbemErrNullSecurityDescriptor = 0x80041068,
    wbemErrTimeout = 0x80041069,
    wbemErrInvalidAssociation = 0x8004106a,
    wbemErrAmbiguousOperation = 0x8004106b,
    wbemErrQuotaViolation = 0x8004106c,
    wbemErrTransactionConflict = 0x8004106d,
    wbemErrForcedRollback = 0x8004106e,
    wbemErrUnsupportedLocale = 0x8004106f,
    wbemErrHandleOutOfDate = 0x80041070,
    wbemErrConnectionFailed = 0x80041071,
    wbemErrInvalidHandleRequest = 0x80041072,
    wbemErrPropertyNameTooWide = 0x80041073,
    wbemErrClassNameTooWide = 0x80041074,
    wbemErrMethodNameTooWide = 0x80041075,
    wbemErrQualifierNameTooWide = 0x80041076,
    wbemErrRerunCommand = 0x80041077,
    wbemErrDatabaseVerMismatch = 0x80041078,
    wbemErrVetoPut = 0x80041079,
    wbemErrVetoDelete = 0x8004107a,
    wbemErrInvalidLocale = 0x80041080,
    wbemErrProviderSuspended = 0x80041081,
    wbemErrSynchronizationRequired = 0x80041082,
    wbemErrNoSchema = 0x80041083,
    wbemErrProviderAlreadyRegistered = 0x80041084,
    wbemErrProviderNotRegistered = 0x80041085,
    wbemErrFatalTransportError = 0x80041086,
    wbemErrEncryptedConnectionRequired = 0x80041087,
    wbemErrRegistrationTooBroad = 0x80042001,
    wbemErrRegistrationTooPrecise = 0x80042002,
    wbemErrTimedout = 0x80043001,
    wbemErrResetToDefault = 0x80043002,
}}
ENUM!{enum WbemAuthenticationLevelEnum {
    wbemAuthenticationLevelDefault = 0,
    wbemAuthenticationLevelNone = 1,
    wbemAuthenticationLevelConnect = 2,
    wbemAuthenticationLevelCall = 3,
    wbemAuthenticationLevelPkt = 4,
    wbemAuthenticationLevelPktIntegrity = 5,
    wbemAuthenticationLevelPktPrivacy = 6,
}}
ENUM!{enum WbemImpersonationLevelEnum {
    wbemImpersonationLevelAnonymous = 1,
    wbemImpersonationLevelIdentify = 2,
    wbemImpersonationLevelImpersonate = 3,
    wbemImpersonationLevelDelegate = 4,
}}
ENUM!{enum WbemPrivilegeEnum {
    wbemPrivilegeCreateToken = 1,
    wbemPrivilegePrimaryToken = 2,
    wbemPrivilegeLockMemory = 3,
    wbemPrivilegeIncreaseQuota = 4,
    wbemPrivilegeMachineAccount = 5,
    wbemPrivilegeTcb = 6,
    wbemPrivilegeSecurity = 7,
    wbemPrivilegeTakeOwnership = 8,
    wbemPrivilegeLoadDriver = 9,
    wbemPrivilegeSystemProfile = 10,
    wbemPrivilegeSystemtime = 11,
    wbemPrivilegeProfileSingleProcess = 12,
    wbemPrivilegeIncreaseBasePriority = 13,
    wbemPrivilegeCreatePagefile = 14,
    wbemPrivilegeCreatePermanent = 15,
    wbemPrivilegeBackup = 16,
    wbemPrivilegeRestore = 17,
    wbemPrivilegeShutdown = 18,
    wbemPrivilegeDebug = 19,
    wbemPrivilegeAudit = 20,
    wbemPrivilegeSystemEnvironment = 21,
    wbemPrivilegeChangeNotify = 22,
    wbemPrivilegeRemoteShutdown = 23,
    wbemPrivilegeUndock = 24,
    wbemPrivilegeSyncAgent = 25,
    wbemPrivilegeEnableDelegation = 26,
    wbemPrivilegeManageVolume = 27,
}}
ENUM!{enum WbemObjectTextFormatEnum {
    wbemObjectTextFormatCIMDTD20 = 1,
    wbemObjectTextFormatWMIDTD20 = 2,
}}
ENUM!{enum WbemConnectOptionsEnum {
    wbemConnectFlagUseMaxWait = 0x80,
}}
// EXTERN_C const IID LIBID_WbemScripting;
// EXTERN_C const IID IID_ISWbemServices;
DEFINE_GUID!{IID_ISWbemServices,
    0x76a6415c, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x76a6415c, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemServices(ISWbemServicesVtbl): IDispatch(IDispatchVtbl) {
    fn Get(
        strObjectPath: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObject: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn GetAsync(
        objWbemSink: *mut IDispatch,
        strObjectPath: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn Delete(
        strObjectPath: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
    ) -> HRESULT,
    fn DeleteAsync(
        objWbemSink: *mut IDispatch,
        strObjectPath: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn InstancesOf(
        strClass: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn InstancesOfAsync(
        objWbemSink: *mut IDispatch,
        strClass: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn SubclassesOf(
        strSuperclass: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn SubclassesOfAsync(
        objWbemSink: *mut IDispatch,
        strSuperclass: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn ExecQuery(
        strQuery: BSTR,
        strQueryLanguage: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn ExecQueryAsync(
        objWbemSink: *mut IDispatch,
        strQuery: BSTR,
        strQueryLanguage: BSTR,
        lFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn AssociatorsOf(
        strObjectPath: BSTR,
        strAssocClass: BSTR,
        strResultClass: BSTR,
        strResultRole: BSTR,
        strRole: BSTR,
        bClassesOnly: VARIANT_BOOL,
        bSchemaOnly: VARIANT_BOOL,
        strRequiredAssocQualifier: BSTR,
        strRequiredQualifier: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn AssociatorsOfAsync(
        objWbemSink: *mut IDispatch,
        strObjectPath: BSTR,
        strAssocClass: BSTR,
        strResultClass: BSTR,
        strResultRole: BSTR,
        strRole: BSTR,
        bClassesOnly: VARIANT_BOOL,
        bSchemaOnly: VARIANT_BOOL,
        strRequiredAssocQualifier: BSTR,
        strRequiredQualifier: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn ReferencesTo(
        strObjectPath: BSTR,
        strResultClass: BSTR,
        strRole: BSTR,
        bClassesOnly: VARIANT_BOOL,
        bSchemaOnly: VARIANT_BOOL,
        strRequiredQualifier: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn ReferencesToAsync(
        objWbemSink: *mut IDispatch,
        strObjectPath: BSTR,
        strResultClass: BSTR,
        strRole: BSTR,
        bClassesOnly: VARIANT_BOOL,
        bSchemaOnly: VARIANT_BOOL,
        strRequiredQualifier: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn ExecNotificationQuery(
        strQuery: BSTR,
        strQueryLanguage: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemEventSource: *mut *mut ISWbemEventSource,
    ) -> HRESULT,
    fn ExecNotificationQueryAsync(
        objWbemSink: *mut IDispatch,
        strQuery: BSTR,
        strQueryLanguage: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn ExecMethod(
        strObjectPath: BSTR,
        strMethodName: BSTR,
        objWbemInParameters: *mut IDispatch,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemOutParameters: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn ExecMethodAsync(
        objWbemSink: *mut IDispatch,
        strObjectPath: BSTR,
        strMethodName: BSTR,
        objWbemInParameters: *mut IDispatch,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn get_Security_(
        objWbemSecurity: *mut *mut ISWbemSecurity,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemLocator;
DEFINE_GUID!{IID_ISWbemLocator,
    0x76a6415b, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x76a6415b, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemLocator(ISWbemLocatorVtbl): IDispatch(IDispatchVtbl) {
    fn ConnectServer(
        strServer: BSTR,
        strNamespace: BSTR,
        strUser: BSTR,
        strPassword: BSTR,
        strLocale: BSTR,
        strAuthority: BSTR,
        iSecurityFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemServices: *mut *mut ISWbemServices,
    ) -> HRESULT,
    fn get_Security_(
        objWbemSecurity: *mut *mut ISWbemSecurity,
    ) -> HRESULT,
}}
pub const WBEMS_DISPID_DERIVATION: DISPID = 23;
// EXTERN_C const IID IID_ISWbemObject;
DEFINE_GUID!{IID_ISWbemObject,
    0x76a6415a, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x76a6415a, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemObject(ISWbemObjectVtbl): IDispatch(IDispatchVtbl) {
    fn Put_(
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectPath: *mut *mut ISWbemObjectPath,
    ) -> HRESULT,
    fn PutAsync_(
        objWbemSink: *mut IDispatch,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn Delete_(
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
    ) -> HRESULT,
    fn DeleteAsync_(
        objWbemSink: *mut IDispatch,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn Instances_(
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn InstancesAsync_(
        objWbemSink: *mut IDispatch,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn Subclasses_(
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn SubclassesAsync_(
        objWbemSink: *mut IDispatch,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn Associators_(
        strAssocClass: BSTR,
        strResultClass: BSTR,
        strResultRole: BSTR,
        strRole: BSTR,
        bClassesOnly: VARIANT_BOOL,
        bSchemaOnly: VARIANT_BOOL,
        strRequiredAssocQualifier: BSTR,
        strRequiredQualifier: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn AssociatorsAsync_(
        objWbemSink: *mut IDispatch,
        strAssocClass: BSTR,
        strResultClass: BSTR,
        strResultRole: BSTR,
        strRole: BSTR,
        bClassesOnly: VARIANT_BOOL,
        bSchemaOnly: VARIANT_BOOL,
        strRequiredAssocQualifier: BSTR,
        strRequiredQualifier: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn References_(
        strResultClass: BSTR,
        strRole: BSTR,
        bClassesOnly: VARIANT_BOOL,
        bSchemaOnly: VARIANT_BOOL,
        strRequiredQualifier: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn ReferencesAsync_(
        objWbemSink: *mut IDispatch,
        strResultClass: BSTR,
        strRole: BSTR,
        bClassesOnly: VARIANT_BOOL,
        bSchemaOnly: VARIANT_BOOL,
        strRequiredQualifier: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn ExecMethod_(
        strMethodName: BSTR,
        objWbemInParameters: *mut IDispatch,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemOutParameters: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn ExecMethodAsync_(
        objWbemSink: *mut IDispatch,
        strMethodName: BSTR,
        objWbemInParameters: *mut IDispatch,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
    fn Clone_(
        objWbemObject: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn GetObjectText_(
        iFlags: c_long,
        strObjectText: *mut BSTR,
    ) -> HRESULT,
    fn SpawnDerivedClass_(
        iFlags: c_long,
        objWbemObject: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn SpawnInstance_(
        iFlags: c_long,
        objWbemObject: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn CompareTo_(
        objWbemObject: *mut IDispatch,
        iFlags: c_long,
        bResult: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Qualifiers_(
        objWbemQualifierSet: *mut *mut ISWbemQualifierSet,
    ) -> HRESULT,
    fn get_Properties_(
        objWbemPropertySet: *mut *mut ISWbemPropertySet,
    ) -> HRESULT,
    fn get_Methods_(
        objWbemMethodSet: *mut *mut ISWbemMethodSet,
    ) -> HRESULT,
    fn get_Derivation_(
        strClassNameArray: *mut VARIANT,
    ) -> HRESULT,
    fn get_Path_(
        objWbemObjectPath: *mut *mut ISWbemObjectPath,
    ) -> HRESULT,
    fn get_Security_(
        objWbemSecurity: *mut *mut ISWbemSecurity,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemObjectSet;
DEFINE_GUID!{IID_ISWbemObjectSet,
    0x76a6415f, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x76a6415f, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemObjectSet(ISWbemObjectSetVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Item(
        strObjectPath: BSTR,
        iFlags: c_long,
        objWbemObject: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn get_Count(
        iCount: *mut c_long,
    ) -> HRESULT,
    fn get_Security_(
        objWbemSecurity: *mut *mut ISWbemSecurity,
    ) -> HRESULT,
    fn ItemIndex(
        lIndex: c_long,
        objWbemObject: *mut *mut ISWbemObject,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemNamedValue;
DEFINE_GUID!{IID_ISWbemNamedValue,
    0x76a64164, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x76a64164, 0xcb41, 0x11d1, 0x8b, 0x02, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemNamedValue(ISWbemNamedValueVtbl): IDispatch(IDispatchVtbl) {
    fn get_Value(
        varValue: *mut VARIANT,
    ) -> HRESULT,
    fn put_Value(
        varValue: *mut VARIANT,
    ) -> HRESULT,
    fn get_Name(
        strName: *mut BSTR,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemNamedValueSet;
DEFINE_GUID!{IID_ISWbemNamedValueSet,
    0xcf2376ea, 0xce8c, 0x11d1, 0x8b, 0x05, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0xcf2376ea, 0xce8c, 0x11d1, 0x8b, 0x05, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemNamedValueSet(ISWbemNamedValueSetVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Item(
        strName: BSTR,
        iFlags: c_long,
        objWbemNamedValue: *mut *mut ISWbemNamedValue,
    ) -> HRESULT,
    fn get_Count(
        iCount: *mut c_long,
    ) -> HRESULT,
    fn Add(
        strName: BSTR,
        varValue: *mut VARIANT,
        iFlags: c_long,
        objWbemNamedValue: *mut *mut ISWbemNamedValue,
    ) -> HRESULT,
    fn Remove(
        strName: BSTR,
        iFlags: c_long,
    ) -> HRESULT,
    fn Clone(
        objWbemNamedValueSet: *mut *mut ISWbemNamedValueSet,
    ) -> HRESULT,
    fn DeleteAll() -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemQualifier;
DEFINE_GUID!{IID_ISWbemQualifier,
    0x79b05932, 0xd3b7, 0x11d1, 0x8b, 0x06, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x79b05932, 0xd3b7, 0x11d1, 0x8b, 0x06, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemQualifier(ISWbemQualifierVtbl): IDispatch(IDispatchVtbl) {
    fn get_Value(
        varValue: *mut VARIANT,
    ) -> HRESULT,
    fn put_Value(
        varValue: *mut VARIANT,
    ) -> HRESULT,
    fn get_Name(
        strName: *mut BSTR,
    ) -> HRESULT,
    fn get_IsLocal(
        bIsLocal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PropagatesToSubclass(
        bPropagatesToSubclass: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PropagatesToSubclass(
        bPropagatesToSubclass: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_PropagatesToInstance(
        bPropagatesToInstance: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_PropagatesToInstance(
        bPropagatesToInstance: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsOverridable(
        bIsOverridable: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_IsOverridable(
        bIsOverridable: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsAmended(
        bIsAmended: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemQualifierSet;
DEFINE_GUID!{IID_ISWbemQualifierSet,
    0x9b16ed16, 0xd3df, 0x11d1, 0x8b, 0x08, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x9b16ed16, 0xd3df, 0x11d1, 0x8b, 0x08, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemQualifierSet(ISWbemQualifierSetVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Item(
        name: BSTR,
        iFlags: c_long,
        objWbemQualifier: *mut *mut ISWbemQualifier,
    ) -> HRESULT,
    fn get_Count(
        iCount: *mut c_long,
    ) -> HRESULT,
    fn Add(
        strName: BSTR,
        varVal: *mut VARIANT,
        bPropagatesToSubclass: VARIANT_BOOL,
        bPropagatesToInstance: VARIANT_BOOL,
        bIsOverridable: VARIANT_BOOL,
        iFlags: c_long,
        objWbemQualifier: *mut *mut ISWbemQualifier,
    ) -> HRESULT,
    fn Remove(
        strName: BSTR,
        iFlags: c_long,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemProperty;
DEFINE_GUID!{IID_ISWbemProperty,
    0x1a388f98, 0xd4ba, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x1a388f98, 0xd4ba, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemProperty(ISWbemPropertyVtbl): IDispatch(IDispatchVtbl) {
    fn get_Value(
        varValue: *mut VARIANT,
    ) -> HRESULT,
    fn put_Value(
        varValue: *mut VARIANT,
    ) -> HRESULT,
    fn get_Name(
        strName: *mut BSTR,
    ) -> HRESULT,
    fn get_IsLocal(
        bIsLocal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Origin(
        strOrigin: *mut BSTR,
    ) -> HRESULT,
    fn get_CIMType(
        iCimType: *mut WbemCimtypeEnum,
    ) -> HRESULT,
    fn get_Qualifiers_(
        objWbemQualifierSet: *mut *mut ISWbemQualifierSet,
    ) -> HRESULT,
    fn get_IsArray(
        bIsArray: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemPropertySet;
DEFINE_GUID!{IID_ISWbemPropertySet,
    0xdea0a7b2, 0xd4ba, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0xdea0a7b2, 0xd4ba, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemPropertySet(ISWbemPropertySetVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Item(
        strName: BSTR,
        iFlags: c_long,
        objWbemProperty: *mut *mut ISWbemProperty,
    ) -> HRESULT,
    fn get_Count(
        iCount: *mut c_long,
    ) -> HRESULT,
    fn Add(
        strName: BSTR,
        iCIMType: WbemCimtypeEnum,
        bIsArray: VARIANT_BOOL,
        iFlags: c_long,
        objWbemProperty: *mut *mut ISWbemProperty,
    ) -> HRESULT,
    fn Remove(
        strName: BSTR,
        iFlags: c_long,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemMethod;
DEFINE_GUID!{IID_ISWbemMethod,
    0x422e8e90, 0xd955, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x422e8e90, 0xd955, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemMethod(ISWbemMethodVtbl): IDispatch(IDispatchVtbl) {
    fn get_Name(
        strName: *mut BSTR,
    ) -> HRESULT,
    fn get_Origin(
        strOrigin: *mut BSTR,
    ) -> HRESULT,
    fn get_InParameters(
        objWbemInParameters: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn get_OutParameters(
        objWbemOutParameters: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn get_Qualifiers_(
        objWbemQualifierSet: *mut *mut ISWbemQualifierSet,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemMethodSet;
DEFINE_GUID!{IID_ISWbemMethodSet,
    0xc93ba292, 0xd955, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0xc93ba292, 0xd955, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemMethodSet(ISWbemMethodSetVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Item(
        strName: BSTR,
        iFlags: c_long,
        objWbemMethod: *mut *mut ISWbemMethod,
    ) -> HRESULT,
    fn get_Count(
        iCount: *mut c_long,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemEventSource;
DEFINE_GUID!{IID_ISWbemEventSource,
    0x27d54d92, 0x0ebe, 0x11d2, 0x8b, 0x22, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x27d54d92, 0x0ebe, 0x11d2, 0x8b, 0x22, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemEventSource(ISWbemEventSourceVtbl): IDispatch(IDispatchVtbl) {
    fn NextEvent(
        iTimeoutMs: c_long,
        objWbemObject: *mut *mut ISWbemObject,
    ) -> HRESULT,
    fn get_Security_(
        objWbemSecurity: *mut *mut ISWbemSecurity,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemObjectPath;
DEFINE_GUID!{IID_ISWbemObjectPath,
    0x5791bc27, 0xce9c, 0x11d1, 0x97, 0xbf, 0x00, 0x00, 0xf8, 0x1e, 0x84, 0x9c}
RIDL!{#[uuid(0x5791bc27, 0xce9c, 0x11d1, 0x97, 0xbf, 0x00, 0x00, 0xf8, 0x1e, 0x84, 0x9c)]
interface ISWbemObjectPath(ISWbemObjectPathVtbl): IDispatch(IDispatchVtbl) {
    fn get_Path(
        strPath: *mut BSTR,
    ) -> HRESULT,
    fn put_Path(
        strPath: BSTR,
    ) -> HRESULT,
    fn get_RelPath(
        strRelPath: *mut BSTR,
    ) -> HRESULT,
    fn put_RelPath(
        strRelPath: BSTR,
    ) -> HRESULT,
    fn get_Server(
        strServer: *mut BSTR,
    ) -> HRESULT,
    fn put_Server(
        strServer: BSTR,
    ) -> HRESULT,
    fn get_Namespace(
        strNamespace: *mut BSTR,
    ) -> HRESULT,
    fn put_Namespace(
        strNamespace: BSTR,
    ) -> HRESULT,
    fn get_ParentNamespace(
        strParentNamespace: *mut BSTR,
    ) -> HRESULT,
    fn get_DisplayName(
        strDisplayName: *mut BSTR,
    ) -> HRESULT,
    fn put_DisplayName(
        strDisplayName: BSTR,
    ) -> HRESULT,
    fn get_Class(
        strClass: *mut BSTR,
    ) -> HRESULT,
    fn put_Class(
        strClass: BSTR,
    ) -> HRESULT,
    fn get_IsClass(
        bIsClass: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn SetAsClass() -> HRESULT,
    fn get_IsSingleton(
        bIsSingleton: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn SetAsSingleton() -> HRESULT,
    fn get_Keys(
        objWbemNamedValueSet: *mut *mut ISWbemNamedValueSet,
    ) -> HRESULT,
    fn get_Security_(
        objWbemSecurity: *mut *mut ISWbemSecurity,
    ) -> HRESULT,
    fn get_Locale(
        strLocale: *mut BSTR,
    ) -> HRESULT,
    fn put_Locale(
        strLocale: BSTR,
    ) -> HRESULT,
    fn get_Authority(
        strAuthority: *mut BSTR,
    ) -> HRESULT,
    fn put_Authority(
        strAuthority: BSTR,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemLastError;
DEFINE_GUID!{IID_ISWbemLastError,
    0xd962db84, 0xd4bb, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0xd962db84, 0xd4bb, 0x11d1, 0x8b, 0x09, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemLastError(ISWbemLastErrorVtbl): ISWbemObject(ISWbemObjectVtbl) {}}
// EXTERN_C const IID DIID_ISWbemSinkEvents;
DEFINE_GUID!{IID_ISWbemSinkEvents,
    0x75718ca0, 0xf029, 0x11d1, 0xa1, 0xac, 0x00, 0xc0, 0x4f, 0xb6, 0xc2, 0x23}
RIDL!{#[uuid(0x75718ca0, 0xf029, 0x11d1, 0xa1, 0xac, 0x00, 0xc0, 0x4f, 0xb6, 0xc2, 0x23)]
interface ISWbemSinkEvents(ISWbemSinkEventsVtbl): IDispatch(IDispatchVtbl) {}}
// EXTERN_C const IID IID_ISWbemSink;
DEFINE_GUID!{IID_ISWbemSink,
    0x75718c9f, 0xf029, 0x11d1, 0xa1, 0xac, 0x00, 0xc0, 0x4f, 0xb6, 0xc2, 0x23}
RIDL!{#[uuid(0x75718c9f, 0xf029, 0x11d1, 0xa1, 0xac, 0x00, 0xc0, 0x4f, 0xb6, 0xc2, 0x23)]
interface ISWbemSink(ISWbemSinkVtbl): IDispatch(IDispatchVtbl) {
    fn Cancel() -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemSecurity;
DEFINE_GUID!{IID_ISWbemSecurity,
    0xb54d66e6, 0x2287, 0x11d2, 0x8b, 0x33, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0xb54d66e6, 0x2287, 0x11d2, 0x8b, 0x33, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemSecurity(ISWbemSecurityVtbl): IDispatch(IDispatchVtbl) {
    fn get_ImpersonationLevel(
        iImpersonationLevel: *mut WbemImpersonationLevelEnum,
    ) -> HRESULT,
    fn put_ImpersonationLevel(
        iImpersonationLevel: WbemImpersonationLevelEnum,
    ) -> HRESULT,
    fn get_AuthenticationLevel(
        iAuthenticationLevel: *mut WbemAuthenticationLevelEnum,
    ) -> HRESULT,
    fn put_AuthenticationLevel(
        iAuthenticationLevel: WbemAuthenticationLevelEnum,
    ) -> HRESULT,
    fn get_Privileges(
        objWbemPrivilegeSet: *mut *mut ISWbemPrivilegeSet,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemPrivilege;
DEFINE_GUID!{IID_ISWbemPrivilege,
    0x26ee67bd, 0x5804, 0x11d2, 0x8b, 0x4a, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x26ee67bd, 0x5804, 0x11d2, 0x8b, 0x4a, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemPrivilege(ISWbemPrivilegeVtbl): IDispatch(IDispatchVtbl) {
    fn get_IsEnabled(
        bIsEnabled: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_IsEnabled(
        bIsEnabled: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Name(
        strDisplayName: *mut BSTR,
    ) -> HRESULT,
    fn get_DisplayName(
        strDisplayName: *mut BSTR,
    ) -> HRESULT,
    fn get_Identifier(
        iPrivilege: *mut WbemPrivilegeEnum,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemPrivilegeSet;
DEFINE_GUID!{IID_ISWbemPrivilegeSet,
    0x26ee67bf, 0x5804, 0x11d2, 0x8b, 0x4a, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6}
RIDL!{#[uuid(0x26ee67bf, 0x5804, 0x11d2, 0x8b, 0x4a, 0x00, 0x60, 0x08, 0x06, 0xd9, 0xb6)]
interface ISWbemPrivilegeSet(ISWbemPrivilegeSetVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Item(
        iPrivilege: WbemPrivilegeEnum,
        objWbemPrivilege: *mut *mut ISWbemPrivilege,
    ) -> HRESULT,
    fn get_Count(
        iCount: *mut c_long,
    ) -> HRESULT,
    fn Add(
        iPrivilege: WbemPrivilegeEnum,
        bIsEnabled: VARIANT_BOOL,
        objWbemPrivilege: *mut *mut ISWbemPrivilege,
    ) -> HRESULT,
    fn Remove(
        iPrivilege: WbemPrivilegeEnum,
    ) -> HRESULT,
    fn DeleteAll() -> HRESULT,
    fn AddAsString(
        strPrivilege: BSTR,
        bIsEnabled: VARIANT_BOOL,
        objWbemPrivilege: *mut *mut ISWbemPrivilege,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemServicesEx;
DEFINE_GUID!{IID_ISWbemServicesEx,
    0xd2f68443, 0x85dc, 0x427e, 0x91, 0xd8, 0x36, 0x65, 0x54, 0xcc, 0x75, 0x4c}
RIDL!{#[uuid(0xd2f68443, 0x85dc, 0x427e, 0x91, 0xd8, 0x36, 0x65, 0x54, 0xcc, 0x75, 0x4c)]
interface ISWbemServicesEx(ISWbemServicesExVtbl): ISWbemServices(ISWbemServicesVtbl) {
    fn Put(
        objWbemObject: *mut ISWbemObjectEx,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemObjectPath: *mut *mut ISWbemObjectPath,
    ) -> HRESULT,
    fn PutAsync(
        objWbemSink: *mut ISWbemSink,
        objWbemObject: *mut ISWbemObjectEx,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemAsyncContext: *mut IDispatch,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemObjectEx;
DEFINE_GUID!{IID_ISWbemObjectEx,
    0x269ad56a, 0x8a67, 0x4129, 0xbc, 0x8c, 0x05, 0x06, 0xdc, 0xfe, 0x98, 0x80}
RIDL!{#[uuid(0x269ad56a, 0x8a67, 0x4129, 0xbc, 0x8c, 0x05, 0x06, 0xdc, 0xfe, 0x98, 0x80)]
interface ISWbemObjectEx(ISWbemObjectExVtbl): ISWbemObject(ISWbemObjectVtbl) {
    fn Refresh_(
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
    ) -> HRESULT,
    fn get_SystemProperties_(
        objWbemPropertySet: *mut *mut ISWbemPropertySet,
    ) -> HRESULT,
    fn GetText_(
        iObjectTextFormat: WbemObjectTextFormatEnum,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        bsText: *mut BSTR,
    ) -> HRESULT,
    fn SetFromText_(
        bsText: BSTR,
        iObjectTextFormat: WbemObjectTextFormatEnum,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemDateTime;
DEFINE_GUID!{IID_ISWbemDateTime,
    0x5e97458a, 0xcf77, 0x11d3, 0xb3, 0x8f, 0x00, 0x10, 0x5a, 0x1f, 0x47, 0x3a}
RIDL!{#[uuid(0x5e97458a, 0xcf77, 0x11d3, 0xb3, 0x8f, 0x00, 0x10, 0x5a, 0x1f, 0x47, 0x3a)]
interface ISWbemDateTime(ISWbemDateTimeVtbl): IDispatch(IDispatchVtbl) {
    fn get_Value(
        strValue: *mut BSTR,
    ) -> HRESULT,
    fn put_Value(
        strValue: BSTR,
    ) -> HRESULT,
    fn get_Year(
        iYear: *mut c_long,
    ) -> HRESULT,
    fn put_Year(
        iYear: c_long,
    ) -> HRESULT,
    fn get_YearSpecified(
        bYearSpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_YearSpecified(
        bYearSpecified: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Month(
        iMonth: *mut c_long,
    ) -> HRESULT,
    fn put_Month(
        iMonth: c_long,
    ) -> HRESULT,
    fn get_MonthSpecified(
        bMonthSpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MonthSpecified(
        bMonthSpecified: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Day(
        iDay: *mut c_long,
    ) -> HRESULT,
    fn put_Day(
        iDay: c_long,
    ) -> HRESULT,
    fn get_DaySpecified(
        bDaySpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_DaySpecified(
        bDaySpecified: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Hours(
        iHours: *mut c_long,
    ) -> HRESULT,
    fn put_Hours(
        iHours: c_long,
    ) -> HRESULT,
    fn get_HoursSpecified(
        bHoursSpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_HoursSpecified(
        bHoursSpecified: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Minutes(
        iMinutes: *mut c_long,
    ) -> HRESULT,
    fn put_Minutes(
        iMinutes: c_long,
    ) -> HRESULT,
    fn get_MinutesSpecified(
        bMinutesSpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MinutesSpecified(
        bMinutesSpecified: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Seconds(
        iSeconds: *mut c_long,
    ) -> HRESULT,
    fn put_Seconds(
        iSeconds: c_long,
    ) -> HRESULT,
    fn get_SecondsSpecified(
        bSecondsSpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_SecondsSpecified(
        bSecondsSpecified: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Microseconds(
        iMicroseconds: *mut c_long,
    ) -> HRESULT,
    fn put_Microseconds(
        iMicroseconds: c_long,
    ) -> HRESULT,
    fn get_MicrosecondsSpecified(
        bMicrosecondsSpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MicrosecondsSpecified(
        bMicrosecondsSpecified: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_UTC(
        iUTC: *mut c_long,
    ) -> HRESULT,
    fn put_UTC(
        iUTC: c_long,
    ) -> HRESULT,
    fn get_UTCSpecified(
        bUTCSpecified: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_UTCSpecified(
        bUTCSpecified: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsInterval(
        bIsInterval: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_IsInterval(
        bIsInterval: VARIANT_BOOL,
    ) -> HRESULT,
    fn GetVarDate(
        bIsLocal: VARIANT_BOOL,
        dVarDate: *mut DATE,
    ) -> HRESULT,
    fn SetVarDate(
        dVarDate: DATE,
        bIsLocal: VARIANT_BOOL,
    ) -> HRESULT,
    fn GetFileTime(
        bIsLocal: VARIANT_BOOL,
        strFileTime: *mut BSTR,
    ) -> HRESULT,
    fn SetFileTime(
        strFileTime: BSTR,
        bIsLocal: VARIANT_BOOL,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemRefresher;
DEFINE_GUID!{IID_ISWbemRefresher,
    0x14d8250e, 0xd9c2, 0x11d3, 0xb3, 0x8f, 0x00, 0x10, 0x5a, 0x1f, 0x47, 0x3a}
RIDL!{#[uuid(0x14d8250e, 0xd9c2, 0x11d3, 0xb3, 0x8f, 0x00, 0x10, 0x5a, 0x1f, 0x47, 0x3a)]
interface ISWbemRefresher(ISWbemRefresherVtbl): IDispatch(IDispatchVtbl) {
    fn get__NewEnum(
        pUnk: *mut *mut IUnknown,
    ) -> HRESULT,
    fn Item(
        iIndex: c_long,
        objWbemRefreshableItem: *mut *mut ISWbemRefreshableItem,
    ) -> HRESULT,
    fn get_Count(
        iCount: *mut c_long,
    ) -> HRESULT,
    fn Add(
        objWbemServices: *mut ISWbemServicesEx,
        bsInstancePath: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemRefreshableItem: *mut *mut ISWbemRefreshableItem,
    ) -> HRESULT,
    fn AddEnum(
        objWbemServices: *mut ISWbemServicesEx,
        bsClassName: BSTR,
        iFlags: c_long,
        objWbemNamedValueSet: *mut IDispatch,
        objWbemRefreshableItem: *mut *mut ISWbemRefreshableItem,
    ) -> HRESULT,
    fn Remove(
        iIndex: c_long,
        iFlags: c_long,
    ) -> HRESULT,
    fn Refresh(
        iFlags: c_long,
    ) -> HRESULT,
    fn get_AutoReconnect(
        bCount: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AutoReconnect(
        bCount: VARIANT_BOOL,
    ) -> HRESULT,
    fn DeleteAll() -> HRESULT,
}}
// EXTERN_C const IID IID_ISWbemRefreshableItem;
DEFINE_GUID!{IID_ISWbemRefreshableItem,
    0x5ad4bf92, 0xdaab, 0x11d3, 0xb3, 0x8f, 0x00, 0x10, 0x5a, 0x1f, 0x47, 0x3a}
RIDL!{#[uuid(0x5ad4bf92, 0xdaab, 0x11d3, 0xb3, 0x8f, 0x00, 0x10, 0x5a, 0x1f, 0x47, 0x3a)]
interface ISWbemRefreshableItem(ISWbemRefreshableItemVtbl): IDispatch(IDispatchVtbl) {
    fn get_Index(
        iIndex: *mut c_long,
    ) -> HRESULT,
    fn get_Refresher(
        objWbemRefresher: *mut *mut ISWbemRefresher,
    ) -> HRESULT,
    fn get_IsSet(
        bIsSet: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Object(
        objWbemObject: *mut *mut ISWbemObjectEx,
    ) -> HRESULT,
    fn get_ObjectSet(
        objWbemObjectSet: *mut *mut ISWbemObjectSet,
    ) -> HRESULT,
    fn Remove(
        iFlags: c_long,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_SWbemLocator,
    0x76A64158, 0xCB41, 0x11d1, 0x8B, 0x02, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("76A64158-CB41-11d1-8B02-00600806D9B6")
// SWbemLocator;
DEFINE_GUID!{CLSID_SWbemNamedValueSet,
    0x9AED384E, 0xCE8B, 0x11d1, 0x8B, 0x05, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("9AED384E-CE8B-11d1-8B05-00600806D9B6")
// SWbemNamedValueSet;
DEFINE_GUID!{CLSID_SWbemObjectPath,
    0x5791BC26, 0xCE9C, 0x11d1, 0x97, 0xBF, 0x00, 0x00, 0xF8, 0x1E, 0x84, 0x9C}
// class DECLSPEC_UUID("5791BC26-CE9C-11d1-97BF-0000F81E849C")
// SWbemObjectPath;
DEFINE_GUID!{CLSID_SWbemLastError,
    0xC2FEEEAC, 0xCFCD, 0x11d1, 0x8B, 0x05, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("C2FEEEAC-CFCD-11d1-8B05-00600806D9B6")
// SWbemLastError;
DEFINE_GUID!{CLSID_SWbemSink,
    0x75718C9A, 0xF029, 0x11d1, 0xA1, 0xAC, 0x00, 0xC0, 0x4F, 0xB6, 0xC2, 0x23}
// class DECLSPEC_UUID("75718C9A-F029-11d1-A1AC-00C04FB6C223")
// SWbemSink;
DEFINE_GUID!{CLSID_SWbemDateTime,
    0x47DFBE54, 0xCF76, 0x11d3, 0xB3, 0x8F, 0x00, 0x10, 0x5A, 0x1F, 0x47, 0x3A}
// class DECLSPEC_UUID("47DFBE54-CF76-11d3-B38F-00105A1F473A")
// SWbemDateTime;
DEFINE_GUID!{CLSID_SWbemRefresher,
    0xD269BF5C, 0xD9C1, 0x11d3, 0xB3, 0x8F, 0x00, 0x10, 0x5A, 0x1F, 0x47, 0x3A}
// class DECLSPEC_UUID("D269BF5C-D9C1-11d3-B38F-00105A1F473A")
// SWbemRefresher;
DEFINE_GUID!{CLSID_SWbemServices,
    0x04B83D63, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D63-21AE-11d2-8B33-00600806D9B6")
// SWbemServices;
DEFINE_GUID!{CLSID_SWbemServicesEx,
    0x62E522DC, 0x8CF3, 0x40a8, 0x8B, 0x2E, 0x37, 0xD5, 0x95, 0x65, 0x1E, 0x40}
// class DECLSPEC_UUID("62E522DC-8CF3-40a8-8B2E-37D595651E40")
// SWbemServicesEx;
DEFINE_GUID!{CLSID_SWbemObject,
    0x04B83D62, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D62-21AE-11d2-8B33-00600806D9B6")
// SWbemObject;
DEFINE_GUID!{CLSID_SWbemObjectEx,
    0xD6BDAFB2, 0x9435, 0x491f, 0xBB, 0x87, 0x6A, 0xA0, 0xF0, 0xBC, 0x31, 0xA2}
// class DECLSPEC_UUID("D6BDAFB2-9435-491f-BB87-6AA0F0BC31A2")
// SWbemObjectEx;
DEFINE_GUID!{CLSID_SWbemObjectSet,
    0x04B83D61, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D61-21AE-11d2-8B33-00600806D9B6")
// SWbemObjectSet;
DEFINE_GUID!{CLSID_SWbemNamedValue,
    0x04B83D60, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D60-21AE-11d2-8B33-00600806D9B6")
// SWbemNamedValue;
DEFINE_GUID!{CLSID_SWbemQualifier,
    0x04B83D5F, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D5F-21AE-11d2-8B33-00600806D9B6")
// SWbemQualifier;
DEFINE_GUID!{CLSID_SWbemQualifierSet,
    0x04B83D5E, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D5E-21AE-11d2-8B33-00600806D9B6")
// SWbemQualifierSet;
DEFINE_GUID!{CLSID_SWbemProperty,
    0x04B83D5D, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D5D-21AE-11d2-8B33-00600806D9B6")
// SWbemProperty;
DEFINE_GUID!{CLSID_SWbemPropertySet,
    0x04B83D5C, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D5C-21AE-11d2-8B33-00600806D9B6")
// SWbemPropertySet;
DEFINE_GUID!{CLSID_SWbemMethod,
    0x04B83D5B, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D5B-21AE-11d2-8B33-00600806D9B6")
// SWbemMethod;
DEFINE_GUID!{CLSID_SWbemMethodSet,
    0x04B83D5A, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D5A-21AE-11d2-8B33-00600806D9B6")
// SWbemMethodSet;
DEFINE_GUID!{CLSID_SWbemEventSource,
    0x04B83D58, 0x21AE, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("04B83D58-21AE-11d2-8B33-00600806D9B6")
// SWbemEventSource;
DEFINE_GUID!{CLSID_SWbemSecurity,
    0xB54D66E9, 0x2287, 0x11d2, 0x8B, 0x33, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("B54D66E9-2287-11d2-8B33-00600806D9B6")
// SWbemSecurity;
DEFINE_GUID!{CLSID_SWbemPrivilege,
    0x26EE67BC, 0x5804, 0x11d2, 0x8B, 0x4A, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("26EE67BC-5804-11d2-8B4A-00600806D9B6")
// SWbemPrivilege;
DEFINE_GUID!{CLSID_SWbemPrivilegeSet,
    0x26EE67BE, 0x5804, 0x11d2, 0x8B, 0x4A, 0x00, 0x60, 0x08, 0x06, 0xD9, 0xB6}
// class DECLSPEC_UUID("26EE67BE-5804-11d2-8B4A-00600806D9B6")
// SWbemPrivilegeSet;
DEFINE_GUID!{CLSID_SWbemRefreshableItem,
    0x8C6854BC, 0xDE4B, 0x11d3, 0xB3, 0x90, 0x00, 0x10, 0x5A, 0x1F, 0x47, 0x3A}
// class DECLSPEC_UUID("8C6854BC-DE4B-11d3-B390-00105A1F473A")
// SWbemRefreshableItem;
pub const WBEMS_DISPID_OBJECT_READY: DISPID = 1;
pub const WBEMS_DISPID_COMPLETED: DISPID = 2;
pub const WBEMS_DISPID_PROGRESS: DISPID = 3;
pub const WBEMS_DISPID_OBJECT_PUT: DISPID = 4;
pub const WBEMS_DISPID_CONNECTION_READY: DISPID = 5;
// extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0018_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0018_v0_0_s_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0026_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemdisp_0000_0026_v0_0_s_ifspec;

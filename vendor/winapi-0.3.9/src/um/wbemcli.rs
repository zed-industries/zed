// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{__uint64, c_long, c_uchar};
use shared::minwindef::{BYTE, DWORD, ULONG};
use shared::rpcndr::byte;
use shared::winerror::HRESULT;
use shared::wtypes::BSTR;
use um::oaidl::{SAFEARRAY, VARIANT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{LCID, LONG, LPCWSTR, LPWSTR};
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0000_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0000_v0_0_s_ifspec;
ENUM!{enum WBEM_GENUS_TYPE {
    WBEM_GENUS_CLASS = 1,
    WBEM_GENUS_INSTANCE = 2,
}}
ENUM!{enum WBEM_CHANGE_FLAG_TYPE {
    WBEM_FLAG_CREATE_OR_UPDATE = 0,
    WBEM_FLAG_UPDATE_ONLY = 0x1,
    WBEM_FLAG_CREATE_ONLY = 0x2,
    WBEM_FLAG_UPDATE_COMPATIBLE = 0,
    WBEM_FLAG_UPDATE_SAFE_MODE = 0x20,
    WBEM_FLAG_UPDATE_FORCE_MODE = 0x40,
    WBEM_MASK_UPDATE_MODE = 0x60,
    WBEM_FLAG_ADVISORY = 0x10000,
}}
ENUM!{enum WBEM_GENERIC_FLAG_TYPE {
    WBEM_FLAG_RETURN_IMMEDIATELY = 0x10,
    WBEM_FLAG_RETURN_WBEM_COMPLETE = 0,
    WBEM_FLAG_BIDIRECTIONAL = 0,
    WBEM_FLAG_FORWARD_ONLY = 0x20,
    WBEM_FLAG_NO_ERROR_OBJECT = 0x40,
    WBEM_FLAG_RETURN_ERROR_OBJECT = 0,
    WBEM_FLAG_SEND_STATUS = 0x80,
    WBEM_FLAG_DONT_SEND_STATUS = 0,
    WBEM_FLAG_ENSURE_LOCATABLE = 0x100,
    WBEM_FLAG_DIRECT_READ = 0x200,
    WBEM_FLAG_SEND_ONLY_SELECTED = 0,
    WBEM_RETURN_WHEN_COMPLETE = 0,
    WBEM_RETURN_IMMEDIATELY = 0x10,
    WBEM_MASK_RESERVED_FLAGS = 0x1f000,
    WBEM_FLAG_USE_AMENDED_QUALIFIERS = 0x20000,
    WBEM_FLAG_STRONG_VALIDATION = 0x100000,
}}
ENUM!{enum WBEM_STATUS_TYPE {
    WBEM_STATUS_COMPLETE = 0,
    WBEM_STATUS_REQUIREMENTS = 1,
    WBEM_STATUS_PROGRESS = 2,
    WBEM_STATUS_LOGGING_INFORMATION = 0x100,
    WBEM_STATUS_LOGGING_INFORMATION_PROVIDER = 0x200,
    WBEM_STATUS_LOGGING_INFORMATION_HOST = 0x400,
    WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY = 0x800,
    WBEM_STATUS_LOGGING_INFORMATION_ESS = 0x1000,
}}
ENUM!{enum WBEM_TIMEOUT_TYPE {
    WBEM_NO_WAIT = 0,
    WBEM_INFINITE = 0xffffffff,
}}
ENUM!{enum WBEM_CONDITION_FLAG_TYPE {
    WBEM_FLAG_ALWAYS = 0,
    WBEM_FLAG_ONLY_IF_TRUE = 0x1,
    WBEM_FLAG_ONLY_IF_FALSE = 0x2,
    WBEM_FLAG_ONLY_IF_IDENTICAL = 0x3,
    WBEM_MASK_PRIMARY_CONDITION = 0x3,
    WBEM_FLAG_KEYS_ONLY = 0x4,
    WBEM_FLAG_REFS_ONLY = 0x8,
    WBEM_FLAG_LOCAL_ONLY = 0x10,
    WBEM_FLAG_PROPAGATED_ONLY = 0x20,
    WBEM_FLAG_SYSTEM_ONLY = 0x30,
    WBEM_FLAG_NONSYSTEM_ONLY = 0x40,
    WBEM_MASK_CONDITION_ORIGIN = 0x70,
    WBEM_FLAG_CLASS_OVERRIDES_ONLY = 0x100,
    WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES = 0x200,
    WBEM_MASK_CLASS_CONDITION = 0x300,
}}
ENUM!{enum WBEM_FLAVOR_TYPE {
    WBEM_FLAVOR_DONT_PROPAGATE = 0,
    WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE = 0x1,
    WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS = 0x2,
    WBEM_FLAVOR_MASK_PROPAGATION = 0xf,
    WBEM_FLAVOR_OVERRIDABLE = 0,
    WBEM_FLAVOR_NOT_OVERRIDABLE = 0x10,
    WBEM_FLAVOR_MASK_PERMISSIONS = 0x10,
    WBEM_FLAVOR_ORIGIN_LOCAL = 0,
    WBEM_FLAVOR_ORIGIN_PROPAGATED = 0x20,
    WBEM_FLAVOR_ORIGIN_SYSTEM = 0x40,
    WBEM_FLAVOR_MASK_ORIGIN = 0x60,
    WBEM_FLAVOR_NOT_AMENDED = 0,
    WBEM_FLAVOR_AMENDED = 0x80,
    WBEM_FLAVOR_MASK_AMENDED = 0x80,
}}
ENUM!{enum WBEM_QUERY_FLAG_TYPE {
    WBEM_FLAG_DEEP = 0,
    WBEM_FLAG_SHALLOW = 1,
    WBEM_FLAG_PROTOTYPE = 2,
}}
ENUM!{enum WBEM_SECURITY_FLAGS {
    WBEM_ENABLE = 1,
    WBEM_METHOD_EXECUTE = 2,
    WBEM_FULL_WRITE_REP = 4,
    WBEM_PARTIAL_WRITE_REP = 8,
    WBEM_WRITE_PROVIDER = 0x10,
    WBEM_REMOTE_ACCESS = 0x20,
    WBEM_RIGHT_SUBSCRIBE = 0x40,
    WBEM_RIGHT_PUBLISH = 0x80,
}}
ENUM!{enum WBEM_LIMITATION_FLAG_TYPE {
    WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS = 0x10,
    WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS = 0x20,
}}
ENUM!{enum WBEM_TEXT_FLAG_TYPE {
    WBEM_FLAG_NO_FLAVORS = 0x1,
}}
ENUM!{enum WBEM_COMPARISON_FLAG {
    WBEM_COMPARISON_INCLUDE_ALL = 0,
    WBEM_FLAG_IGNORE_QUALIFIERS = 0x1,
    WBEM_FLAG_IGNORE_OBJECT_SOURCE = 0x2,
    WBEM_FLAG_IGNORE_DEFAULT_VALUES = 0x4,
    WBEM_FLAG_IGNORE_CLASS = 0x8,
    WBEM_FLAG_IGNORE_CASE = 0x10,
    WBEM_FLAG_IGNORE_FLAVOR = 0x20,
}}
ENUM!{enum WBEM_LOCKING_FLAG_TYPE {
    WBEM_FLAG_ALLOW_READ = 0x1,
}}
ENUM!{enum CIMTYPE_ENUMERATION {
    CIM_ILLEGAL = 0xfff,
    CIM_EMPTY = 0,
    CIM_SINT8 = 16,
    CIM_UINT8 = 17,
    CIM_SINT16 = 2,
    CIM_UINT16 = 18,
    CIM_SINT32 = 3,
    CIM_UINT32 = 19,
    CIM_SINT64 = 20,
    CIM_UINT64 = 21,
    CIM_REAL32 = 4,
    CIM_REAL64 = 5,
    CIM_BOOLEAN = 11,
    CIM_STRING = 8,
    CIM_DATETIME = 101,
    CIM_REFERENCE = 102,
    CIM_CHAR16 = 103,
    CIM_OBJECT = 13,
    CIM_FLAG_ARRAY = 0x2000,
}}
ENUM!{enum WBEM_BACKUP_RESTORE_FLAGS {
    WBEM_FLAG_BACKUP_RESTORE_DEFAULT = 0,
    WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN = 1,
}}
ENUM!{enum WBEM_REFRESHER_FLAGS {
    WBEM_FLAG_REFRESH_AUTO_RECONNECT = 0,
    WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT = 1,
}}
ENUM!{enum WBEM_SHUTDOWN_FLAGS {
    WBEM_SHUTDOWN_UNLOAD_COMPONENT = 1,
    WBEM_SHUTDOWN_WMI = 2,
    WBEM_SHUTDOWN_OS = 3,
}}
pub type CIMTYPE = c_long;
ENUM!{enum WBEMSTATUS_FORMAT {
    WBEMSTATUS_FORMAT_NEWLINE = 0,
    WBEMSTATUS_FORMAT_NO_NEWLINE = 1,
}}
ENUM!{enum WBEM_LIMITS {
    WBEM_MAX_IDENTIFIER = 0x1000,
    WBEM_MAX_QUERY = 0x4000,
    WBEM_MAX_PATH = 0x2000,
    WBEM_MAX_OBJECT_NESTING = 64,
    WBEM_MAX_USER_PROPERTIES = 1024,
}}
ENUM!{enum WBEMSTATUS {
    WBEM_NO_ERROR = 0,
    WBEM_S_NO_ERROR = 0,
    WBEM_S_SAME = 0,
    WBEM_S_FALSE = 1,
    WBEM_S_ALREADY_EXISTS = 0x40001,
    WBEM_S_RESET_TO_DEFAULT = 0x40002,
    WBEM_S_DIFFERENT = 0x40003,
    WBEM_S_TIMEDOUT = 0x40004,
    WBEM_S_NO_MORE_DATA = 0x40005,
    WBEM_S_OPERATION_CANCELLED = 0x40006,
    WBEM_S_PENDING = 0x40007,
    WBEM_S_DUPLICATE_OBJECTS = 0x40008,
    WBEM_S_ACCESS_DENIED = 0x40009,
    WBEM_S_PARTIAL_RESULTS = 0x40010,
    WBEM_S_SOURCE_NOT_AVAILABLE = 0x40017,
    WBEM_E_FAILED = 0x80041001,
    WBEM_E_NOT_FOUND = 0x80041002,
    WBEM_E_ACCESS_DENIED = 0x80041003,
    WBEM_E_PROVIDER_FAILURE = 0x80041004,
    WBEM_E_TYPE_MISMATCH = 0x80041005,
    WBEM_E_OUT_OF_MEMORY = 0x80041006,
    WBEM_E_INVALID_CONTEXT = 0x80041007,
    WBEM_E_INVALID_PARAMETER = 0x80041008,
    WBEM_E_NOT_AVAILABLE = 0x80041009,
    WBEM_E_CRITICAL_ERROR = 0x8004100a,
    WBEM_E_INVALID_STREAM = 0x8004100b,
    WBEM_E_NOT_SUPPORTED = 0x8004100c,
    WBEM_E_INVALID_SUPERCLASS = 0x8004100d,
    WBEM_E_INVALID_NAMESPACE = 0x8004100e,
    WBEM_E_INVALID_OBJECT = 0x8004100f,
    WBEM_E_INVALID_CLASS = 0x80041010,
    WBEM_E_PROVIDER_NOT_FOUND = 0x80041011,
    WBEM_E_INVALID_PROVIDER_REGISTRATION = 0x80041012,
    WBEM_E_PROVIDER_LOAD_FAILURE = 0x80041013,
    WBEM_E_INITIALIZATION_FAILURE = 0x80041014,
    WBEM_E_TRANSPORT_FAILURE = 0x80041015,
    WBEM_E_INVALID_OPERATION = 0x80041016,
    WBEM_E_INVALID_QUERY = 0x80041017,
    WBEM_E_INVALID_QUERY_TYPE = 0x80041018,
    WBEM_E_ALREADY_EXISTS = 0x80041019,
    WBEM_E_OVERRIDE_NOT_ALLOWED = 0x8004101a,
    WBEM_E_PROPAGATED_QUALIFIER = 0x8004101b,
    WBEM_E_PROPAGATED_PROPERTY = 0x8004101c,
    WBEM_E_UNEXPECTED = 0x8004101d,
    WBEM_E_ILLEGAL_OPERATION = 0x8004101e,
    WBEM_E_CANNOT_BE_KEY = 0x8004101f,
    WBEM_E_INCOMPLETE_CLASS = 0x80041020,
    WBEM_E_INVALID_SYNTAX = 0x80041021,
    WBEM_E_NONDECORATED_OBJECT = 0x80041022,
    WBEM_E_READ_ONLY = 0x80041023,
    WBEM_E_PROVIDER_NOT_CAPABLE = 0x80041024,
    WBEM_E_CLASS_HAS_CHILDREN = 0x80041025,
    WBEM_E_CLASS_HAS_INSTANCES = 0x80041026,
    WBEM_E_QUERY_NOT_IMPLEMENTED = 0x80041027,
    WBEM_E_ILLEGAL_NULL = 0x80041028,
    WBEM_E_INVALID_QUALIFIER_TYPE = 0x80041029,
    WBEM_E_INVALID_PROPERTY_TYPE = 0x8004102a,
    WBEM_E_VALUE_OUT_OF_RANGE = 0x8004102b,
    WBEM_E_CANNOT_BE_SINGLETON = 0x8004102c,
    WBEM_E_INVALID_CIM_TYPE = 0x8004102d,
    WBEM_E_INVALID_METHOD = 0x8004102e,
    WBEM_E_INVALID_METHOD_PARAMETERS = 0x8004102f,
    WBEM_E_SYSTEM_PROPERTY = 0x80041030,
    WBEM_E_INVALID_PROPERTY = 0x80041031,
    WBEM_E_CALL_CANCELLED = 0x80041032,
    WBEM_E_SHUTTING_DOWN = 0x80041033,
    WBEM_E_PROPAGATED_METHOD = 0x80041034,
    WBEM_E_UNSUPPORTED_PARAMETER = 0x80041035,
    WBEM_E_MISSING_PARAMETER_ID = 0x80041036,
    WBEM_E_INVALID_PARAMETER_ID = 0x80041037,
    WBEM_E_NONCONSECUTIVE_PARAMETER_IDS = 0x80041038,
    WBEM_E_PARAMETER_ID_ON_RETVAL = 0x80041039,
    WBEM_E_INVALID_OBJECT_PATH = 0x8004103a,
    WBEM_E_OUT_OF_DISK_SPACE = 0x8004103b,
    WBEM_E_BUFFER_TOO_SMALL = 0x8004103c,
    WBEM_E_UNSUPPORTED_PUT_EXTENSION = 0x8004103d,
    WBEM_E_UNKNOWN_OBJECT_TYPE = 0x8004103e,
    WBEM_E_UNKNOWN_PACKET_TYPE = 0x8004103f,
    WBEM_E_MARSHAL_VERSION_MISMATCH = 0x80041040,
    WBEM_E_MARSHAL_INVALID_SIGNATURE = 0x80041041,
    WBEM_E_INVALID_QUALIFIER = 0x80041042,
    WBEM_E_INVALID_DUPLICATE_PARAMETER = 0x80041043,
    WBEM_E_TOO_MUCH_DATA = 0x80041044,
    WBEM_E_SERVER_TOO_BUSY = 0x80041045,
    WBEM_E_INVALID_FLAVOR = 0x80041046,
    WBEM_E_CIRCULAR_REFERENCE = 0x80041047,
    WBEM_E_UNSUPPORTED_CLASS_UPDATE = 0x80041048,
    WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE = 0x80041049,
    WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE = 0x80041050,
    WBEM_E_TOO_MANY_PROPERTIES = 0x80041051,
    WBEM_E_UPDATE_TYPE_MISMATCH = 0x80041052,
    WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED = 0x80041053,
    WBEM_E_UPDATE_PROPAGATED_METHOD = 0x80041054,
    WBEM_E_METHOD_NOT_IMPLEMENTED = 0x80041055,
    WBEM_E_METHOD_DISABLED = 0x80041056,
    WBEM_E_REFRESHER_BUSY = 0x80041057,
    WBEM_E_UNPARSABLE_QUERY = 0x80041058,
    WBEM_E_NOT_EVENT_CLASS = 0x80041059,
    WBEM_E_MISSING_GROUP_WITHIN = 0x8004105a,
    WBEM_E_MISSING_AGGREGATION_LIST = 0x8004105b,
    WBEM_E_PROPERTY_NOT_AN_OBJECT = 0x8004105c,
    WBEM_E_AGGREGATING_BY_OBJECT = 0x8004105d,
    WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY = 0x8004105f,
    WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING = 0x80041060,
    WBEM_E_QUEUE_OVERFLOW = 0x80041061,
    WBEM_E_PRIVILEGE_NOT_HELD = 0x80041062,
    WBEM_E_INVALID_OPERATOR = 0x80041063,
    WBEM_E_LOCAL_CREDENTIALS = 0x80041064,
    WBEM_E_CANNOT_BE_ABSTRACT = 0x80041065,
    WBEM_E_AMENDED_OBJECT = 0x80041066,
    WBEM_E_CLIENT_TOO_SLOW = 0x80041067,
    WBEM_E_NULL_SECURITY_DESCRIPTOR = 0x80041068,
    WBEM_E_TIMED_OUT = 0x80041069,
    WBEM_E_INVALID_ASSOCIATION = 0x8004106a,
    WBEM_E_AMBIGUOUS_OPERATION = 0x8004106b,
    WBEM_E_QUOTA_VIOLATION = 0x8004106c,
    WBEM_E_RESERVED_001 = 0x8004106d,
    WBEM_E_RESERVED_002 = 0x8004106e,
    WBEM_E_UNSUPPORTED_LOCALE = 0x8004106f,
    WBEM_E_HANDLE_OUT_OF_DATE = 0x80041070,
    WBEM_E_CONNECTION_FAILED = 0x80041071,
    WBEM_E_INVALID_HANDLE_REQUEST = 0x80041072,
    WBEM_E_PROPERTY_NAME_TOO_WIDE = 0x80041073,
    WBEM_E_CLASS_NAME_TOO_WIDE = 0x80041074,
    WBEM_E_METHOD_NAME_TOO_WIDE = 0x80041075,
    WBEM_E_QUALIFIER_NAME_TOO_WIDE = 0x80041076,
    WBEM_E_RERUN_COMMAND = 0x80041077,
    WBEM_E_DATABASE_VER_MISMATCH = 0x80041078,
    WBEM_E_VETO_DELETE = 0x80041079,
    WBEM_E_VETO_PUT = 0x8004107a,
    WBEM_E_INVALID_LOCALE = 0x80041080,
    WBEM_E_PROVIDER_SUSPENDED = 0x80041081,
    WBEM_E_SYNCHRONIZATION_REQUIRED = 0x80041082,
    WBEM_E_NO_SCHEMA = 0x80041083,
    WBEM_E_PROVIDER_ALREADY_REGISTERED = 0x80041084,
    WBEM_E_PROVIDER_NOT_REGISTERED = 0x80041085,
    WBEM_E_FATAL_TRANSPORT_ERROR = 0x80041086,
    WBEM_E_ENCRYPTED_CONNECTION_REQUIRED = 0x80041087,
    WBEM_E_PROVIDER_TIMED_OUT = 0x80041088,
    WBEM_E_NO_KEY = 0x80041089,
    WBEM_E_PROVIDER_DISABLED = 0x8004108a,
    WBEMESS_E_REGISTRATION_TOO_BROAD = 0x80042001,
    WBEMESS_E_REGISTRATION_TOO_PRECISE = 0x80042002,
    WBEMESS_E_AUTHZ_NOT_PRIVILEGED = 0x80042003,
    WBEMMOF_E_EXPECTED_QUALIFIER_NAME = 0x80044001,
    WBEMMOF_E_EXPECTED_SEMI = 0x80044002,
    WBEMMOF_E_EXPECTED_OPEN_BRACE = 0x80044003,
    WBEMMOF_E_EXPECTED_CLOSE_BRACE = 0x80044004,
    WBEMMOF_E_EXPECTED_CLOSE_BRACKET = 0x80044005,
    WBEMMOF_E_EXPECTED_CLOSE_PAREN = 0x80044006,
    WBEMMOF_E_ILLEGAL_CONSTANT_VALUE = 0x80044007,
    WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER = 0x80044008,
    WBEMMOF_E_EXPECTED_OPEN_PAREN = 0x80044009,
    WBEMMOF_E_UNRECOGNIZED_TOKEN = 0x8004400a,
    WBEMMOF_E_UNRECOGNIZED_TYPE = 0x8004400b,
    WBEMMOF_E_EXPECTED_PROPERTY_NAME = 0x8004400c,
    WBEMMOF_E_TYPEDEF_NOT_SUPPORTED = 0x8004400d,
    WBEMMOF_E_UNEXPECTED_ALIAS = 0x8004400e,
    WBEMMOF_E_UNEXPECTED_ARRAY_INIT = 0x8004400f,
    WBEMMOF_E_INVALID_AMENDMENT_SYNTAX = 0x80044010,
    WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT = 0x80044011,
    WBEMMOF_E_INVALID_PRAGMA = 0x80044012,
    WBEMMOF_E_INVALID_NAMESPACE_SYNTAX = 0x80044013,
    WBEMMOF_E_EXPECTED_CLASS_NAME = 0x80044014,
    WBEMMOF_E_TYPE_MISMATCH = 0x80044015,
    WBEMMOF_E_EXPECTED_ALIAS_NAME = 0x80044016,
    WBEMMOF_E_INVALID_CLASS_DECLARATION = 0x80044017,
    WBEMMOF_E_INVALID_INSTANCE_DECLARATION = 0x80044018,
    WBEMMOF_E_EXPECTED_DOLLAR = 0x80044019,
    WBEMMOF_E_CIMTYPE_QUALIFIER = 0x8004401a,
    WBEMMOF_E_DUPLICATE_PROPERTY = 0x8004401b,
    WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION = 0x8004401c,
    WBEMMOF_E_OUT_OF_RANGE = 0x8004401d,
    WBEMMOF_E_INVALID_FILE = 0x8004401e,
    WBEMMOF_E_ALIASES_IN_EMBEDDED = 0x8004401f,
    WBEMMOF_E_NULL_ARRAY_ELEM = 0x80044020,
    WBEMMOF_E_DUPLICATE_QUALIFIER = 0x80044021,
    WBEMMOF_E_EXPECTED_FLAVOR_TYPE = 0x80044022,
    WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES = 0x80044023,
    WBEMMOF_E_MULTIPLE_ALIASES = 0x80044024,
    WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2 = 0x80044025,
    WBEMMOF_E_NO_ARRAYS_RETURNED = 0x80044026,
    WBEMMOF_E_MUST_BE_IN_OR_OUT = 0x80044027,
    WBEMMOF_E_INVALID_FLAGS_SYNTAX = 0x80044028,
    WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE = 0x80044029,
    WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE = 0x8004402a,
    WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE = 0x8004402b,
    WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX = 0x8004402c,
    WBEMMOF_E_INVALID_QUALIFIER_SYNTAX = 0x8004402d,
    WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE = 0x8004402e,
    WBEMMOF_E_ERROR_CREATING_TEMP_FILE = 0x8004402f,
    WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE = 0x80044030,
    WBEMMOF_E_INVALID_DELETECLASS_SYNTAX = 0x80044031,
}}
// EXTERN_C const IID LIBID_WbemClient_v1;
// EXTERN_C const IID IID_IWbemClassObject;
DEFINE_GUID!{IID_IWbemClassObject,
    0xdc12a681, 0x737f, 0x11cf, 0x88, 0x4d, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24}
RIDL!{#[uuid(0xdc12a681, 0x737f, 0x11cf, 0x88, 0x4d, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24)]
interface IWbemClassObject(IWbemClassObjectVtbl): IUnknown(IUnknownVtbl) {
    fn GetQualifierSet(
        ppQualSet: *mut *mut IWbemQualifierSet,
    ) -> HRESULT,
    fn Get(
        wszName: LPCWSTR,
        lFlags: c_long,
        pVal: *mut VARIANT,
        pType: *mut CIMTYPE,
        plFlavor: *mut c_long,
    ) -> HRESULT,
    fn Put(
        wszName: LPCWSTR,
        lFlags: c_long,
        pVal: *mut VARIANT,
        Type: CIMTYPE,
    ) -> HRESULT,
    fn Delete(
        wszName: LPCWSTR,
    ) -> HRESULT,
    fn GetNames(
        wszQualifierName: LPCWSTR,
        lFlags: c_long,
        pQualifierVal: *mut VARIANT,
        pNames: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn BeginEnumeration(
        lEnumFlags: c_long,
    ) -> HRESULT,
    fn Next(
        lFlags: c_long,
        strName: *mut BSTR,
        pVal: *mut VARIANT,
        pType: *mut CIMTYPE,
        plFlavor: *mut c_long,
    ) -> HRESULT,
    fn EndEnumeration() -> HRESULT,
    fn GetPropertyQualifierSet(
        wszProperty: LPCWSTR,
        ppQualSet: *mut *mut IWbemQualifierSet,
    ) -> HRESULT,
    fn Clone(
        ppCopy: *mut *mut IWbemClassObject,
    ) -> HRESULT,
    fn GetObjectText(
        lFlags: c_long,
        pstrObjectText: *mut BSTR,
    ) -> HRESULT,
    fn SpawnDerivedClass(
        lFlags: c_long,
        ppNewClass: *mut *mut IWbemClassObject,
    ) -> HRESULT,
    fn SpawnInstance(
        lFlags: c_long,
        ppNewInstance: *mut *mut IWbemClassObject,
    ) -> HRESULT,
    fn CompareTo(
        lFlags: c_long,
        pCompareTo: *mut IWbemClassObject,
    ) -> HRESULT,
    fn GetPropertyOrigin(
        wszName: LPCWSTR,
        pstrClassName: *mut BSTR,
    ) -> HRESULT,
    fn InheritsFrom(
        strAncestor: LPCWSTR,
    ) -> HRESULT,
    fn GetMethod(
        wszName: LPCWSTR,
        lFlags: c_long,
        ppInSignature: *mut *mut IWbemClassObject,
        ppOutSignature: *mut *mut IWbemClassObject,
    ) -> HRESULT,
    fn PutMethod(
        wszName: LPCWSTR,
        lFlags: c_long,
        pInSignature: *mut IWbemClassObject,
        pOutSignature: *mut IWbemClassObject,
    ) -> HRESULT,
    fn DeleteMethod(
        wszName: LPCWSTR,
    ) -> HRESULT,
    fn BeginMethodEnumeration(
        lEnumFlags: c_long,
    ) -> HRESULT,
    fn NextMethod(
        lFlags: c_long,
        pstrName: *mut BSTR,
        ppInSignature: *mut *mut IWbemClassObject,
        ppOutSignature: *mut *mut IWbemClassObject,
    ) -> HRESULT,
    fn EndMethodEnumeration() -> HRESULT,
    fn GetMethodQualifierSet(
        wszMethod: LPCWSTR,
        ppQualSet: *mut *mut IWbemQualifierSet,
    ) -> HRESULT,
    fn GetMethodOrigin(
        wszMethodName: LPCWSTR,
        pstrClassName: *mut BSTR,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemObjectAccess;
DEFINE_GUID!{IID_IWbemObjectAccess,
    0x49353c9a, 0x516b, 0x11d1, 0xae, 0xa6, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0x49353c9a, 0x516b, 0x11d1, 0xae, 0xa6, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemObjectAccess(IWbemObjectAccessVtbl): IWbemClassObject(IWbemClassObjectVtbl) {
    fn GetPropertyHandle(
        wszPropertyName: LPCWSTR,
        pType: *mut CIMTYPE,
        plHandle: *mut c_long,
    ) -> HRESULT,
    fn WritePropertyValue(
        lHandle: c_long,
        lNumBytes: c_long,
        aData: *const byte,
    ) -> HRESULT,
    fn ReadPropertyValue(
        lHandle: c_long,
        lBufferSize: c_long,
        plNumBytes: *mut c_long,
        aData: *mut byte,
    ) -> HRESULT,
    fn ReadDWORD(
        lHandle: c_long,
        pdw: *mut DWORD,
    ) -> HRESULT,
    fn WriteDWORD(
        lHandle: c_long,
        dw: DWORD,
    ) -> HRESULT,
    fn ReadQWORD(
        lHandle: c_long,
        pqw: *mut __uint64,
    ) -> HRESULT,
    fn WriteQWORD(
        lHandle: c_long,
        pw: __uint64,
    ) -> HRESULT,
    fn GetPropertyInfoByHandle(
        lHandle: c_long,
        pstrName: *mut BSTR,
        pType: *mut CIMTYPE,
    ) -> HRESULT,
    fn Lock(
        lFlags: c_long,
    ) -> HRESULT,
    fn Unlock(
        lFlags: c_long,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemQualifierSet;
DEFINE_GUID!{IID_IWbemQualifierSet,
    0xdc12a680, 0x737f, 0x11cf, 0x88, 0x4d, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24}
RIDL!{#[uuid(0xdc12a680, 0x737f, 0x11cf, 0x88, 0x4d, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24)]
interface IWbemQualifierSet(IWbemQualifierSetVtbl): IUnknown(IUnknownVtbl) {
    fn Get(
        wszName: LPCWSTR,
        lFlags: c_long,
        pVal: *mut VARIANT,
        plFlavor: *mut c_long,
    ) -> HRESULT,
    fn Put(
        wszName: LPCWSTR,
        pVal: *mut VARIANT,
        lFlavor: c_long,
    ) -> HRESULT,
    fn Delete(
        wszName: LPCWSTR,
    ) -> HRESULT,
    fn GetNames(
        lFlags: c_long,
        pNames: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn BeginEnumeration(
        lFlags: c_long,
    ) -> HRESULT,
    fn Next(
        lFlags: c_long,
        pstrName: *mut BSTR,
        pVal: *mut VARIANT,
        plFlavor: *mut c_long,
    ) -> HRESULT,
    fn EndEnumeration() -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemServices;
DEFINE_GUID!{IID_IWbemServices,
    0x9556dc99, 0x828c, 0x11cf, 0xa3, 0x7e, 0x00, 0xaa, 0x00, 0x32, 0x40, 0xc7}
RIDL!{#[uuid(0x9556dc99, 0x828c, 0x11cf, 0xa3, 0x7e, 0x00, 0xaa, 0x00, 0x32, 0x40, 0xc7)]
interface IWbemServices(IWbemServicesVtbl): IUnknown(IUnknownVtbl) {
    fn OpenNamespace(
        strNamespace: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppWorkingNamespace: *mut *mut IWbemServices,
        ppResult: *mut *mut IWbemCallResult,
    ) -> HRESULT,
    fn CancelAsyncCall(
        pSink: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn QueryObjectSink(
        lFlags: c_long,
        ppResponseHandler: *mut *mut IWbemObjectSink,
    ) -> HRESULT,
    fn GetObject(
        strObjectPath: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppObject: *mut *mut IWbemClassObject,
        ppCallResult: *mut *mut IWbemCallResult,
    ) -> HRESULT,
    fn GetObjectAsync(
        strObjectPath: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn PutClass(
        pObject: *mut IWbemClassObject,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppCallResult: *mut *mut IWbemCallResult,
    ) -> HRESULT,
    fn PutClassAsync(
        pObject: *mut IWbemClassObject,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn DeleteClass(
        strClass: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppCallResult: *mut *mut IWbemCallResult,
    ) -> HRESULT,
    fn DeleteClassAsync(
        strClass: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn CreateClassEnum(
        strSuperclass: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppEnum: *mut *mut IEnumWbemClassObject,
    ) -> HRESULT,
    fn CreateClassEnumAsync(
        strSuperclass: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn PutInstance(
        pInst: *mut IWbemClassObject,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppCallResult: *mut *mut IWbemCallResult,
    ) -> HRESULT,
    fn PutInstanceAsync(
        pInst: *mut IWbemClassObject,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn DeleteInstance(
        strObjectPath: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppCallResult: *mut *mut IWbemCallResult,
    ) -> HRESULT,
    fn DeleteInstanceAsync(
        strObjectPath: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn CreateInstanceEnum(
        strFilter: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppEnum: *mut *mut IEnumWbemClassObject,
    ) -> HRESULT,
    fn CreateInstanceEnumAsync(
        strFilter: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn ExecQuery(
        strQueryLanguage: BSTR,
        strQuery: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppEnum: *mut *mut IEnumWbemClassObject,
    ) -> HRESULT,
    fn ExecQueryAsync(
        strQueryLanguage: BSTR,
        strQuery: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn ExecNotificationQuery(
        strQueryLanguage: BSTR,
        strQuery: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        ppEnum: *mut *mut IEnumWbemClassObject,
    ) -> HRESULT,
    fn ExecNotificationQueryAsync(
        strQueryLanguage: BSTR,
        strQuery: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn ExecMethod(
        strObjectPath: BSTR,
        strMethodName: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pInParams: *mut IWbemClassObject,
        ppOutParams: *mut *mut IWbemClassObject,
        ppCallResult: *mut *mut IWbemCallResult,
    ) -> HRESULT,
    fn ExecMethodAsync(
        strObjectPath: BSTR,
        strMethodName: BSTR,
        lFlags: c_long,
        pCtx: *mut IWbemContext,
        pInParams: *mut IWbemClassObject,
        pResponseHandler: *mut IWbemObjectSink,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemLocator;
DEFINE_GUID!{IID_IWbemLocator,
    0xdc12a687, 0x737f, 0x11cf, 0x88, 0x4d, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24}
RIDL!{#[uuid(0xdc12a687, 0x737f, 0x11cf, 0x88, 0x4d, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24)]
interface IWbemLocator(IWbemLocatorVtbl): IUnknown(IUnknownVtbl) {
    fn ConnectServer(
        strNetworkResource: BSTR,
        strUser: BSTR,
        strPassword: BSTR,
        strLocale: BSTR,
        lSecurityFlags: c_long,
        strAuthority: BSTR,
        pCtx: *mut IWbemContext,
        ppNamespace: *mut *mut IWbemServices,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemObjectSink;
DEFINE_GUID!{IID_IWbemObjectSink,
    0x7c857801, 0x7381, 0x11cf, 0x88, 0x4d, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24}
RIDL!{#[uuid(0x7c857801, 0x7381, 0x11cf, 0x88, 0x4d, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24)]
interface IWbemObjectSink(IWbemObjectSinkVtbl): IUnknown(IUnknownVtbl) {
    fn Indicate(
        lObjectCount: c_long,
        apObjArray: *mut *mut IWbemClassObject,
    ) -> HRESULT,
    fn SetStatus(
        lFlags: c_long,
        hResult: HRESULT,
        strParam: BSTR,
        pObjParam: *mut IWbemClassObject,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IEnumWbemClassObject;
DEFINE_GUID!{IID_IEnumWbemClassObject,
    0x027947e1, 0xd731, 0x11ce, 0xa3, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01}
RIDL!{#[uuid(0x027947e1, 0xd731, 0x11ce, 0xa3, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01)]
interface IEnumWbemClassObject(IEnumWbemClassObjectVtbl): IUnknown(IUnknownVtbl) {
    fn Reset() -> HRESULT,
    fn Next(
        lTimeout: c_long,
        uCount: ULONG,
        apObjects: *mut *mut IWbemClassObject,
        puReturned: *mut ULONG,
    ) -> HRESULT,
    fn NextAsync(
        uCount: ULONG,
        pSink: *mut IWbemObjectSink,
    ) -> HRESULT,
    fn Clone(
        ppEnum: *mut *mut IEnumWbemClassObject,
    ) -> HRESULT,
    fn Skip(
        lTimeout: c_long,
        nCount: ULONG,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemCallResult;
DEFINE_GUID!{IID_IWbemCallResult,
    0x44aca675, 0xe8fc, 0x11d0, 0xa0, 0x7c, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0x44aca675, 0xe8fc, 0x11d0, 0xa0, 0x7c, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemCallResult(IWbemCallResultVtbl): IUnknown(IUnknownVtbl) {
    fn GetResultObject(
        lTimeout: c_long,
        ppResultObject: *mut *mut IWbemClassObject,
    ) -> HRESULT,
    fn GetResultString(
        lTimeout: c_long,
        pstrResultString: *mut BSTR,
    ) -> HRESULT,
    fn GetResultServices(
        lTimeout: c_long,
        ppServices: *mut *mut IWbemServices,
    ) -> HRESULT,
    fn GetCallStatus(
        lTimeout: c_long,
        plStatus: *mut c_long,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemContext;
DEFINE_GUID!{IID_IWbemContext,
    0x44aca674, 0xe8fc, 0x11d0, 0xa0, 0x7c, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0x44aca674, 0xe8fc, 0x11d0, 0xa0, 0x7c, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemContext(IWbemContextVtbl): IUnknown(IUnknownVtbl) {
    fn Clone(
        ppNewCopy: *mut *mut IWbemContext,
    ) -> HRESULT,
    fn GetNames(
        lFlags: c_long,
        pNames: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn BeginEnumeration(
        lFlags: c_long,
    ) -> HRESULT,
    fn Next(
        lFlags: c_long,
        pstrName: *mut BSTR,
        pValue: *mut VARIANT,
    ) -> HRESULT,
    fn EndEnumeration() -> HRESULT,
    fn SetValue(
        wszName: LPCWSTR,
        lFlags: c_long,
        pValue: *mut VARIANT,
    ) -> HRESULT,
    fn GetValue(
        wszName: LPCWSTR,
        lFlags: c_long,
        pValue: *mut VARIANT,
    ) -> HRESULT,
    fn DeleteValue(
        wszName: LPCWSTR,
        lFlags: c_long,
    ) -> HRESULT,
    fn DeleteAll() -> HRESULT,
}}
// EXTERN_C const IID IID_IUnsecuredApartment;
DEFINE_GUID!{IID_IUnsecuredApartment,
    0x1cfaba8c, 0x1523, 0x11d1, 0xad, 0x79, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
RIDL!{#[uuid(0x1cfaba8c, 0x1523, 0x11d1, 0xad, 0x79, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff)]
interface IUnsecuredApartment(IUnsecuredApartmentVtbl): IUnknown(IUnknownVtbl) {
    fn CreateObjectStub(
        pObject: *mut IUnknown,
        ppStub: *mut *mut IUnknown,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemUnsecuredApartment;
DEFINE_GUID!{IID_IWbemUnsecuredApartment,
    0x31739d04, 0x3471, 0x4cf4, 0x9a, 0x7c, 0x57, 0xa4, 0x4a, 0xe7, 0x19, 0x56}
RIDL!{#[uuid(0x31739d04, 0x3471, 0x4cf4, 0x9a, 0x7c, 0x57, 0xa4, 0x4a, 0xe7, 0x19, 0x56)]
interface IWbemUnsecuredApartment(IWbemUnsecuredApartmentVtbl):
IUnsecuredApartment(IUnsecuredApartmentVtbl) {
    fn CreateSinkStub(
        pSink: *mut IWbemObjectSink,
        dwFlags: DWORD,
        wszReserved: LPCWSTR,
        ppStub: *mut *mut IWbemObjectSink,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemStatusCodeText;
DEFINE_GUID!{IID_IWbemStatusCodeText,
    0xeb87e1bc, 0x3233, 0x11d2, 0xae, 0xc9, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0xeb87e1bc, 0x3233, 0x11d2, 0xae, 0xc9, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemStatusCodeText(IWbemStatusCodeTextVtbl): IUnknown(IUnknownVtbl) {
    fn GetErrorCodeText(
        hRes: HRESULT,
        LocaleId: LCID,
        lFlags: c_long,
        MessageText: *mut BSTR,
    ) -> HRESULT,
    fn GetFacilityCodeText(
        hRes: HRESULT,
        LocaleId: LCID,
        lFlags: c_long,
        MessageText: *mut BSTR,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemBackupRestore;
DEFINE_GUID!{IID_IWbemBackupRestore,
    0xc49e32c7, 0xbc8b, 0x11d2, 0x85, 0xd4, 0x00, 0x10, 0x5a, 0x1f, 0x83, 0x04}
RIDL!{#[uuid(0xc49e32c7, 0xbc8b, 0x11d2, 0x85, 0xd4, 0x00, 0x10, 0x5a, 0x1f, 0x83, 0x04)]
interface IWbemBackupRestore(IWbemBackupRestoreVtbl): IUnknown(IUnknownVtbl) {
    fn Backup(
        strBackupToFile: LPCWSTR,
        lFlags: c_long,
    ) -> HRESULT,
    fn Restore(
        strRestoreFromFile: LPCWSTR,
        lFlags: c_long,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemBackupRestoreEx;
DEFINE_GUID!{IID_IWbemBackupRestoreEx,
    0xa359dec5, 0xe813, 0x4834, 0x8a, 0x2a, 0xba, 0x7f, 0x1d, 0x77, 0x7d, 0x76}
RIDL!{#[uuid(0xa359dec5, 0xe813, 0x4834, 0x8a, 0x2a, 0xba, 0x7f, 0x1d, 0x77, 0x7d, 0x76)]
interface IWbemBackupRestoreEx(IWbemBackupRestoreExVtbl):
IWbemBackupRestore(IWbemBackupRestoreVtbl) {
    fn Pause() -> HRESULT,
    fn Resume() -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemRefresher;
DEFINE_GUID!{IID_IWbemRefresher,
    0x49353c99, 0x516b, 0x11d1, 0xae, 0xa6, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0x49353c99, 0x516b, 0x11d1, 0xae, 0xa6, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemRefresher(IWbemRefresherVtbl): IUnknown(IUnknownVtbl) {
    fn Refresh(
        lFlags: c_long,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemHiPerfEnum;
DEFINE_GUID!{IID_IWbemHiPerfEnum,
    0x2705c288, 0x79ae, 0x11d2, 0xb3, 0x48, 0x00, 0x10, 0x5a, 0x1f, 0x81, 0x77}
RIDL!{#[uuid(0x2705c288, 0x79ae, 0x11d2, 0xb3, 0x48, 0x00, 0x10, 0x5a, 0x1f, 0x81, 0x77)]
interface IWbemHiPerfEnum(IWbemHiPerfEnumVtbl): IUnknown(IUnknownVtbl) {
    fn AddObjects(
        lFlags: c_long,
        uNumObjects: ULONG,
        apIds: *mut c_long,
        apObj: *mut *mut IWbemObjectAccess,
    ) -> HRESULT,
    fn RemoveObjects(
        lFlags: c_long,
        uNumObjects: ULONG,
        apIds: *mut c_long,
    ) -> HRESULT,
    fn GetObjects(
        lFlags: c_long,
        uNumObjects: ULONG,
        apObj: *mut *mut IWbemObjectAccess,
        puReturned: *mut ULONG,
    ) -> HRESULT,
    fn RemoveAll(
        lFlags: c_long,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemConfigureRefresher;
DEFINE_GUID!{IID_IWbemConfigureRefresher,
    0x49353c92, 0x516b, 0x11d1, 0xae, 0xa6, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0x49353c92, 0x516b, 0x11d1, 0xae, 0xa6, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IWbemConfigureRefresher(IWbemConfigureRefresherVtbl): IUnknown(IUnknownVtbl) {
    fn AddObjectByPath(
        pNamespace: *mut IWbemServices,
        wszPath: LPCWSTR,
        lFlags: c_long,
        pContext: *mut IWbemContext,
        ppRefreshable: *mut *mut IWbemClassObject,
        plId: *mut c_long,
    ) -> HRESULT,
    fn AddObjectByTemplate(
        pNamespace: *mut IWbemServices,
        pTemplate: *mut IWbemClassObject,
        lFlags: c_long,
        pContext: *mut IWbemContext,
        ppRefreshable: *mut *mut IWbemClassObject,
        plId: *mut c_long,
    ) -> HRESULT,
    fn AddRefresher(
        pRefresher: *mut IWbemRefresher,
        lFlags: c_long,
        plId: *mut c_long,
    ) -> HRESULT,
    fn Remove(
        lId: c_long,
        lFlags: c_long,
    ) -> HRESULT,
    fn AddEnum(
        pNamespace: *mut IWbemServices,
        wszClassName: LPCWSTR,
        lFlags: c_long,
        pContext: *mut IWbemContext,
        ppEnum: *mut *mut IWbemHiPerfEnum,
        plId: *mut c_long,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_WbemLocator,
    0x4590f811, 0x1d3a, 0x11d0, 0x89, 0x1f, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24}
RIDL!{#[uuid(0x4590f811, 0x1d3a, 0x11d0, 0x89, 0x1f, 0x00, 0xaa, 0x00, 0x4b, 0x2e, 0x24)]
class WbemLocator;}
DEFINE_GUID!{CLSID_WbemContext,
    0x674B6698, 0xEE92, 0x11d0, 0xAD, 0x71, 0x00, 0xC0, 0x4F, 0xD8, 0xFD, 0xFF}
// class DECLSPEC_UUID("674B6698-EE92-11d0-AD71-00C04FD8FDFF")
// WbemContext;
DEFINE_GUID!{CLSID_UnsecuredApartment,
    0x49bd2028, 0x1523, 0x11d1, 0xad, 0x79, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
// class DECLSPEC_UUID("49bd2028-1523-11d1-ad79-00c04fd8fdff")
// UnsecuredApartment;
DEFINE_GUID!{CLSID_WbemClassObject,
    0x9A653086, 0x174F, 0x11d2, 0xB5, 0xF9, 0x00, 0x10, 0x4B, 0x70, 0x3E, 0xFD}
// class DECLSPEC_UUID("9A653086-174F-11d2-B5F9-00104B703EFD")
// WbemClassObject;
DEFINE_GUID!{CLSID_MofCompiler,
    0x6daf9757, 0x2e37, 0x11d2, 0xae, 0xc9, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
// class DECLSPEC_UUID("6daf9757-2e37-11d2-aec9-00c04fb68820")
// MofCompiler;
DEFINE_GUID!{CLSID_WbemStatusCodeText,
    0xeb87e1bd, 0x3233, 0x11d2, 0xae, 0xc9, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
// class DECLSPEC_UUID("eb87e1bd-3233-11d2-aec9-00c04fb68820")
// WbemStatusCodeText;
DEFINE_GUID!{CLSID_WbemBackupRestore,
    0xC49E32C6, 0xBC8B, 0x11d2, 0x85, 0xD4, 0x00, 0x10, 0x5A, 0x1F, 0x83, 0x04}
// class DECLSPEC_UUID("C49E32C6-BC8B-11d2-85D4-00105A1F8304")
// WbemBackupRestore;
DEFINE_GUID!{CLSID_WbemRefresher,
    0xc71566f2, 0x561e, 0x11d1, 0xad, 0x87, 0x00, 0xc0, 0x4f, 0xd8, 0xfd, 0xff}
// class DECLSPEC_UUID("c71566f2-561e-11d1-ad87-00c04fd8fdff")
// WbemRefresher;
DEFINE_GUID!{CLSID_WbemObjectTextSrc,
    0x8D1C559D, 0x84F0, 0x4bb3, 0xA7, 0xD5, 0x56, 0xA7, 0x43, 0x5A, 0x9B, 0xA6}
// class DECLSPEC_UUID("8D1C559D-84F0-4bb3-A7D5-56A7435A9BA6")
// WbemObjectTextSrc;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0001_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0001_v0_0_s_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0003_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0003_v0_0_s_ifspec;
// EXTERN_C const IID IID_IWbemObjectSinkEx;
DEFINE_GUID!{IID_IWbemObjectSinkEx,
    0xe7d35cfa, 0x348b, 0x485e, 0xb5, 0x24, 0x25, 0x27, 0x25, 0xd6, 0x97, 0xca}
RIDL!{#[uuid(0xe7d35cfa, 0x348b, 0x485e, 0xb5, 0x24, 0x25, 0x27, 0x25, 0xd6, 0x97, 0xca)]
interface IWbemObjectSinkEx(IWbemObjectSinkExVtbl): IWbemObjectSink(IWbemObjectSinkVtbl) {
    fn WriteMessage(
        uChannel: ULONG,
        strMessage: BSTR,
    ) -> HRESULT,
    fn WriteError(
        pObjError: *mut IWbemClassObject,
        puReturned: *mut c_uchar,
    ) -> HRESULT,
    fn PromptUser(
        strMessage: BSTR,
        uPromptType: c_uchar,
        puReturned: *mut c_uchar,
    ) -> HRESULT,
    fn WriteProgress(
        strActivity: BSTR,
        strCurrentOperation: BSTR,
        strStatusDescription: BSTR,
        uPercentComplete: ULONG,
        uSecondsRemaining: ULONG,
    ) -> HRESULT,
    fn WriteStreamParameter(
        strName: BSTR,
        vtValue: *mut VARIANT,
        ulType: ULONG,
        ulFlags: ULONG,
    ) -> HRESULT,
}}
// EXTERN_C const IID IID_IWbemShutdown;
DEFINE_GUID!{IID_IWbemShutdown,
    0xb7b31df9, 0xd515, 0x11d3, 0xa1, 0x1c, 0x00, 0x10, 0x5a, 0x1f, 0x51, 0x5a}
RIDL!{#[uuid(0xb7b31df9, 0xd515, 0x11d3, 0xa1, 0x1c, 0x00, 0x10, 0x5a, 0x1f, 0x51, 0x5a)]
interface IWbemShutdown(IWbemShutdownVtbl): IUnknown(IUnknownVtbl) {
    fn Shutdown(
        uReason: LONG,
        uMaxMilliseconds: ULONG,
        pCtx: *mut IWbemContext,
    ) -> HRESULT,
}}
ENUM!{enum WMI_OBJ_TEXT {
    WMI_OBJ_TEXT_CIM_DTD_2_0 = 1,
    WMI_OBJ_TEXT_WMI_DTD_2_0 = 2,
    WMI_OBJ_TEXT_WMI_EXT1 = 3,
    WMI_OBJ_TEXT_WMI_EXT2 = 4,
    WMI_OBJ_TEXT_WMI_EXT3 = 5,
    WMI_OBJ_TEXT_WMI_EXT4 = 6,
    WMI_OBJ_TEXT_WMI_EXT5 = 7,
    WMI_OBJ_TEXT_WMI_EXT6 = 8,
    WMI_OBJ_TEXT_WMI_EXT7 = 9,
    WMI_OBJ_TEXT_WMI_EXT8 = 10,
    WMI_OBJ_TEXT_WMI_EXT9 = 11,
    WMI_OBJ_TEXT_WMI_EXT10 = 12,
    WMI_OBJ_TEXT_LAST = 13,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0011_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0011_v0_0_s_ifspec;
// EXTERN_C const IID IID_IWbemObjectTextSrc;
DEFINE_GUID!{IID_IWbemObjectTextSrc,
    0xbfbf883a, 0xcad7, 0x11d3, 0xa1, 0x1b, 0x00, 0x10, 0x5a, 0x1f, 0x51, 0x5a}
RIDL!{#[uuid(0xbfbf883a, 0xcad7, 0x11d3, 0xa1, 0x1b, 0x00, 0x10, 0x5a, 0x1f, 0x51, 0x5a)]
interface IWbemObjectTextSrc(IWbemObjectTextSrcVtbl): IUnknown(IUnknownVtbl) {
    fn GetText(
        lFlags: c_long,
        pObj: *mut IWbemClassObject,
        uObjTextFormat: ULONG,
        pCtx: *mut IWbemContext,
        strText: *mut BSTR,
    ) -> HRESULT,
    fn CreateFromText(
        lFlags: c_long,
        strText: BSTR,
        uObjTextFormat: ULONG,
        pCtx: *mut IWbemContext,
        pNewObj: *mut *mut IWbemClassObject,
    ) -> HRESULT,
}}
STRUCT!{struct WBEM_COMPILE_STATUS_INFO {
    lPhaseError: c_long,
    hRes: HRESULT,
    ObjectNum: c_long,
    FirstLine: c_long,
    LastLine: c_long,
    dwOutFlags: DWORD,
}}
ENUM!{enum WBEM_COMPILER_OPTIONS {
    WBEM_FLAG_CHECK_ONLY = 0x1,
    WBEM_FLAG_AUTORECOVER = 0x2,
    WBEM_FLAG_WMI_CHECK = 0x4,
    WBEM_FLAG_CONSOLE_PRINT = 0x8,
    WBEM_FLAG_DONT_ADD_TO_LIST = 0x10,
    WBEM_FLAG_SPLIT_FILES = 0x20,
    WBEM_FLAG_STORE_FILE = 0x100,
}}
ENUM!{enum WBEM_CONNECT_OPTIONS {
    WBEM_FLAG_CONNECT_REPOSITORY_ONLY = 0x40,
    WBEM_FLAG_CONNECT_USE_MAX_WAIT = 0x80,
    WBEM_FLAG_CONNECT_PROVIDERS = 0x100,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0013_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0013_v0_0_s_ifspec;
// EXTERN_C const IID IID_IMofCompiler;
DEFINE_GUID!{IID_IMofCompiler,
    0x6daf974e, 0x2e37, 0x11d2, 0xae, 0xc9, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20}
RIDL!{#[uuid(0x6daf974e, 0x2e37, 0x11d2, 0xae, 0xc9, 0x00, 0xc0, 0x4f, 0xb6, 0x88, 0x20)]
interface IMofCompiler(IMofCompilerVtbl): IUnknown(IUnknownVtbl) {
    fn CompileFile(
        FileName: LPWSTR,
        ServerAndNamespace: LPWSTR,
        User: LPWSTR,
        Authority: LPWSTR,
        Password: LPWSTR,
        lOptionFlags: LONG,
        lClassFlags: LONG,
        lInstanceFlags: LONG,
        pInfo: *mut WBEM_COMPILE_STATUS_INFO,
    ) -> HRESULT,
    fn CompileBuffer(
        BuffSize: c_long,
        pBuffer: *mut BYTE,
        ServerAndNamespace: LPWSTR,
        User: LPWSTR,
        Authority: LPWSTR,
        Password: LPWSTR,
        lOptionFlags: LONG,
        lClassFlags: LONG,
        lInstanceFlags: LONG,
        pInfo: *mut WBEM_COMPILE_STATUS_INFO,
    ) -> HRESULT,
    fn CreateBMOF(
        TextFileName: LPWSTR,
        BMOFFileName: LPWSTR,
        ServerAndNamespace: LPWSTR,
        lOptionFlags: LONG,
        lClassFlags: LONG,
        lInstanceFlags: LONG,
        pInfo: *mut WBEM_COMPILE_STATUS_INFO,
    ) -> HRESULT,
}}
ENUM!{enum WBEM_UNSECAPP_FLAG_TYPE {
    WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS = 0,
    WBEM_FLAG_UNSECAPP_CHECK_ACCESS = 1,
    WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS = 2,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0015_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0015_v0_0_s_ifspec;
ENUM!{enum WBEM_INFORMATION_FLAG_TYPE {
    WBEM_FLAG_SHORT_NAME = 0x1,
    WBEM_FLAG_LONG_NAME = 0x2,
}}
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0016_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0016_v0_0_s_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0022_v0_0_c_ifspec;
// extern RPC_IF_HANDLE __MIDL_itf_wbemcli_0000_0022_v0_0_s_ifspec;
// unsigned c_long             __RPC_USER  BSTR_UserSize( __RPC__in unsigned c_long *, unsigned c_long            , __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void                      __RPC_USER  BSTR_UserFree( __RPC__in unsigned c_long *, __RPC__in BSTR * );
// unsigned c_long             __RPC_USER  VARIANT_UserSize( __RPC__in unsigned c_long *, unsigned c_long            , __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserMarshal( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void                      __RPC_USER  VARIANT_UserFree( __RPC__in unsigned c_long *, __RPC__in VARIANT * );
// unsigned c_long             __RPC_USER  BSTR_UserSize64( __RPC__in unsigned c_long *, unsigned c_long            , __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * );
// unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * );
// void                      __RPC_USER  BSTR_UserFree64( __RPC__in unsigned c_long *, __RPC__in BSTR * );
// unsigned c_long             __RPC_USER  VARIANT_UserSize64( __RPC__in unsigned c_long *, unsigned c_long            , __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserMarshal64( __RPC__in unsigned c_long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * );
// unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned c_long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * );
// void                      __RPC_USER  VARIANT_UserFree64( __RPC__in unsigned c_long *, __RPC__in VARIANT * );

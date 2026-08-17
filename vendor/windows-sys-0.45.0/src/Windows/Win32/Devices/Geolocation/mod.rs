pub type ICivicAddressReport = *mut ::core::ffi::c_void;
pub type ICivicAddressReportFactory = *mut ::core::ffi::c_void;
pub type IDefaultLocation = *mut ::core::ffi::c_void;
pub type IDispCivicAddressReport = *mut ::core::ffi::c_void;
pub type IDispLatLongReport = *mut ::core::ffi::c_void;
pub type ILatLongReport = *mut ::core::ffi::c_void;
pub type ILatLongReportFactory = *mut ::core::ffi::c_void;
pub type ILocation = *mut ::core::ffi::c_void;
pub type ILocationEvents = *mut ::core::ffi::c_void;
pub type ILocationPower = *mut ::core::ffi::c_void;
pub type ILocationReport = *mut ::core::ffi::c_void;
pub type ILocationReportFactory = *mut ::core::ffi::c_void;
pub type _ICivicAddressReportFactoryEvents = *mut ::core::ffi::c_void;
pub type _ILatLongReportFactoryEvents = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const BREADCRUMBING_UNSUPPORTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const BREADCRUMBING_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const CivicAddressReport: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xd39e7bdd_7d05_46b8_8721_80cf035f57d7);
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const CivicAddressReportFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a11f42c_3e81_4ad4_9cbe_45579d89671a);
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const DefaultLocation: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8b7fbfe0_5cd7_494a_af8c_283a65707506);
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const DispCivicAddressReport: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4c596aec_8544_4082_ba9f_eb0a7d8e65c6);
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const DispLatLongReport: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7a7c3277_8f84_4636_95b2_ebb5507ff77e);
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSSFORMAT_LTO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSSFORMAT_XTRA1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSSFORMAT_XTRA2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSSFORMAT_XTRA3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSSFORMAT_XTRA3_1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSSFORMAT_XTRA3_2: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSSFORMAT_XTRA_INT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_DRIVER_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_DRIVER_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_DRIVER_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_DRIVER_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_DRIVER_VERSION_5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_DRIVER_VERSION_6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_FIXDETAIL_ACCURACY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_FIXDETAIL_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_FIXDETAIL_SATELLITE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_GEOFENCESUPPORT_CIRCLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_GEOFENCESUPPORT_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_MAXSATELLITE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NMEALOGGING_ALL: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NMEALOGGING_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_OPERMODE_AFLT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_OPERMODE_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_OPERMODE_CELLID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_OPERMODE_MSA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_OPERMODE_MSB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_OPERMODE_MSS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_OPERMODE_OTDOA: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SATELLITE_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SATELLITE_BEIDOU: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SATELLITE_GALILEO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SATELLITE_GLONASS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SATELLITE_GPS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GUID_DEVINTERFACE_GNSS: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x3336e5e4_018a_4669_84c5_bd05f3bd368b);
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_CONFIG_SUPL_CERT: u32 = 2228488u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_CREATE_GEOFENCE: u32 = 2228544u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_DELETE_GEOFENCE: u32 = 2228548u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_EXECUTE_CWTEST: u32 = 2228496u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_EXECUTE_SELFTEST: u32 = 2228500u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_GET_CHIPSETINFO: u32 = 2228504u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_GET_DEVICE_CAPABILITY: u32 = 2228232u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_GET_FIXDATA: u32 = 2228300u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_INJECT_AGNSS: u32 = 2228352u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_LISTEN_AGNSS: u32 = 2228416u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_LISTEN_BREADCRUMBING_ALERT: u32 = 2228680u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_LISTEN_DRIVER_REQUEST: u32 = 2228608u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_LISTEN_ERROR: u32 = 2228420u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_LISTEN_GEOFENCES_TRACKINGSTATUS: u32 = 2228556u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_LISTEN_GEOFENCE_ALERT: u32 = 2228552u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_LISTEN_NI: u32 = 2228480u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_LISTEN_NMEA: u32 = 2228508u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_MODIFY_FIXSESSION: u32 = 2228292u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_POP_BREADCRUMBS: u32 = 2228684u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_RESPOND_NI: u32 = 2228492u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_SEND_DRIVERCOMMAND: u32 = 2228236u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_SEND_PLATFORM_CAPABILITY: u32 = 2228228u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_SET_SUPL_HSLP: u32 = 2228484u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_SET_V2UPL_CONFIG: u32 = 2228512u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_START_BREADCRUMBING: u32 = 2228672u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_START_FIXSESSION: u32 = 2228288u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_STOP_BREADCRUMBING: u32 = 2228676u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const IOCTL_GNSS_STOP_FIXSESSION: u32 = 2228296u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const LOCATION_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const LatLongReport: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xed81c073_1f84_4ca8_a161_183c776bc651);
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const LatLongReportFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x9dcc3cc8_8609_4863_bad4_03601f4c65e8);
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const Location: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe5b8e079_ee6d_4e33_a438_c87f2e959254);
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const MAX_SERVER_URL_NAME: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const MIN_BREADCRUMBS_SUPPORTED: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const MIN_GEOFENCES_REQUIRED: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_AGNSS_REQUEST_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSS_TimeInjection: GNSS_AGNSS_REQUEST_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSS_PositionInjection: GNSS_AGNSS_REQUEST_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_AGNSS_BlobInjection: GNSS_AGNSS_REQUEST_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_DRIVERCOMMAND_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SetLocationServiceEnabled: GNSS_DRIVERCOMMAND_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SetLocationNIRequestAllowed: GNSS_DRIVERCOMMAND_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_ForceSatelliteSystem: GNSS_DRIVERCOMMAND_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_ForceOperationMode: GNSS_DRIVERCOMMAND_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_ResetEngine: GNSS_DRIVERCOMMAND_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_ClearAgnssData: GNSS_DRIVERCOMMAND_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SetSuplVersion: GNSS_DRIVERCOMMAND_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SetNMEALogging: GNSS_DRIVERCOMMAND_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SetUplServerAccessInterval: GNSS_DRIVERCOMMAND_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SetNiTimeoutInterval: GNSS_DRIVERCOMMAND_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_ResetGeofencesTracking: GNSS_DRIVERCOMMAND_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_SetSuplVersion2: GNSS_DRIVERCOMMAND_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_CustomCommand: GNSS_DRIVERCOMMAND_TYPE = 256i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_DRIVER_REQUEST = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const SUPL_CONFIG_DATA: GNSS_DRIVER_REQUEST = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_FixAvailable: GNSS_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_RequireAgnss: GNSS_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_Error: GNSS_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_NiRequest: GNSS_EVENT_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_NmeaData: GNSS_EVENT_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_GeofenceAlertData: GNSS_EVENT_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_GeofencesTrackingStatus: GNSS_EVENT_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_DriverRequest: GNSS_EVENT_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_BreadcrumbAlertEvent: GNSS_EVENT_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_FixAvailable_2: GNSS_EVENT_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Event_Custom: GNSS_EVENT_TYPE = 32768i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_FIXSESSIONTYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_FixSession_SingleShot: GNSS_FIXSESSIONTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_FixSession_DistanceTracking: GNSS_FIXSESSIONTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_FixSession_ContinuousTracking: GNSS_FIXSESSIONTYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_FixSession_LKG: GNSS_FIXSESSIONTYPE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_GEOFENCE_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_GeofenceState_Unknown: GNSS_GEOFENCE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_GeofenceState_Entered: GNSS_GEOFENCE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_GeofenceState_Exited: GNSS_GEOFENCE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_GEOREGIONTYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_GeoRegion_Circle: GNSS_GEOREGIONTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_NI_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_NoNotifyNoVerify: GNSS_NI_NOTIFICATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_NotifyOnly: GNSS_NI_NOTIFICATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_NotifyVerifyDefaultAllow: GNSS_NI_NOTIFICATION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_NotifyVerifyDefaultNotAllow: GNSS_NI_NOTIFICATION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_PrivacyOverride: GNSS_NI_NOTIFICATION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_NI_PLANE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_SUPL: GNSS_NI_PLANE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_CP: GNSS_NI_PLANE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_V2UPL: GNSS_NI_PLANE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_NI_REQUEST_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_Request_SingleShot: GNSS_NI_REQUEST_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_NI_Request_AreaTrigger: GNSS_NI_REQUEST_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_NI_USER_RESPONSE = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Ni_UserResponseAccept: GNSS_NI_USER_RESPONSE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Ni_UserResponseDeny: GNSS_NI_USER_RESPONSE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Ni_UserResponseTimeout: GNSS_NI_USER_RESPONSE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type GNSS_SUPL_CERT_ACTION = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Supl_Cert_Inject: GNSS_SUPL_CERT_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Supl_Cert_Delete: GNSS_SUPL_CERT_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const GNSS_Supl_Cert_Purge: GNSS_SUPL_CERT_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub type LOCATION_REPORT_STATUS = i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const REPORT_NOT_SUPPORTED: LOCATION_REPORT_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const REPORT_ERROR: LOCATION_REPORT_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const REPORT_ACCESS_DENIED: LOCATION_REPORT_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const REPORT_INITIALIZING: LOCATION_REPORT_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub const REPORT_RUNNING: LOCATION_REPORT_STATUS = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_AGNSS_INJECT {
    pub Size: u32,
    pub Version: u32,
    pub InjectionType: GNSS_AGNSS_REQUEST_TYPE,
    pub InjectionStatus: super::super::Foundation::NTSTATUS,
    pub InjectionDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_AGNSS_INJECT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_AGNSS_INJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_AGNSS_INJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_AGNSS_INJECT_0 {
    pub Time: GNSS_AGNSS_INJECTTIME,
    pub Position: GNSS_AGNSS_INJECTPOSITION,
    pub BlobData: GNSS_AGNSS_INJECTBLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_AGNSS_INJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_AGNSS_INJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_AGNSS_INJECTBLOB {
    pub Size: u32,
    pub Version: u32,
    pub BlobOui: u32,
    pub BlobVersion: u32,
    pub AgnssFormat: u32,
    pub BlobSize: u32,
    pub BlobData: [u8; 1],
}
impl ::core::marker::Copy for GNSS_AGNSS_INJECTBLOB {}
impl ::core::clone::Clone for GNSS_AGNSS_INJECTBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_AGNSS_INJECTPOSITION {
    pub Size: u32,
    pub Version: u32,
    pub Age: u32,
    pub BasicData: GNSS_FIXDATA_BASIC,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY,
}
impl ::core::marker::Copy for GNSS_AGNSS_INJECTPOSITION {}
impl ::core::clone::Clone for GNSS_AGNSS_INJECTPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_AGNSS_INJECTTIME {
    pub Size: u32,
    pub Version: u32,
    pub UtcTime: super::super::Foundation::FILETIME,
    pub TimeUncertainty: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_AGNSS_INJECTTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_AGNSS_INJECTTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_AGNSS_REQUEST_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub RequestType: GNSS_AGNSS_REQUEST_TYPE,
    pub BlobFormat: u32,
}
impl ::core::marker::Copy for GNSS_AGNSS_REQUEST_PARAM {}
impl ::core::clone::Clone for GNSS_AGNSS_REQUEST_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_BREADCRUMBING_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_BREADCRUMBING_ALERT_DATA {}
impl ::core::clone::Clone for GNSS_BREADCRUMBING_ALERT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_BREADCRUMBING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MaximumHorizontalUncertainty: u32,
    pub MinDistanceBetweenFixes: u32,
    pub MaximumErrorTimeoutMs: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_BREADCRUMBING_PARAM {}
impl ::core::clone::Clone for GNSS_BREADCRUMBING_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_BREADCRUMB_LIST {
    pub Size: u32,
    pub Version: u32,
    pub NumCrumbs: u32,
    pub Anonymous: GNSS_BREADCRUMB_LIST_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_BREADCRUMB_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_BREADCRUMB_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_BREADCRUMB_LIST_0 {
    pub v1: [GNSS_BREADCRUMB_V1; 50],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_BREADCRUMB_LIST_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_BREADCRUMB_LIST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_BREADCRUMB_V1 {
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub Latitude: f64,
    pub Longitude: f64,
    pub HorizontalAccuracy: u32,
    pub Speed: u16,
    pub SpeedAccuracy: u16,
    pub Altitude: i16,
    pub AltitudeAccuracy: u16,
    pub Heading: i16,
    pub HeadingAccuracy: u8,
    pub FixSuccess: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_BREADCRUMB_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_BREADCRUMB_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_CHIPSETINFO {
    pub Size: u32,
    pub Version: u32,
    pub ManufacturerID: [u16; 25],
    pub HardwareID: [u16; 25],
    pub FirmwareVersion: [u16; 20],
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_CHIPSETINFO {}
impl ::core::clone::Clone for GNSS_CHIPSETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_CONTINUOUSTRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub PreferredInterval: u32,
}
impl ::core::marker::Copy for GNSS_CONTINUOUSTRACKING_PARAM {}
impl ::core::clone::Clone for GNSS_CONTINUOUSTRACKING_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_CP_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub NotificationText: [u16; 260],
}
impl ::core::marker::Copy for GNSS_CP_NI_INFO {}
impl ::core::clone::Clone for GNSS_CP_NI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_CWTESTDATA {
    pub Size: u32,
    pub Version: u32,
    pub TestResultStatus: super::super::Foundation::NTSTATUS,
    pub SignalToNoiseRatio: f64,
    pub Frequency: f64,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_CWTESTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_CWTESTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_DEVICE_CAPABILITY {
    pub Size: u32,
    pub Version: u32,
    pub SupportMultipleFixSessions: super::super::Foundation::BOOL,
    pub SupportMultipleAppSessions: super::super::Foundation::BOOL,
    pub RequireAGnssInjection: super::super::Foundation::BOOL,
    pub AgnssFormatSupported: u32,
    pub AgnssFormatPreferred: u32,
    pub SupportDistanceTracking: super::super::Foundation::BOOL,
    pub SupportContinuousTracking: super::super::Foundation::BOOL,
    pub Reserved1: u32,
    pub Reserved2: super::super::Foundation::BOOL,
    pub Reserved3: super::super::Foundation::BOOL,
    pub Reserved4: super::super::Foundation::BOOL,
    pub Reserved5: super::super::Foundation::BOOL,
    pub GeofencingSupport: u32,
    pub Reserved6: super::super::Foundation::BOOL,
    pub Reserved7: super::super::Foundation::BOOL,
    pub SupportCpLocation: super::super::Foundation::BOOL,
    pub SupportUplV2: super::super::Foundation::BOOL,
    pub SupportSuplV1: super::super::Foundation::BOOL,
    pub SupportSuplV2: super::super::Foundation::BOOL,
    pub SupportedSuplVersion: GNSS_SUPL_VERSION,
    pub MaxGeofencesSupported: u32,
    pub SupportMultipleSuplRootCert: super::super::Foundation::BOOL,
    pub GnssBreadCrumbPayloadVersion: u32,
    pub MaxGnssBreadCrumbFixes: u32,
    pub Unused: [u8; 496],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_DEVICE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_DEVICE_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_DISTANCETRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MovementThreshold: u32,
}
impl ::core::marker::Copy for GNSS_DISTANCETRACKING_PARAM {}
impl ::core::clone::Clone for GNSS_DISTANCETRACKING_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_DRIVERCOMMAND_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub CommandType: GNSS_DRIVERCOMMAND_TYPE,
    pub Reserved: u32,
    pub CommandDataSize: u32,
    pub Unused: [u8; 512],
    pub CommandData: [u8; 1],
}
impl ::core::marker::Copy for GNSS_DRIVERCOMMAND_PARAM {}
impl ::core::clone::Clone for GNSS_DRIVERCOMMAND_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_DRIVER_REQUEST_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Request: GNSS_DRIVER_REQUEST,
    pub RequestFlag: u32,
}
impl ::core::marker::Copy for GNSS_DRIVER_REQUEST_DATA {}
impl ::core::clone::Clone for GNSS_DRIVER_REQUEST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_ERRORINFO {
    pub Size: u32,
    pub Version: u32,
    pub ErrorCode: u32,
    pub IsRecoverable: super::super::Foundation::BOOL,
    pub ErrorDescription: [u16; 256],
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_ERRORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_ERRORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_EVENT {
    pub Size: u32,
    pub Version: u32,
    pub EventType: GNSS_EVENT_TYPE,
    pub EventDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_EVENT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_EVENT_0 {
    pub FixData: GNSS_FIXDATA,
    pub AgnssRequest: GNSS_AGNSS_REQUEST_PARAM,
    pub NiRequest: GNSS_NI_REQUEST_PARAM,
    pub ErrorInformation: GNSS_ERRORINFO,
    pub NmeaData: GNSS_NMEA_DATA,
    pub GeofenceAlertData: GNSS_GEOFENCE_ALERT_DATA,
    pub BreadcrumbAlertData: GNSS_BREADCRUMBING_ALERT_DATA,
    pub GeofencesTrackingStatus: GNSS_GEOFENCES_TRACKINGSTATUS_DATA,
    pub DriverRequestData: GNSS_DRIVER_REQUEST_DATA,
    pub CustomData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_EVENT_2 {
    pub Size: u32,
    pub Version: u32,
    pub EventType: GNSS_EVENT_TYPE,
    pub EventDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_EVENT_2_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_EVENT_2_0 {
    pub FixData: GNSS_FIXDATA,
    pub FixData2: GNSS_FIXDATA_2,
    pub AgnssRequest: GNSS_AGNSS_REQUEST_PARAM,
    pub NiRequest: GNSS_NI_REQUEST_PARAM,
    pub ErrorInformation: GNSS_ERRORINFO,
    pub NmeaData: GNSS_NMEA_DATA,
    pub GeofenceAlertData: GNSS_GEOFENCE_ALERT_DATA,
    pub BreadcrumbAlertData: GNSS_BREADCRUMBING_ALERT_DATA,
    pub GeofencesTrackingStatus: GNSS_GEOFENCES_TRACKINGSTATUS_DATA,
    pub DriverRequestData: GNSS_DRIVER_REQUEST_DATA,
    pub CustomData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub IsFinalFix: super::super::Foundation::BOOL,
    pub FixStatus: super::super::Foundation::NTSTATUS,
    pub FixLevelOfDetails: u32,
    pub BasicData: GNSS_FIXDATA_BASIC,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY,
    pub SatelliteData: GNSS_FIXDATA_SATELLITE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_FIXDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_FIXDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA_2 {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub IsFinalFix: super::super::Foundation::BOOL,
    pub FixStatus: super::super::Foundation::NTSTATUS,
    pub FixLevelOfDetails: u32,
    pub BasicData: GNSS_FIXDATA_BASIC_2,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY_2,
    pub SatelliteData: GNSS_FIXDATA_SATELLITE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_FIXDATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_FIXDATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_FIXDATA_ACCURACY {
    pub Size: u32,
    pub Version: u32,
    pub HorizontalAccuracy: u32,
    pub HorizontalErrorMajorAxis: u32,
    pub HorizontalErrorMinorAxis: u32,
    pub HorizontalErrorAngle: u32,
    pub HeadingAccuracy: u32,
    pub AltitudeAccuracy: u32,
    pub SpeedAccuracy: u32,
    pub HorizontalConfidence: u32,
    pub HeadingConfidence: u32,
    pub AltitudeConfidence: u32,
    pub SpeedConfidence: u32,
    pub PositionDilutionOfPrecision: f32,
    pub HorizontalDilutionOfPrecision: f32,
    pub VerticalDilutionOfPrecision: f32,
}
impl ::core::marker::Copy for GNSS_FIXDATA_ACCURACY {}
impl ::core::clone::Clone for GNSS_FIXDATA_ACCURACY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_FIXDATA_ACCURACY_2 {
    pub Size: u32,
    pub Version: u32,
    pub HorizontalAccuracy: f64,
    pub HorizontalErrorMajorAxis: f64,
    pub HorizontalErrorMinorAxis: f64,
    pub HorizontalErrorAngle: f64,
    pub HeadingAccuracy: f64,
    pub AltitudeAccuracy: f64,
    pub SpeedAccuracy: f64,
    pub HorizontalConfidence: u32,
    pub HeadingConfidence: u32,
    pub AltitudeConfidence: u32,
    pub SpeedConfidence: u32,
    pub PositionDilutionOfPrecision: f64,
    pub HorizontalDilutionOfPrecision: f64,
    pub VerticalDilutionOfPrecision: f64,
    pub GeometricDilutionOfPrecision: f64,
    pub TimeDilutionOfPrecision: f64,
}
impl ::core::marker::Copy for GNSS_FIXDATA_ACCURACY_2 {}
impl ::core::clone::Clone for GNSS_FIXDATA_ACCURACY_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_FIXDATA_BASIC {
    pub Size: u32,
    pub Version: u32,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Speed: f64,
    pub Heading: f64,
}
impl ::core::marker::Copy for GNSS_FIXDATA_BASIC {}
impl ::core::clone::Clone for GNSS_FIXDATA_BASIC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_FIXDATA_BASIC_2 {
    pub Size: u32,
    pub Version: u32,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Speed: f64,
    pub Heading: f64,
    pub AltitudeEllipsoid: f64,
}
impl ::core::marker::Copy for GNSS_FIXDATA_BASIC_2 {}
impl ::core::clone::Clone for GNSS_FIXDATA_BASIC_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA_SATELLITE {
    pub Size: u32,
    pub Version: u32,
    pub SatelliteCount: u32,
    pub SatelliteArray: [GNSS_SATELLITEINFO; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_FIXDATA_SATELLITE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_FIXDATA_SATELLITE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_FIXSESSION_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub SessionType: GNSS_FIXSESSIONTYPE,
    pub HorizontalAccuracy: u32,
    pub HorizontalConfidence: u32,
    pub Reserved: [u32; 9],
    pub FixLevelOfDetails: u32,
    pub Anonymous: GNSS_FIXSESSION_PARAM_0,
    pub Unused: [u8; 256],
}
impl ::core::marker::Copy for GNSS_FIXSESSION_PARAM {}
impl ::core::clone::Clone for GNSS_FIXSESSION_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub union GNSS_FIXSESSION_PARAM_0 {
    pub SingleShotParam: GNSS_SINGLESHOT_PARAM,
    pub DistanceParam: GNSS_DISTANCETRACKING_PARAM,
    pub ContinuousParam: GNSS_CONTINUOUSTRACKING_PARAM,
    pub LkgFixParam: GNSS_LKGFIX_PARAM,
    pub UnusedParam: [u8; 268],
}
impl ::core::marker::Copy for GNSS_FIXSESSION_PARAM_0 {}
impl ::core::clone::Clone for GNSS_FIXSESSION_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Status: super::super::Foundation::NTSTATUS,
    pub StatusTimeStamp: super::super::Foundation::FILETIME,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_GEOFENCE_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub GeofenceState: GNSS_GEOFENCE_STATE,
    pub FixBasicData: GNSS_FIXDATA_BASIC,
    pub FixAccuracyData: GNSS_FIXDATA_ACCURACY,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOFENCE_ALERT_DATA {}
impl ::core::clone::Clone for GNSS_GEOFENCE_ALERT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_GEOFENCE_CREATE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub AlertTypes: u32,
    pub InitialState: GNSS_GEOFENCE_STATE,
    pub Boundary: GNSS_GEOREGION,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOFENCE_CREATE_PARAM {}
impl ::core::clone::Clone for GNSS_GEOFENCE_CREATE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCE_CREATE_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub CreationStatus: super::super::Foundation::NTSTATUS,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_GEOFENCE_CREATE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_GEOFENCE_DELETE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOFENCE_DELETE_PARAM {}
impl ::core::clone::Clone for GNSS_GEOFENCE_DELETE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_GEOREGION {
    pub Size: u32,
    pub Version: u32,
    pub GeoRegionType: GNSS_GEOREGIONTYPE,
    pub Anonymous: GNSS_GEOREGION_0,
}
impl ::core::marker::Copy for GNSS_GEOREGION {}
impl ::core::clone::Clone for GNSS_GEOREGION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub union GNSS_GEOREGION_0 {
    pub Circle: GNSS_GEOREGION_CIRCLE,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOREGION_0 {}
impl ::core::clone::Clone for GNSS_GEOREGION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_GEOREGION_CIRCLE {
    pub Latitude: f64,
    pub Longitude: f64,
    pub RadiusInMeters: f64,
}
impl ::core::marker::Copy for GNSS_GEOREGION_CIRCLE {}
impl ::core::clone::Clone for GNSS_GEOREGION_CIRCLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_LKGFIX_PARAM {
    pub Size: u32,
    pub Version: u32,
}
impl ::core::marker::Copy for GNSS_LKGFIX_PARAM {}
impl ::core::clone::Clone for GNSS_LKGFIX_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_NI_REQUEST_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub RequestId: u32,
    pub RequestType: GNSS_NI_REQUEST_TYPE,
    pub NotificationType: GNSS_NI_NOTIFICATION_TYPE,
    pub RequestPlaneType: GNSS_NI_PLANE_TYPE,
    pub Anonymous: GNSS_NI_REQUEST_PARAM_0,
    pub ResponseTimeInSec: u32,
    pub EmergencyLocation: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_NI_REQUEST_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_NI_REQUEST_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_NI_REQUEST_PARAM_0 {
    pub SuplNiInfo: GNSS_SUPL_NI_INFO,
    pub CpNiInfo: GNSS_CP_NI_INFO,
    pub V2UplNiInfo: GNSS_V2UPL_NI_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_NI_REQUEST_PARAM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_NI_REQUEST_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_NI_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub RequestId: u32,
    pub UserResponse: GNSS_NI_USER_RESPONSE,
}
impl ::core::marker::Copy for GNSS_NI_RESPONSE {}
impl ::core::clone::Clone for GNSS_NI_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_NMEA_DATA {
    pub Size: u32,
    pub Version: u32,
    pub NmeaSentences: [super::super::Foundation::CHAR; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_NMEA_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_NMEA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_PLATFORM_CAPABILITY {
    pub Size: u32,
    pub Version: u32,
    pub SupportAgnssInjection: super::super::Foundation::BOOL,
    pub AgnssFormatSupported: u32,
    pub Unused: [u8; 516],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_PLATFORM_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_PLATFORM_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SATELLITEINFO {
    pub SatelliteId: u32,
    pub UsedInPositiong: super::super::Foundation::BOOL,
    pub Elevation: f64,
    pub Azimuth: f64,
    pub SignalToNoiseRatio: f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SATELLITEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SATELLITEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_SELFTESTCONFIG {
    pub Size: u32,
    pub Version: u32,
    pub TestType: u32,
    pub Unused: [u8; 512],
    pub InBufLen: u32,
    pub InBuffer: [u8; 1],
}
impl ::core::marker::Copy for GNSS_SELFTESTCONFIG {}
impl ::core::clone::Clone for GNSS_SELFTESTCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SELFTESTRESULT {
    pub Size: u32,
    pub Version: u32,
    pub TestResultStatus: super::super::Foundation::NTSTATUS,
    pub Result: u32,
    pub PinFailedBitMask: u32,
    pub Unused: [u8; 512],
    pub OutBufLen: u32,
    pub OutBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SELFTESTRESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SELFTESTRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_SINGLESHOT_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub ResponseTime: u32,
}
impl ::core::marker::Copy for GNSS_SINGLESHOT_PARAM {}
impl ::core::clone::Clone for GNSS_SINGLESHOT_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_STOPFIXSESSION_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_STOPFIXSESSION_PARAM {}
impl ::core::clone::Clone for GNSS_STOPFIXSESSION_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_CERT_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub CertAction: GNSS_SUPL_CERT_ACTION,
    pub SuplCertName: [super::super::Foundation::CHAR; 260],
    pub CertSize: u32,
    pub Unused: [u8; 512],
    pub CertData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SUPL_CERT_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SUPL_CERT_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_HSLP_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub SuplHslp: [super::super::Foundation::CHAR; 260],
    pub SuplHslpFromImsi: [super::super::Foundation::CHAR; 260],
    pub Reserved: u32,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SUPL_HSLP_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SUPL_HSLP_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SUPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub ClientName: [u16; 260],
    pub SuplNiUrl: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SUPL_NI_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SUPL_NI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_SUPL_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl ::core::marker::Copy for GNSS_SUPL_VERSION {}
impl ::core::clone::Clone for GNSS_SUPL_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_SUPL_VERSION_2 {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ServiceIndicator: u32,
}
impl ::core::marker::Copy for GNSS_SUPL_VERSION_2 {}
impl ::core::clone::Clone for GNSS_SUPL_VERSION_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_V2UPL_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub MPC: [super::super::Foundation::CHAR; 260],
    pub PDE: [super::super::Foundation::CHAR; 260],
    pub ApplicationTypeIndicator_MR: u8,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_V2UPL_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_V2UPL_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Geolocation\"`*"]
pub struct GNSS_V2UPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
}
impl ::core::marker::Copy for GNSS_V2UPL_NI_INFO {}
impl ::core::clone::Clone for GNSS_V2UPL_NI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}

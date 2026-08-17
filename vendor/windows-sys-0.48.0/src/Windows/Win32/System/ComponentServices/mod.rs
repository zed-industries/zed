::windows_targets::link ! ( "comsvcs.dll""system" #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"] fn CoCreateActivity ( piunknown : ::windows_sys::core::IUnknown , riid : *const ::windows_sys::core::GUID , ppobj : *mut *mut ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "comsvcs.dll""system" #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"] fn CoEnterServiceDomain ( pconfigobject : ::windows_sys::core::IUnknown ) -> ::windows_sys::core::HRESULT );
#[cfg(feature = "Win32_System_Com")]
::windows_targets::link ! ( "ole32.dll""system" #[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"] fn CoGetDefaultContext ( apttype : super::Com:: APTTYPE , riid : *const ::windows_sys::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "comsvcs.dll""system" #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"] fn CoLeaveServiceDomain ( punkstatus : ::windows_sys::core::IUnknown ) -> ( ) );
::windows_targets::link ! ( "mtxdm.dll""cdecl" #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"] fn GetDispenserManager ( param0 : *mut IDispenserManager ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "comsvcs.dll""system" #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"] fn GetManagedExtensions ( dwexts : *mut u32 ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "comsvcs.dll""system" #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"] fn MTSCreateActivity ( riid : *const ::windows_sys::core::GUID , ppobj : *mut *mut ::core::ffi::c_void ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "comsvcs.dll""cdecl" #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"] fn RecycleSurrogate ( lreasoncode : i32 ) -> ::windows_sys::core::HRESULT );
::windows_targets::link ! ( "comsvcs.dll""cdecl" #[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"] fn SafeRef ( rid : *const ::windows_sys::core::GUID , punk : ::windows_sys::core::IUnknown ) -> *mut ::core::ffi::c_void );
pub type ContextInfo = *mut ::core::ffi::c_void;
pub type ContextInfo2 = *mut ::core::ffi::c_void;
pub type IAppDomainHelper = *mut ::core::ffi::c_void;
pub type IAssemblyLocator = *mut ::core::ffi::c_void;
pub type IAsyncErrorNotify = *mut ::core::ffi::c_void;
pub type ICOMAdminCatalog = *mut ::core::ffi::c_void;
pub type ICOMAdminCatalog2 = *mut ::core::ffi::c_void;
pub type ICOMLBArguments = *mut ::core::ffi::c_void;
pub type ICatalogCollection = *mut ::core::ffi::c_void;
pub type ICatalogObject = *mut ::core::ffi::c_void;
pub type ICheckSxsConfig = *mut ::core::ffi::c_void;
pub type IComActivityEvents = *mut ::core::ffi::c_void;
pub type IComApp2Events = *mut ::core::ffi::c_void;
pub type IComAppEvents = *mut ::core::ffi::c_void;
pub type IComCRMEvents = *mut ::core::ffi::c_void;
pub type IComExceptionEvents = *mut ::core::ffi::c_void;
pub type IComIdentityEvents = *mut ::core::ffi::c_void;
pub type IComInstance2Events = *mut ::core::ffi::c_void;
pub type IComInstanceEvents = *mut ::core::ffi::c_void;
pub type IComLTxEvents = *mut ::core::ffi::c_void;
pub type IComMethod2Events = *mut ::core::ffi::c_void;
pub type IComMethodEvents = *mut ::core::ffi::c_void;
pub type IComMtaThreadPoolKnobs = *mut ::core::ffi::c_void;
pub type IComObjectConstruction2Events = *mut ::core::ffi::c_void;
pub type IComObjectConstructionEvents = *mut ::core::ffi::c_void;
pub type IComObjectEvents = *mut ::core::ffi::c_void;
pub type IComObjectPool2Events = *mut ::core::ffi::c_void;
pub type IComObjectPoolEvents = *mut ::core::ffi::c_void;
pub type IComObjectPoolEvents2 = *mut ::core::ffi::c_void;
pub type IComQCEvents = *mut ::core::ffi::c_void;
pub type IComResourceEvents = *mut ::core::ffi::c_void;
pub type IComSecurityEvents = *mut ::core::ffi::c_void;
pub type IComStaThreadPoolKnobs = *mut ::core::ffi::c_void;
pub type IComStaThreadPoolKnobs2 = *mut ::core::ffi::c_void;
pub type IComThreadEvents = *mut ::core::ffi::c_void;
pub type IComTrackingInfoCollection = *mut ::core::ffi::c_void;
pub type IComTrackingInfoEvents = *mut ::core::ffi::c_void;
pub type IComTrackingInfoObject = *mut ::core::ffi::c_void;
pub type IComTrackingInfoProperties = *mut ::core::ffi::c_void;
pub type IComTransaction2Events = *mut ::core::ffi::c_void;
pub type IComTransactionEvents = *mut ::core::ffi::c_void;
pub type IComUserEvent = *mut ::core::ffi::c_void;
pub type IContextProperties = *mut ::core::ffi::c_void;
pub type IContextSecurityPerimeter = *mut ::core::ffi::c_void;
pub type IContextState = *mut ::core::ffi::c_void;
pub type ICreateWithLocalTransaction = *mut ::core::ffi::c_void;
pub type ICreateWithTipTransactionEx = *mut ::core::ffi::c_void;
pub type ICreateWithTransactionEx = *mut ::core::ffi::c_void;
pub type ICrmCompensator = *mut ::core::ffi::c_void;
pub type ICrmCompensatorVariants = *mut ::core::ffi::c_void;
pub type ICrmFormatLogRecords = *mut ::core::ffi::c_void;
pub type ICrmLogControl = *mut ::core::ffi::c_void;
pub type ICrmMonitor = *mut ::core::ffi::c_void;
pub type ICrmMonitorClerks = *mut ::core::ffi::c_void;
pub type ICrmMonitorLogRecords = *mut ::core::ffi::c_void;
pub type IDispenserDriver = *mut ::core::ffi::c_void;
pub type IDispenserManager = *mut ::core::ffi::c_void;
pub type IEnumNames = *mut ::core::ffi::c_void;
pub type IEventServerTrace = *mut ::core::ffi::c_void;
pub type IGetAppTrackerData = *mut ::core::ffi::c_void;
pub type IGetContextProperties = *mut ::core::ffi::c_void;
pub type IGetSecurityCallContext = *mut ::core::ffi::c_void;
pub type IHolder = *mut ::core::ffi::c_void;
pub type ILBEvents = *mut ::core::ffi::c_void;
pub type IMTSActivity = *mut ::core::ffi::c_void;
pub type IMTSCall = *mut ::core::ffi::c_void;
pub type IMTSLocator = *mut ::core::ffi::c_void;
pub type IManagedActivationEvents = *mut ::core::ffi::c_void;
pub type IManagedObjectInfo = *mut ::core::ffi::c_void;
pub type IManagedPoolAction = *mut ::core::ffi::c_void;
pub type IManagedPooledObj = *mut ::core::ffi::c_void;
pub type IMessageMover = *mut ::core::ffi::c_void;
pub type IMtsEventInfo = *mut ::core::ffi::c_void;
pub type IMtsEvents = *mut ::core::ffi::c_void;
pub type IMtsGrp = *mut ::core::ffi::c_void;
pub type IObjPool = *mut ::core::ffi::c_void;
pub type IObjectConstruct = *mut ::core::ffi::c_void;
pub type IObjectConstructString = *mut ::core::ffi::c_void;
pub type IObjectContext = *mut ::core::ffi::c_void;
pub type IObjectContextActivity = *mut ::core::ffi::c_void;
pub type IObjectContextInfo = *mut ::core::ffi::c_void;
pub type IObjectContextInfo2 = *mut ::core::ffi::c_void;
pub type IObjectContextTip = *mut ::core::ffi::c_void;
pub type IObjectControl = *mut ::core::ffi::c_void;
pub type IPlaybackControl = *mut ::core::ffi::c_void;
pub type IPoolManager = *mut ::core::ffi::c_void;
pub type IProcessInitializer = *mut ::core::ffi::c_void;
pub type ISecurityCallContext = *mut ::core::ffi::c_void;
pub type ISecurityCallersColl = *mut ::core::ffi::c_void;
pub type ISecurityIdentityColl = *mut ::core::ffi::c_void;
pub type ISecurityProperty = *mut ::core::ffi::c_void;
pub type ISelectCOMLBServer = *mut ::core::ffi::c_void;
pub type ISendMethodEvents = *mut ::core::ffi::c_void;
pub type IServiceActivity = *mut ::core::ffi::c_void;
pub type IServiceCall = *mut ::core::ffi::c_void;
pub type IServiceComTIIntrinsicsConfig = *mut ::core::ffi::c_void;
pub type IServiceIISIntrinsicsConfig = *mut ::core::ffi::c_void;
pub type IServiceInheritanceConfig = *mut ::core::ffi::c_void;
pub type IServicePartitionConfig = *mut ::core::ffi::c_void;
pub type IServicePool = *mut ::core::ffi::c_void;
pub type IServicePoolConfig = *mut ::core::ffi::c_void;
pub type IServiceSxsConfig = *mut ::core::ffi::c_void;
pub type IServiceSynchronizationConfig = *mut ::core::ffi::c_void;
pub type IServiceSysTxnConfig = *mut ::core::ffi::c_void;
pub type IServiceThreadPoolConfig = *mut ::core::ffi::c_void;
pub type IServiceTrackerConfig = *mut ::core::ffi::c_void;
pub type IServiceTransactionConfig = *mut ::core::ffi::c_void;
pub type IServiceTransactionConfigBase = *mut ::core::ffi::c_void;
pub type ISharedProperty = *mut ::core::ffi::c_void;
pub type ISharedPropertyGroup = *mut ::core::ffi::c_void;
pub type ISharedPropertyGroupManager = *mut ::core::ffi::c_void;
pub type ISystemAppEventData = *mut ::core::ffi::c_void;
pub type IThreadPoolKnobs = *mut ::core::ffi::c_void;
pub type ITransactionContext = *mut ::core::ffi::c_void;
pub type ITransactionContextEx = *mut ::core::ffi::c_void;
pub type ITransactionProperty = *mut ::core::ffi::c_void;
pub type ITransactionProxy = *mut ::core::ffi::c_void;
pub type ITransactionResourcePool = *mut ::core::ffi::c_void;
pub type ITransactionStatus = *mut ::core::ffi::c_void;
pub type ITxProxyHolder = *mut ::core::ffi::c_void;
pub type ObjectContext = *mut ::core::ffi::c_void;
pub type ObjectControl = *mut ::core::ffi::c_void;
pub type SecurityProperty = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const AppDomainHelper: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xef24f689_14f8_4d92_b4af_d7b1f0e70fd4);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const ByotServerEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0aa_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCatalog: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf618c514_dfb8_11d1_a2cf_00805fc79235);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCatalogCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf618c516_dfb8_11d1_a2cf_00805fc79235);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCatalogObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf618c515_dfb8_11d1_a2cf_00805fc79235);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0ab_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMClerk: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0bd_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMRecoveryClerk: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0be_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CServiceConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0c8_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const ClrAssemblyLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x458aa3b5_265a_4b75_bc05_9bea4630cf18);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CoMTSLocator: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0ac_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const ComServiceEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0c3_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const ComSystemAppEventData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0c6_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DispenserManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0c0_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const Dummy30040732: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0a9_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const EventServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabafbc_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GUID_STRING_SIZE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GetSecurityCallContextAppObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0a8_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const LBEvents: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0c1_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const MessageMover: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0bf_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const MtsGrp: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4b2e958d_0393_11d1_b1ab_00aa00ba3258);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const PoolMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabafb5_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const SecurityCallContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0a7_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const SecurityCallers: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0a6_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const SecurityIdentity: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0a5_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const ServicePool: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0c9_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const ServicePoolConfig: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabb0ca_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const SharedProperty: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a005c05_a5de_11cf_9e66_00aa00a3f464);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const SharedPropertyGroup: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a005c0b_a5de_11cf_9e66_00aa00a3f464);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const SharedPropertyGroupManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x2a005c11_a5de_11cf_9e66_00aa00a3f464);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRACKER_INIT_EVENT: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Global\\COM+ Tracker Init Event");
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRACKER_STARTSTOP_EVENT: ::windows_sys::core::PCWSTR = ::windows_sys::core::w!("Global\\COM+ Tracker Push Event");
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TrackerServer: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xecabafb9_7f19_11d2_978e_0000f8757e2a);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TransactionContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x7999fc25_d3c6_11cf_acab_00a024a55aef);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TransactionContextEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5cb66670_d3d4_11cf_acab_00a024a55aef);
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type AutoSvcs_Error_Constants = u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = 2147803138u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = 2147803139u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = 2147803140u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = 2147803141u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = 2147803142u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = 2147803143u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = 2147803148u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = 2147803149u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = 2147803150u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = 2147803151u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = 2148599296u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = 2148599297u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = 2148599298u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = 2148599299u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = 2148599300u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrOutParam: AutoSvcs_Error_Constants = 2148599301u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = 2148599302u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = 2148599303u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = 2148599304u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = 2148599376u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = 2148599377u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = 2148599378u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = 2148599379u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = 2148599380u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = 2148599381u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = 2148599382u32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminAccessChecksLevelOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminActivationOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminActivationInproc: COMAdminActivationOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminActivationLocal: COMAdminActivationOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminApplicationExportOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminApplicationInstallOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminAuthenticationCapabilitiesOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = 64i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminAuthenticationLevelOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = 6i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminComponentFlags = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = 8i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminComponentType = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdmin32BitComponent: COMAdminComponentType = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdmin64BitComponent: COMAdminComponentType = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminErrorCodes = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = -2146368511i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = -2146368510i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = -2146368509i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = -2146368508i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = -2146368505i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = -2146368504i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = -2146368503i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBadPath: COMAdminErrorCodes = -2146368502i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = -2146368501i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRoleExists: COMAdminErrorCodes = -2146368500i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = -2146368499i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNoUser: COMAdminErrorCodes = -2146368497i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = -2146368496i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = -2146368495i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = -2146368494i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = -2146368493i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = -2146368492i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = -2146368488i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = -2146368487i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = -2146368486i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = -2146368485i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = -2146368483i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = -2146368482i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = -2146368481i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = -2146368477i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = -2146368476i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = -2146368475i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = -2146368474i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = -2146368473i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = -2146368472i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = -2146368471i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = -2146368470i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = -2146368469i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrSession: COMAdminErrorCodes = -2146368468i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = -2146368467i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = -2146368466i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = -2146368464i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrSystemApp: COMAdminErrorCodes = -2146368461i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = -2146368460i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = -2146368459i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = -2146368458i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = -2146368457i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectExists: COMAdminErrorCodes = -2146368456i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrComponentExists: COMAdminErrorCodes = -2146368455i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = -2146368453i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = -2146368452i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = -2146368450i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = -2146368449i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = -2146368442i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = -2146368441i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = -2146368440i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = -2146368439i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = -2146367998i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = -2146367480i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = -2146367479i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = -2146368438i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = -2146368437i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = -2146368436i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = -2146368435i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = -2146367478i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = -2146368434i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = -2146368433i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = -2146368432i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = -2146368425i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = -2146368423i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = -2146368421i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = -2146368398i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = -2146368397i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = -2146368396i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = -2146368395i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = -2146368384i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = -2146368383i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = -2146368382i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = -2146368381i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = -2146368380i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = -2146368379i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = -2146368378i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = -2146367473i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = -2146367471i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = -2146367470i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = -2146367469i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = -2146367477i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = -2146367463i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = -2146368431i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = -2146367460i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = -2146367459i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = -2146367458i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = -2146367456i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminFileFlags = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagCOM: COMAdminFileFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = 8i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = 64i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = 128i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = 256i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = 512i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = 1024i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = 2048i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = 4096i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = 8192i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = 16384i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = 32768i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = 65536i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = 131072i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminFileFlagError: COMAdminFileFlags = 262144i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminImpersonationLevelOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminInUse = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminNotInUse: COMAdminInUse = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByCatalog: COMAdminInUse = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminOS = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSNotInitialized: COMAdminOS = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows3_1: COMAdminOS = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows9x: COMAdminOS = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows2000: COMAdminOS = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows2000Unknown: COMAdminOS = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSUnknown: COMAdminOS = 6i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = 11i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = 12i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = 13i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = 14i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = 15i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = 17i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = 18i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = 19i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = 20i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = 21i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = 22i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7Personal: COMAdminOS = 23i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7Professional: COMAdminOS = 24i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7StandardServer: COMAdminOS = 25i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = 26i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = 27i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows7WebServer: COMAdminOS = 28i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8Personal: COMAdminOS = 29i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8Professional: COMAdminOS = 30i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8StandardServer: COMAdminOS = 31i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = 33i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindows8WebServer: COMAdminOS = 34i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = 35i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = 36i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = 37i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = 38i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = 39i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = 40i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminQCMessageAuthenticateOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminServiceOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminServiceStatusOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = 6i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = 7i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminSynchronizationOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminThreadingModels = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = 5i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminTransactionOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionNone: COMAdminTransactionOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMAdminTxIsolationLevelOptions = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type COMPLUS_APPTYPE = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = -1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const APPTYPE_SWC: COMPLUS_APPTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CRMFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CRMREGFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = 7i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_Binding = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoBinding: CSC_Binding = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_BindToPoolThread: CSC_Binding = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_COMTIIntrinsicsConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_IISIntrinsicsConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_InheritanceConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_Inherit: CSC_InheritanceConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_Ignore: CSC_InheritanceConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_PartitionConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoPartition: CSC_PartitionConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritPartition: CSC_PartitionConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewPartition: CSC_PartitionConfig = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_SxsConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoSxs: CSC_SxsConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_InheritSxs: CSC_SxsConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewSxs: CSC_SxsConfig = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_SynchronizationConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_ThreadPool = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_ThreadPoolNone: CSC_ThreadPool = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_STAThreadPool: CSC_ThreadPool = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_MTAThreadPool: CSC_ThreadPool = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_TrackerConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_DontUseTracker: CSC_TrackerConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_UseTracker: CSC_TrackerConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CSC_TransactionConfig = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NoTransaction: CSC_TransactionConfig = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const CSC_NewTransaction: CSC_TransactionConfig = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type CrmTransactionState = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Active: CrmTransactionState = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Committed: CrmTransactionState = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Aborted: CrmTransactionState = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxState_Indoubt: CrmTransactionState = 3i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type DUMPTYPE = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DUMPTYPE_FULL: DUMPTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DUMPTYPE_MINI: DUMPTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const DUMPTYPE_NONE: DUMPTYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type GetAppTrackerDataFlags = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = 4i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = 8i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = 16i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type LockModes = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const LockSetGet: LockModes = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const LockMethod: LockModes = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type ReleaseModes = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const Standard: ReleaseModes = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const Process: ReleaseModes = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type TRACKING_COLL_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub type TransactionVote = i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxCommit: TransactionVote = 0i32;
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub const TxAbort: TransactionVote = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct APPDATA {
    pub m_idApp: u32,
    pub m_szAppGuid: [u16; 40],
    pub m_dwAppProcessId: u32,
    pub m_AppStatistics: APPSTATISTICS,
}
impl ::core::marker::Copy for APPDATA {}
impl ::core::clone::Clone for APPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct APPSTATISTICS {
    pub m_cTotalCalls: u32,
    pub m_cTotalInstances: u32,
    pub m_cTotalClasses: u32,
    pub m_cCallsPerSecond: u32,
}
impl ::core::marker::Copy for APPSTATISTICS {}
impl ::core::clone::Clone for APPSTATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationProcessRecycleInfo {
    pub IsRecyclable: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
    pub TimeRecycled: super::super::Foundation::FILETIME,
    pub TimeToTerminate: super::super::Foundation::FILETIME,
    pub RecycleReasonCode: i32,
    pub IsPendingRecycle: super::super::Foundation::BOOL,
    pub HasAutomaticLifetimeRecycling: super::super::Foundation::BOOL,
    pub TimeForAutomaticRecycling: super::super::Foundation::FILETIME,
    pub MemoryLimitInKB: u32,
    pub MemoryUsageInKBLastCheck: u32,
    pub ActivationLimit: u32,
    pub NumActivationsLastReported: u32,
    pub CallLimit: u32,
    pub NumCallsLastReported: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ApplicationProcessRecycleInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ApplicationProcessRecycleInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ApplicationProcessStatistics {
    pub NumCallsOutstanding: u32,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
    pub AvgCallsPerSecond: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl ::core::marker::Copy for ApplicationProcessStatistics {}
impl ::core::clone::Clone for ApplicationProcessStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: ::windows_sys::core::GUID,
    pub ApplicationIdPrimaryApplication: ::windows_sys::core::GUID,
    pub ApplicationInstanceId: ::windows_sys::core::GUID,
    pub ProcessId: u32,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: ::windows_sys::core::PWSTR,
    pub IsService: super::super::Foundation::BOOL,
    pub IsPaused: super::super::Foundation::BOOL,
    pub IsRecycled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ApplicationProcessSummary {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ApplicationProcessSummary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: ::windows_sys::core::GUID,
    pub PartitionId: ::windows_sys::core::GUID,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: ::windows_sys::core::PWSTR,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
}
impl ::core::marker::Copy for ApplicationSummary {}
impl ::core::clone::Clone for ApplicationSummary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CLSIDDATA {
    pub m_clsid: ::windows_sys::core::GUID,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl ::core::marker::Copy for CLSIDDATA {}
impl ::core::clone::Clone for CLSIDDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct CLSIDDATA2 {
    pub m_clsid: ::windows_sys::core::GUID,
    pub m_appid: ::windows_sys::core::GUID,
    pub m_partid: ::windows_sys::core::GUID,
    pub m_pwszAppName: ::windows_sys::core::PWSTR,
    pub m_pwszCtxName: ::windows_sys::core::PWSTR,
    pub m_eAppType: COMPLUS_APPTYPE,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
impl ::core::marker::Copy for CLSIDDATA2 {}
impl ::core::clone::Clone for CLSIDDATA2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct COMSVCSEVENTINFO {
    pub cbSize: u32,
    pub dwPid: u32,
    pub lTime: i64,
    pub lMicroTime: i32,
    pub perfCount: i64,
    pub guidApp: ::windows_sys::core::GUID,
    pub sMachineName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for COMSVCSEVENTINFO {}
impl ::core::clone::Clone for COMSVCSEVENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ComponentHangMonitorInfo {
    pub IsMonitored: super::super::Foundation::BOOL,
    pub TerminateOnHang: super::super::Foundation::BOOL,
    pub AvgCallThresholdInMs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ComponentHangMonitorInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ComponentHangMonitorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ComponentStatistics {
    pub NumInstances: u32,
    pub NumBoundReferences: u32,
    pub NumPooledObjects: u32,
    pub NumObjectsInCall: u32,
    pub AvgResponseTimeInMs: u32,
    pub NumCallsCompletedRecent: u32,
    pub NumCallsFailedRecent: u32,
    pub NumCallsCompletedTotal: u32,
    pub NumCallsFailedTotal: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
impl ::core::marker::Copy for ComponentStatistics {}
impl ::core::clone::Clone for ComponentStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct ComponentSummary {
    pub ApplicationInstanceId: ::windows_sys::core::GUID,
    pub PartitionId: ::windows_sys::core::GUID,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub Clsid: ::windows_sys::core::GUID,
    pub ClassName: ::windows_sys::core::PWSTR,
    pub ApplicationName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for ComponentSummary {}
impl ::core::clone::Clone for ComponentSummary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct CrmLogRecordRead {
    pub dwCrmFlags: u32,
    pub dwSequenceNumber: u32,
    pub blobUserData: super::Com::BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for CrmLogRecordRead {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for CrmLogRecordRead {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HANG_INFO {
    pub fAppHangMonitorEnabled: super::super::Foundation::BOOL,
    pub fTerminateOnHang: super::super::Foundation::BOOL,
    pub DumpType: DUMPTYPE,
    pub dwHangTimeout: u32,
    pub dwDumpCount: u32,
    pub dwInfoMsgCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HANG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HANG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`*"]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: ::windows_sys::core::GUID,
    pub ProcessStartTime: i64,
    pub dwRecycleLifetimeLimit: u32,
    pub dwRecycleMemoryLimit: u32,
    pub dwRecycleExpirationTimeout: u32,
}
impl ::core::marker::Copy for RECYCLE_INFO {}
impl ::core::clone::Clone for RECYCLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}

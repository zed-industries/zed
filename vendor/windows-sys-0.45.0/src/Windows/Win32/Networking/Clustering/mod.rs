::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn AddClusterGroupDependency ( hdependentgroup : *const _HGROUP , hprovidergroup : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn AddClusterGroupSetDependency ( hdependentgroupset : *const _HGROUPSET , hprovidergroupset : *const _HGROUPSET ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn AddClusterGroupToGroupSetDependency ( hdependentgroup : *const _HGROUP , hprovidergroupset : *const _HGROUPSET ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn AddClusterNode ( hcluster : *const _HCLUSTER , lpsznodename : :: windows_sys::core::PCWSTR , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> *mut _HNODE );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn AddClusterNodeEx ( hcluster : *const _HCLUSTER , lpsznodename : :: windows_sys::core::PCWSTR , dwflags : u32 , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> *mut _HNODE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn AddClusterResourceDependency ( hresource : *const _HRESOURCE , hdependson : *const _HRESOURCE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn AddClusterResourceNode ( hresource : *const _HRESOURCE , hnode : *const _HNODE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn AddClusterStorageNode ( hcluster : *const _HCLUSTER , lpsznodename : :: windows_sys::core::PCWSTR , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void , lpszclusterstoragenodedescription : :: windows_sys::core::PCWSTR , lpszclusterstoragenodelocation : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn AddCrossClusterGroupSetDependency ( hdependentgroupset : *const _HGROUPSET , lpremoteclustername : :: windows_sys::core::PCWSTR , lpremotegroupsetname : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn AddResourceToClusterSharedVolumes ( hresource : *const _HRESOURCE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn BackupClusterDatabase ( hcluster : *const _HCLUSTER , lpszpathname : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CanResourceBeDependent ( hresource : *const _HRESOURCE , hresourcedependent : *const _HRESOURCE ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn CancelClusterGroupOperation ( hgroup : *const _HGROUP , dwcancelflags_reserved : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ChangeClusterResourceGroup ( hresource : *const _HRESOURCE , hgroup : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ChangeClusterResourceGroupEx ( hresource : *const _HRESOURCE , hgroup : *const _HGROUP , flags : u64 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CloseCluster ( hcluster : *const _HCLUSTER ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn CloseClusterCryptProvider ( hcluscryptprovider : *const _HCLUSCRYPTPROVIDER ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CloseClusterGroup ( hgroup : *const _HGROUP ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CloseClusterGroupSet ( hgroupset : *const _HGROUPSET ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CloseClusterNetInterface ( hnetinterface : *const _HNETINTERFACE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CloseClusterNetwork ( hnetwork : *const _HNETWORK ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CloseClusterNode ( hnode : *const _HNODE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CloseClusterNotifyPort ( hchange : *const _HCHANGE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CloseClusterResource ( hresource : *const _HRESOURCE ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusAddClusterHealthFault ( hcluster : *const _HCLUSTER , failure : *const CLUSTER_HEALTH_FAULT , param2 : u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusGetClusterHealthFaults ( hcluster : *const _HCLUSTER , objects : *mut CLUSTER_HEALTH_FAULT_ARRAY , flags : u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusRemoveClusterHealthFault ( hcluster : *const _HCLUSTER , id : :: windows_sys::core::PCWSTR , flags : u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusWorkerCheckTerminate ( lpworker : *mut CLUS_WORKER ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusWorkerCreate ( lpworker : *mut CLUS_WORKER , lpstartaddress : PWORKER_START_ROUTINE , lpparameter : *mut ::core::ffi::c_void ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusWorkerTerminate ( lpworker : *const CLUS_WORKER ) -> ( ) );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusWorkerTerminateEx ( clusworker : *mut CLUS_WORKER , timeoutinmilliseconds : u32 , waitonly : super::super::Foundation:: BOOL ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusWorkersTerminate ( clusworkers : *mut *mut CLUS_WORKER , clusworkerscount : usize , timeoutinmilliseconds : u32 , waitonly : super::super::Foundation:: BOOL ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterAddGroupToAffinityRule ( hcluster : *const _HCLUSTER , rulename : :: windows_sys::core::PCWSTR , hgroup : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterAddGroupToGroupSet ( hgroupset : *const _HGROUPSET , hgroup : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterAddGroupToGroupSetWithDomains ( hgroupset : *const _HGROUPSET , hgroup : *const _HGROUP , faultdomain : u32 , updatedomain : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterAffinityRuleControl ( hcluster : *const _HCLUSTER , affinityrulename : :: windows_sys::core::PCWSTR , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , cbinbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , cboutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterClearBackupStateForSharedVolume ( lpszvolumepathname : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterCloseEnum ( henum : *const _HCLUSENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterCloseEnumEx ( hclusterenum : *const _HCLUSENUMEX ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterControl ( hcluster : *const _HCLUSTER , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterCreateAffinityRule ( hcluster : *const _HCLUSTER , rulename : :: windows_sys::core::PCWSTR , ruletype : CLUS_AFFINITY_RULE_TYPE ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterDecrypt ( hcluscryptprovider : *const _HCLUSCRYPTPROVIDER , pcryptinput : *const u8 , cbcryptinput : u32 , ppcryptoutput : *mut *mut u8 , pcbcryptoutput : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterEncrypt ( hcluscryptprovider : *const _HCLUSCRYPTPROVIDER , pdata : *const u8 , cbdata : u32 , ppdata : *mut *mut u8 , pcbdata : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterEnum ( henum : *const _HCLUSENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterEnumEx ( hclusterenum : *const _HCLUSENUMEX , dwindex : u32 , pitem : *mut CLUSTER_ENUM_ITEM , cbitem : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGetEnumCount ( henum : *const _HCLUSENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGetEnumCountEx ( hclusterenum : *const _HCLUSENUMEX ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusterGetVolumeNameForVolumeMountPoint ( lpszvolumemountpoint : :: windows_sys::core::PCWSTR , lpszvolumename : :: windows_sys::core::PWSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusterGetVolumePathName ( lpszfilename : :: windows_sys::core::PCWSTR , lpszvolumepathname : :: windows_sys::core::PWSTR , cchbufferlength : u32 ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupCloseEnum ( hgroupenum : *const _HGROUPENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupCloseEnumEx ( hgroupenumex : *const _HGROUPENUMEX ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupControl ( hgroup : *const _HGROUP , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupEnum ( hgroupenum : *const _HGROUPENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszresourcename : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupEnumEx ( hgroupenumex : *const _HGROUPENUMEX , dwindex : u32 , pitem : *mut CLUSTER_GROUP_ENUM_ITEM , cbitem : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupGetEnumCount ( hgroupenum : *const _HGROUPENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupGetEnumCountEx ( hgroupenumex : *const _HGROUPENUMEX ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupOpenEnum ( hgroup : *const _HGROUP , dwtype : u32 ) -> *mut _HGROUPENUM );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupOpenEnumEx ( hcluster : *const _HCLUSTER , lpszproperties : :: windows_sys::core::PCWSTR , cbproperties : u32 , lpszroproperties : :: windows_sys::core::PCWSTR , cbroproperties : u32 , dwflags : u32 ) -> *mut _HGROUPENUMEX );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupSetCloseEnum ( hgroupsetenum : *mut _HGROUPSETENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupSetControl ( hgroupset : *const _HGROUPSET , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , cbinbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , cboutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupSetEnum ( hgroupsetenum : *const _HGROUPSETENUM , dwindex : u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupSetGetEnumCount ( hgroupsetenum : *mut _HGROUPSETENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterGroupSetOpenEnum ( hcluster : *mut _HCLUSTER ) -> *mut _HGROUPSETENUM );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusterIsPathOnSharedVolume ( lpszpathname : :: windows_sys::core::PCWSTR ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNetInterfaceCloseEnum ( hnetinterfaceenum : *const _HNETINTERFACEENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNetInterfaceControl ( hnetinterface : *const _HNETINTERFACE , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNetInterfaceEnum ( hnetinterfaceenum : *const _HNETINTERFACEENUM , dwindex : u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNetInterfaceOpenEnum ( hcluster : *const _HCLUSTER , lpsznodename : :: windows_sys::core::PCWSTR , lpsznetworkname : :: windows_sys::core::PCWSTR ) -> *mut _HNETINTERFACEENUM );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNetworkCloseEnum ( hnetworkenum : *const _HNETWORKENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNetworkControl ( hnetwork : *const _HNETWORK , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNetworkEnum ( hnetworkenum : *const _HNETWORKENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNetworkGetEnumCount ( hnetworkenum : *const _HNETWORKENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNetworkOpenEnum ( hnetwork : *const _HNETWORK , dwtype : u32 ) -> *mut _HNETWORKENUM );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeCloseEnum ( hnodeenum : *const _HNODEENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeCloseEnumEx ( hnodeenum : *const _HNODEENUMEX ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeControl ( hnode : *const _HNODE , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeEnum ( hnodeenum : *const _HNODEENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeEnumEx ( hnodeenum : *const _HNODEENUMEX , dwindex : u32 , pitem : *mut CLUSTER_ENUM_ITEM , cbitem : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeGetEnumCount ( hnodeenum : *const _HNODEENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeGetEnumCountEx ( hnodeenum : *const _HNODEENUMEX ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeOpenEnum ( hnode : *const _HNODE , dwtype : u32 ) -> *mut _HNODEENUM );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeOpenEnumEx ( hnode : *const _HNODE , dwtype : u32 , poptions : *const ::core::ffi::c_void ) -> *mut _HNODEENUMEX );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterNodeReplacement ( hcluster : *const _HCLUSTER , lpsznodenamecurrent : :: windows_sys::core::PCWSTR , lpsznodenamenew : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterOpenEnum ( hcluster : *const _HCLUSTER , dwtype : u32 ) -> *mut _HCLUSENUM );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterOpenEnumEx ( hcluster : *const _HCLUSTER , dwtype : u32 , poptions : *const ::core::ffi::c_void ) -> *mut _HCLUSENUMEX );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterPrepareSharedVolumeForBackup ( lpszfilename : :: windows_sys::core::PCWSTR , lpszvolumepathname : :: windows_sys::core::PWSTR , lpcchvolumepathname : *mut u32 , lpszvolumename : :: windows_sys::core::PWSTR , lpcchvolumename : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegBatchAddCommand ( hregbatch : *const _HREGBATCH , dwcommand : CLUSTER_REG_COMMAND , wzname : :: windows_sys::core::PCWSTR , dwoptions : u32 , lpdata : *const ::core::ffi::c_void , cbdata : u32 ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegBatchCloseNotification ( hbatchnotification : *const _HREGBATCHNOTIFICATION ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegBatchReadCommand ( hbatchnotification : *const _HREGBATCHNOTIFICATION , pbatchcommand : *mut CLUSTER_BATCH_COMMAND ) -> i32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusterRegCloseBatch ( hregbatch : *const _HREGBATCH , bcommit : super::super::Foundation:: BOOL , failedcommandnumber : *mut i32 ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegCloseBatchEx ( hregbatch : *const _HREGBATCH , flags : u32 , failedcommandnumber : *mut i32 ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegCloseBatchNotifyPort ( hbatchnotifyport : *const _HREGBATCHPORT ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegCloseKey ( hkey : super::super::System::Registry:: HKEY ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegCloseReadBatch ( hregreadbatch : *const _HREGREADBATCH , phregreadbatchreply : *mut *mut _HREGREADBATCHREPLY ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegCloseReadBatchEx ( hregreadbatch : *const _HREGREADBATCH , flags : u32 , phregreadbatchreply : *mut *mut _HREGREADBATCHREPLY ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegCloseReadBatchReply ( hregreadbatchreply : *const _HREGREADBATCHREPLY ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegCreateBatch ( hkey : super::super::System::Registry:: HKEY , phregbatch : *mut *mut _HREGBATCH ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegCreateBatchNotifyPort ( hkey : super::super::System::Registry:: HKEY , phbatchnotifyport : *mut *mut _HREGBATCHPORT ) -> i32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegCreateKey ( hkey : super::super::System::Registry:: HKEY , lpszsubkey : :: windows_sys::core::PCWSTR , dwoptions : u32 , samdesired : u32 , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , phkresult : *mut super::super::System::Registry:: HKEY , lpdwdisposition : *mut u32 ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegCreateReadBatch ( hkey : super::super::System::Registry:: HKEY , phregreadbatch : *mut *mut _HREGREADBATCH ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegDeleteKey ( hkey : super::super::System::Registry:: HKEY , lpszsubkey : :: windows_sys::core::PCWSTR ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegDeleteValue ( hkey : super::super::System::Registry:: HKEY , lpszvaluename : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegEnumKey ( hkey : super::super::System::Registry:: HKEY , dwindex : u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 , lpftlastwritetime : *mut super::super::Foundation:: FILETIME ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegEnumValue ( hkey : super::super::System::Registry:: HKEY , dwindex : u32 , lpszvaluename : :: windows_sys::core::PWSTR , lpcchvaluename : *mut u32 , lpdwtype : *mut u32 , lpdata : *mut u8 , lpcbdata : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegGetBatchNotification ( hbatchnotify : *const _HREGBATCHPORT , phbatchnotification : *mut *mut _HREGBATCHNOTIFICATION ) -> i32 );
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegGetKeySecurity ( hkey : super::super::System::Registry:: HKEY , requestedinformation : u32 , psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR , lpcbsecuritydescriptor : *mut u32 ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegOpenKey ( hkey : super::super::System::Registry:: HKEY , lpszsubkey : :: windows_sys::core::PCWSTR , samdesired : u32 , phkresult : *mut super::super::System::Registry:: HKEY ) -> i32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegQueryInfoKey ( hkey : super::super::System::Registry:: HKEY , lpcsubkeys : *const u32 , lpcchmaxsubkeylen : *const u32 , lpcvalues : *const u32 , lpcchmaxvaluenamelen : *const u32 , lpcbmaxvaluelen : *const u32 , lpcbsecuritydescriptor : *const u32 , lpftlastwritetime : *const super::super::Foundation:: FILETIME ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegQueryValue ( hkey : super::super::System::Registry:: HKEY , lpszvaluename : :: windows_sys::core::PCWSTR , lpdwvaluetype : *mut u32 , lpdata : *mut u8 , lpcbdata : *mut u32 ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegReadBatchAddCommand ( hregreadbatch : *const _HREGREADBATCH , wzsubkeyname : :: windows_sys::core::PCWSTR , wzvaluename : :: windows_sys::core::PCWSTR ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegReadBatchReplyNextCommand ( hregreadbatchreply : *const _HREGREADBATCHREPLY , pbatchcommand : *mut CLUSTER_READ_BATCH_COMMAND ) -> i32 );
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegSetKeySecurity ( hkey : super::super::System::Registry:: HKEY , securityinformation : u32 , psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR ) -> i32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ClusterRegSetValue ( hkey : super::super::System::Registry:: HKEY , lpszvaluename : :: windows_sys::core::PCWSTR , dwtype : u32 , lpdata : *const u8 , cbdata : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRegSyncDatabase ( hcluster : *const _HCLUSTER , flags : u32 ) -> i32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRemoveAffinityRule ( hcluster : *const _HCLUSTER , rulename : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRemoveGroupFromAffinityRule ( hcluster : *const _HCLUSTER , rulename : :: windows_sys::core::PCWSTR , hgroup : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterRemoveGroupFromGroupSet ( hgroup : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceCloseEnum ( hresenum : *const _HRESENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceCloseEnumEx ( hresourceenumex : *const _HRESENUMEX ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceControl ( hresource : *const _HRESOURCE , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , cbinbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , cboutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceControlAsUser ( hresource : *const _HRESOURCE , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , cbinbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , cboutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceEnum ( hresenum : *const _HRESENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceEnumEx ( hresourceenumex : *const _HRESENUMEX , dwindex : u32 , pitem : *mut CLUSTER_RESOURCE_ENUM_ITEM , cbitem : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceGetEnumCount ( hresenum : *const _HRESENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceGetEnumCountEx ( hresourceenumex : *const _HRESENUMEX ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceOpenEnum ( hresource : *const _HRESOURCE , dwtype : u32 ) -> *mut _HRESENUM );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceOpenEnumEx ( hcluster : *const _HCLUSTER , lpszproperties : :: windows_sys::core::PCWSTR , cbproperties : u32 , lpszroproperties : :: windows_sys::core::PCWSTR , cbroproperties : u32 , dwflags : u32 ) -> *mut _HRESENUMEX );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceTypeCloseEnum ( hrestypeenum : *const _HRESTYPEENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceTypeControl ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows_sys::core::PCWSTR , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceTypeControlAsUser ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows_sys::core::PCWSTR , hhostnode : *const _HNODE , dwcontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceTypeEnum ( hrestypeenum : *const _HRESTYPEENUM , dwindex : u32 , lpdwtype : *mut u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceTypeGetEnumCount ( hrestypeenum : *const _HRESTYPEENUM ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterResourceTypeOpenEnum ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows_sys::core::PCWSTR , dwtype : u32 ) -> *mut _HRESTYPEENUM );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterSetAccountAccess ( hcluster : *const _HCLUSTER , szaccountsid : :: windows_sys::core::PCWSTR , dwaccess : u32 , dwcontroltype : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ClusterSharedVolumeSetSnapshotState ( guidsnapshotset : :: windows_sys::core::GUID , lpszvolumename : :: windows_sys::core::PCWSTR , state : CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ClusterUpgradeFunctionalLevel ( hcluster : *const _HCLUSTER , perform : super::super::Foundation:: BOOL , pfnprogresscallback : PCLUSTER_UPGRADE_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CreateCluster ( pconfig : *const CREATE_CLUSTER_CONFIG , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> *mut _HCLUSTER );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CreateClusterAvailabilitySet ( hcluster : *const _HCLUSTER , lpavailabilitysetname : :: windows_sys::core::PCWSTR , pavailabilitysetconfig : *const CLUSTER_AVAILABILITY_SET_CONFIG ) -> *mut _HGROUPSET );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn CreateClusterGroup ( hcluster : *const _HCLUSTER , lpszgroupname : :: windows_sys::core::PCWSTR ) -> *mut _HGROUP );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn CreateClusterGroupEx ( hcluster : *const _HCLUSTER , lpszgroupname : :: windows_sys::core::PCWSTR , pgroupinfo : *const CLUSTER_CREATE_GROUP_INFO ) -> *mut _HGROUP );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn CreateClusterGroupSet ( hcluster : *const _HCLUSTER , groupsetname : :: windows_sys::core::PCWSTR ) -> *mut _HGROUPSET );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn CreateClusterNameAccount ( hcluster : *const _HCLUSTER , pconfig : *const CREATE_CLUSTER_NAME_ACCOUNT , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn CreateClusterNotifyPort ( hchange : *const _HCHANGE , hcluster : *const _HCLUSTER , dwfilter : u32 , dwnotifykey : usize ) -> *mut _HCHANGE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn CreateClusterNotifyPortV2 ( hchange : *const _HCHANGE , hcluster : *const _HCLUSTER , filters : *const NOTIFY_FILTER_AND_TYPE , dwfiltercount : u32 , dwnotifykey : usize ) -> *mut _HCHANGE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn CreateClusterResource ( hgroup : *const _HGROUP , lpszresourcename : :: windows_sys::core::PCWSTR , lpszresourcetype : :: windows_sys::core::PCWSTR , dwflags : u32 ) -> *mut _HRESOURCE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn CreateClusterResourceType ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows_sys::core::PCWSTR , lpszdisplayname : :: windows_sys::core::PCWSTR , lpszresourcetypedll : :: windows_sys::core::PCWSTR , dwlooksalivepollinterval : u32 , dwisalivepollinterval : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn DeleteClusterGroup ( hgroup : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn DeleteClusterGroupSet ( hgroupset : *const _HGROUPSET ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn DeleteClusterResource ( hresource : *const _HRESOURCE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn DeleteClusterResourceType ( hcluster : *const _HCLUSTER , lpszresourcetypename : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn DestroyCluster ( hcluster : *const _HCLUSTER , pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK , pvcallbackarg : *const ::core::ffi::c_void , fdeletevirtualcomputerobjects : super::super::Foundation:: BOOL ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn DestroyClusterGroup ( hgroup : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn DetermineCNOResTypeFromCluster ( hcluster : *const _HCLUSTER , pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn DetermineCNOResTypeFromNodelist ( cnodes : u32 , ppsznodenames : *const :: windows_sys::core::PCWSTR , pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn DetermineClusterCloudTypeFromCluster ( hcluster : *const _HCLUSTER , pcloudtype : *mut CLUSTER_CLOUD_TYPE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn DetermineClusterCloudTypeFromNodelist ( cnodes : u32 , ppsznodenames : *const :: windows_sys::core::PCWSTR , pcloudtype : *mut CLUSTER_CLOUD_TYPE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn EvictClusterNode ( hnode : *const _HNODE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn EvictClusterNodeEx ( hnode : *const _HNODE , dwtimeout : u32 , phrcleanupstatus : *mut :: windows_sys::core::HRESULT ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn FailClusterResource ( hresource : *const _HRESOURCE ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn FreeClusterCrypt ( pcryptinfo : *const ::core::ffi::c_void ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn FreeClusterHealthFault ( clusterhealthfault : *mut CLUSTER_HEALTH_FAULT ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn FreeClusterHealthFaultArray ( clusterhealthfaultarray : *mut CLUSTER_HEALTH_FAULT_ARRAY ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterFromGroup ( hgroup : *const _HGROUP ) -> *mut _HCLUSTER );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterFromNetInterface ( hnetinterface : *const _HNETINTERFACE ) -> *mut _HCLUSTER );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterFromNetwork ( hnetwork : *const _HNETWORK ) -> *mut _HCLUSTER );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterFromNode ( hnode : *const _HNODE ) -> *mut _HCLUSTER );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterFromResource ( hresource : *const _HRESOURCE ) -> *mut _HCLUSTER );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn GetClusterGroupKey ( hgroup : *const _HGROUP , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterGroupState ( hgroup : *const _HGROUP , lpsznodename : :: windows_sys::core::PWSTR , lpcchnodename : *mut u32 ) -> CLUSTER_GROUP_STATE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterInformation ( hcluster : *const _HCLUSTER , lpszclustername : :: windows_sys::core::PWSTR , lpcchclustername : *mut u32 , lpclusterinfo : *mut CLUSTERVERSIONINFO ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn GetClusterKey ( hcluster : *const _HCLUSTER , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterNetInterface ( hcluster : *const _HCLUSTER , lpsznodename : :: windows_sys::core::PCWSTR , lpsznetworkname : :: windows_sys::core::PCWSTR , lpszinterfacename : :: windows_sys::core::PWSTR , lpcchinterfacename : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn GetClusterNetInterfaceKey ( hnetinterface : *const _HNETINTERFACE , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterNetInterfaceState ( hnetinterface : *const _HNETINTERFACE ) -> CLUSTER_NETINTERFACE_STATE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterNetworkId ( hnetwork : *const _HNETWORK , lpsznetworkid : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn GetClusterNetworkKey ( hnetwork : *const _HNETWORK , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterNetworkState ( hnetwork : *const _HNETWORK ) -> CLUSTER_NETWORK_STATE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterNodeId ( hnode : *const _HNODE , lpsznodeid : :: windows_sys::core::PWSTR , lpcchname : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn GetClusterNodeKey ( hnode : *const _HNODE , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterNodeState ( hnode : *const _HNODE ) -> CLUSTER_NODE_STATE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterNotify ( hchange : *const _HCHANGE , lpdwnotifykey : *mut usize , lpdwfiltertype : *mut u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 , dwmilliseconds : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterNotifyV2 ( hchange : *const _HCHANGE , lpdwnotifykey : *mut usize , pfilterandtype : *mut NOTIFY_FILTER_AND_TYPE , buffer : *mut u8 , lpbbuffersize : *mut u32 , lpszobjectid : :: windows_sys::core::PWSTR , lpcchobjectid : *mut u32 , lpszparentid : :: windows_sys::core::PWSTR , lpcchparentid : *mut u32 , lpszname : :: windows_sys::core::PWSTR , lpcchname : *mut u32 , lpsztype : :: windows_sys::core::PWSTR , lpcchtype : *mut u32 , dwmilliseconds : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterQuorumResource ( hcluster : *const _HCLUSTER , lpszresourcename : :: windows_sys::core::PWSTR , lpcchresourcename : *mut u32 , lpszdevicename : :: windows_sys::core::PWSTR , lpcchdevicename : *mut u32 , lpdwmaxquorumlogsize : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterResourceDependencyExpression ( hresource : *const _HRESOURCE , lpszdependencyexpression : :: windows_sys::core::PWSTR , lpcchdependencyexpression : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn GetClusterResourceKey ( hresource : *const _HRESOURCE , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn GetClusterResourceNetworkName ( hresource : *const _HRESOURCE , lpbuffer : :: windows_sys::core::PWSTR , nsize : *mut u32 ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetClusterResourceState ( hresource : *const _HRESOURCE , lpsznodename : :: windows_sys::core::PWSTR , lpcchnodename : *mut u32 , lpszgroupname : :: windows_sys::core::PWSTR , lpcchgroupname : *mut u32 ) -> CLUSTER_RESOURCE_STATE );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn GetClusterResourceTypeKey ( hcluster : *const _HCLUSTER , lpsztypename : :: windows_sys::core::PCWSTR , samdesired : u32 ) -> super::super::System::Registry:: HKEY );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetNodeCloudTypeDW ( ppsznodename : :: windows_sys::core::PCWSTR , nodecloudtype : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn GetNodeClusterState ( lpsznodename : :: windows_sys::core::PCWSTR , pdwclusterstate : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn GetNotifyEventHandle ( hchange : *const _HCHANGE , lphtargetevent : *mut super::super::Foundation:: HANDLE ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn InitializeClusterHealthFault ( clusterhealthfault : *mut CLUSTER_HEALTH_FAULT ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn InitializeClusterHealthFaultArray ( clusterhealthfaultarray : *mut CLUSTER_HEALTH_FAULT_ARRAY ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn IsFileOnClusterSharedVolume ( lpszpathname : :: windows_sys::core::PCWSTR , pbfileisonsharedvolume : *mut super::super::Foundation:: BOOL ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn MoveClusterGroup ( hgroup : *const _HGROUP , hdestinationnode : *const _HNODE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn MoveClusterGroupEx ( hgroup : *const _HGROUP , hdestinationnode : *const _HNODE , dwmoveflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OfflineClusterGroup ( hgroup : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OfflineClusterGroupEx ( hgroup : *const _HGROUP , dwofflineflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OfflineClusterResource ( hresource : *const _HRESOURCE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OfflineClusterResourceEx ( hresource : *const _HRESOURCE , dwofflineflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OnlineClusterGroup ( hgroup : *const _HGROUP , hdestinationnode : *const _HNODE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OnlineClusterGroupEx ( hgroup : *const _HGROUP , hdestinationnode : *const _HNODE , dwonlineflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OnlineClusterResource ( hresource : *const _HRESOURCE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OnlineClusterResourceEx ( hresource : *const _HRESOURCE , dwonlineflags : u32 , lpinbuffer : *const u8 , cbinbuffersize : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenCluster ( lpszclustername : :: windows_sys::core::PCWSTR ) -> *mut _HCLUSTER );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterCryptProvider ( lpszresource : :: windows_sys::core::PCWSTR , lpszprovider : *const i8 , dwtype : u32 , dwflags : u32 ) -> *mut _HCLUSCRYPTPROVIDER );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterCryptProviderEx ( lpszresource : :: windows_sys::core::PCWSTR , lpszkeyname : :: windows_sys::core::PCWSTR , lpszprovider : *const i8 , dwtype : u32 , dwflags : u32 ) -> *mut _HCLUSCRYPTPROVIDER );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterEx ( lpszclustername : :: windows_sys::core::PCWSTR , desiredaccess : u32 , grantedaccess : *mut u32 ) -> *mut _HCLUSTER );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterGroup ( hcluster : *const _HCLUSTER , lpszgroupname : :: windows_sys::core::PCWSTR ) -> *mut _HGROUP );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterGroupEx ( hcluster : *const _HCLUSTER , lpszgroupname : :: windows_sys::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HGROUP );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterGroupSet ( hcluster : *const _HCLUSTER , lpszgroupsetname : :: windows_sys::core::PCWSTR ) -> *mut _HGROUPSET );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterNetInterface ( hcluster : *const _HCLUSTER , lpszinterfacename : :: windows_sys::core::PCWSTR ) -> *mut _HNETINTERFACE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterNetInterfaceEx ( hcluster : *const _HCLUSTER , lpszinterfacename : :: windows_sys::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HNETINTERFACE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterNetwork ( hcluster : *const _HCLUSTER , lpsznetworkname : :: windows_sys::core::PCWSTR ) -> *mut _HNETWORK );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterNetworkEx ( hcluster : *const _HCLUSTER , lpsznetworkname : :: windows_sys::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HNETWORK );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterNode ( hcluster : *const _HCLUSTER , lpsznodename : :: windows_sys::core::PCWSTR ) -> *mut _HNODE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterNodeById ( hcluster : *const _HCLUSTER , nodeid : u32 ) -> *mut _HNODE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterNodeEx ( hcluster : *const _HCLUSTER , lpsznodename : :: windows_sys::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HNODE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterResource ( hcluster : *const _HCLUSTER , lpszresourcename : :: windows_sys::core::PCWSTR ) -> *mut _HRESOURCE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn OpenClusterResourceEx ( hcluster : *const _HCLUSTER , lpszresourcename : :: windows_sys::core::PCWSTR , dwdesiredaccess : u32 , lpdwgrantedaccess : *mut u32 ) -> *mut _HRESOURCE );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn PauseClusterNode ( hnode : *const _HNODE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn PauseClusterNodeEx ( hnode : *const _HNODE , bdrainnode : super::super::Foundation:: BOOL , dwpauseflags : u32 , hnodedraintarget : *const _HNODE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "ntlanman.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn QueryAppInstanceVersion ( appinstanceid : *const :: windows_sys::core::GUID , instanceversionhigh : *mut u64 , instanceversionlow : *mut u64 , versionstatus : *mut super::super::Foundation:: NTSTATUS ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "ntlanman.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn RegisterAppInstance ( processhandle : super::super::Foundation:: HANDLE , appinstanceid : *const :: windows_sys::core::GUID , childreninheritappinstance : super::super::Foundation:: BOOL ) -> u32 );
::windows_sys::core::link ! ( "ntlanman.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RegisterAppInstanceVersion ( appinstanceid : *const :: windows_sys::core::GUID , instanceversionhigh : u64 , instanceversionlow : u64 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn RegisterClusterNotify ( hchange : *const _HCHANGE , dwfiltertype : u32 , hobject : super::super::Foundation:: HANDLE , dwnotifykey : usize ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn RegisterClusterNotifyV2 ( hchange : *const _HCHANGE , filter : NOTIFY_FILTER_AND_TYPE , hobject : super::super::Foundation:: HANDLE , dwnotifykey : usize ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RegisterClusterResourceTypeNotifyV2 ( hchange : *const _HCHANGE , hcluster : *const _HCLUSTER , flags : i64 , restypename : :: windows_sys::core::PCWSTR , dwnotifykey : usize ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RemoveClusterGroupDependency ( hgroup : *const _HGROUP , hdependson : *const _HGROUP ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RemoveClusterGroupSetDependency ( hgroupset : *const _HGROUPSET , hdependson : *const _HGROUPSET ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RemoveClusterGroupToGroupSetDependency ( hgroup : *const _HGROUP , hdependson : *const _HGROUPSET ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn RemoveClusterNameAccount ( hcluster : *const _HCLUSTER , bdeletecomputerobjects : super::super::Foundation:: BOOL ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RemoveClusterResourceDependency ( hresource : *const _HRESOURCE , hdependson : *const _HRESOURCE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RemoveClusterResourceNode ( hresource : *const _HRESOURCE , hnode : *const _HNODE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RemoveClusterStorageNode ( hcluster : *const _HCLUSTER , lpszclusterstorageenclosurename : :: windows_sys::core::PCWSTR , dwtimeout : u32 , dwflags : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RemoveCrossClusterGroupSetDependency ( hdependentgroupset : *const _HGROUPSET , lpremoteclustername : :: windows_sys::core::PCWSTR , lpremotegroupsetname : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RemoveResourceFromClusterSharedVolumes ( hresource : *const _HRESOURCE ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilAddUnknownProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertylist : *mut ::core::ffi::c_void , pcboutpropertylistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilCreateDirectoryTree ( pszpath : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilDupGroup ( group : *mut _HGROUP , copy : *mut *mut _HGROUP ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilDupParameterBlock ( poutparams : *mut u8 , pinparams : *const u8 , ppropertytable : *const RESUTIL_PROPERTY_ITEM ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilDupResource ( group : *mut _HRESOURCE , copy : *mut *mut _HRESOURCE ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilDupString ( pszinstring : :: windows_sys::core::PCWSTR ) -> :: windows_sys::core::PWSTR );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilEnumGroups ( hcluster : *mut _HCLUSTER , hself : *mut _HGROUP , prescallback : LPGROUP_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilEnumGroupsEx ( hcluster : *mut _HCLUSTER , hself : *mut _HGROUP , grouptype : CLUSGROUP_TYPE , prescallback : LPGROUP_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilEnumPrivateProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszoutproperties : :: windows_sys::core::PWSTR , cboutpropertiessize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilEnumProperties ( ppropertytable : *const RESUTIL_PROPERTY_ITEM , pszoutproperties : :: windows_sys::core::PWSTR , cboutpropertiessize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilEnumResources ( hself : *mut _HRESOURCE , lpszrestypename : :: windows_sys::core::PCWSTR , prescallback : LPRESOURCE_CALLBACK , pparameter : *mut ::core::ffi::c_void ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilEnumResourcesEx ( hcluster : *mut _HCLUSTER , hself : *mut _HRESOURCE , lpszrestypename : :: windows_sys::core::PCWSTR , prescallback : LPRESOURCE_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilEnumResourcesEx2 ( hcluster : *mut _HCLUSTER , hself : *mut _HRESOURCE , lpszrestypename : :: windows_sys::core::PCWSTR , prescallback : LPRESOURCE_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void , dwdesiredaccess : u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilExpandEnvironmentStrings ( pszsrc : :: windows_sys::core::PCWSTR ) -> :: windows_sys::core::PWSTR );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFindBinaryProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows_sys::core::PCWSTR , pbpropertyvalue : *mut *mut u8 , pcbpropertyvaluesize : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFindDependentDiskResourceDriveLetter ( hcluster : *const _HCLUSTER , hresource : *const _HRESOURCE , pszdriveletter : :: windows_sys::core::PWSTR , pcchdriveletter : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFindDwordProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows_sys::core::PCWSTR , pdwpropertyvalue : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFindExpandSzProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows_sys::core::PCWSTR , pszpropertyvalue : *mut :: windows_sys::core::PWSTR ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFindExpandedSzProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows_sys::core::PCWSTR , pszpropertyvalue : *mut :: windows_sys::core::PWSTR ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilFindFileTimeProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows_sys::core::PCWSTR , pftpropertyvalue : *mut super::super::Foundation:: FILETIME ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFindLongProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows_sys::core::PCWSTR , plpropertyvalue : *mut i32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFindMultiSzProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows_sys::core::PCWSTR , pszpropertyvalue : *mut :: windows_sys::core::PWSTR , pcbpropertyvaluesize : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFindSzProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows_sys::core::PCWSTR , pszpropertyvalue : *mut :: windows_sys::core::PWSTR ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFindULargeIntegerProperty ( ppropertylist : *const ::core::ffi::c_void , cbpropertylistsize : u32 , pszpropertyname : :: windows_sys::core::PCWSTR , plpropertyvalue : *mut u64 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilFreeEnvironment ( lpenvironment : *mut ::core::ffi::c_void ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilFreeParameterBlock ( poutparams : *mut u8 , pinparams : *const u8 , ppropertytable : *const RESUTIL_PROPERTY_ITEM ) -> ( ) );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetAllProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertylist : *mut ::core::ffi::c_void , cboutpropertylistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetBinaryProperty ( ppboutvalue : *mut *mut u8 , pcboutvaluesize : *mut u32 , pvaluestruct : *const CLUSPROP_BINARY , pboldvalue : *const u8 , cboldvaluesize : u32 , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetBinaryValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR , ppboutvalue : *mut *mut u8 , pcboutvaluesize : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetClusterGroupType ( hgroup : *mut _HGROUP , grouptype : *mut CLUSGROUP_TYPE ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetClusterId ( hcluster : *mut _HCLUSTER , guid : *mut :: windows_sys::core::GUID ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetClusterRoleState ( hcluster : *const _HCLUSTER , eclusterrole : CLUSTER_ROLE ) -> CLUSTER_ROLE_STATE );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetCoreClusterResources ( hcluster : *const _HCLUSTER , phclusternameresource : *mut *mut _HRESOURCE , phclusteripaddressresource : *mut *mut _HRESOURCE , phclusterquorumresource : *mut *mut _HRESOURCE ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetCoreClusterResourcesEx ( hclusterin : *const _HCLUSTER , phclusternameresourceout : *mut *mut _HRESOURCE , phclusterquorumresourceout : *mut *mut _HRESOURCE , dwdesiredaccess : u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetCoreGroup ( hcluster : *mut _HCLUSTER ) -> *mut _HGROUP );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetDwordProperty ( pdwoutvalue : *mut u32 , pvaluestruct : *const CLUSPROP_DWORD , dwoldvalue : u32 , dwminimum : u32 , dwmaximum : u32 , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetDwordValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR , pdwoutvalue : *mut u32 , dwdefaultvalue : u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetEnvironmentWithNetName ( hresource : *const _HRESOURCE ) -> *mut ::core::ffi::c_void );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilGetFileTimeProperty ( pftoutvalue : *mut super::super::Foundation:: FILETIME , pvaluestruct : *const CLUSPROP_FILETIME , ftoldvalue : super::super::Foundation:: FILETIME , ftminimum : super::super::Foundation:: FILETIME , ftmaximum : super::super::Foundation:: FILETIME , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetLongProperty ( ploutvalue : *mut i32 , pvaluestruct : *const CLUSPROP_LONG , loldvalue : i32 , lminimum : i32 , lmaximum : i32 , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetMultiSzProperty ( ppszoutvalue : *mut :: windows_sys::core::PWSTR , pcboutvaluesize : *mut u32 , pvaluestruct : *const CLUSPROP_SZ , pszoldvalue : :: windows_sys::core::PCWSTR , cboldvaluesize : u32 , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetPrivateProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , poutpropertylist : *mut ::core::ffi::c_void , cboutpropertylistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertylist : *mut ::core::ffi::c_void , cboutpropertylistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetPropertiesToParameterBlock ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutparams : *mut u8 , bcheckforrequiredproperties : super::super::Foundation:: BOOL , psznameofpropinerror : *mut :: windows_sys::core::PWSTR ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetProperty ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytableitem : *const RESUTIL_PROPERTY_ITEM , poutpropertyitem : *mut *mut ::core::ffi::c_void , pcboutpropertyitemsize : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilGetPropertyFormats ( ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertyformatlist : *mut ::core::ffi::c_void , cbpropertyformatlistsize : u32 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetPropertySize ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytableitem : *const RESUTIL_PROPERTY_ITEM , pcboutpropertylistsize : *mut u32 , pnpropertycount : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetQwordValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR , pqwoutvalue : *mut u64 , qwdefaultvalue : u64 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilGetResourceDependency ( hself : super::super::Foundation:: HANDLE , lpszresourcetype : :: windows_sys::core::PCWSTR ) -> *mut _HRESOURCE );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilGetResourceDependencyByClass ( hcluster : *mut _HCLUSTER , hself : super::super::Foundation:: HANDLE , prci : *mut CLUS_RESOURCE_CLASS_INFO , brecurse : super::super::Foundation:: BOOL ) -> *mut _HRESOURCE );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilGetResourceDependencyByClassEx ( hcluster : *mut _HCLUSTER , hself : super::super::Foundation:: HANDLE , prci : *mut CLUS_RESOURCE_CLASS_INFO , brecurse : super::super::Foundation:: BOOL , dwdesiredaccess : u32 ) -> *mut _HRESOURCE );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilGetResourceDependencyByName ( hcluster : *mut _HCLUSTER , hself : super::super::Foundation:: HANDLE , lpszresourcetype : :: windows_sys::core::PCWSTR , brecurse : super::super::Foundation:: BOOL ) -> *mut _HRESOURCE );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilGetResourceDependencyByNameEx ( hcluster : *mut _HCLUSTER , hself : super::super::Foundation:: HANDLE , lpszresourcetype : :: windows_sys::core::PCWSTR , brecurse : super::super::Foundation:: BOOL , dwdesiredaccess : u32 ) -> *mut _HRESOURCE );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilGetResourceDependencyEx ( hself : super::super::Foundation:: HANDLE , lpszresourcetype : :: windows_sys::core::PCWSTR , dwdesiredaccess : u32 ) -> *mut _HRESOURCE );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetResourceDependentIPAddressProps ( hresource : *const _HRESOURCE , pszaddress : :: windows_sys::core::PWSTR , pcchaddress : *mut u32 , pszsubnetmask : :: windows_sys::core::PWSTR , pcchsubnetmask : *mut u32 , psznetwork : :: windows_sys::core::PWSTR , pcchnetwork : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetResourceName ( hresource : *const _HRESOURCE , pszresourcename : :: windows_sys::core::PWSTR , pcchresourcenameinout : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetResourceNameDependency ( lpszresourcename : :: windows_sys::core::PCWSTR , lpszresourcetype : :: windows_sys::core::PCWSTR ) -> *mut _HRESOURCE );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetResourceNameDependencyEx ( lpszresourcename : :: windows_sys::core::PCWSTR , lpszresourcetype : :: windows_sys::core::PCWSTR , dwdesiredaccess : u32 ) -> *mut _HRESOURCE );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilGetSzProperty ( ppszoutvalue : *mut :: windows_sys::core::PWSTR , pvaluestruct : *const CLUSPROP_SZ , pszoldvalue : :: windows_sys::core::PCWSTR , pppropertylist : *mut *mut u8 , pcbpropertylistsize : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilGetSzValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR ) -> :: windows_sys::core::PWSTR );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilGroupsEqual ( hself : *mut _HGROUP , hgroup : *mut _HGROUP , pequal : *mut super::super::Foundation:: BOOL ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilIsPathValid ( pszpath : :: windows_sys::core::PCWSTR ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilIsResourceClassEqual ( prci : *mut CLUS_RESOURCE_CLASS_INFO , hresource : *mut _HRESOURCE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilLeftPaxosIsLessThanRight ( left : *const PaxosTagCStruct , right : *const PaxosTagCStruct ) -> super::super::Foundation:: BOOL );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilNodeEnum ( hcluster : *mut _HCLUSTER , pnodecallback : LPNODE_CALLBACK , pparameter : *mut ::core::ffi::c_void ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilPaxosComparer ( left : *const PaxosTagCStruct , right : *const PaxosTagCStruct ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilPropertyListFromParameterBlock ( ppropertytable : *const RESUTIL_PROPERTY_ITEM , poutpropertylist : *mut ::core::ffi::c_void , pcboutpropertylistsize : *mut u32 , pinparams : *const u8 , pcbbytesreturned : *mut u32 , pcbrequired : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilRemoveResourceServiceEnvironment ( pszservicename : :: windows_sys::core::PCWSTR , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilResourceDepEnum ( hself : *mut _HRESOURCE , enumtype : u32 , prescallback : LPRESOURCE_CALLBACK_EX , pparameter : *mut ::core::ffi::c_void ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilResourceTypesEqual ( lpszresourcetypename : :: windows_sys::core::PCWSTR , hresource : *mut _HRESOURCE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilResourcesEqual ( hself : *mut _HRESOURCE , hresource : *mut _HRESOURCE ) -> super::super::Foundation:: BOOL );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetBinaryValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR , pbnewvalue : *const u8 , cbnewvaluesize : u32 , ppboutvalue : *mut *mut u8 , pcboutvaluesize : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetDwordValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR , dwnewvalue : u32 , pdwoutvalue : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetExpandSzValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR , psznewvalue : :: windows_sys::core::PCWSTR , ppszoutstring : *mut :: windows_sys::core::PWSTR ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetMultiSzValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR , psznewvalue : :: windows_sys::core::PCWSTR , cbnewvaluesize : u32 , ppszoutvalue : *mut :: windows_sys::core::PWSTR , pcboutvaluesize : *mut u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetPrivatePropertyList ( hkeyclusterkey : super::super::System::Registry:: HKEY , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetPropertyParameterBlock ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , pinparams : *const u8 , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , poutparams : *mut u8 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetPropertyParameterBlockEx ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , pinparams : *const u8 , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , bforcewrite : super::super::Foundation:: BOOL , poutparams : *mut u8 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetPropertyTable ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , ballowunknownproperties : super::super::Foundation:: BOOL , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , poutparams : *mut u8 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetPropertyTableEx ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , ballowunknownproperties : super::super::Foundation:: BOOL , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , bforcewrite : super::super::Foundation:: BOOL , poutparams : *mut u8 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetQwordValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR , qwnewvalue : u64 , pqwoutvalue : *mut u64 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilSetResourceServiceEnvironment ( pszservicename : :: windows_sys::core::PCWSTR , hresource : *mut _HRESOURCE , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
#[cfg(feature = "Win32_Security")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"] fn ResUtilSetResourceServiceStartParameters ( pszservicename : :: windows_sys::core::PCWSTR , schscmhandle : super::super::Security:: SC_HANDLE , phservice : *mut isize , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
#[cfg(feature = "Win32_Security")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"] fn ResUtilSetResourceServiceStartParametersEx ( pszservicename : :: windows_sys::core::PCWSTR , schscmhandle : super::super::Security:: SC_HANDLE , phservice : *mut isize , dwdesiredaccess : u32 , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetSzValue ( hkeyclusterkey : super::super::System::Registry:: HKEY , pszvaluename : :: windows_sys::core::PCWSTR , psznewvalue : :: windows_sys::core::PCWSTR , ppszoutstring : *mut :: windows_sys::core::PWSTR ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetUnknownProperties ( hkeyclusterkey : super::super::System::Registry:: HKEY , ppropertytable : *const RESUTIL_PROPERTY_ITEM , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 ) -> u32 );
#[cfg(feature = "Win32_System_Registry")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"] fn ResUtilSetValueEx ( hkeyclusterkey : super::super::System::Registry:: HKEY , valuename : :: windows_sys::core::PCWSTR , valuetype : u32 , valuedata : *const u8 , valuesize : u32 , flags : u32 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilStartResourceService ( pszservicename : :: windows_sys::core::PCWSTR , phservicehandle : *mut isize ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilStopResourceService ( pszservicename : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(feature = "Win32_Security")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"] fn ResUtilStopService ( hservicehandle : super::super::Security:: SC_HANDLE ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilTerminateServiceProcessFromResDll ( dwservicepid : u32 , boffline : super::super::Foundation:: BOOL , pdwresourcestate : *mut u32 , pfnlogevent : PLOG_EVENT_ROUTINE , hresourcehandle : isize ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilVerifyPrivatePropertyList ( pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn ResUtilVerifyPropertyTable ( ppropertytable : *const RESUTIL_PROPERTY_ITEM , reserved : *mut ::core::ffi::c_void , ballowunknownproperties : super::super::Foundation:: BOOL , pinpropertylist : *const ::core::ffi::c_void , cbinpropertylistsize : u32 , poutparams : *mut u8 ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilVerifyResourceService ( pszservicename : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(feature = "Win32_Security")]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"] fn ResUtilVerifyService ( hservicehandle : super::super::Security:: SC_HANDLE ) -> u32 );
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResUtilVerifyShutdownSafe ( flags : u32 , reason : u32 , presult : *mut u32 ) -> u32 );
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
::windows_sys::core::link ! ( "resutils.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"] fn ResUtilsDeleteKeyTree ( key : super::super::System::Registry:: HKEY , keyname : :: windows_sys::core::PCWSTR , treatnokeyaserror : super::super::Foundation:: BOOL ) -> u32 );
::windows_sys::core::link ! ( "ntlanman.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResetAllAppInstanceVersions ( ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn RestartClusterResource ( hresource : *const _HRESOURCE , dwflags : u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn RestoreClusterDatabase ( lpszpathname : :: windows_sys::core::PCWSTR , bforce : super::super::Foundation:: BOOL , lpszquorumdriveletter : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResumeClusterNode ( hnode : *const _HNODE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn ResumeClusterNodeEx ( hnode : *const _HNODE , eresumefailbacktype : CLUSTER_NODE_RESUME_FAILBACK_TYPE , dwresumeflagsreserved : u32 ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "ntlanman.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn SetAppInstanceCsvFlags ( processhandle : super::super::Foundation:: HANDLE , mask : u32 , flags : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetClusterGroupName ( hgroup : *const _HGROUP , lpszgroupname : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetClusterGroupNodeList ( hgroup : *const _HGROUP , nodecount : u32 , nodelist : *const *const _HNODE ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetClusterGroupSetDependencyExpression ( hgroupset : *const _HGROUPSET , lpszdependencyexprssion : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetClusterName ( hcluster : *const _HCLUSTER , lpsznewclustername : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetClusterNetworkName ( hnetwork : *const _HNETWORK , lpszname : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetClusterNetworkPriorityOrder ( hcluster : *const _HCLUSTER , networkcount : u32 , networklist : *const *const _HNETWORK ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetClusterQuorumResource ( hresource : *const _HRESOURCE , lpszdevicename : :: windows_sys::core::PCWSTR , dwmaxquologsize : u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetClusterResourceDependencyExpression ( hresource : *const _HRESOURCE , lpszdependencyexpression : :: windows_sys::core::PCWSTR ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetClusterResourceName ( hresource : *const _HRESOURCE , lpszresourcename : :: windows_sys::core::PCWSTR ) -> u32 );
#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"] fn SetClusterServiceAccountPassword ( lpszclustername : :: windows_sys::core::PCWSTR , lpsznewpassword : :: windows_sys::core::PCWSTR , dwflags : u32 , lpreturnstatusbuffer : *mut CLUSTER_SET_PASSWORD_STATUS , lpcbreturnstatusbuffersize : *mut u32 ) -> u32 );
::windows_sys::core::link ! ( "clusapi.dll""system" #[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"] fn SetGroupDependencyExpression ( hgroup : *const _HGROUP , lpszdependencyexpression : :: windows_sys::core::PCWSTR ) -> u32 );
pub type IGetClusterDataInfo = *mut ::core::ffi::c_void;
pub type IGetClusterGroupInfo = *mut ::core::ffi::c_void;
pub type IGetClusterNetInterfaceInfo = *mut ::core::ffi::c_void;
pub type IGetClusterNetworkInfo = *mut ::core::ffi::c_void;
pub type IGetClusterNodeInfo = *mut ::core::ffi::c_void;
pub type IGetClusterObjectInfo = *mut ::core::ffi::c_void;
pub type IGetClusterResourceInfo = *mut ::core::ffi::c_void;
pub type IGetClusterUIInfo = *mut ::core::ffi::c_void;
pub type ISClusApplication = *mut ::core::ffi::c_void;
pub type ISClusCryptoKeys = *mut ::core::ffi::c_void;
pub type ISClusDisk = *mut ::core::ffi::c_void;
pub type ISClusDisks = *mut ::core::ffi::c_void;
pub type ISClusNetInterface = *mut ::core::ffi::c_void;
pub type ISClusNetInterfaces = *mut ::core::ffi::c_void;
pub type ISClusNetwork = *mut ::core::ffi::c_void;
pub type ISClusNetworkNetInterfaces = *mut ::core::ffi::c_void;
pub type ISClusNetworks = *mut ::core::ffi::c_void;
pub type ISClusNode = *mut ::core::ffi::c_void;
pub type ISClusNodeNetInterfaces = *mut ::core::ffi::c_void;
pub type ISClusNodes = *mut ::core::ffi::c_void;
pub type ISClusPartition = *mut ::core::ffi::c_void;
pub type ISClusPartitionEx = *mut ::core::ffi::c_void;
pub type ISClusPartitions = *mut ::core::ffi::c_void;
pub type ISClusProperties = *mut ::core::ffi::c_void;
pub type ISClusProperty = *mut ::core::ffi::c_void;
pub type ISClusPropertyValue = *mut ::core::ffi::c_void;
pub type ISClusPropertyValueData = *mut ::core::ffi::c_void;
pub type ISClusPropertyValues = *mut ::core::ffi::c_void;
pub type ISClusRefObject = *mut ::core::ffi::c_void;
pub type ISClusRegistryKeys = *mut ::core::ffi::c_void;
pub type ISClusResDependencies = *mut ::core::ffi::c_void;
pub type ISClusResDependents = *mut ::core::ffi::c_void;
pub type ISClusResGroup = *mut ::core::ffi::c_void;
pub type ISClusResGroupPreferredOwnerNodes = *mut ::core::ffi::c_void;
pub type ISClusResGroupResources = *mut ::core::ffi::c_void;
pub type ISClusResGroups = *mut ::core::ffi::c_void;
pub type ISClusResPossibleOwnerNodes = *mut ::core::ffi::c_void;
pub type ISClusResType = *mut ::core::ffi::c_void;
pub type ISClusResTypePossibleOwnerNodes = *mut ::core::ffi::c_void;
pub type ISClusResTypeResources = *mut ::core::ffi::c_void;
pub type ISClusResTypes = *mut ::core::ffi::c_void;
pub type ISClusResource = *mut ::core::ffi::c_void;
pub type ISClusResources = *mut ::core::ffi::c_void;
pub type ISClusScsiAddress = *mut ::core::ffi::c_void;
pub type ISClusVersion = *mut ::core::ffi::c_void;
pub type ISCluster = *mut ::core::ffi::c_void;
pub type ISClusterNames = *mut ::core::ffi::c_void;
pub type ISDomainNames = *mut ::core::ffi::c_void;
pub type IWCContextMenuCallback = *mut ::core::ffi::c_void;
pub type IWCPropertySheetCallback = *mut ::core::ffi::c_void;
pub type IWCWizard97Callback = *mut ::core::ffi::c_void;
pub type IWCWizardCallback = *mut ::core::ffi::c_void;
pub type IWEExtendContextMenu = *mut ::core::ffi::c_void;
pub type IWEExtendPropertySheet = *mut ::core::ffi::c_void;
pub type IWEExtendWizard = *mut ::core::ffi::c_void;
pub type IWEExtendWizard97 = *mut ::core::ffi::c_void;
pub type IWEInvokeCommand = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerDecrypted: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerDecrypting: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerEnabled: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerPaused: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitLockerStopped: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitlockerEncrypted: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const BitlockerEncrypting: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CA_UPGRADE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLUSTER_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GLOBAL_SHIFT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INTERNAL_SHIFT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_MODIFY_SHIFT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_USER_SHIFT: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLOUD_WITNESS_CONTAINER_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("msft-cloud-witness");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLRES_VERSION_V1_00: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLRES_VERSION_V2_00: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLRES_VERSION_V3_00: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLRES_VERSION_V4_00: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_CHANGE_ACCESS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_CHANGE_RESOURCE_GROUP_FORCE_MOVE_TO_CSV: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_FAILBACK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_HIGH_PRIORITY_START: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_IGNORE_AFFINITY_RULE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_QUEUE_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_MOVE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_ONLINE_IGNORE_AFFINITY_RULE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_GROUP_ONLINE_SYNCHRONOUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_NODE_AVOID_PLACEMENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_NODE_PAUSE_REMAIN_ON_PAUSED_NODE_ON_MOVE_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_NODE_PAUSE_RETRY_DRAIN_ON_FAILURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_NO_ACCESS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_READ_ACCESS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_FORCE_WITH_TERMINATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_DELETED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_RESTARTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_MOVING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_PREEMPTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_SHUTTING_DOWN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_USER_REQUESTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_BEST_POSSIBLE_NODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_AFFINITY_RULE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_RESOURCE_ONLINE_NECESSARY_FOR_QUORUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VALID_CHANGE_RESOURCE_GROUP_FLAGS: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_RS3: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_SERVER2008: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_SERVER2008R2: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_WINDOWS8: u32 = 1793u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_WINDOWSBLUE: u32 = 1794u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSAPI_VERSION_WINTHRESHOLD: u32 = 1795u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_ACCESS_MODE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_ACCESS_SHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CONTROL_CODE_MASK: u32 = 4194303u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_FUNCTION_SHIFT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GET_OPERATION_CONTEXT_PARAMS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_OBJECT_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_OBJECT_SHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_ADD_VOLUME_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_FILTER_BY_POOL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_INCLUDE_NON_SHARED_DISKS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGROUPSET_STATUS_APPLICATION_READY: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGROUPSET_STATUS_GROUPS_ONLINE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGROUPSET_STATUS_GROUPS_PENDING: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGROUPSET_STATUS_OS_HEARTBEAT: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_APPLICATION_READY: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_EMBEDDED_FAILURE: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_LOCKED_MODE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_NETWORK_FAILURE: u64 = 128u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_OFFLINE_DUE_TO_ANTIAFFINITY_CONFLICT: u64 = 64u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 2048u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_OS_HEARTBEAT: u64 = 512u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_PHYSICAL_RESOURCES_LACKING: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_PREEMPTED: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_UNMONITORED: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_WAITING_FOR_DEPENDENCIES: u64 = 4096u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_WAITING_IN_QUEUE_FOR_MOVE: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSGRP_STATUS_WAITING_TO_START: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_DATABASE_ISOLATE_READ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_DATABASE_SYNC_WRITE_TO_ALL_NODES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_KEYNAME_OBJECTGUIDS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ObjectGUIDs");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_AFFINITYRULE_ENABLED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Enabled");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_AFFINITYRULE_GROUPS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Groups");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_AFFINITYRULE_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_AFFINITYRULE_TYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RuleType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_ACCOUNT_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AccountName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_CONTAINER_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ContainerName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_ENDPOINT_INFO: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EndpointInfo");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_KEY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PrimaryKey");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_TOKEN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PrimaryToken");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLUS_DEFAULT_NETWORK_ROLE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DefaultNetworkRole");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLUS_DESC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CLUS_SD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Security Descriptor");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CROSS_SITE_DELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CrossSiteDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CROSS_SITE_THRESHOLD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CrossSiteThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CROSS_SUBNET_DELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CrossSubnetDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CROSS_SUBNET_THRESHOLD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CrossSubnetThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CSV_BLOCK_CACHE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("BlockCacheSize");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_CSV_MDS_SD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SharedVolumeSecurityDescriptor");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DATABASE_READ_WRITE_MODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DatabaseReadWriteMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DDA_DEVICE_ALLOCATIONS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DdaDeviceAllocations");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DHCP_BACKUP_PATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("BackupPath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DHCP_DATABASE_PATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DatabasePath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_DRAIN_ON_SHUTDOWN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DrainOnShutdown");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_ENABLED_EVENT_LOGS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EnabledEventLogs");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FAILOVER_MOVE_MIGRATION_TYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FailoverMoveMigrationType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_CA_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CATimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_HIDE_SUBDIR_SHARES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("HideSubDirShares");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_IS_DFS_ROOT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IsDfsRoot");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_MAX_USERS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MaxUsers");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_PATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Path");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_REMARK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Remark");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Security Descriptor");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SERVER_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ServerName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SHARE_FLAGS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ShareFlags");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SHARE_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ShareName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FILESHR_SHARE_SUBDIRS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ShareSubDirs");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FIXQUORUM: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FixQuorum");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FSWITNESS_ARB_DELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ArbitrationDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FSWITNESS_IMPERSONATE_CNO: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ImpersonateCNO");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FSWITNESS_SHARE_PATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SharePath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_FUNCTIONAL_LEVEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ClusterFunctionalLevel");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENAPP_COMMAND_LINE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CommandLine");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENAPP_CURRENT_DIRECTORY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CurrentDirectory");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENAPP_USE_NETWORK_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UseNetworkName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENSCRIPT_SCRIPT_FILEPATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ScriptFilepath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENSVC_SERVICE_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ServiceName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENSVC_STARTUP_PARAMS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StartupParameters");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GENSVC_USE_NETWORK_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UseNetworkName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GPUP_DEVICE_ALLOCATIONS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("GpupDeviceAllocations");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_AVAILABILITY_SET_INDEX_TO_NODE_MAPPING: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NodeDomainInfo");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_FAULT_DOMAINS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FaultDomains");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_IS_AVAILABILITY_SET: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IsAvailabilitySet");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_IS_GLOBAL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IsGlobal");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_RESERVE_NODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ReserveSpareNode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_STARTUP_COUNT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StartupCount");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_STARTUP_DELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StartupDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_STARTUP_SETTING: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StartupSetting");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_STATUS_INFORMATION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StatusInformation");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUPSET_UPDATE_DOMAINS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UpdateDomains");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GROUP_DEPENDENCY_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("GroupDependencyTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_ANTI_AFFINITY_CLASS_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AntiAffinityClassNames");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_CCF_EPOCH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CCFEpoch");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_CCF_EPOCH_HIGH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CCFEpochHigh");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_COLD_START_SETTING: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ColdStartSetting");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_DEFAULT_OWNER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DefaultOwner");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_DESC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILBACK_TYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AutoFailbackType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_END: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FailbackWindowEnd");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_START: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FailbackWindowStart");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILOVER_PERIOD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FailoverPeriod");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAILOVER_THRESHOLD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FailoverThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_FAULT_DOMAIN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FaultDomain");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_LOCK_MOVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LockedFromMoving");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_PERSISTENT_STATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PersistentState");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_PLACEMENT_OPTIONS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PlacementOptions");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_PREFERRED_SITE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PreferredSite");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_PRIORITY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Priority");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_RESILIENCY_PERIOD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ResiliencyPeriod");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_START_DELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("GroupStartDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_STATUS_INFORMATION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StatusInformation");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_TYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("GroupType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_GRP_UPDATE_DOMAIN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UpdateDomain");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IGNORE_PERSISTENT_STATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IgnorePersistentStateOnStartup");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_ADDRESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_DHCP_ADDRESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DhcpAddress");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_DHCP_SERVER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DhcpServer");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_DHCP_SUBNET_MASK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DhcpSubnetMask");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_ENABLE_DHCP: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EnableDhcp");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_ENABLE_NETBIOS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EnableNetBIOS");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_LEASE_OBTAINED_TIME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LeaseObtainedTime");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_LEASE_TERMINATES_TIME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LeaseExpiresTime");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_NETWORK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Network");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_OVERRIDE_ADDRMATCH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("OverrideAddressMatch");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_PROBE_FAILURE_THRESHOLD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProbeFailureThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_PROBE_PORT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ProbePort");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_SHARED_NETNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SharedNetname");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_SUBNET_MASK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SubnetMask");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_T1: ::windows_sys::core::PCWSTR = ::windows_sys::w!("T1");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPADDR_T2: ::windows_sys::core::PCWSTR = ::windows_sys::w!("T2");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_NATIVE_ADDRESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_NATIVE_NETWORK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Network");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_NATIVE_PREFIX_LENGTH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PrefixLength");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_TUNNEL_ADDRESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_IPV6_TUNNEL_TUNNELTYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TunnelType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_LAST_RECENT_EVENTS_RESET_TIME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RecentEventsResetTime");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_LOG_FILE_PATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LogFilePath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_MESSAGE_BUFFER_LENGTH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MessageBufferLength");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_MIXED_MODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MixedMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETFT_IPSEC_ENABLED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NetftIPSecEnabled");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_ADAPTER_ID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AdapterId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_ADAPTER_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Adapter");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_ADDRESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_DESC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_DHCP_ENABLED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DhcpEnabled");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_IPV4_ADDRESSES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IPv4Addresses");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_IPV6_ADDRESSES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IPv6Addresses");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_NETWORK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Network");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETIFACE_NODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Node");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_AD_AWARE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ADAware");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_ALIASES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Aliases");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_CONTAINERGUID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CryptoContainerGUID");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_CREATING_DC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CreatingDC");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_DNN_DISABLE_CLONES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DisableClones");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_DNS_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DnsName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_DNS_SUFFIX: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DnsSuffix");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_EXCLUDE_NETWORKS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ExcludeNetworks");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_HOST_TTL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("HostRecordTTL");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_IN_USE_NETWORKS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("InUseNetworks");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_LAST_DNS_UPDATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LastDNSUpdateTime");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_OBJECT_ID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ObjectGUID");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_PUBLISH_PTR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PublishPTRRecords");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_REGISTER_ALL_IP: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RegisterAllProvidersIP");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_REMAP_PIPE_NAMES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RemapPipeNames");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_REMOVEVCO_ONDELETE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DeleteVcoOnResCleanup");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_RESOURCE_DATA: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ResourceData");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_STATUS_DNS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StatusDNS");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_STATUS_KERBEROS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StatusKerberos");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_STATUS_NETBIOS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StatusNetBIOS");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NETNAME_VCO_CONTAINER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VcoContainer");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_ADDRESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_ADDRESS_MASK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AddressMask");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_AUTOMETRIC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AutoMetric");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_DESC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_IPV4_ADDRESSES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IPv4Addresses");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_IPV4_PREFIXLENGTHS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IPv4PrefixLengths");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_IPV6_ADDRESSES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IPv6Addresses");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_IPV6_PREFIXLENGTHS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IPv6PrefixLengths");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_METRIC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Metric");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_RDMA_CAPABLE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RdmaCapable");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_ROLE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Role");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_RSS_CAPABLE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RssCapable");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NET_SPEED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LinkSpeed");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_BUILD_NUMBER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("BuildNumber");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_CSDVERSION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CSDVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_DESC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_DRAIN_STATUS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NodeDrainStatus");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_DRAIN_TARGET: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NodeDrainTarget");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_DYNAMIC_WEIGHT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DynamicWeight");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_FAULT_DOMAIN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FaultDomain");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_FDID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FaultDomainId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_HIGHEST_VERSION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NodeHighestVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_IS_PRIMARY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IsPrimary");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_LOWEST_VERSION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NodeLowestVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_MAJOR_VERSION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MajorVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_MANUFACTURER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Manufacturer");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_MINOR_VERSION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MinorVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_MODEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Model");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NodeName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_NEEDS_PQ: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NeedsPreventQuorum");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_SERIALNUMBER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SerialNumber");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_STATUS_INFO: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StatusInformation");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_NODE_WEIGHT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NodeWeight");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_CSVBLOCKCACHE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EnableBlockCache");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTAGELIMIT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SnapshotAgeLimit");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTDIFFAREASIZE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SnapshotDiffSize");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_CSVWRITETHROUGH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CsvEnforceWriteThrough");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKARBINTERVAL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskArbInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKARBTYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskArbType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKGUID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskGuid");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKIDGUID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskIdGuid");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKIDTYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskIdType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKIODELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MaxIoLatency");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKPATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskPath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKRECOVERYACTION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskRecoveryAction");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKRELOAD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskReload");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKRUNCHKDSK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskRunChkDsk");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKSIGNATURE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskSignature");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKUNIQUEIDS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskUniqueIds");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_DISKVOLUMEINFO: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DiskVolumeInfo");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_FASTONLINEARBITRATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("FastOnlineArbitrate");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_MAINTMODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MaintenanceMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_MIGRATEFIXUP: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MigrateDriveLetters");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_SPACEIDGUID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualDiskId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PHYSDISK_VOLSNAPACTIVATETIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VolsnapActivateTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PLACEMENT_OPTIONS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PlacementOptions");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PLUMB_ALL_CROSS_SUBNET_ROUTES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PlumbAllCrossSubnetRoutes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PREVENTQUORUM: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PreventQuorum");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PRTSPOOL_DEFAULT_SPOOL_DIR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DefaultSpoolDirectory");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_PRTSPOOL_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("JobCompletionTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_QUARANTINE_DURATION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("QuarantineDuration");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_QUARANTINE_THRESHOLD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("QuarantineThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_QUORUM_ARBITRATION_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("QuorumArbitrationTimeMax");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESILIENCY_DEFAULT_SECONDS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ResiliencyDefaultPeriod");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESILIENCY_LEVEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ResiliencyLevel");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_ADMIN_EXTENSIONS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AdminExtensions");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DEADLOCK_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DeadlockTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DESC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DLL_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DllName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DUMP_LOG_QUERY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DumpLogQuery");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DUMP_POLICY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DumpPolicy");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_DUMP_SERVICES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DumpServices");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_ENABLED_EVENT_LOGS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EnabledEventLogs");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_IS_ALIVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IsAlivePollInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_LOOKS_ALIVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LooksAlivePollInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_MAX_MONITORS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MaximumMonitors");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_PENDING_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PendingTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_WPR_PROFILES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WprProfiles");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RESTYPE_WPR_START_AFTER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WprStartAfter");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_DATA1: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ResourceSpecificData1");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_DATA2: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ResourceSpecificData2");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_DEADLOCK_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DeadlockTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_DESC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_EMBEDDED_FAILURE_ACTION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EmbeddedFailureAction");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_IS_ALIVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IsAlivePollInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_LAST_OPERATION_STATUS_CODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LastOperationStatusCode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_LOOKS_ALIVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LooksAlivePollInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_MONITOR_PID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MonitorProcessId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_PENDING_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PendingTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_PERSISTENT_STATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PersistentState");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RESTART_ACTION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RestartAction");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RESTART_DELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RestartDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RESTART_PERIOD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RestartPeriod");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RESTART_THRESHOLD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RestartThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_RETRY_PERIOD_ON_FAILURE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RetryPeriodOnFailure");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_SEPARATE_MONITOR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SeparateMonitor");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_STATUS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ResourceSpecificStatus");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_STATUS_INFORMATION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StatusInformation");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_RES_TYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Type");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_ROUTE_HISTORY_LENGTH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RouteHistoryLength");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_SAME_SUBNET_DELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SameSubnetDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_SAME_SUBNET_THRESHOLD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SameSubnetThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_SHUTDOWN_TIMEOUT_MINUTES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ShutdownTimeoutInMinutes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_SOFS_SMBASYMMETRYMODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SmbAsymmetryMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_START_MEMORY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("StartMemory");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_DESCRIPTION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualDiskDescription");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_HEALTH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualDiskHealth");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualDiskName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLARBITRATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Arbitrate");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLCONSUMEDCAPACITY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ConsumedCapacity");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLDESC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLDRIVEIDS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DriveIds");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLHEALTH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Health");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLIDGUID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PoolId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMSHARE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PoolQuorumShare");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMUSERACCOUNT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PoolQuorumUserAccount");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLREEVALTIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ReEvaluatePlacementTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLSTATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("State");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_POOLTOTALCAPACITY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TotalCapacity");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_PROVISIONING: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualDiskProvisioning");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYCOLUMNS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualDiskResiliencyColumns");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYINTERLEAVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualDiskResiliencyInterleave");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYTYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualDiskResiliencyType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_STORAGESPACE_STATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualDiskState");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_UPGRADE_VERSION: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ClusterUpgradeVersion");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_ADAPTER_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AdapterName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_ADDRESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_PREFIX_LENGTH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PrefixLength");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_RDID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RDID");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIP_VSID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VSID");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VIRTUAL_NUMA_COUNT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("VirtualNumaCount");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VSSTASK_APPNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ApplicationName");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VSSTASK_APPPARAMS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ApplicationParams");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VSSTASK_CURRENTDIRECTORY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("CurrentDirectory");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_VSSTASK_TRIGGERARRAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TriggerArray");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_WINS_BACKUP_PATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("BackupPath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_WINS_DATABASE_PATH: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DatabasePath");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_NAME_WITNESS_DYNAMIC_WEIGHT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WitnessDynamicWeight");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_DO_NOT_COLLECT_WER_REPORT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_DUMP_NOW: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_INSUFFICIENT_MEMORY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_INSUFFICIENT_OTHER_RESOURCES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_INSUFFICIENT_PROCESSOR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_INVALID_PARAMETERS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_NETWORK_NOT_AVAILABLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_BUSY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_REJECTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_THROTTLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_SOURCE_THROTTLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_NAME_GET_OPERATION_CONTEXT_FLAGS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Flags");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_APPLICATION_READY: u64 = 256u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_EMBEDDED_FAILURE: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_CPU: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_GENERIC_RESOURCES: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_MEMORY: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_LOCKED_MODE: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_NETWORK_FAILURE: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 512u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_OS_HEARTBEAT: u64 = 128u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSRES_STATUS_UNMONITORED: u64 = 64u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ADD_EVICT_DELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AddEvictDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_AVAILABILITY_SET_CONFIG_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CONFIGURED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CREATE_GROUP_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CREATE_GROUP_INFO_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CSA_VSS_STATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("BackupInProgress");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CSV_COMPATIBLE_FILTERS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SharedVolumeCompatibleFilters");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CSV_INCOMPATIBLE_FILTERS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SharedVolumeIncompatibleFilters");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_DELETE_ACCESS_CONTROL_ENTRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENFORCED_ANTIAFFINITY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ClusterEnforcedAntiaffinity");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_ITEM_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_ITEM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_WAIT_DELAY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ClusterGroupWaitDelay");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HANG_RECOVERY_ACTION_KEYNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("HangRecoveryAction");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HANG_TIMEOUT_KEYNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ClusSvcHangTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ARGS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION_LABEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Description");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORCODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORCODE_LABEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ErrorCode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE_LABEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ErrorType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_FLAGS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_FLAGS_LABEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Flags");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_ID_LABEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Id");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_PROPERTY_NAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ClusterHealth");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_PROVIDER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_PROVIDER_LABEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Provider");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_RESERVED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_HEALTH_FAULT_RESERVED_LABEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Reserved");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NAME_AUTO_BALANCER_LEVEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AutoBalancerLevel");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NAME_AUTO_BALANCER_MODE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("AutoBalancerMode");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NAME_PREFERRED_SITE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("PreferredSite");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_REQUEST_REPLY_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("RequestReplyTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RUNNING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_BUS_TYPES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("S2DBusTypes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_BEHAVIOR_FLAGS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("S2DCacheBehavior");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_DESIRED_STATE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("S2DCacheDesiredState");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_FLASH_RESERVE_PERCENT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("S2DCacheFlashReservePercent");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_METADATA_RESERVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("S2DCacheMetadataReserveBytes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_CACHE_PAGE_SIZE_KBYTES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("S2DCachePageSizeKBytes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_ENABLED: ::windows_sys::core::PCWSTR = ::windows_sys::w!("S2DEnabled");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_IO_LATENCY_THRESHOLD: ::windows_sys::core::PCWSTR = ::windows_sys::w!("S2DIOLatencyThreshold");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_S2D_OPTIMIZATIONS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("S2DOptimizations");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_SET_ACCESS_TYPE_ALLOWED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_SET_ACCESS_TYPE_DENIED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_SHARED_VOLUMES_ROOT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SharedVolumesRoot");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_SHARED_VOLUME_VSS_WRITER_OPERATION_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SharedVolumeVssWriterOperationTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_VERSION_FLAG_MIXED_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_VERSION_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_WITNESS_DATABASE_WRITE_TIMEOUT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WitnessDatabaseWriteTimeout");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_WITNESS_FAILED_RESTART_INTERVAL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WitnessRestartInterval");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_ACCESS_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_ACCESS_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_ACCESS_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CREATE_CRYPT_CONTAINER_NOT_FOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GRP_MOVE_ALLOWED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GRP_MOVE_LOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_HYBRID_QUORUM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_MODIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_CLUSTER_GROUPID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ClusterGroupId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_DATA_RESID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DataResourceId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_LOG_MULTIPLE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LogSizeMultiple");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_LOG_RESID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LogResourceId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_LOG_VOLUME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("LogVolume");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_MINIMUM_LOG_SIZE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MinimumLogSizeInBytes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUPID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ReplicationGroupId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUP_TYPE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ReplicationClusterGroupType");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_SOURCE_RESID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SourceResourceId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_SOURCE_VOLUMES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SourceVolumes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_TARGET_RESID: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TargetResourceId");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_TARGET_VOLUMES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("TargetVolumes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NAME_RES_TYPE_UNIT_LOG_SIZE_CHANGE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UnitOfLogSizeChangeInBytes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NODE_MAJORITY_QUORUM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NOT_GLOBAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_NO_MODIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_DUE_TO_EMBEDDED_FAILURE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_QUEUE_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_RETURNING_TO_SOURCE_NODE_BECAUSE_OF_ERROR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OFFLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_RECOVER_MONITOR_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_RESTORE_ONLINE_STATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_ONLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OPEN_DONT_DELETE_TEMP_DISK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESDLL_OPEN_RECOVER_MONITOR_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_CAU: ::windows_sys::core::PCWSTR = ::windows_sys::w!("ClusterAwareUpdatingResource");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_CLOUD_WITNESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Cloud Witness");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_CONTAINER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Container");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_CROSS_CLUSTER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Cross Cluster Dependency Orchestrator");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_DFS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Distributed File System");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_DFSR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DFS Replicated Folder");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_DHCP: ::windows_sys::core::PCWSTR = ::windows_sys::w!("DHCP Service");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_DNN: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Distributed Network Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_FILESERVER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("File Server");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_FILESHR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("File Share");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_FSWITNESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("File Share Witness");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_GENAPP: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Generic Application");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_GENSCRIPT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Generic Script");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_GENSVC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Generic Service");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_HARDDISK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Physical Disk");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_HCSVM: ::windows_sys::core::PCWSTR = ::windows_sys::w!("HCS Virtual Machine");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_HEALTH_SERVICE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Health Service");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_IPADDR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IP Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_IPV6_NATIVE: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IPv6 Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_IPV6_TUNNEL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("IPv6 Tunnel Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_ISCSITARGET: ::windows_sys::core::PCWSTR = ::windows_sys::w!("iSCSI Target Server");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_ISNS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Microsoft iSNS");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_MSDTC: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Distributed Transaction Coordinator");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_MSMQ: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Microsoft Message Queue Server");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_MSMQ_TRIGGER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSMQTriggers");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NAT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Nat");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NETNAME: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Network Name");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NETWORK_FILE_SYSTEM: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Network File System");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NEW_MSMQ: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MSMQ");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NFS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NFS Share");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NFS_MSNS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("NFS Multi Server Namespace");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NFS_V2: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Network File System");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_NV_PROVIDER_ADDRESS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Provider Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_PHYS_DISK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Physical Disk");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_PRTSPLR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Print Spooler");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_SCALEOUT_MASTER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Scaleout Master");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_SCALEOUT_WORKER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Scaleout Worker");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_SDDC_MANAGEMENT: ::windows_sys::core::PCWSTR = ::windows_sys::w!("SDDC Management");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_SODAFILESERVER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Scale Out File Server");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_STORAGE_POLICIES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Storage Policies");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_STORAGE_POOL: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Storage Pool");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_STORAGE_REPLICA: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Storage Replica");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_STORQOS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Storage QoS Policy Manager");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_TASKSCHEDULER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Task Scheduler");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV4: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Disjoint IPv4 Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV6: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Disjoint IPv6 Address");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VM: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Virtual Machine");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VMREPLICA_BROKER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Virtual Machine Replication Broker");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VMREPLICA_COORDINATOR: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Virtual Machine Replication Coordinator");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VM_CONFIG: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Virtual Machine Configuration");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VM_WMI: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Virtual Machine Cluster WMI");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_VSSTASK: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Volume Shadow Copy Service Task");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESTYPE_NAME_WINS: ::windows_sys::core::PCWSTR = ::windows_sys::w!("WINS Service");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RES_NAME_SCALEOUT_MASTER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Scaleout Master");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RES_NAME_SCALEOUT_WORKER: ::windows_sys::core::PCWSTR = ::windows_sys::w!("Scaleout Worker");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CREATEDC_PRESENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CREATE_CLUSTER_MAJOR_VERSION_MASK: u32 = 4294967040u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CREATE_CLUSTER_VERSION: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606e5_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusCryptoKeys: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e6072b_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusDisk: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60723_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusDisks: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60725_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetInterface: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606ed_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetInterfaces: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606ef_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetwork: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606f1_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetworkNetInterfaces: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606f5_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNetworks: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606f3_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNode: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606f7_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNodeNetInterfaces: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606fb_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusNodes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606f9_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPartition: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e6071f_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPartitionEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x53d51d26_b51b_4a79_b2c3_5048d93a98fc);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPartitions: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60721_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606ff_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusProperty: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606fd_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPropertyValue: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60719_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPropertyValueData: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e6071d_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusPropertyValues: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e6071b_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusRefObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60701_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusRegistryKeys: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60729_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResDependencies: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60703_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResDependents: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e6072d_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResGroup: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60705_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResGroupPreferredOwnerNodes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606e7_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResGroupResources: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606e9_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResGroups: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60707_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResPossibleOwnerNodes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e6070d_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResType: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e6070f_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResTypePossibleOwnerNodes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60717_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResTypeResources: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60713_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResTypes: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60711_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResource: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60709_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusResources: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e6070b_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusScsiAddress: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60727_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusVersion: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e60715_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const Cluster: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606e3_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNames: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606eb_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const DNS_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const DomainNames: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xf2e606e1_2631_11d1_89f1_00a0c90d061e);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ENABLE_CLUSTER_SHARED_VOLUMES: ::windows_sys::core::PCWSTR = ::windows_sys::w!("EnableSharedVolumes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FE_UPGRADE_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUPSET_READY_SETTING_APPLICATION_READY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUPSET_READY_SETTING_DELAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUPSET_READY_SETTING_ONLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUPSET_READY_SETTING_OS_HEARTBEAT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GROUP_FAILURE_INFO_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GUID_PRESENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const HCI_UPGRADE_BIT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOCKED_MODE_FLAGS_DONT_REMOVE_FROM_MOVE_QUEUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAINTENANCE_MODE_V2_SIG: u32 = 2881155087u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CLUSTERNAME_LENGTH: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CO_PASSWORD_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CO_PASSWORD_LENGTHEX: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CO_PASSWORD_STORAGEEX: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_CREATINGDC_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MAX_OBJECTID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MINIMUM_NEVER_PREEMPT_PRIORITY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MinimumNeverPreemptPriority");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MINIMUM_PREEMPTOR_PRIORITY: ::windows_sys::core::PCWSTR = ::windows_sys::w!("MinimumPreemptorPriority");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MN_UPGRADE_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NINETEEN_H1_UPGRADE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NINETEEN_H2_UPGRADE_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NNLEN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT10_MAJOR_VERSION: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT11_MAJOR_VERSION: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT12_MAJOR_VERSION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT13_MAJOR_VERSION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT4SP4_MAJOR_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT4_MAJOR_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT51_MAJOR_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT5_MAJOR_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT6_MAJOR_VERSION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT7_MAJOR_VERSION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT8_MAJOR_VERSION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NT9_MAJOR_VERSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESOURCE_FAILURE_INFO_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESTYPE_MONITOR_SHUTTING_DOWN_CLUSSVC_CRASH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESTYPE_MONITOR_SHUTTING_DOWN_NODE_STOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESUTIL_PROPITEM_IN_MEMORY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESUTIL_PROPITEM_READ_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESUTIL_PROPITEM_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RESUTIL_PROPITEM_SIGNED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RS3_UPGRADE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RS4_UPGRADE_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RS5_UPGRADE_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonBitLockerInitializing: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonFileSystemTiering: u64 = 8u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonMax: u64 = 9223372036854775808u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonReFs: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonUnsafeFileSystemFilter: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonUnsafeVolumeFilter: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RedirectedIOReasonUserRequest: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SET_APPINSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SR_REPLICATED_PARTITION_DISALLOW_MULTINODE_IO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const STARTUP_EX_ROUTINE: ::windows_sys::core::PCSTR = ::windows_sys::s!("StartupEx");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const STARTUP_ROUTINE: ::windows_sys::core::PCSTR = ::windows_sys::s!("Startup");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const USE_CLIENT_ACCESS_NETWORKS_FOR_CSV: ::windows_sys::core::PCWSTR = ::windows_sys::w!("UseClientAccessNetworksForSharedVolumes");
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeRedirectedIOReasonMax: u64 = 9223372036854775808u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeRedirectedIOReasonNoDiskConnectivity: u64 = 1u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeRedirectedIOReasonStorageSpaceNotAttached: u64 = 2u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeRedirectedIOReasonVolumeReplicationEnabled: u64 = 4u64;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const WS2016_RTM_UPGRADE_VERSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const WS2016_TP4_UPGRADE_VERSION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const WS2016_TP5_UPGRADE_VERSION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLCTL_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_UNKNOWN: CLCTL_CODES = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CHARACTERISTICS: CLCTL_CODES = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_FLAGS: CLCTL_CODES = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CLASS_INFO: CLCTL_CODES = 13i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_REQUIRED_DEPENDENCIES: CLCTL_CODES = 17i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_ARB_TIMEOUT: CLCTL_CODES = 21i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_FAILURE_INFO: CLCTL_CODES = 25i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NAME: CLCTL_CODES = 41i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_RESOURCE_TYPE: CLCTL_CODES = 45i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NODE: CLCTL_CODES = 49i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NETWORK: CLCTL_CODES = 53i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_ID: CLCTL_CODES = 57i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_FQDN: CLCTL_CODES = 61i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLCTL_CODES = 65i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CHECK_VOTER_EVICT: CLCTL_CODES = 69i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CHECK_VOTER_DOWN: CLCTL_CODES = 73i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SHUTDOWN: CLCTL_CODES = 77i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ENUM_COMMON_PROPERTIES: CLCTL_CODES = 81i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_RO_COMMON_PROPERTIES: CLCTL_CODES = 85i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_COMMON_PROPERTIES: CLCTL_CODES = 89i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_COMMON_PROPERTIES: CLCTL_CODES = 4194398i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_COMMON_PROPERTIES: CLCTL_CODES = 97i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_COMMON_PROPERTY_FMTS: CLCTL_CODES = 101i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = 105i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ENUM_PRIVATE_PROPERTIES: CLCTL_CODES = 121i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_RO_PRIVATE_PROPERTIES: CLCTL_CODES = 125i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_PRIVATE_PROPERTIES: CLCTL_CODES = 129i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_PRIVATE_PROPERTIES: CLCTL_CODES = 4194438i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_PRIVATE_PROPERTIES: CLCTL_CODES = 137i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_PRIVATE_PROPERTY_FMTS: CLCTL_CODES = 141i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = 145i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_REGISTRY_CHECKPOINT: CLCTL_CODES = 4194466i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_DELETE_REGISTRY_CHECKPOINT: CLCTL_CODES = 4194470i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_REGISTRY_CHECKPOINTS: CLCTL_CODES = 169i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_CRYPTO_CHECKPOINT: CLCTL_CODES = 4194478i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_DELETE_CRYPTO_CHECKPOINT: CLCTL_CODES = 4194482i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CRYPTO_CHECKPOINTS: CLCTL_CODES = 181i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RESOURCE_UPGRADE_DLL: CLCTL_CODES = 4194490i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT: CLCTL_CODES = 4194494i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT: CLCTL_CODES = 4194498i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_LOADBAL_PROCESS_LIST: CLCTL_CODES = 201i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_ACCOUNT_ACCESS: CLCTL_CODES = 4194546i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NETWORK_NAME: CLCTL_CODES = 361i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLCTL_CODES = 365i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_REGISTER_DNS_RECORDS: CLCTL_CODES = 370i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_DNS_NAME: CLCTL_CODES = 373i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_SET_PWD_INFO: CLCTL_CODES = 378i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_DELETE_CO: CLCTL_CODES = 382i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_VALIDATE_VCO: CLCTL_CODES = 385i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_RESET_VCO: CLCTL_CODES = 389i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_REPAIR_VCO: CLCTL_CODES = 397i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISK_INFO: CLCTL_CODES = 401i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS: CLCTL_CODES = 405i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_IS_PATH_VALID: CLCTL_CODES = 409i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_SYNC_CLUSDISK_DB: CLCTL_CODES = 4194718i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISK_NUMBER_INFO: CLCTL_CODES = 417i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_QUERY_DELETE: CLCTL_CODES = 441i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IPADDRESS_RENEW_LEASE: CLCTL_CODES = 4194750i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IPADDRESS_RELEASE_LEASE: CLCTL_CODES = 4194754i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_QUERY_MAINTENANCE_MODE: CLCTL_CODES = 481i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_MAINTENANCE_MODE: CLCTL_CODES = 4194790i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_SET_DRIVELETTER: CLCTL_CODES = 4194794i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DRIVELETTERS: CLCTL_CODES = 493i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISK_INFO_EX: CLCTL_CODES = 497i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX: CLCTL_CODES = 501i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISK_INFO_EX2: CLCTL_CODES = 505i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_CLUSPORT_DISK_COUNT: CLCTL_CODES = 509i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_REMAP_DRIVELETTER: CLCTL_CODES = 513i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DISKID: CLCTL_CODES = 517i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_IS_CLUSTERABLE: CLCTL_CODES = 521i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_REMOVE_VM_OWNERSHIP: CLCTL_CODES = 4194830i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_MOUNTPOINTS: CLCTL_CODES = 529i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_DIRTY: CLCTL_CODES = 537i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_INFO: CLCTL_CODES = 549i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_IS_CSV_FILE: CLCTL_CODES = 553i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_RESOURCEID: CLCTL_CODES = 557i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_PATH: CLCTL_CODES = 561i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_NETNAME: CLCTL_CODES = 565i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_DIRECTORY: CLCTL_CODES = 569i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_BATCH_BLOCK_KEY: CLCTL_CODES = 574i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_BATCH_UNBLOCK_KEY: CLCTL_CODES = 577i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FILESERVER_SHARE_ADD: CLCTL_CODES = 4194886i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FILESERVER_SHARE_DEL: CLCTL_CODES = 4194890i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FILESERVER_SHARE_MODIFY: CLCTL_CODES = 4194894i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FILESERVER_SHARE_REPORT: CLCTL_CODES = 593i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_GET_OU_FOR_VCO: CLCTL_CODES = 4194926i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = 4194954i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = 4194958i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_SHARED_VOLUME_ID: CLCTL_CODES = 657i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_CSV_MAINTENANCE_MODE: CLCTL_CODES = 4194966i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_SHARED_VOLUME_BACKUP_MODE: CLCTL_CODES = 4194970i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLCTL_CODES = 669i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_STATES: CLCTL_CODES = 4194978i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_IS_SHARED_VOLUME: CLCTL_CODES = 677i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_CLUSDB_TIMESTAMP: CLCTL_CODES = 681i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RW_MODIFY_NOOP: CLCTL_CODES = 4194990i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IS_QUORUM_BLOCKED: CLCTL_CODES = 689i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_POOL_GET_DRIVE_INFO: CLCTL_CODES = 693i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_GUM_LOCK_OWNER: CLCTL_CODES = 697i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_STUCK_NODES: CLCTL_CODES = 701i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INJECT_GEM_FAULT: CLCTL_CODES = 705i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INTRODUCE_GEM_REPAIR_DELAY: CLCTL_CODES = 709i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SEND_DUMMY_GEM_MESSAGES: CLCTL_CODES = 713i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_BLOCK_GEM_SEND_RECV: CLCTL_CODES = 717i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_GEMID_VECTOR: CLCTL_CODES = 721i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_CRYPTO_CHECKPOINT_EX: CLCTL_CODES = 4195030i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUP_GET_LAST_MOVE_TIME: CLCTL_CODES = 729i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_STORAGE_CONFIGURATION: CLCTL_CODES = 4195042i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_STORAGE_CONFIGURATION: CLCTL_CODES = 741i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES: CLCTL_CODES = 745i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REMOVE_NODE: CLCTL_CODES = 4195054i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IS_FEATURE_INSTALLED: CLCTL_CODES = 753i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_IS_S2D_FEATURE_SUPPORTED: CLCTL_CODES = 757i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_PHYSICAL_DISK_INFO: CLCTL_CODES = 761i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHS: CLCTL_CODES = 765i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHINFO: CLCTL_CODES = 769i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLEAR_NODE_CONNECTION_INFO: CLCTL_CODES = 4195078i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_DNS_DOMAIN: CLCTL_CODES = 4195082i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CTCTL_GET_ROUTESTATUS_BASIC: CLCTL_CODES = 781i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CTCTL_GET_ROUTESTATUS_EXTENDED: CLCTL_CODES = 785i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CTCTL_GET_FAULT_DOMAIN_STATE: CLCTL_CODES = 789i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_SET_PWD_INFOEX: CLCTL_CODES = 794i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLCTL_CODES = 8161i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLCTL_CODES = 8417i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLCTL_CODES = 4202726i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RESOURCE_PREPARE_UPGRADE: CLCTL_CODES = 4202730i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RESOURCE_UPGRADE_COMPLETED: CLCTL_CODES = 4202734i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLCTL_CODES = 8433i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLCTL_CODES = 4202742i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_ADD_REPLICATION_GROUP: CLCTL_CODES = 8514i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_LOG_INFO: CLCTL_CODES = 8517i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLCTL_CODES = 8521i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLCTL_CODES = 8525i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLCTL_CODES = 8529i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_REPLICATED_DISKS: CLCTL_CODES = 8533i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_REPLICA_VOLUMES: CLCTL_CODES = 8537i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_LOG_VOLUME: CLCTL_CODES = 8541i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_RESOURCE_GROUP: CLCTL_CODES = 8545i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLCTL_CODES = 8549i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_STATE_CHANGE_TIME: CLCTL_CODES = 11613i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_CLUSTER_S2D_ENABLED: CLCTL_CODES = 4205922i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLCTL_CODES = 4205934i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUPSET_GET_GROUPS: CLCTL_CODES = 11633i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPS: CLCTL_CODES = 11637i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLCTL_CODES = 11641i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUP_GET_PROVIDER_GROUPS: CLCTL_CODES = 11645i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUP_GET_PROVIDER_GROUPSETS: CLCTL_CODES = 11649i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GROUP_SET_CCF_FROM_MASTER: CLCTL_CODES = 4205958i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = 11657i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = 4205966i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_INFRASTRUCTURE_SOFS_CHANGED: CLCTL_CODES = 4205970i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SCALEOUT_COMMAND: CLCTL_CODES = 4205974i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SCALEOUT_CONTROL: CLCTL_CODES = 4205978i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SCALEOUT_GET_CLUSTERS: CLCTL_CODES = 4205981i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RELOAD_AUTOLOGGER_CONFIG: CLCTL_CODES = 11730i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME: CLCTL_CODES = 11734i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID: CLCTL_CODES = 11738i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ENUM_AFFINITY_RULE_NAMES: CLCTL_CODES = 11741i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_NODES_IN_FD: CLCTL_CODES = 11745i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FORCE_DB_FLUSH: CLCTL_CODES = 4206054i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_DELETE: CLCTL_CODES = 5242886i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INSTALL_NODE: CLCTL_CODES = 5242890i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_EVICT_NODE: CLCTL_CODES = 5242894i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_DEPENDENCY: CLCTL_CODES = 5242898i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REMOVE_DEPENDENCY: CLCTL_CODES = 5242902i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_ADD_OWNER: CLCTL_CODES = 5242906i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_REMOVE_OWNER: CLCTL_CODES = 5242910i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_SET_NAME: CLCTL_CODES = 5242918i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLUSTER_NAME_CHANGED: CLCTL_CODES = 5242922i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CLUSTER_VERSION_CHANGED: CLCTL_CODES = 5242926i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FIXUP_ON_UPGRADE: CLCTL_CODES = 5242930i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STARTING_PHASE1: CLCTL_CODES = 5242934i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STARTING_PHASE2: CLCTL_CODES = 5242938i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_HOLD_IO: CLCTL_CODES = 5242942i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_RESUME_IO: CLCTL_CODES = 5242946i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FORCE_QUORUM: CLCTL_CODES = 5242950i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_INITIALIZE: CLCTL_CODES = 5242954i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_STATE_CHANGE_REASON: CLCTL_CODES = 5242958i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_PROVIDER_STATE_CHANGE: CLCTL_CODES = 5242962i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_LEAVING_GROUP: CLCTL_CODES = 5242966i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_JOINING_GROUP: CLCTL_CODES = 5242970i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FSWITNESS_GET_EPOCH_INFO: CLCTL_CODES = 1048669i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FSWITNESS_SET_EPOCH_INFO: CLCTL_CODES = 5242978i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_FSWITNESS_RELEASE_LOCK: CLCTL_CODES = 5242982i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NETNAME_CREDS_NOTIFYCAM: CLCTL_CODES = 5242986i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_QUORUM_STATUS: CLCTL_CODES = 5243006i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN: CLCTL_CODES = 1048705i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_UNDELETE: CLCTL_CODES = 5243014i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_GET_OPERATION_CONTEXT: CLCTL_CODES = 1057001i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_OWNER_CHANGE: CLCTL_CODES = 5251362i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_VALIDATE_CHANGE_GROUP: CLCTL_CODES = 1057061i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_CHECK_DRAIN_VETO: CLCTL_CODES = 1057069i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLCTL_NOTIFY_DRAIN_COMPLETE: CLCTL_CODES = 1057073i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUADMEX_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_NONE: CLUADMEX_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_CLUSTER: CLUADMEX_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_NODE: CLUADMEX_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_GROUP: CLUADMEX_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_RESOURCE: CLUADMEX_OBJECT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_RESOURCETYPE: CLUADMEX_OBJECT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_NETWORK: CLUADMEX_OBJECT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUADMEX_OT_NETINTERFACE: CLUADMEX_OBJECT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSCTL_AFFINITYRULE_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_GET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = 150995033i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_GET_RO_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = 150995029i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_SET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = 155189342i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_GET_ID: CLUSCTL_AFFINITYRULE_CODES = 150995001i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_AFFINITYRULE_GET_GROUPNAMES: CLUSCTL_AFFINITYRULE_CODES = 151006577i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSCTL_CLUSTER_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_UNKNOWN: CLUSCTL_CLUSTER_CODES = 117440512i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_FQDN: CLUSCTL_CLUSTER_CODES = 117440573i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = 121635554i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = 117441253i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIG_ATTRIBUTES: CLUSCTL_CLUSTER_CODES = 117441257i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_ENUM_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440593i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_RO_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440597i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440601i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 121634910i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_VALIDATE_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440609i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_ENUM_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440633i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440637i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440641i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 121634950i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440649i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = 117440613i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = 117440653i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_CHECK_VOTER_EVICT: CLUSCTL_CLUSTER_CODES = 117440581i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_CHECK_VOTER_DOWN: CLUSCTL_CLUSTER_CODES = 117440585i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SHUTDOWN: CLUSCTL_CLUSTER_CODES = 117440589i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_BATCH_BLOCK_KEY: CLUSCTL_CLUSTER_CODES = 117441086i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_BATCH_UNBLOCK_KEY: CLUSCTL_CLUSTER_CODES = 117441089i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_SHARED_VOLUME_ID: CLUSCTL_CLUSTER_CODES = 117441169i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_CLUSDB_TIMESTAMP: CLUSCTL_CLUSTER_CODES = 117441193i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_GUM_LOCK_OWNER: CLUSCTL_CLUSTER_CODES = 117441209i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_REMOVE_NODE: CLUSCTL_CLUSTER_CODES = 121635566i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_ACCOUNT_ACCESS: CLUSCTL_CLUSTER_CODES = 121635058i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_CLEAR_NODE_CONNECTION_INFO: CLUSCTL_CLUSTER_CODES = 121635590i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_DNS_DOMAIN: CLUSCTL_CLUSTER_CODES = 121635594i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_ENABLED: CLUSCTL_CLUSTER_CODES = 121646434i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLUSCTL_CLUSTER_CODES = 121646446i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_CLUSTER_CODES = 117452246i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_CLUSTER_CODES = 117452250i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_RELOAD_AUTOLOGGER_CONFIG: CLUSCTL_CLUSTER_CODES = 117452242i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_ENUM_AFFINITY_RULE_NAMES: CLUSCTL_CLUSTER_CODES = 117452253i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_NODES_IN_FD: CLUSCTL_CLUSTER_CODES = 117452257i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_FORCE_FLUSH_DB: CLUSCTL_CLUSTER_CODES = 121646566i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLUSTER_GET_CLMUSR_TOKEN: CLUSCTL_CLUSTER_CODES = 117440877i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSCTL_GROUPSET_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = 134217817i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = 134217813i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_SET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = 138412126i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_GROUPS: CLUSCTL_GROUPSET_CODES = 134229361i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = 134229365i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = 134229369i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = 134229373i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = 134229377i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUPSET_GET_ID: CLUSCTL_GROUPSET_CODES = 134217785i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSCTL_GROUP_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_UNKNOWN: CLUSCTL_GROUP_CODES = 50331648i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_CHARACTERISTICS: CLUSCTL_GROUP_CODES = 50331653i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_FLAGS: CLUSCTL_GROUP_CODES = 50331657i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_NAME: CLUSCTL_GROUP_CODES = 50331689i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_ID: CLUSCTL_GROUP_CODES = 50331705i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_ENUM_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 50331729i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 50331733i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 50331737i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_SET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 54526046i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_VALIDATE_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 50331745i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_ENUM_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 50331769i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 50331773i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 50331777i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_SET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 54526086i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 50331785i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_QUERY_DELETE: CLUSCTL_GROUP_CODES = 50332089i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = 50331749i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = 50331789i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_FAILURE_INFO: CLUSCTL_GROUP_CODES = 50331673i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_GET_LAST_MOVE_TIME: CLUSCTL_GROUP_CODES = 50332377i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_GROUP_SET_CCF_FROM_MASTER: CLUSCTL_GROUP_CODES = 54537606i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSCTL_NETINTERFACE_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_UNKNOWN: CLUSCTL_NETINTERFACE_CODES = 100663296i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_CHARACTERISTICS: CLUSCTL_NETINTERFACE_CODES = 100663301i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_FLAGS: CLUSCTL_NETINTERFACE_CODES = 100663305i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_NAME: CLUSCTL_NETINTERFACE_CODES = 100663337i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_ID: CLUSCTL_NETINTERFACE_CODES = 100663353i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_NODE: CLUSCTL_NETINTERFACE_CODES = 100663345i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_NETWORK: CLUSCTL_NETINTERFACE_CODES = 100663349i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_ENUM_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663377i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663381i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663385i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_SET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 104857694i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663393i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663417i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663421i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663425i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_SET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 104857734i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663433i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = 100663397i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = 100663437i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSCTL_NETWORK_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_UNKNOWN: CLUSCTL_NETWORK_CODES = 83886080i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_CHARACTERISTICS: CLUSCTL_NETWORK_CODES = 83886085i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_FLAGS: CLUSCTL_NETWORK_CODES = 83886089i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_NAME: CLUSCTL_NETWORK_CODES = 83886121i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_ID: CLUSCTL_NETWORK_CODES = 83886137i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_ENUM_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886161i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886165i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886169i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_SET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 88080478i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886177i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886201i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886205i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886209i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_SET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 88080518i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886217i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = 83886181i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = 83886221i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSCTL_NODE_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_UNKNOWN: CLUSCTL_NODE_CODES = 67108864i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_CHARACTERISTICS: CLUSCTL_NODE_CODES = 67108869i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_FLAGS: CLUSCTL_NODE_CODES = 67108873i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_NAME: CLUSCTL_NODE_CODES = 67108905i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_ID: CLUSCTL_NODE_CODES = 67108921i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_ENUM_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 67108945i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 67108949i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 67108953i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_SET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 71303262i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 67108961i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 67108985i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 67108989i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 67108993i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_SET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 71303302i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 67109001i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NODE_CODES = 67108965i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NODE_CODES = 67109005i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLUSCTL_NODE_CODES = 67108929i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_STUCK_NODES: CLUSCTL_NODE_CODES = 67109565i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_INJECT_GEM_FAULT: CLUSCTL_NODE_CODES = 67109569i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_INTRODUCE_GEM_REPAIR_DELAY: CLUSCTL_NODE_CODES = 67109573i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_SEND_DUMMY_GEM_MESSAGES: CLUSCTL_NODE_CODES = 67109577i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_BLOCK_GEM_SEND_RECV: CLUSCTL_NODE_CODES = 67109581i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_NODE_GET_GEMID_VECTOR: CLUSCTL_NODE_CODES = 67109585i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSCTL_RESOURCE_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_UNKNOWN: CLUSCTL_RESOURCE_CODES = 16777216i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_CODES = 16777221i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_FLAGS: CLUSCTL_RESOURCE_CODES = 16777225i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_CLASS_INFO: CLUSCTL_RESOURCE_CODES = 16777229i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_CODES = 16777233i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_NAME: CLUSCTL_RESOURCE_CODES = 16777257i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_ID: CLUSCTL_RESOURCE_CODES = 16777273i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_RESOURCE_TYPE: CLUSCTL_RESOURCE_CODES = 16777261i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777297i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777301i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777305i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 20971614i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777313i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = 16777317i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777337i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777341i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777345i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 20971654i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777353i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = 16777357i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = 20971682i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_DELETE_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = 20971686i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = 16777385i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = 20971694i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_DELETE_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = 20971698i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT_EX: CLUSCTL_RESOURCE_CODES = 20972246i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = 16777397i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_LOADBAL_PROCESS_LIST: CLUSCTL_RESOURCE_CODES = 16777417i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_NETWORK_NAME: CLUSCTL_RESOURCE_CODES = 16777577i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = 16777581i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = 16777594i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = 16778010i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_DELETE_CO: CLUSCTL_RESOURCE_CODES = 16777598i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_VALIDATE_VCO: CLUSCTL_RESOURCE_CODES = 16777601i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_RESET_VCO: CLUSCTL_RESOURCE_CODES = 16777605i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_REPAIR_VCO: CLUSCTL_RESOURCE_CODES = 16777613i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_REGISTER_DNS_RECORDS: CLUSCTL_RESOURCE_CODES = 16777586i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_DNS_NAME: CLUSCTL_RESOURCE_CODES = 16777589i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO: CLUSCTL_RESOURCE_CODES = 16777617i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_NUMBER_INFO: CLUSCTL_RESOURCE_CODES = 16777633i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_IS_PATH_VALID: CLUSCTL_RESOURCE_CODES = 16777625i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_QUERY_DELETE: CLUSCTL_RESOURCE_CODES = 16777657i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_UPGRADE_DLL: CLUSCTL_RESOURCE_CODES = 20971706i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_IPADDRESS_RENEW_LEASE: CLUSCTL_RESOURCE_CODES = 20971966i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_IPADDRESS_RELEASE_LEASE: CLUSCTL_RESOURCE_CODES = 20971970i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_64BIT: CLUSCTL_RESOURCE_CODES = 20971710i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_32BIT: CLUSCTL_RESOURCE_CODES = 20971714i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_QUERY_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = 16777697i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = 20972006i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_SET_DRIVELETTER: CLUSCTL_RESOURCE_CODES = 20972010i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX: CLUSCTL_RESOURCE_CODES = 16777713i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX2: CLUSCTL_RESOURCE_CODES = 16777721i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_MOUNTPOINTS: CLUSCTL_RESOURCE_CODES = 16777745i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DIRTY: CLUSCTL_RESOURCE_CODES = 16777753i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_INFO: CLUSCTL_RESOURCE_CODES = 16777765i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_CSV_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = 20972182i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ENABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = 20972170i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_DISABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = 20972174i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_SHARED_VOLUME_BACKUP_MODE: CLUSCTL_RESOURCE_CODES = 20972186i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLUSCTL_RESOURCE_CODES = 16777885i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_FAILURE_INFO: CLUSCTL_RESOURCE_CODES = 16777241i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_CODES = 16777733i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_STATES: CLUSCTL_RESOURCE_CODES = 20972194i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_IS_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = 16777893i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_IS_QUORUM_BLOCKED: CLUSCTL_RESOURCE_CODES = 16777905i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_POOL_GET_DRIVE_INFO: CLUSCTL_RESOURCE_CODES = 16777909i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_RLUA_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = 16777581i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = 16777594i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = 16778010i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_DELETE: CLUSCTL_RESOURCE_CODES = 22020102i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_UNDELETE: CLUSCTL_RESOURCE_CODES = 22020230i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_INSTALL_NODE: CLUSCTL_RESOURCE_CODES = 22020106i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_EVICT_NODE: CLUSCTL_RESOURCE_CODES = 22020110i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_DEPENDENCY: CLUSCTL_RESOURCE_CODES = 22020114i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_REMOVE_DEPENDENCY: CLUSCTL_RESOURCE_CODES = 22020118i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_ADD_OWNER: CLUSCTL_RESOURCE_CODES = 22020122i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_REMOVE_OWNER: CLUSCTL_RESOURCE_CODES = 22020126i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_NAME: CLUSCTL_RESOURCE_CODES = 22020134i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_CLUSTER_NAME_CHANGED: CLUSCTL_RESOURCE_CODES = 22020138i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_CODES = 22020142i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_FORCE_QUORUM: CLUSCTL_RESOURCE_CODES = 22020166i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_INITIALIZE: CLUSCTL_RESOURCE_CODES = 22020170i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON: CLUSCTL_RESOURCE_CODES = 22020174i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_PROVIDER_STATE_CHANGE: CLUSCTL_RESOURCE_CODES = 22020178i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_LEAVING_GROUP: CLUSCTL_RESOURCE_CODES = 22020182i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_JOINING_GROUP: CLUSCTL_RESOURCE_CODES = 22020186i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_FSWITNESS_GET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = 17825885i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_FSWITNESS_SET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = 22020194i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_FSWITNESS_RELEASE_LOCK: CLUSCTL_RESOURCE_CODES = 22020198i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NETNAME_CREDS_NOTIFYCAM: CLUSCTL_RESOURCE_CODES = 22020202i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_OPERATION_CONTEXT: CLUSCTL_RESOURCE_CODES = 17834217i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_RW_MODIFY_NOOP: CLUSCTL_RESOURCE_CODES = 20972206i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NOTIFY_QUORUM_STATUS: CLUSCTL_RESOURCE_CODES = 22020222i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NOTIFY_OWNER_CHANGE: CLUSCTL_RESOURCE_CODES = 22028578i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_VALIDATE_CHANGE_GROUP: CLUSCTL_RESOURCE_CODES = 17834277i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = 16788950i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_RESOURCE_CODES = 16788954i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLUSCTL_RESOURCE_CODES = 20979942i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLUSCTL_RESOURCE_CODES = 20979958i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_CODES = 20979946i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_CODES = 20979950i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_STATE_CHANGE_TIME: CLUSCTL_RESOURCE_CODES = 16788829i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = 16788873i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = 20983182i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SCALEOUT_COMMAND: CLUSCTL_RESOURCE_CODES = 20983190i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SCALEOUT_CONTROL: CLUSCTL_RESOURCE_CODES = 20983194i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_SCALEOUT_GET_CLUSTERS: CLUSCTL_RESOURCE_CODES = 20983197i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_CODES = 17834285i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_CODES = 17834289i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_GET_NODES_IN_FD: CLUSCTL_RESOURCE_CODES = 16788961i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSCTL_RESOURCE_TYPE_CODES = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_UNKNOWN: CLUSCTL_RESOURCE_TYPE_CODES = 33554432i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_TYPE_CODES = 33554437i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_FLAGS: CLUSCTL_RESOURCE_TYPE_CODES = 33554441i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_CLASS_INFO: CLUSCTL_RESOURCE_TYPE_CODES = 33554445i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554449i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_ARB_TIMEOUT: CLUSCTL_RESOURCE_TYPE_CODES = 33554453i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554513i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554517i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554521i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554529i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 37748830i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554533i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554537i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554553i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554557i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554561i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 37748870i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554569i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554573i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554577i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554601i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554613i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33554837i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_SYNC_CLUSDISK_DB: CLUSCTL_RESOURCE_TYPE_CODES = 37749150i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_VALIDATE_NETNAME: CLUSCTL_RESOURCE_TYPE_CODES = 33554997i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_GET_OU_FOR_VCO: CLUSCTL_RESOURCE_TYPE_CODES = 37749358i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = 33554993i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_DIRECTORY: CLUSCTL_RESOURCE_TYPE_CODES = 33555001i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_GEN_SCRIPT_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = 33554993i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_QUERY_DELETE: CLUSCTL_RESOURCE_TYPE_CODES = 33554873i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DRIVELETTERS: CLUSCTL_RESOURCE_TYPE_CODES = 33554925i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX: CLUSCTL_RESOURCE_TYPE_CODES = 33554933i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMAP_DRIVELETTER: CLUSCTL_RESOURCE_TYPE_CODES = 33554945i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_TYPE_CODES = 33554949i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_RESOURCEID: CLUSCTL_RESOURCE_TYPE_CODES = 33554989i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CLUSTERABLE: CLUSCTL_RESOURCE_TYPE_CODES = 33554953i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMOVE_VM_OWNERSHIP: CLUSCTL_RESOURCE_TYPE_CODES = 37749262i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CSV_FILE: CLUSCTL_RESOURCE_TYPE_CODES = 16777769i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_WITNESS_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = 33554993i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_INSTALL_NODE: CLUSCTL_RESOURCE_TYPE_CODES = 38797322i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_EVICT_NODE: CLUSCTL_RESOURCE_TYPE_CODES = 38797326i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_TYPE_CODES = 38797358i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_FIXUP_ON_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = 38797362i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE1: CLUSCTL_RESOURCE_TYPE_CODES = 38797366i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE2: CLUSCTL_RESOURCE_TYPE_CODES = 38797370i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_HOLD_IO: CLUSCTL_RESOURCE_TYPE_CODES = 38797374i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_RESUME_IO: CLUSCTL_RESOURCE_TYPE_CODES = 38797378i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLUSCTL_RESOURCE_TYPE_CODES = 33562593i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33562953i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33562957i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33562961i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33562965i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICA_VOLUMES: CLUSCTL_RESOURCE_TYPE_CODES = 33562969i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_VOLUME: CLUSCTL_RESOURCE_TYPE_CODES = 33562973i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_RESOURCE_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = 33562977i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLUSCTL_RESOURCE_TYPE_CODES = 33562981i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_INFO: CLUSCTL_RESOURCE_TYPE_CODES = 33562949i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_ADD_REPLICATION_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = 33562946i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLUSCTL_RESOURCE_TYPE_CODES = 33562849i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLUSCTL_RESOURCE_TYPE_CODES = 33562865i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = 37757162i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_TYPE_CODES = 37757166i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_MONITOR_SHUTTING_DOWN: CLUSCTL_RESOURCE_TYPE_CODES = 34603137i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_TYPE_CODES = 34611501i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_TYPE_CODES = 34611505i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSGROUP_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeCoreCluster: CLUSGROUP_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeAvailableStorage: CLUSGROUP_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeTemporary: CLUSGROUP_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeSharedVolume: CLUSGROUP_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeStoragePool: CLUSGROUP_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeFileServer: CLUSGROUP_TYPE = 100i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypePrintServer: CLUSGROUP_TYPE = 101i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeDhcpServer: CLUSGROUP_TYPE = 102i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeDtc: CLUSGROUP_TYPE = 103i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeMsmq: CLUSGROUP_TYPE = 104i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeWins: CLUSGROUP_TYPE = 105i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeStandAloneDfs: CLUSGROUP_TYPE = 106i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeGenericApplication: CLUSGROUP_TYPE = 107i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeGenericService: CLUSGROUP_TYPE = 108i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeGenericScript: CLUSGROUP_TYPE = 109i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeIScsiNameService: CLUSGROUP_TYPE = 110i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeVirtualMachine: CLUSGROUP_TYPE = 111i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeTsSessionBroker: CLUSGROUP_TYPE = 112i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeIScsiTarget: CLUSGROUP_TYPE = 113i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeScaleoutFileServer: CLUSGROUP_TYPE = 114i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeVMReplicaBroker: CLUSGROUP_TYPE = 115i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeTaskScheduler: CLUSGROUP_TYPE = 116i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeClusterUpdateAgent: CLUSGROUP_TYPE = 117i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeScaleoutCluster: CLUSGROUP_TYPE = 118i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeStorageReplica: CLUSGROUP_TYPE = 119i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeVMReplicaCoordinator: CLUSGROUP_TYPE = 120i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeCrossClusterOrchestrator: CLUSGROUP_TYPE = 121i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeInfrastructureFileServer: CLUSGROUP_TYPE = 122i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeCoreSddc: CLUSGROUP_TYPE = 123i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusGroupTypeUnknown: CLUSGROUP_TYPE = 9999i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSPROP_IPADDR_ENABLENETBIOS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_IPADDR_ENABLENETBIOS_DISABLED: CLUSPROP_IPADDR_ENABLENETBIOS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_IPADDR_ENABLENETBIOS_ENABLED: CLUSPROP_IPADDR_ENABLENETBIOS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_IPADDR_ENABLENETBIOS_TRACK_NIC: CLUSPROP_IPADDR_ENABLENETBIOS = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSPROP_PIFLAGS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_STICKY: CLUSPROP_PIFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_REMOVABLE: CLUSPROP_PIFLAGS = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_USABLE: CLUSPROP_PIFLAGS = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_DEFAULT_QUORUM: CLUSPROP_PIFLAGS = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_USABLE_FOR_CSV: CLUSPROP_PIFLAGS = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_ENCRYPTION_ENABLED: CLUSPROP_PIFLAGS = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_RAW: CLUSPROP_PIFLAGS = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_PIFLAG_UNKNOWN: CLUSPROP_PIFLAGS = -2147483648i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTERSET_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTERSET_OBJECT_TYPE_NONE: CLUSTERSET_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTERSET_OBJECT_TYPE_MEMBER: CLUSTERSET_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTERSET_OBJECT_TYPE_WORKLOAD: CLUSTERSET_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTERSET_OBJECT_TYPE_DATABASE: CLUSTERSET_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_STATE: CLUSTER_CHANGE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_DELETED: CLUSTER_CHANGE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_ADDED: CLUSTER_CHANGE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_PROPERTY: CLUSTER_CHANGE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_NAME: CLUSTER_CHANGE = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES: CLUSTER_CHANGE = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_VALUE: CLUSTER_CHANGE = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE: CLUSTER_CHANGE = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_STATE: CLUSTER_CHANGE = 256i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DELETED: CLUSTER_CHANGE = 512i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_ADDED: CLUSTER_CHANGE = 1024i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_PROPERTY: CLUSTER_CHANGE = 2048i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_STATE: CLUSTER_CHANGE = 4096i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_DELETED: CLUSTER_CHANGE = 8192i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_ADDED: CLUSTER_CHANGE = 16384i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_PROPERTY: CLUSTER_CHANGE = 32768i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED: CLUSTER_CHANGE = 65536i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ADDED: CLUSTER_CHANGE = 131072i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY: CLUSTER_CHANGE = 262144i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT: CLUSTER_CHANGE = 524288i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_STATE: CLUSTER_CHANGE = 1048576i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_DELETED: CLUSTER_CHANGE = 2097152i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_ADDED: CLUSTER_CHANGE = 4194304i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_PROPERTY: CLUSTER_CHANGE = 8388608i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_STATE: CLUSTER_CHANGE = 16777216i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED: CLUSTER_CHANGE = 33554432i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_ADDED: CLUSTER_CHANGE = 67108864i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_PROPERTY: CLUSTER_CHANGE = 134217728i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_QUORUM_STATE: CLUSTER_CHANGE = 268435456i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_STATE: CLUSTER_CHANGE = 536870912i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_PROPERTY: CLUSTER_CHANGE = 1073741824i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_HANDLE_CLOSE: CLUSTER_CHANGE = -2147483648i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_ALL: CLUSTER_CHANGE = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_CLUSTER_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT_V2: CLUSTER_CHANGE_CLUSTER_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_STATE_V2: CLUSTER_CHANGE_CLUSTER_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2: CLUSTER_CHANGE_CLUSTER_V2 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = 256i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2: CLUSTER_CHANGE_CLUSTER_V2 = 512i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_RENAME_V2: CLUSTER_CHANGE_CLUSTER_V2 = 1024i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2: CLUSTER_CHANGE_CLUSTER_V2 = 2048i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_UPGRADED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 4096i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_CLUSTER_ALL_V2: CLUSTER_CHANGE_CLUSTER_V2 = 8191i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_GROUPSET_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_DELETED_v2: CLUSTER_CHANGE_GROUPSET_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_STATE_V2: CLUSTER_CHANGE_GROUPSET_V2 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_GROUP_ADDED: CLUSTER_CHANGE_GROUPSET_V2 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED: CLUSTER_CHANGE_GROUPSET_V2 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2: CLUSTER_CHANGE_GROUPSET_V2 = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2: CLUSTER_CHANGE_GROUPSET_V2 = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2: CLUSTER_CHANGE_GROUPSET_V2 = 256i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUPSET_ALL_V2: CLUSTER_CHANGE_GROUPSET_V2 = 511i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_GROUP_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_DELETED_V2: CLUSTER_CHANGE_GROUP_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_STATE_V2: CLUSTER_CHANGE_GROUP_V2 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_OWNER_NODE_V2: CLUSTER_CHANGE_GROUP_V2 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2: CLUSTER_CHANGE_GROUP_V2 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2: CLUSTER_CHANGE_GROUP_V2 = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2: CLUSTER_CHANGE_GROUP_V2 = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2: CLUSTER_CHANGE_GROUP_V2 = 256i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2: CLUSTER_CHANGE_GROUP_V2 = 512i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_GROUP_ALL_V2: CLUSTER_CHANGE_GROUP_V2 = 1023i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_NETINTERFACE_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_STATE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETINTERFACE_ALL_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 31i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_NETWORK_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_DELETED_V2: CLUSTER_CHANGE_NETWORK_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_STATE_V2: CLUSTER_CHANGE_NETWORK_V2 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETWORK_V2 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NETWORK_ALL_V2: CLUSTER_CHANGE_NETWORK_V2 = 31i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_UPGRADE_NODE_PREPARE: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_UPGRADE_NODE_COMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_UPGRADE_ALL: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_NODE_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2: CLUSTER_CHANGE_NODE_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_DELETED_V2: CLUSTER_CHANGE_NODE_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_STATE_V2: CLUSTER_CHANGE_NODE_V2 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_GROUP_GAINED_V2: CLUSTER_CHANGE_NODE_V2 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_GROUP_LOST_V2: CLUSTER_CHANGE_NODE_V2 = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NODE_V2 = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_NODE_ALL_V2: CLUSTER_CHANGE_NODE_V2 = 255i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_QUORUM_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_QUORUM_STATE_V2: CLUSTER_CHANGE_QUORUM_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_QUORUM_ALL_V2: CLUSTER_CHANGE_QUORUM_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_REGISTRY_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2: CLUSTER_CHANGE_REGISTRY_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_NAME_V2: CLUSTER_CHANGE_REGISTRY_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE_V2: CLUSTER_CHANGE_REGISTRY_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_VALUE_V2: CLUSTER_CHANGE_REGISTRY_V2 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2: CLUSTER_CHANGE_REGISTRY_V2 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_REGISTRY_ALL_V2: CLUSTER_CHANGE_REGISTRY_V2 = 31i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_RESOURCE_TYPE_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_TYPE_SPECIFIC_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ALL_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 63i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_RESOURCE_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2: CLUSTER_CHANGE_RESOURCE_V2 = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2: CLUSTER_CHANGE_RESOURCE_V2 = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2: CLUSTER_CHANGE_RESOURCE_V2 = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_V2 = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_V2 = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_V2 = 256i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_RESOURCE_V2 = 512i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = 1024i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_RESOURCE_ALL_V2: CLUSTER_CHANGE_RESOURCE_V2 = 2047i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_SHARED_VOLUME_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SHARED_VOLUME_ALL_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CHANGE_SPACEPORT_V2 = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CHANGE_SPACEPORT_CUSTOM_PNP_V2: CLUSTER_CHANGE_SPACEPORT_V2 = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CLOUD_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CLOUD_TYPE_NONE: CLUSTER_CLOUD_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CLOUD_TYPE_AZURE: CLUSTER_CLOUD_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CLOUD_TYPE_MIXED: CLUSTER_CLOUD_TYPE = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_CLOUD_TYPE_UNKNOWN: CLUSTER_CLOUD_TYPE = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CONTROL_OBJECT = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_INVALID: CLUSTER_CONTROL_OBJECT = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_RESOURCE: CLUSTER_CONTROL_OBJECT = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_RESOURCE_TYPE: CLUSTER_CONTROL_OBJECT = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_GROUP: CLUSTER_CONTROL_OBJECT = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_NODE: CLUSTER_CONTROL_OBJECT = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_NETWORK: CLUSTER_CONTROL_OBJECT = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_NETINTERFACE: CLUSTER_CONTROL_OBJECT = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_CLUSTER: CLUSTER_CONTROL_OBJECT = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_GROUPSET: CLUSTER_CONTROL_OBJECT = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_AFFINITYRULE: CLUSTER_CONTROL_OBJECT = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_OBJECT_USER: CLUSTER_CONTROL_OBJECT = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_CSV_VOLUME_FAULT_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateNoFaults: CLUSTER_CSV_VOLUME_FAULT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateNoDirectIO: CLUSTER_CSV_VOLUME_FAULT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateNoAccess: CLUSTER_CSV_VOLUME_FAULT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateInMaintenance: CLUSTER_CSV_VOLUME_FAULT_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeStateDismounted: CLUSTER_CSV_VOLUME_FAULT_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_ENUM = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_NODE: CLUSTER_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_RESTYPE: CLUSTER_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_RESOURCE: CLUSTER_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_GROUP: CLUSTER_ENUM = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_NETWORK: CLUSTER_ENUM = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_NETINTERFACE: CLUSTER_ENUM = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_SHARED_VOLUME_GROUP: CLUSTER_ENUM = 536870912i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_SHARED_VOLUME_RESOURCE: CLUSTER_ENUM = 1073741824i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_INTERNAL_NETWORK: CLUSTER_ENUM = -2147483648i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_ENUM_ALL: CLUSTER_ENUM = 63i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_GROUP_AUTOFAILBACK_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupPreventFailback: CLUSTER_GROUP_AUTOFAILBACK_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupAllowFailback: CLUSTER_GROUP_AUTOFAILBACK_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupFailbackTypeCount: CLUSTER_GROUP_AUTOFAILBACK_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_GROUP_ENUM = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_CONTAINS: CLUSTER_GROUP_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_NODES: CLUSTER_GROUP_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_GROUP_ENUM_ALL: CLUSTER_GROUP_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_GROUP_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PriorityDisabled: CLUSTER_GROUP_PRIORITY = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PriorityLow: CLUSTER_GROUP_PRIORITY = 1000i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PriorityMedium: CLUSTER_GROUP_PRIORITY = 2000i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PriorityHigh: CLUSTER_GROUP_PRIORITY = 3000i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_GROUP_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupStateUnknown: CLUSTER_GROUP_STATE = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupOnline: CLUSTER_GROUP_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupOffline: CLUSTER_GROUP_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupFailed: CLUSTER_GROUP_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupPartialOnline: CLUSTER_GROUP_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterGroupPending: CLUSTER_GROUP_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_MGMT_POINT_RESTYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_RESTYPE_AUTO: CLUSTER_MGMT_POINT_RESTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_RESTYPE_SNN: CLUSTER_MGMT_POINT_RESTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_RESTYPE_DNN: CLUSTER_MGMT_POINT_RESTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_MGMT_POINT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_TYPE_NONE: CLUSTER_MGMT_POINT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_TYPE_CNO: CLUSTER_MGMT_POINT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_TYPE_DNS_ONLY: CLUSTER_MGMT_POINT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_MGMT_POINT_TYPE_CNO_ONLY: CLUSTER_MGMT_POINT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NETINTERFACE_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceStateUnknown: CLUSTER_NETINTERFACE_STATE = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceUnavailable: CLUSTER_NETINTERFACE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceFailed: CLUSTER_NETINTERFACE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceUnreachable: CLUSTER_NETINTERFACE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetInterfaceUp: CLUSTER_NETINTERFACE_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NETWORK_ENUM = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NETWORK_ENUM_NETINTERFACES: CLUSTER_NETWORK_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NETWORK_ENUM_ALL: CLUSTER_NETWORK_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NETWORK_ROLE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkRoleNone: CLUSTER_NETWORK_ROLE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkRoleInternalUse: CLUSTER_NETWORK_ROLE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkRoleClientAccess: CLUSTER_NETWORK_ROLE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkRoleInternalAndClient: CLUSTER_NETWORK_ROLE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NETWORK_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkStateUnknown: CLUSTER_NETWORK_STATE = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkUnavailable: CLUSTER_NETWORK_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkDown: CLUSTER_NETWORK_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkPartitioned: CLUSTER_NETWORK_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNetworkUp: CLUSTER_NETWORK_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NODE_DRAIN_STATUS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeDrainStatusNotInitiated: CLUSTER_NODE_DRAIN_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeDrainStatusInProgress: CLUSTER_NODE_DRAIN_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeDrainStatusCompleted: CLUSTER_NODE_DRAIN_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeDrainStatusFailed: CLUSTER_NODE_DRAIN_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeDrainStatusCount: CLUSTER_NODE_DRAIN_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NODE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NODE_ENUM_NETINTERFACES: CLUSTER_NODE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NODE_ENUM_GROUPS: CLUSTER_NODE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NODE_ENUM_PREFERRED_GROUPS: CLUSTER_NODE_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NODE_ENUM_ALL: CLUSTER_NODE_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NODE_RESUME_FAILBACK_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const DoNotFailbackGroups: CLUSTER_NODE_RESUME_FAILBACK_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FailbackGroupsImmediately: CLUSTER_NODE_RESUME_FAILBACK_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FailbackGroupsPerPolicy: CLUSTER_NODE_RESUME_FAILBACK_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeResumeFailbackTypeCount: CLUSTER_NODE_RESUME_FAILBACK_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NODE_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeStateUnknown: CLUSTER_NODE_STATE = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeUp: CLUSTER_NODE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeDown: CLUSTER_NODE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodePaused: CLUSTER_NODE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterNodeJoining: CLUSTER_NODE_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NODE_STATUS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusNormal: CLUSTER_NODE_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusIsolated: CLUSTER_NODE_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusQuarantined: CLUSTER_NODE_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusDrainInProgress: CLUSTER_NODE_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusDrainCompleted: CLUSTER_NODE_STATUS = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusDrainFailed: CLUSTER_NODE_STATUS = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusAvoidPlacement: CLUSTER_NODE_STATUS = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const NodeStatusMax: CLUSTER_NODE_STATUS = 51i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_NOTIFICATIONS_VERSION = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NOTIFICATIONS_V1: CLUSTER_NOTIFICATIONS_VERSION = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_NOTIFICATIONS_V2: CLUSTER_NOTIFICATIONS_VERSION = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_NONE: CLUSTER_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_CLUSTER: CLUSTER_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_GROUP: CLUSTER_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_RESOURCE: CLUSTER_OBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_RESOURCE_TYPE: CLUSTER_OBJECT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_NETWORK_INTERFACE: CLUSTER_OBJECT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_NETWORK: CLUSTER_OBJECT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_NODE: CLUSTER_OBJECT_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_REGISTRY: CLUSTER_OBJECT_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_QUORUM: CLUSTER_OBJECT_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_SHARED_VOLUME: CLUSTER_OBJECT_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_GROUPSET: CLUSTER_OBJECT_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_OBJECT_TYPE_AFFINITYRULE: CLUSTER_OBJECT_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_PROPERTY_FORMAT = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_UNKNOWN: CLUSTER_PROPERTY_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_BINARY: CLUSTER_PROPERTY_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_DWORD: CLUSTER_PROPERTY_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_SZ: CLUSTER_PROPERTY_FORMAT = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_EXPAND_SZ: CLUSTER_PROPERTY_FORMAT = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_MULTI_SZ: CLUSTER_PROPERTY_FORMAT = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_ULARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_LONG: CLUSTER_PROPERTY_FORMAT = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_EXPANDED_SZ: CLUSTER_PROPERTY_FORMAT = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_FORMAT = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_LARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = 10i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_WORD: CLUSTER_PROPERTY_FORMAT = 11i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_FILETIME: CLUSTER_PROPERTY_FORMAT = 12i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_VALUE_LIST: CLUSTER_PROPERTY_FORMAT = 13i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_PROPERTY_LIST: CLUSTER_PROPERTY_FORMAT = 14i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_FORMAT_USER: CLUSTER_PROPERTY_FORMAT = 32768i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_PROPERTY_SYNTAX = u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_ENDMARK: CLUSTER_PROPERTY_SYNTAX = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_NAME: CLUSTER_PROPERTY_SYNTAX = 262147u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_RESCLASS: CLUSTER_PROPERTY_SYNTAX = 131074u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_SZ: CLUSTER_PROPERTY_SYNTAX = 65539u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPAND_SZ: CLUSTER_PROPERTY_SYNTAX = 65540u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_DWORD: CLUSTER_PROPERTY_SYNTAX = 65538u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_BINARY: CLUSTER_PROPERTY_SYNTAX = 65537u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_MULTI_SZ: CLUSTER_PROPERTY_SYNTAX = 65541u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_LONG: CLUSTER_PROPERTY_SYNTAX = 65543u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPANDED_SZ: CLUSTER_PROPERTY_SYNTAX = 65544u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = 65545u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_LARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = 65546u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_ULARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = 65542u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_WORD: CLUSTER_PROPERTY_SYNTAX = 65547u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_PROPERTY_LIST: CLUSTER_PROPERTY_SYNTAX = 65550u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_LIST_VALUE_FILETIME: CLUSTER_PROPERTY_SYNTAX = 65548u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_SIGNATURE: CLUSTER_PROPERTY_SYNTAX = 327682u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_SCSI_ADDRESS: CLUSTER_PROPERTY_SYNTAX = 393218u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_NUMBER: CLUSTER_PROPERTY_SYNTAX = 458754u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_PARTITION_INFO: CLUSTER_PROPERTY_SYNTAX = 524289u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_FTSET_INFO: CLUSTER_PROPERTY_SYNTAX = 589825u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_SERIALNUMBER: CLUSTER_PROPERTY_SYNTAX = 655363u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_GUID: CLUSTER_PROPERTY_SYNTAX = 720899u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_DISK_SIZE: CLUSTER_PROPERTY_SYNTAX = 786438u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX: CLUSTER_PROPERTY_SYNTAX = 851969u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX2: CLUSTER_PROPERTY_SYNTAX = 917505u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_SYNTAX_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = 983041u32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_PROPERTY_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_UNKNOWN: CLUSTER_PROPERTY_TYPE = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_ENDMARK: CLUSTER_PROPERTY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_LIST_VALUE: CLUSTER_PROPERTY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_RESCLASS: CLUSTER_PROPERTY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_RESERVED1: CLUSTER_PROPERTY_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_NAME: CLUSTER_PROPERTY_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_SIGNATURE: CLUSTER_PROPERTY_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_SCSI_ADDRESS: CLUSTER_PROPERTY_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_DISK_NUMBER: CLUSTER_PROPERTY_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_PARTITION_INFO: CLUSTER_PROPERTY_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_FTSET_INFO: CLUSTER_PROPERTY_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_DISK_SERIALNUMBER: CLUSTER_PROPERTY_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_DISK_GUID: CLUSTER_PROPERTY_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_DISK_SIZE: CLUSTER_PROPERTY_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_PARTITION_INFO_EX: CLUSTER_PROPERTY_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_PARTITION_INFO_EX2: CLUSTER_PROPERTY_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSPROP_TYPE_USER: CLUSTER_PROPERTY_TYPE = 32768i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_QUORUM_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const OperationalQuorum: CLUSTER_QUORUM_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ModifyQuorum: CLUSTER_QUORUM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_QUORUM_VALUE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_QUORUM_MAINTAINED: CLUSTER_QUORUM_VALUE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_QUORUM_LOST: CLUSTER_QUORUM_VALUE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_REG_COMMAND = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_COMMAND_NONE: CLUSTER_REG_COMMAND = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_SET_VALUE: CLUSTER_REG_COMMAND = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CREATE_KEY: CLUSTER_REG_COMMAND = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_DELETE_KEY: CLUSTER_REG_COMMAND = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_DELETE_VALUE: CLUSTER_REG_COMMAND = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_SET_KEY_SECURITY: CLUSTER_REG_COMMAND = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_VALUE_DELETED: CLUSTER_REG_COMMAND = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_READ_KEY: CLUSTER_REG_COMMAND = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_READ_VALUE: CLUSTER_REG_COMMAND = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_READ_ERROR: CLUSTER_REG_COMMAND = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONTROL_COMMAND: CLUSTER_REG_COMMAND = 10i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_EXISTS: CLUSTER_REG_COMMAND = 11i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_NOT_EXISTS: CLUSTER_REG_COMMAND = 12i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_IS_EQUAL: CLUSTER_REG_COMMAND = 13i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_IS_NOT_EQUAL: CLUSTER_REG_COMMAND = 14i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_IS_GREATER_THAN: CLUSTER_REG_COMMAND = 15i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_IS_LESS_THAN: CLUSTER_REG_COMMAND = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_KEY_EXISTS: CLUSTER_REG_COMMAND = 17i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_CONDITION_KEY_NOT_EXISTS: CLUSTER_REG_COMMAND = 18i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSREG_LAST_COMMAND: CLUSTER_REG_COMMAND = 19i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_RESOURCE_APPLICATION_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceApplicationStateUnknown: CLUSTER_RESOURCE_APPLICATION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceApplicationOSHeartBeat: CLUSTER_RESOURCE_APPLICATION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceApplicationReady: CLUSTER_RESOURCE_APPLICATION_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_RESOURCE_CLASS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESCLASS_UNKNOWN: CLUSTER_RESOURCE_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESCLASS_STORAGE: CLUSTER_RESOURCE_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESCLASS_NETWORK: CLUSTER_RESOURCE_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESCLASS_USER: CLUSTER_RESOURCE_CLASS = 32768i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_RESOURCE_CREATE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_DEFAULT_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_SEPARATE_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_VALID_FLAGS: CLUSTER_RESOURCE_CREATE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceEmbeddedFailureActionNone: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceEmbeddedFailureActionLogOnly: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceEmbeddedFailureActionRecover: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_RESOURCE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_DEPENDS: CLUSTER_RESOURCE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_PROVIDES: CLUSTER_RESOURCE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_NODES: CLUSTER_RESOURCE_ENUM = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_ENUM_ALL: CLUSTER_RESOURCE_ENUM = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_RESOURCE_RESTART_ACTION = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceDontRestart: CLUSTER_RESOURCE_RESTART_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceRestartNoNotify: CLUSTER_RESOURCE_RESTART_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceRestartNotify: CLUSTER_RESOURCE_RESTART_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceRestartActionCount: CLUSTER_RESOURCE_RESTART_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_RESOURCE_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceStateUnknown: CLUSTER_RESOURCE_STATE = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceInherited: CLUSTER_RESOURCE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceInitializing: CLUSTER_RESOURCE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceOnline: CLUSTER_RESOURCE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceOffline: CLUSTER_RESOURCE_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceFailed: CLUSTER_RESOURCE_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourcePending: CLUSTER_RESOURCE_STATE = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceOnlinePending: CLUSTER_RESOURCE_STATE = 129i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterResourceOfflinePending: CLUSTER_RESOURCE_STATE = 130i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_RESOURCE_STATE_CHANGE_REASON = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonUnknown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonFailover: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonFailedMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonShutdown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const eResourceStateChangeReasonRundown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_RESOURCE_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_TYPE_ENUM_NODES: CLUSTER_RESOURCE_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES: CLUSTER_RESOURCE_TYPE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUSTER_RESOURCE_TYPE_ENUM_ALL: CLUSTER_RESOURCE_TYPE_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_ROLE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDHCP: CLUSTER_ROLE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDTC: CLUSTER_ROLE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleFileServer: CLUSTER_ROLE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleGenericApplication: CLUSTER_ROLE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleGenericScript: CLUSTER_ROLE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleGenericService: CLUSTER_ROLE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleISCSINameServer: CLUSTER_ROLE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleMSMQ: CLUSTER_ROLE = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleNFS: CLUSTER_ROLE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRolePrintServer: CLUSTER_ROLE = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleStandAloneNamespaceServer: CLUSTER_ROLE = 10i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleVolumeShadowCopyServiceTask: CLUSTER_ROLE = 11i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleWINS: CLUSTER_ROLE = 12i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleTaskScheduler: CLUSTER_ROLE = 13i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleNetworkFileSystem: CLUSTER_ROLE = 14i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDFSReplicatedFolder: CLUSTER_ROLE = 15i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDistributedFileSystem: CLUSTER_ROLE = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleDistributedNetworkName: CLUSTER_ROLE = 17i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleFileShare: CLUSTER_ROLE = 18i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleFileShareWitness: CLUSTER_ROLE = 19i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleHardDisk: CLUSTER_ROLE = 20i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleIPAddress: CLUSTER_ROLE = 21i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleIPV6Address: CLUSTER_ROLE = 22i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleIPV6TunnelAddress: CLUSTER_ROLE = 23i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleISCSITargetServer: CLUSTER_ROLE = 24i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleNetworkName: CLUSTER_ROLE = 25i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRolePhysicalDisk: CLUSTER_ROLE = 26i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleSODAFileServer: CLUSTER_ROLE = 27i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleStoragePool: CLUSTER_ROLE = 28i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleVirtualMachine: CLUSTER_ROLE = 29i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleVirtualMachineConfiguration: CLUSTER_ROLE = 30i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleVirtualMachineReplicaBroker: CLUSTER_ROLE = 31i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_ROLE_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleUnknown: CLUSTER_ROLE_STATE = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleClustered: CLUSTER_ROLE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterRoleUnclustered: CLUSTER_ROLE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_SETUP_PHASE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseInitialize: CLUSTER_SETUP_PHASE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseValidateNodeState: CLUSTER_SETUP_PHASE = 100i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseValidateNetft: CLUSTER_SETUP_PHASE = 102i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseValidateClusDisk: CLUSTER_SETUP_PHASE = 103i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseConfigureClusSvc: CLUSTER_SETUP_PHASE = 104i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseStartingClusSvc: CLUSTER_SETUP_PHASE = 105i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseQueryClusterNameAccount: CLUSTER_SETUP_PHASE = 106i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseValidateClusterNameAccount: CLUSTER_SETUP_PHASE = 107i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateClusterAccount: CLUSTER_SETUP_PHASE = 108i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseConfigureClusterAccount: CLUSTER_SETUP_PHASE = 109i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseFormingCluster: CLUSTER_SETUP_PHASE = 200i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseAddClusterProperties: CLUSTER_SETUP_PHASE = 201i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateResourceTypes: CLUSTER_SETUP_PHASE = 202i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateGroups: CLUSTER_SETUP_PHASE = 203i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateIPAddressResources: CLUSTER_SETUP_PHASE = 204i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCreateNetworkName: CLUSTER_SETUP_PHASE = 205i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseClusterGroupOnline: CLUSTER_SETUP_PHASE = 206i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseGettingCurrentMembership: CLUSTER_SETUP_PHASE = 300i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseAddNodeToCluster: CLUSTER_SETUP_PHASE = 301i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseNodeUp: CLUSTER_SETUP_PHASE = 302i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseMoveGroup: CLUSTER_SETUP_PHASE = 400i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseDeleteGroup: CLUSTER_SETUP_PHASE = 401i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCleanupCOs: CLUSTER_SETUP_PHASE = 402i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseOfflineGroup: CLUSTER_SETUP_PHASE = 403i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseEvictNode: CLUSTER_SETUP_PHASE = 404i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCleanupNode: CLUSTER_SETUP_PHASE = 405i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseCoreGroupCleanup: CLUSTER_SETUP_PHASE = 406i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseFailureCleanup: CLUSTER_SETUP_PHASE = 999i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_SETUP_PHASE_SEVERITY = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseInformational: CLUSTER_SETUP_PHASE_SEVERITY = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseWarning: CLUSTER_SETUP_PHASE_SEVERITY = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseFatal: CLUSTER_SETUP_PHASE_SEVERITY = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_SETUP_PHASE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseStart: CLUSTER_SETUP_PHASE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseContinue: CLUSTER_SETUP_PHASE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseEnd: CLUSTER_SETUP_PHASE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSetupPhaseReport: CLUSTER_SETUP_PHASE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_SHARED_VOLUME_BACKUP_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeBackupNone: CLUSTER_SHARED_VOLUME_BACKUP_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VolumeBackupInProgress: CLUSTER_SHARED_VOLUME_BACKUP_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeNone: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeVolumeOffset: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeVolumeId: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeVolumeName: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeRenameInputTypeVolumeGuid: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeSnapshotStateUnknown: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumePrepareForHWSnapshot: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumeHWSnapshotCompleted: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterSharedVolumePrepareForFreeze: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_SHARED_VOLUME_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStateUnavailable: CLUSTER_SHARED_VOLUME_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStatePaused: CLUSTER_SHARED_VOLUME_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStateActive: CLUSTER_SHARED_VOLUME_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStateActiveRedirected: CLUSTER_SHARED_VOLUME_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SharedVolumeStateActiveVolumeRedirected: CLUSTER_SHARED_VOLUME_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_STORAGENODE_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeStateUnknown: CLUSTER_STORAGENODE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeUp: CLUSTER_STORAGENODE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeDown: CLUSTER_STORAGENODE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodePaused: CLUSTER_STORAGENODE_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeStarting: CLUSTER_STORAGENODE_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStorageNodeStopping: CLUSTER_STORAGENODE_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUSTER_UPGRADE_PHASE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseInitialize: CLUSTER_UPGRADE_PHASE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseValidatingUpgrade: CLUSTER_UPGRADE_PHASE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseUpgradingComponents: CLUSTER_UPGRADE_PHASE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseInstallingNewComponents: CLUSTER_UPGRADE_PHASE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterUpgradePhaseUpgradeComplete: CLUSTER_UPGRADE_PHASE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUS_AFFINITY_RULE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_NONE: CLUS_AFFINITY_RULE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_SAME_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_SAME_NODE: CLUS_AFFINITY_RULE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_DIFFERENT_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_DIFFERENT_NODE: CLUS_AFFINITY_RULE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_MIN: CLUS_AFFINITY_RULE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_AFFINITY_RULE_MAX: CLUS_AFFINITY_RULE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUS_CHARACTERISTICS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_UNKNOWN: CLUS_CHARACTERISTICS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_QUORUM: CLUS_CHARACTERISTICS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_DELETE_REQUIRES_ALL_NODES: CLUS_CHARACTERISTICS = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_LOCAL_QUORUM: CLUS_CHARACTERISTICS = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_LOCAL_QUORUM_DEBUG: CLUS_CHARACTERISTICS = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_REQUIRES_STATE_CHANGE_REASON: CLUS_CHARACTERISTICS = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_BROADCAST_DELETE: CLUS_CHARACTERISTICS = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_SINGLE_CLUSTER_INSTANCE: CLUS_CHARACTERISTICS = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_SINGLE_GROUP_INSTANCE: CLUS_CHARACTERISTICS = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_COEXIST_IN_SHARED_VOLUME_GROUP: CLUS_CHARACTERISTICS = 256i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_PLACEMENT_DATA: CLUS_CHARACTERISTICS = 512i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_MONITOR_DETACH: CLUS_CHARACTERISTICS = 1024i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_MONITOR_REATTACH: CLUS_CHARACTERISTICS = 2048i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_OPERATION_CONTEXT: CLUS_CHARACTERISTICS = 4096i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_CLONES: CLUS_CHARACTERISTICS = 8192i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_NOT_PREEMPTABLE: CLUS_CHARACTERISTICS = 16384i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_NOTIFY_NEW_OWNER: CLUS_CHARACTERISTICS = 32768i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_SUPPORTS_UNMONITORED_STATE: CLUS_CHARACTERISTICS = 65536i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_INFRASTRUCTURE: CLUS_CHARACTERISTICS = 131072i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_VETO_DRAIN: CLUS_CHARACTERISTICS = 262144i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_CHAR_DRAIN_LOCAL_OFFLINE: CLUS_CHARACTERISTICS = 524288i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUS_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_FLAG_CORE: CLUS_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUS_GROUP_START_SETTING = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GROUP_START_ALWAYS: CLUS_GROUP_START_SETTING = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GROUP_DO_NOT_START: CLUS_GROUP_START_SETTING = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_GROUP_START_ALLOWED: CLUS_GROUP_START_SETTING = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUS_RESSUBCLASS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_SHARED: CLUS_RESSUBCLASS = -2147483648i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUS_RESSUBCLASS_NETWORK = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_NETWORK_INTERNET_PROTOCOL: CLUS_RESSUBCLASS_NETWORK = -2147483648i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type CLUS_RESSUBCLASS_STORAGE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_STORAGE_SHARED_BUS: CLUS_RESSUBCLASS_STORAGE = -2147483648i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_STORAGE_DISK: CLUS_RESSUBCLASS_STORAGE = 1073741824i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const CLUS_RESSUBCLASS_STORAGE_REPLICATION: CLUS_RESSUBCLASS_STORAGE = 268435456i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type FAILURE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FAILURE_TYPE_GENERAL: FAILURE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FAILURE_TYPE_EMBEDDED: FAILURE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FAILURE_TYPE_NETWORK_LOSS: FAILURE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type FILESHARE_CHANGE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FILESHARE_CHANGE_NONE: FILESHARE_CHANGE_ENUM = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FILESHARE_CHANGE_ADD: FILESHARE_CHANGE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FILESHARE_CHANGE_DEL: FILESHARE_CHANGE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const FILESHARE_CHANGE_MODIFY: FILESHARE_CHANGE_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type GRP_PLACEMENT_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GRP_PLACEMENT_OPTIONS_MIN_VALUE: GRP_PLACEMENT_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GRP_PLACEMENT_OPTIONS_DEFAULT: GRP_PLACEMENT_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GRP_PLACEMENT_OPTIONS_DISABLE_AUTOBALANCING: GRP_PLACEMENT_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const GRP_PLACEMENT_OPTIONS_ALL: GRP_PLACEMENT_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type LOG_LEVEL = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOG_INFORMATION: LOG_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOG_WARNING: LOG_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOG_ERROR: LOG_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const LOG_SEVERE: LOG_LEVEL = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type MAINTENANCE_MODE_TYPE_ENUM = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MaintenanceModeTypeDisableIsAliveCheck: MAINTENANCE_MODE_TYPE_ENUM = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MaintenanceModeTypeOfflineResource: MAINTENANCE_MODE_TYPE_ENUM = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const MaintenanceModeTypeUnclusterResource: MAINTENANCE_MODE_TYPE_ENUM = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type NODE_CLUSTER_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStateNotInstalled: NODE_CLUSTER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStateNotConfigured: NODE_CLUSTER_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStateNotRunning: NODE_CLUSTER_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ClusterStateRunning: NODE_CLUSTER_STATE = 19i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PLACEMENT_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_MIN_VALUE: PLACEMENT_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DEFAULT_PLACEMENT_OPTIONS: PLACEMENT_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DISABLE_CSV_VM_DEPENDENCY: PLACEMENT_OPTIONS = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_CONSIDER_OFFLINE_VMS: PLACEMENT_OPTIONS = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_USE_MEMORY: PLACEMENT_OPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_USE_CPU: PLACEMENT_OPTIONS = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_USE_LOCAL_TEMP_DISK: PLACEMENT_OPTIONS = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_RESUME_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_SAVE_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_DONT_RESUME_AVAILABILTY_SET_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = 128i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_SAVE_AVAILABILTY_SET_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = 256i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_AVAILABILITY_SET_DOMAIN_AFFINITY: PLACEMENT_OPTIONS = 512i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const PLACEMENT_OPTIONS_ALL: PLACEMENT_OPTIONS = 1023i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type RESDLL_CONTEXT_OPERATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeFailback: RESDLL_CONTEXT_OPERATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeDrain: RESDLL_CONTEXT_OPERATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeDrainFailure: RESDLL_CONTEXT_OPERATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeEmbeddedFailure: RESDLL_CONTEXT_OPERATION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypePreemption: RESDLL_CONTEXT_OPERATION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeNetworkDisconnect: RESDLL_CONTEXT_OPERATION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResdllContextOperationTypeNetworkDisconnectMoveRetry: RESDLL_CONTEXT_OPERATION_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type RESOURCE_EXIT_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResourceExitStateContinue: RESOURCE_EXIT_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResourceExitStateTerminate: RESOURCE_EXIT_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const ResourceExitStateMax: RESOURCE_EXIT_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type RESOURCE_MONITOR_STATE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonInitializing: RESOURCE_MONITOR_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonIdle: RESOURCE_MONITOR_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonStartingResource: RESOURCE_MONITOR_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonInitializingResource: RESOURCE_MONITOR_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonOnlineResource: RESOURCE_MONITOR_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonOfflineResource: RESOURCE_MONITOR_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonShutdownResource: RESOURCE_MONITOR_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonDeletingResource: RESOURCE_MONITOR_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonIsAlivePoll: RESOURCE_MONITOR_STATE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonLooksAlivePoll: RESOURCE_MONITOR_STATE = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonArbitrateResource: RESOURCE_MONITOR_STATE = 10i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonReleaseResource: RESOURCE_MONITOR_STATE = 11i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonResourceControl: RESOURCE_MONITOR_STATE = 12i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonResourceTypeControl: RESOURCE_MONITOR_STATE = 13i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonTerminateResource: RESOURCE_MONITOR_STATE = 14i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const RmonDeadlocked: RESOURCE_MONITOR_STATE = 15i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type SR_DISK_REPLICATION_ELIGIBLE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleNone: SR_DISK_REPLICATION_ELIGIBLE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleYes: SR_DISK_REPLICATION_ELIGIBLE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleOffline: SR_DISK_REPLICATION_ELIGIBLE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleNotGpt: SR_DISK_REPLICATION_ELIGIBLE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligiblePartitionLayoutMismatch: SR_DISK_REPLICATION_ELIGIBLE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleInsufficientFreeSpace: SR_DISK_REPLICATION_ELIGIBLE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleNotInSameSite: SR_DISK_REPLICATION_ELIGIBLE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleInSameSite: SR_DISK_REPLICATION_ELIGIBLE = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleFileSystemNotSupported: SR_DISK_REPLICATION_ELIGIBLE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleAlreadyInReplication: SR_DISK_REPLICATION_ELIGIBLE = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleSameAsSpecifiedDisk: SR_DISK_REPLICATION_ELIGIBLE = 10i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrDiskReplicationEligibleOther: SR_DISK_REPLICATION_ELIGIBLE = 9999i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type SR_REPLICATED_DISK_TYPE = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeNone: SR_REPLICATED_DISK_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeSource: SR_REPLICATED_DISK_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeLogSource: SR_REPLICATED_DISK_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeDestination: SR_REPLICATED_DISK_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeLogDestination: SR_REPLICATED_DISK_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeNotInParthership: SR_REPLICATED_DISK_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeLogNotInParthership: SR_REPLICATED_DISK_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const SrReplicatedDiskTypeOther: SR_REPLICATED_DISK_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type VM_RESDLL_CONTEXT = i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextTurnOff: VM_RESDLL_CONTEXT = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextSave: VM_RESDLL_CONTEXT = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextShutdown: VM_RESDLL_CONTEXT = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextShutdownForce: VM_RESDLL_CONTEXT = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub const VmResdllContextLiveMigration: VM_RESDLL_CONTEXT = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLRES_CALLBACK_FUNCTION_TABLE {
    pub LogEvent: PLOG_EVENT_ROUTINE,
    pub SetResourceStatusEx: PSET_RESOURCE_STATUS_ROUTINE_EX,
    pub SetResourceLockedMode: PSET_RESOURCE_LOCKED_MODE_ROUTINE,
    pub SignalFailure: PSIGNAL_FAILURE_ROUTINE,
    pub SetResourceInMemoryNodeLocalProperties: PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE,
    pub EndControlCall: PEND_CONTROL_CALL,
    pub EndTypeControlCall: PEND_TYPE_CONTROL_CALL,
    pub ExtendControlCall: PEXTEND_RES_CONTROL_CALL,
    pub ExtendTypeControlCall: PEXTEND_RES_TYPE_CONTROL_CALL,
    pub RaiseResTypeNotification: PRAISE_RES_TYPE_NOTIFICATION,
    pub ChangeResourceProcessForDumps: PCHANGE_RESOURCE_PROCESS_FOR_DUMPS,
    pub ChangeResTypeProcessForDumps: PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS,
    pub SetInternalState: PSET_INTERNAL_STATE,
    pub SetResourceLockedModeEx: PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE,
    pub RequestDump: PREQUEST_DUMP_ROUTINE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLRES_CALLBACK_FUNCTION_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLRES_CALLBACK_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_FUNCTION_TABLE {
    pub TableSize: u32,
    pub Version: u32,
    pub Anonymous: CLRES_FUNCTION_TABLE_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_FUNCTION_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub union CLRES_FUNCTION_TABLE_0 {
    pub V1Functions: CLRES_V1_FUNCTIONS,
    pub V2Functions: CLRES_V2_FUNCTIONS,
    pub V3Functions: CLRES_V3_FUNCTIONS,
    pub V4Functions: CLRES_V4_FUNCTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_FUNCTION_TABLE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_FUNCTION_TABLE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V1_FUNCTIONS {
    pub Open: POPEN_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_ROUTINE,
    pub Offline: POFFLINE_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V1_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V1_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V2_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V2_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V2_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V3_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V3_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V3_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V4_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
    pub BeginResourceControlAsUser: PBEGIN_RESCALL_AS_USER_ROUTINE,
    pub BeginResourceTypeControlAsUser: PBEGIN_RESTYPECALL_AS_USER_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V4_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V4_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    pub GetTickCount64: u64,
    pub GetSystemTime: super::super::Foundation::SYSTEMTIME,
    pub NodeId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub eReason: CLUSTER_RESOURCE_STATE_CHANGE_REASON,
}
impl ::core::marker::Copy for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {}
impl ::core::clone::Clone for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    pub dwFlags: u32,
    pub guidPoolFilter: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {}
impl ::core::clone::Clone for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_BINARY {
    pub Base: CLUSPROP_VALUE,
    pub rgb: [u8; 1],
}
impl ::core::marker::Copy for CLUSPROP_BINARY {}
impl ::core::clone::Clone for CLUSPROP_BINARY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union CLUSPROP_BUFFER_HELPER {
    pub pb: *mut u8,
    pub pw: *mut u16,
    pub pdw: *mut u32,
    pub pl: *mut i32,
    pub psz: ::windows_sys::core::PWSTR,
    pub pList: *mut CLUSPROP_LIST,
    pub pSyntax: *mut CLUSPROP_SYNTAX,
    pub pName: *mut CLUSPROP_SZ,
    pub pValue: *mut CLUSPROP_VALUE,
    pub pBinaryValue: *mut CLUSPROP_BINARY,
    pub pWordValue: *mut CLUSPROP_WORD,
    pub pDwordValue: *mut CLUSPROP_DWORD,
    pub pLongValue: *mut CLUSPROP_LONG,
    pub pULargeIntegerValue: *mut CLUSPROP_ULARGE_INTEGER,
    pub pLargeIntegerValue: *mut CLUSPROP_LARGE_INTEGER,
    pub pStringValue: *mut CLUSPROP_SZ,
    pub pMultiSzValue: *mut CLUSPROP_SZ,
    pub pSecurityDescriptor: *mut CLUSPROP_SECURITY_DESCRIPTOR,
    pub pResourceClassValue: *mut CLUSPROP_RESOURCE_CLASS,
    pub pResourceClassInfoValue: *mut CLUSPROP_RESOURCE_CLASS_INFO,
    pub pDiskSignatureValue: *mut CLUSPROP_DWORD,
    pub pScsiAddressValue: *mut CLUSPROP_SCSI_ADDRESS,
    pub pDiskNumberValue: *mut CLUSPROP_DWORD,
    pub pPartitionInfoValue: *mut CLUSPROP_PARTITION_INFO,
    pub pRequiredDependencyValue: *mut CLUSPROP_REQUIRED_DEPENDENCY,
    pub pPartitionInfoValueEx: *mut CLUSPROP_PARTITION_INFO_EX,
    pub pPartitionInfoValueEx2: *mut CLUSPROP_PARTITION_INFO_EX2,
    pub pFileTimeValue: *mut CLUSPROP_FILETIME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for CLUSPROP_BUFFER_HELPER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for CLUSPROP_BUFFER_HELPER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_DWORD {
    pub Base: CLUSPROP_VALUE,
    pub dw: u32,
}
impl ::core::marker::Copy for CLUSPROP_DWORD {}
impl ::core::clone::Clone for CLUSPROP_DWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSPROP_FILETIME {
    pub Base: CLUSPROP_VALUE,
    pub ft: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSPROP_FILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSPROP_FILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_FTSET_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_FTSET_INFO,
}
impl ::core::marker::Copy for CLUSPROP_FTSET_INFO {}
impl ::core::clone::Clone for CLUSPROP_FTSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_LARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: i64,
}
impl ::core::marker::Copy for CLUSPROP_LARGE_INTEGER {}
impl ::core::clone::Clone for CLUSPROP_LARGE_INTEGER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_LIST {
    pub nPropertyCount: u32,
    pub PropertyName: CLUSPROP_SZ,
}
impl ::core::marker::Copy for CLUSPROP_LIST {}
impl ::core::clone::Clone for CLUSPROP_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_LONG {
    pub Base: CLUSPROP_VALUE,
    pub l: i32,
}
impl ::core::marker::Copy for CLUSPROP_LONG {}
impl ::core::clone::Clone for CLUSPROP_LONG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_PARTITION_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO,
}
impl ::core::marker::Copy for CLUSPROP_PARTITION_INFO {}
impl ::core::clone::Clone for CLUSPROP_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_PARTITION_INFO_EX {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO_EX,
}
impl ::core::marker::Copy for CLUSPROP_PARTITION_INFO_EX {}
impl ::core::clone::Clone for CLUSPROP_PARTITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_PARTITION_INFO_EX2 {
    pub Base: CLUSPROP_PARTITION_INFO_EX,
    pub Base2: CLUS_PARTITION_INFO_EX2,
}
impl ::core::marker::Copy for CLUSPROP_PARTITION_INFO_EX2 {}
impl ::core::clone::Clone for CLUSPROP_PARTITION_INFO_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUSPROP_REQUIRED_DEPENDENCY {
    pub Value: CLUSPROP_VALUE,
    pub ResClass: CLUSPROP_RESOURCE_CLASS,
    pub ResTypeName: CLUSPROP_SZ,
}
impl ::core::marker::Copy for CLUSPROP_REQUIRED_DEPENDENCY {}
impl ::core::clone::Clone for CLUSPROP_REQUIRED_DEPENDENCY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_RESOURCE_CLASS {
    pub Base: CLUSPROP_VALUE,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl ::core::marker::Copy for CLUSPROP_RESOURCE_CLASS {}
impl ::core::clone::Clone for CLUSPROP_RESOURCE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_RESOURCE_CLASS_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_RESOURCE_CLASS_INFO,
}
impl ::core::marker::Copy for CLUSPROP_RESOURCE_CLASS_INFO {}
impl ::core::clone::Clone for CLUSPROP_RESOURCE_CLASS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_SCSI_ADDRESS {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_SCSI_ADDRESS,
}
impl ::core::marker::Copy for CLUSPROP_SCSI_ADDRESS {}
impl ::core::clone::Clone for CLUSPROP_SCSI_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct CLUSPROP_SECURITY_DESCRIPTOR {
    pub Base: CLUSPROP_VALUE,
    pub Anonymous: CLUSPROP_SECURITY_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for CLUSPROP_SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for CLUSPROP_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub union CLUSPROP_SECURITY_DESCRIPTOR_0 {
    pub sd: super::super::Security::SECURITY_DESCRIPTOR_RELATIVE,
    pub rgbSecurityDescriptor: [u8; 1],
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for CLUSPROP_SECURITY_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for CLUSPROP_SECURITY_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUSPROP_SYNTAX {
    pub dw: u32,
    pub Anonymous: CLUSPROP_SYNTAX_0,
}
impl ::core::marker::Copy for CLUSPROP_SYNTAX {}
impl ::core::clone::Clone for CLUSPROP_SYNTAX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_SYNTAX_0 {
    pub wFormat: u16,
    pub wType: u16,
}
impl ::core::marker::Copy for CLUSPROP_SYNTAX_0 {}
impl ::core::clone::Clone for CLUSPROP_SYNTAX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_SZ {
    pub Base: CLUSPROP_VALUE,
    pub sz: [u16; 1],
}
impl ::core::marker::Copy for CLUSPROP_SZ {}
impl ::core::clone::Clone for CLUSPROP_SZ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_ULARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: u64,
}
impl ::core::marker::Copy for CLUSPROP_ULARGE_INTEGER {}
impl ::core::clone::Clone for CLUSPROP_ULARGE_INTEGER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_VALUE {
    pub Syntax: CLUSPROP_SYNTAX,
    pub cbLength: u32,
}
impl ::core::marker::Copy for CLUSPROP_VALUE {}
impl ::core::clone::Clone for CLUSPROP_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSPROP_WORD {
    pub Base: CLUSPROP_VALUE,
    pub w: u16,
}
impl ::core::marker::Copy for CLUSPROP_WORD {}
impl ::core::clone::Clone for CLUSPROP_WORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTERVERSIONINFO {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
    pub dwClusterHighestVersion: u32,
    pub dwClusterLowestVersion: u32,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for CLUSTERVERSIONINFO {}
impl ::core::clone::Clone for CLUSTERVERSIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTERVERSIONINFO_NT4 {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
}
impl ::core::marker::Copy for CLUSTERVERSIONINFO_NT4 {}
impl ::core::clone::Clone for CLUSTERVERSIONINFO_NT4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_AVAILABILITY_SET_CONFIG {
    pub dwVersion: u32,
    pub dwUpdateDomains: u32,
    pub dwFaultDomains: u32,
    pub bReserveSpareNode: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_AVAILABILITY_SET_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_AVAILABILITY_SET_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzName: ::windows_sys::core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl ::core::marker::Copy for CLUSTER_BATCH_COMMAND {}
impl ::core::clone::Clone for CLUSTER_BATCH_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_CREATE_GROUP_INFO {
    pub dwVersion: u32,
    pub groupType: CLUSGROUP_TYPE,
}
impl ::core::marker::Copy for CLUSTER_CREATE_GROUP_INFO {}
impl ::core::clone::Clone for CLUSTER_CREATE_GROUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_ENUM_ITEM {
    pub dwVersion: u32,
    pub dwType: u32,
    pub cbId: u32,
    pub lpszId: ::windows_sys::core::PWSTR,
    pub cbName: u32,
    pub lpszName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CLUSTER_ENUM_ITEM {}
impl ::core::clone::Clone for CLUSTER_ENUM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_GROUP_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: ::windows_sys::core::PWSTR,
    pub cbName: u32,
    pub lpszName: ::windows_sys::core::PWSTR,
    pub state: CLUSTER_GROUP_STATE,
    pub cbOwnerNode: u32,
    pub lpszOwnerNode: ::windows_sys::core::PWSTR,
    pub dwFlags: u32,
    pub cbProperties: u32,
    pub pProperties: *mut ::core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CLUSTER_GROUP_ENUM_ITEM {}
impl ::core::clone::Clone for CLUSTER_GROUP_ENUM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_HEALTH_FAULT {
    pub Id: ::windows_sys::core::PWSTR,
    pub ErrorType: u32,
    pub ErrorCode: u32,
    pub Description: ::windows_sys::core::PWSTR,
    pub Provider: ::windows_sys::core::PWSTR,
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for CLUSTER_HEALTH_FAULT {}
impl ::core::clone::Clone for CLUSTER_HEALTH_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_HEALTH_FAULT_ARRAY {
    pub numFaults: u32,
    pub faults: *mut CLUSTER_HEALTH_FAULT,
}
impl ::core::marker::Copy for CLUSTER_HEALTH_FAULT_ARRAY {}
impl ::core::clone::Clone for CLUSTER_HEALTH_FAULT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_IP_ENTRY {
    pub lpszIpAddress: ::windows_sys::core::PCWSTR,
    pub dwPrefixLength: u32,
}
impl ::core::marker::Copy for CLUSTER_IP_ENTRY {}
impl ::core::clone::Clone for CLUSTER_IP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_MEMBERSHIP_INFO {
    pub HasQuorum: super::super::Foundation::BOOL,
    pub UpnodesSize: u32,
    pub Upnodes: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_MEMBERSHIP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_MEMBERSHIP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_READ_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzSubkeyName: ::windows_sys::core::PCWSTR,
    pub wzValueName: ::windows_sys::core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl ::core::marker::Copy for CLUSTER_READ_BATCH_COMMAND {}
impl ::core::clone::Clone for CLUSTER_READ_BATCH_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_RESOURCE_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: ::windows_sys::core::PWSTR,
    pub cbName: u32,
    pub lpszName: ::windows_sys::core::PWSTR,
    pub cbOwnerGroupName: u32,
    pub lpszOwnerGroupName: ::windows_sys::core::PWSTR,
    pub cbOwnerGroupId: u32,
    pub lpszOwnerGroupId: ::windows_sys::core::PWSTR,
    pub cbProperties: u32,
    pub pProperties: *mut ::core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CLUSTER_RESOURCE_ENUM_ITEM {}
impl ::core::clone::Clone for CLUSTER_RESOURCE_ENUM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_SET_PASSWORD_STATUS {
    pub NodeId: u32,
    pub SetAttempted: super::super::Foundation::BOOLEAN,
    pub ReturnStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_SET_PASSWORD_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_SET_PASSWORD_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    pub NewVolumeName: [u16; 260],
    pub NewVolumeGuid: [u16; 50],
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    pub NewVolumeName: [u16; 260],
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    pub InputType: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE,
    pub Anonymous: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    pub VolumeOffset: u64,
    pub VolumeId: [u16; 260],
    pub VolumeName: [u16; 260],
    pub VolumeGuid: [u16; 50],
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_STATE_INFO {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_STATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub RedirectedIOReason: u64,
    pub VolumeRedirectedIOReason: u64,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_VALIDATE_CSV_FILENAME {
    pub szFileName: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_CSV_FILENAME {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_CSV_FILENAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_VALIDATE_DIRECTORY {
    pub szPath: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_DIRECTORY {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_DIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_VALIDATE_NETNAME {
    pub szNetworkName: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_NETNAME {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_NETNAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUSTER_VALIDATE_PATH {
    pub szPath: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_PATH {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_PATH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CHKDSK_INFO {
    pub PartitionNumber: u32,
    pub ChkdskState: u32,
    pub FileIdCount: u32,
    pub FileIdList: [u64; 1],
}
impl ::core::marker::Copy for CLUS_CHKDSK_INFO {}
impl ::core::clone::Clone for CLUS_CHKDSK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    pub FileServerName: [u16; 16],
}
impl ::core::marker::Copy for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {}
impl ::core::clone::Clone for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    pub FileServerName: [u16; 260],
}
impl ::core::marker::Copy for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {}
impl ::core::clone::Clone for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_CSV_MAINTENANCE_MODE_INFO {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub VolumeName: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_CSV_MAINTENANCE_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_CSV_MAINTENANCE_MODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CSV_VOLUME_INFO {
    pub VolumeOffset: u64,
    pub PartitionNumber: u32,
    pub FaultState: CLUSTER_CSV_VOLUME_FAULT_STATE,
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub szVolumeName: [u16; 50],
}
impl ::core::marker::Copy for CLUS_CSV_VOLUME_INFO {}
impl ::core::clone::Clone for CLUS_CSV_VOLUME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_CSV_VOLUME_NAME {
    pub VolumeOffset: i64,
    pub szVolumeName: [u16; 260],
    pub szRootPath: [u16; 263],
}
impl ::core::marker::Copy for CLUS_CSV_VOLUME_NAME {}
impl ::core::clone::Clone for CLUS_CSV_VOLUME_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_DISK_NUMBER_INFO {
    pub DiskNumber: u32,
    pub BytesPerSector: u32,
}
impl ::core::marker::Copy for CLUS_DISK_NUMBER_INFO {}
impl ::core::clone::Clone for CLUS_DISK_NUMBER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_DNN_LEADER_STATUS {
    pub IsOnline: super::super::Foundation::BOOL,
    pub IsFileServerPresent: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_DNN_LEADER_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_DNN_LEADER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_DNN_SODAFS_CLONE_STATUS {
    pub NodeId: u32,
    pub Status: CLUSTER_RESOURCE_STATE,
}
impl ::core::marker::Copy for CLUS_DNN_SODAFS_CLONE_STATUS {}
impl ::core::clone::Clone for CLUS_DNN_SODAFS_CLONE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_FORCE_QUORUM_INFO {
    pub dwSize: u32,
    pub dwNodeBitMask: u32,
    pub dwMaxNumberofNodes: u32,
    pub multiszNodeList: [u16; 1],
}
impl ::core::marker::Copy for CLUS_FORCE_QUORUM_INFO {}
impl ::core::clone::Clone for CLUS_FORCE_QUORUM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_FTSET_INFO {
    pub dwRootSignature: u32,
    pub dwFtType: u32,
}
impl ::core::marker::Copy for CLUS_FTSET_INFO {}
impl ::core::clone::Clone for CLUS_FTSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_MAINTENANCE_MODE_INFO {
    pub InMaintenance: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_MAINTENANCE_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_MAINTENANCE_MODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_MAINTENANCE_MODE_INFOEX {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub MaintainenceModeType: MAINTENANCE_MODE_TYPE_ENUM,
    pub InternalState: CLUSTER_RESOURCE_STATE,
    pub Signature: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_MAINTENANCE_MODE_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_MAINTENANCE_MODE_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_NETNAME_IP_INFO_ENTRY {
    pub NodeId: u32,
    pub AddressSize: u32,
    pub Address: [u8; 1],
}
impl ::core::marker::Copy for CLUS_NETNAME_IP_INFO_ENTRY {}
impl ::core::clone::Clone for CLUS_NETNAME_IP_INFO_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    pub szName: [u16; 64],
    pub NumEntries: u32,
    pub IpInfo: [CLUS_NETNAME_IP_INFO_ENTRY; 1],
}
impl ::core::marker::Copy for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {}
impl ::core::clone::Clone for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_NETNAME_PWD_INFO {
    pub Flags: u32,
    pub Password: [u16; 16],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl ::core::marker::Copy for CLUS_NETNAME_PWD_INFO {}
impl ::core::clone::Clone for CLUS_NETNAME_PWD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_NETNAME_PWD_INFOEX {
    pub Flags: u32,
    pub Password: [u16; 128],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl ::core::marker::Copy for CLUS_NETNAME_PWD_INFOEX {}
impl ::core::clone::Clone for CLUS_NETNAME_PWD_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_NETNAME_VS_TOKEN_INFO {
    pub ProcessID: u32,
    pub DesiredAccess: u32,
    pub InheritHandle: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_NETNAME_VS_TOKEN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_NETNAME_VS_TOKEN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_PARTITION_INFO {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
}
impl ::core::marker::Copy for CLUS_PARTITION_INFO {}
impl ::core::clone::Clone for CLUS_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_PARTITION_INFO_EX {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
    pub TotalSizeInBytes: u64,
    pub FreeSizeInBytes: u64,
    pub DeviceNumber: u32,
    pub PartitionNumber: u32,
    pub VolumeGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CLUS_PARTITION_INFO_EX {}
impl ::core::clone::Clone for CLUS_PARTITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_PARTITION_INFO_EX2 {
    pub GptPartitionId: ::windows_sys::core::GUID,
    pub szPartitionName: [u16; 260],
    pub EncryptionFlags: u32,
}
impl ::core::marker::Copy for CLUS_PARTITION_INFO_EX2 {}
impl ::core::clone::Clone for CLUS_PARTITION_INFO_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_PROVIDER_STATE_CHANGE_INFO {
    pub dwSize: u32,
    pub resourceState: CLUSTER_RESOURCE_STATE,
    pub szProviderId: [u16; 1],
}
impl ::core::marker::Copy for CLUS_PROVIDER_STATE_CHANGE_INFO {}
impl ::core::clone::Clone for CLUS_PROVIDER_STATE_CHANGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_RESOURCE_CLASS_INFO {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUS_RESOURCE_CLASS_INFO_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0,
    pub li: u64,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO_0 {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_RESOURCE_CLASS_INFO_0_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0_0,
    pub SubClass: u32,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO_0_0 {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    pub dw: u32,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO_0_0_0 {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_SCSI_ADDRESS {
    pub Anonymous: CLUS_SCSI_ADDRESS_0,
}
impl ::core::marker::Copy for CLUS_SCSI_ADDRESS {}
impl ::core::clone::Clone for CLUS_SCSI_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub union CLUS_SCSI_ADDRESS_0 {
    pub Anonymous: CLUS_SCSI_ADDRESS_0_0,
    pub dw: u32,
}
impl ::core::marker::Copy for CLUS_SCSI_ADDRESS_0 {}
impl ::core::clone::Clone for CLUS_SCSI_ADDRESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_SCSI_ADDRESS_0_0 {
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
impl ::core::marker::Copy for CLUS_SCSI_ADDRESS_0_0 {}
impl ::core::clone::Clone for CLUS_SCSI_ADDRESS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_SET_MAINTENANCE_MODE_INPUT {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub ExtraParameterSize: u32,
    pub ExtraParameter: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_SET_MAINTENANCE_MODE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_SET_MAINTENANCE_MODE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_SHARED_VOLUME_BACKUP_MODE {
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub DelayTimerInSecs: u32,
    pub VolumeName: [u16; 260],
}
impl ::core::marker::Copy for CLUS_SHARED_VOLUME_BACKUP_MODE {}
impl ::core::clone::Clone for CLUS_SHARED_VOLUME_BACKUP_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_STARTING_PARAMS {
    pub dwSize: u32,
    pub bForm: super::super::Foundation::BOOL,
    pub bFirst: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_STARTING_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_STARTING_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    pub AvailDrivelettersMask: u32,
}
impl ::core::marker::Copy for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {}
impl ::core::clone::Clone for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_STORAGE_REMAP_DRIVELETTER {
    pub CurrentDriveLetterMask: u32,
    pub TargetDriveLetterMask: u32,
}
impl ::core::marker::Copy for CLUS_STORAGE_REMAP_DRIVELETTER {}
impl ::core::clone::Clone for CLUS_STORAGE_REMAP_DRIVELETTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct CLUS_STORAGE_SET_DRIVELETTER {
    pub PartitionNumber: u32,
    pub DriveLetterMask: u32,
}
impl ::core::marker::Copy for CLUS_STORAGE_SET_DRIVELETTER {}
impl ::core::clone::Clone for CLUS_STORAGE_SET_DRIVELETTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_WORKER {
    pub hThread: super::super::Foundation::HANDLE,
    pub Terminate: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_WORKER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_WORKER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_CLUSTER_CONFIG {
    pub dwVersion: u32,
    pub lpszClusterName: ::windows_sys::core::PCWSTR,
    pub cNodes: u32,
    pub ppszNodeNames: *mut ::windows_sys::core::PWSTR,
    pub cIpEntries: u32,
    pub pIpEntries: *mut CLUSTER_IP_ENTRY,
    pub fEmptyCluster: super::super::Foundation::BOOLEAN,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_CLUSTER_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_CLUSTER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_CLUSTER_NAME_ACCOUNT {
    pub dwVersion: u32,
    pub lpszClusterName: ::windows_sys::core::PCWSTR,
    pub dwFlags: u32,
    pub pszUserName: ::windows_sys::core::PCWSTR,
    pub pszPassword: ::windows_sys::core::PCWSTR,
    pub pszDomain: ::windows_sys::core::PCWSTR,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
    pub bUpgradeVCOs: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_CLUSTER_NAME_ACCOUNT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_CLUSTER_NAME_ACCOUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct FILESHARE_CHANGE {
    pub Change: FILESHARE_CHANGE_ENUM,
    pub ShareName: [u16; 84],
}
impl ::core::marker::Copy for FILESHARE_CHANGE {}
impl ::core::clone::Clone for FILESHARE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct FILESHARE_CHANGE_LIST {
    pub NumEntries: u32,
    pub ChangeEntry: [FILESHARE_CHANGE; 1],
}
impl ::core::marker::Copy for FILESHARE_CHANGE_LIST {}
impl ::core::clone::Clone for FILESHARE_CHANGE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct GET_OPERATION_CONTEXT_PARAMS {
    pub Size: u32,
    pub Version: u32,
    pub Type: RESDLL_CONTEXT_OPERATION_TYPE,
    pub Priority: u32,
}
impl ::core::marker::Copy for GET_OPERATION_CONTEXT_PARAMS {}
impl ::core::clone::Clone for GET_OPERATION_CONTEXT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct GROUP_FAILURE_INFO {
    pub dwFailoverAttemptsRemaining: u32,
    pub dwFailoverPeriodRemaining: u32,
}
impl ::core::marker::Copy for GROUP_FAILURE_INFO {}
impl ::core::clone::Clone for GROUP_FAILURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct GROUP_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: GROUP_FAILURE_INFO,
}
impl ::core::marker::Copy for GROUP_FAILURE_INFO_BUFFER {}
impl ::core::clone::Clone for GROUP_FAILURE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MONITOR_STATE {
    pub LastUpdate: i64,
    pub State: RESOURCE_MONITOR_STATE,
    pub ActiveResource: super::super::Foundation::HANDLE,
    pub ResmonStop: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONITOR_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONITOR_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct NOTIFY_FILTER_AND_TYPE {
    pub dwObjectType: u32,
    pub FilterFlags: i64,
}
impl ::core::marker::Copy for NOTIFY_FILTER_AND_TYPE {}
impl ::core::clone::Clone for NOTIFY_FILTER_AND_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct NodeUtilizationInfoElement {
    pub Id: u64,
    pub AvailableMemory: u64,
    pub AvailableMemoryAfterReclamation: u64,
}
impl ::core::marker::Copy for NodeUtilizationInfoElement {}
impl ::core::clone::Clone for NodeUtilizationInfoElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct POST_UPGRADE_VERSION_INFO {
    pub newMajorVersion: u32,
    pub newUpgradeVersion: u32,
    pub oldMajorVersion: u32,
    pub oldUpgradeVersion: u32,
    pub reserved: u32,
}
impl ::core::marker::Copy for POST_UPGRADE_VERSION_INFO {}
impl ::core::clone::Clone for POST_UPGRADE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct PaxosTagCStruct {
    pub __padding__PaxosTagVtable: u64,
    pub __padding__NextEpochVtable: u64,
    pub __padding__NextEpoch_DateTimeVtable: u64,
    pub NextEpoch_DateTime_ticks: u64,
    pub NextEpoch_Value: i32,
    pub __padding__BoundryNextEpoch: u32,
    pub __padding__EpochVtable: u64,
    pub __padding__Epoch_DateTimeVtable: u64,
    pub Epoch_DateTime_ticks: u64,
    pub Epoch_Value: i32,
    pub __padding__BoundryEpoch: u32,
    pub Sequence: i32,
    pub __padding__BoundrySequence: u32,
}
impl ::core::marker::Copy for PaxosTagCStruct {}
impl ::core::clone::Clone for PaxosTagCStruct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct RESOURCE_FAILURE_INFO {
    pub dwRestartAttemptsRemaining: u32,
    pub dwRestartPeriodRemaining: u32,
}
impl ::core::marker::Copy for RESOURCE_FAILURE_INFO {}
impl ::core::clone::Clone for RESOURCE_FAILURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct RESOURCE_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: RESOURCE_FAILURE_INFO,
}
impl ::core::marker::Copy for RESOURCE_FAILURE_INFO_BUFFER {}
impl ::core::clone::Clone for RESOURCE_FAILURE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCE_STATUS {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub WaitHint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCE_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCE_STATUS_EX {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub ApplicationSpecificErrorCode: u32,
    pub Flags: u32,
    pub WaitHint: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCE_STATUS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCE_STATUS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    pub isTerminalFailure: super::super::Foundation::BOOL,
    pub restartPeriodRemaining: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESUTIL_FILETIME_DATA {
    pub Default: super::super::Foundation::FILETIME,
    pub Minimum: super::super::Foundation::FILETIME,
    pub Maximum: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESUTIL_FILETIME_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESUTIL_FILETIME_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct RESUTIL_LARGEINT_DATA {
    pub Default: i64,
    pub Minimum: i64,
    pub Maximum: i64,
}
impl ::core::marker::Copy for RESUTIL_LARGEINT_DATA {}
impl ::core::clone::Clone for RESUTIL_LARGEINT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESUTIL_PROPERTY_ITEM {
    pub Name: ::windows_sys::core::PWSTR,
    pub KeyName: ::windows_sys::core::PWSTR,
    pub Format: u32,
    pub Anonymous: RESUTIL_PROPERTY_ITEM_0,
    pub Minimum: u32,
    pub Maximum: u32,
    pub Flags: u32,
    pub Offset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESUTIL_PROPERTY_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESUTIL_PROPERTY_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union RESUTIL_PROPERTY_ITEM_0 {
    pub DefaultPtr: usize,
    pub Default: u32,
    pub lpDefault: *mut ::core::ffi::c_void,
    pub LargeIntData: *mut RESUTIL_LARGEINT_DATA,
    pub ULargeIntData: *mut RESUTIL_ULARGEINT_DATA,
    pub FileTimeData: *mut RESUTIL_FILETIME_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESUTIL_PROPERTY_ITEM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESUTIL_PROPERTY_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct RESUTIL_ULARGEINT_DATA {
    pub Default: u64,
    pub Minimum: u64,
    pub Maximum: u64,
}
impl ::core::marker::Copy for RESUTIL_ULARGEINT_DATA {}
impl ::core::clone::Clone for RESUTIL_ULARGEINT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct ResourceUtilizationInfoElement {
    pub PhysicalNumaId: u64,
    pub CurrentMemory: u64,
}
impl ::core::marker::Copy for ResourceUtilizationInfoElement {}
impl ::core::clone::Clone for ResourceUtilizationInfoElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    pub ReplicationGroupName: [u16; 260],
    pub Description: [u16; 260],
    pub LogPath: [u16; 260],
    pub MaxLogSizeInBytes: u64,
    pub LogType: u16,
    pub ReplicationMode: u32,
    pub MinimumPartnersInSync: u32,
    pub EnableWriteConsistency: super::super::Foundation::BOOLEAN,
    pub EnableEncryption: super::super::Foundation::BOOLEAN,
    pub CertificateThumbprint: [u16; 260],
    pub VolumeNameCount: u32,
    pub VolumeNames: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    pub Result: u32,
    pub ErrorString: [u16; 260],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_DISK_INFO {
    pub Reason: SR_DISK_REPLICATION_ELIGIBLE,
    pub DiskGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_DISK_INFO {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_DISK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    pub Count: u16,
    pub DiskInfo: [SR_RESOURCE_TYPE_DISK_INFO; 1],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    pub DataDiskGuid: ::windows_sys::core::GUID,
    pub IncludeOfflineDisks: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    pub DataDiskGuid: ::windows_sys::core::GUID,
    pub IncludeAvailableStoargeDisks: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    pub SourceDataDiskGuid: ::windows_sys::core::GUID,
    pub TargetReplicationGroupGuid: ::windows_sys::core::GUID,
    pub SkipConnectivityCheck: super::super::Foundation::BOOLEAN,
    pub IncludeOfflineDisks: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISK {
    pub Type: SR_REPLICATED_DISK_TYPE,
    pub ClusterDiskResourceGuid: ::windows_sys::core::GUID,
    pub ReplicationGroupId: ::windows_sys::core::GUID,
    pub ReplicationGroupName: [u16; 260],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_DISK {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_DISK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    pub Count: u16,
    pub ReplicatedDisks: [SR_RESOURCE_TYPE_REPLICATED_DISK; 1],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    pub Count: u32,
    pub PartitionArray: [SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO; 1],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    pub PartitionOffset: u64,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct WitnessTagHelper {
    pub Version: i32,
    pub paxosToValidate: PaxosTagCStruct,
}
impl ::core::marker::Copy for WitnessTagHelper {}
impl ::core::clone::Clone for WitnessTagHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub struct WitnessTagUpdateHelper {
    pub Version: i32,
    pub paxosToSet: PaxosTagCStruct,
    pub paxosToValidate: PaxosTagCStruct,
}
impl ::core::marker::Copy for WitnessTagUpdateHelper {}
impl ::core::clone::Clone for WitnessTagUpdateHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _HCHANGE(pub u8);
#[repr(C)]
pub struct _HCLUSCRYPTPROVIDER(pub u8);
#[repr(C)]
pub struct _HCLUSENUM(pub u8);
#[repr(C)]
pub struct _HCLUSENUMEX(pub u8);
#[repr(C)]
pub struct _HCLUSTER(pub u8);
#[repr(C)]
pub struct _HGROUP(pub u8);
#[repr(C)]
pub struct _HGROUPENUM(pub u8);
#[repr(C)]
pub struct _HGROUPENUMEX(pub u8);
#[repr(C)]
pub struct _HGROUPSET(pub u8);
#[repr(C)]
pub struct _HGROUPSETENUM(pub u8);
#[repr(C)]
pub struct _HNETINTERFACE(pub u8);
#[repr(C)]
pub struct _HNETINTERFACEENUM(pub u8);
#[repr(C)]
pub struct _HNETWORK(pub u8);
#[repr(C)]
pub struct _HNETWORKENUM(pub u8);
#[repr(C)]
pub struct _HNODE(pub u8);
#[repr(C)]
pub struct _HNODEENUM(pub u8);
#[repr(C)]
pub struct _HNODEENUMEX(pub u8);
#[repr(C)]
pub struct _HREGBATCH(pub u8);
#[repr(C)]
pub struct _HREGBATCHNOTIFICATION(pub u8);
#[repr(C)]
pub struct _HREGBATCHPORT(pub u8);
#[repr(C)]
pub struct _HREGREADBATCH(pub u8);
#[repr(C)]
pub struct _HREGREADBATCHREPLY(pub u8);
#[repr(C)]
pub struct _HRESENUM(pub u8);
#[repr(C)]
pub struct _HRESENUMEX(pub u8);
#[repr(C)]
pub struct _HRESOURCE(pub u8);
#[repr(C)]
pub struct _HRESTYPEENUM(pub u8);
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type LPGROUP_CALLBACK_EX = ::core::option::Option<unsafe extern "system" fn(param0: *mut _HCLUSTER, param1: *mut _HGROUP, param2: *mut _HGROUP, param3: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type LPNODE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut _HCLUSTER, param1: *mut _HNODE, param2: CLUSTER_NODE_STATE, param3: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type LPRESOURCE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut _HRESOURCE, param1: *mut _HRESOURCE, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type LPRESOURCE_CALLBACK_EX = ::core::option::Option<unsafe extern "system" fn(param0: *mut _HCLUSTER, param1: *mut _HRESOURCE, param2: *mut _HRESOURCE, param3: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PARBITRATE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, lostquorumresource: PQUORUM_RESOURCE_LOST) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESCALL_AS_USER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESCALL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESTYPECALL_AS_USER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetypename: ::windows_sys::core::PCWSTR, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESTYPECALL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetypename: ::windows_sys::core::PCWSTR, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCANCEL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, cancelflags_reserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCHANGE_RESOURCE_PROCESS_FOR_DUMPS = ::core::option::Option<unsafe extern "system" fn(resource: isize, processname: ::windows_sys::core::PCWSTR, processid: u32, isadd: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS = ::core::option::Option<unsafe extern "system" fn(resourcetypename: ::windows_sys::core::PCWSTR, processname: ::windows_sys::core::PCWSTR, processid: u32, isadd: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLOSE_CLUSTER_CRYPT_PROVIDER = ::core::option::Option<unsafe extern "system" fn(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLOSE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPIClusWorkerCheckTerminate = ::core::option::Option<unsafe extern "system" fn(lpworker: *mut CLUS_WORKER) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_ADD_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows_sys::core::PCWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_ADD_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows_sys::core::PCWSTR, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hdependson: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hnode: *mut _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_CROSS_CLUSTER_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: ::windows_sys::core::PCWSTR, lpremotegroupsetname: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ADD_RESOURCE_TO_CLUSTER_SHARED_VOLUMES = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_BACKUP_CLUSTER_DATABASE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszpathname: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CAN_RESOURCE_BE_DEPENDENT = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hresourcedependent: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hgroup: *mut _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hgroup: *mut _HGROUP, flags: u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NETWORK = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NET_INTERFACE = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NOTIFY_PORT = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_AFFINITY_RULE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: ::windows_sys::core::PCWSTR, hgroup: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_AFFINITY_RULE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, affinityrulename: ::windows_sys::core::PCWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(henum: *const _HCLUSENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_CONTROL = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_CREATE_AFFINITY_RULE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: ::windows_sys::core::PCWSTR, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_ENUM = ::core::option::Option<unsafe extern "system" fn(henum: *const _HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows_sys::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(henum: *const _HCLUSENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT_EX = ::core::option::Option<unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hgroupenum: *mut _HGROUPENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_CONTROL = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_ENUM = ::core::option::Option<unsafe extern "system" fn(hgroupenum: *const _HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: ::windows_sys::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hgroupenum: *const _HGROUPENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT_EX = ::core::option::Option<unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, dwtype: u32) -> *mut _HGROUPENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszproperties: ::windows_sys::core::PCWSTR, cbproperties: u32, lpszroproperties: ::windows_sys::core::PCWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HGROUPENUMEX>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_CONTROL = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_ENUM = ::core::option::Option<unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows_sys::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NETWORK_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, dwtype: u32) -> *mut _HNETWORKENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_ENUM = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows_sys::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT_EX = ::core::option::Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtype: u32) -> *mut _HNODEENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HNODEENUMEX>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, dwtype: u32) -> *mut _HCLUSENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_OPEN_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HCLUSENUMEX>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CLOSE_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CREATE_BATCH = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregbatch: *mut *mut _HREGBATCH) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_CREATE_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: ::windows_sys::core::PCWSTR, dwoptions: u32, samdesired: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: ::windows_sys::core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_VALUE = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_ENUM_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszname: ::windows_sys::core::PWSTR, lpcchname: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_ENUM_VALUE = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszvaluename: ::windows_sys::core::PWSTR, lpcchvaluename: *mut u32, lpdwtype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_GET_KEY_SECURITY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, requestedinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_OPEN_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: ::windows_sys::core::PCWSTR, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_QUERY_INFO_KEY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_QUERY_VALUE = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: ::windows_sys::core::PCWSTR, lpdwvaluetype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_SET_VALUE = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: ::windows_sys::core::PCWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_REG_SYNC_DATABASE = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, flags: u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_REMOVE_AFFINITY_RULE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_AFFINITY_RULE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: ::windows_sys::core::PCWSTR, hgroup: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hgroupname: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hresenum: *mut _HRESENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM = ::core::option::Option<unsafe extern "system" fn(hresenum: *const _HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows_sys::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hresenum: *const _HRESENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT_EX = ::core::option::Option<unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, dwtype: u32) -> *mut _HRESENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszproperties: ::windows_sys::core::PCWSTR, cbproperties: u32, lpszroproperties: ::windows_sys::core::PCWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HRESENUMEX>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CLOSE_ENUM = ::core::option::Option<unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: ::windows_sys::core::PCWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_ENUM = ::core::option::Option<unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: ::windows_sys::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_GET_ENUM_COUNT = ::core::option::Option<unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_OPEN_ENUM = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: ::windows_sys::core::PCWSTR, dwtype: u32) -> *mut _HRESTYPEENUM>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_UPGRADE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, perform: super::super::Foundation::BOOL, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUS_WORKER_CREATE = ::core::option::Option<unsafe extern "system" fn(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUS_WORKER_TERMINATE = ::core::option::Option<unsafe extern "system" fn(lpworker: *const CLUS_WORKER) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER = ::core::option::Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_AVAILABILITY_SET = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpavailabilitysetname: ::windows_sys::core::PCWSTR, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> *mut _HGROUPSET>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_CNOLESS = ::core::option::Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: ::windows_sys::core::PCWSTR) -> *mut _HGROUP>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_GROUPEX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: ::windows_sys::core::PCWSTR, pgroupinfo: *const CLUSTER_CREATE_GROUP_INFO) -> *mut _HGROUP>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupsetname: ::windows_sys::core::PCWSTR) -> *mut _HGROUPSET>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_NAME_ACCOUNT = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> *mut _HCHANGE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT_V2 = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> *mut _HCHANGE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, lpszresourcename: ::windows_sys::core::PCWSTR, lpszresourcetype: ::windows_sys::core::PCWSTR, dwflags: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: ::windows_sys::core::PCWSTR, lpszdisplayname: ::windows_sys::core::PCWSTR, lpszresourcetypedll: ::windows_sys::core::PCWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DELETE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, lpszresourcetypename: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_DESTROY_CLUSTER = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void, fdeletevirtualcomputerobjects: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_DESTROY_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_EVICT_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_EVICT_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut ::windows_sys::core::HRESULT) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_FAIL_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_NETWORK = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_NET_INTERFACE = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_FROM_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_GROUP_KEY = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_GROUP_STATE = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, lpsznodename: ::windows_sys::core::PWSTR, lpcchnodename: *mut u32) -> CLUSTER_GROUP_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_INFORMATION = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszclustername: ::windows_sys::core::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: *mut CLUSTERVERSIONINFO) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_KEY = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_ID = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, lpsznetworkid: ::windows_sys::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_KEY = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_STATE = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> CLUSTER_NETWORK_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows_sys::core::PCWSTR, lpsznetworkname: ::windows_sys::core::PCWSTR, lpszinterfacename: ::windows_sys::core::PWSTR, lpcchinterfacename: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_KEY = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_STATE = ::core::option::Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NODE_ID = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, lpsznodeid: ::windows_sys::core::PWSTR, lpcchname: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NODE_KEY = ::core::option::Option<unsafe extern "system" fn(hnode: *mut _HNODE, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NODE_STATE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> CLUSTER_NODE_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NOTIFY = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: ::windows_sys::core::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_NOTIFY_V2 = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: *mut NOTIFY_FILTER_AND_TYPE, buffer: *mut u8, lpcchbuffersize: *mut u32, lpszobjectid: ::windows_sys::core::PWSTR, lpcchobjectid: *mut u32, lpszparentid: ::windows_sys::core::PWSTR, lpcchparentid: *mut u32, lpszname: ::windows_sys::core::PWSTR, lpcchname: *mut u32, lpsztype: ::windows_sys::core::PWSTR, lpcchtype: *mut u32, dwmilliseconds: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_QUORUM_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcename: ::windows_sys::core::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: ::windows_sys::core::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdependencyexpression: ::windows_sys::core::PWSTR, lpcchdependencyexpression: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_KEY = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_NETWORK_NAME = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpbuffer: ::windows_sys::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_STATE = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpsznodename: ::windows_sys::core::PWSTR, lpcchnodename: *mut u32, lpszgroupname: ::windows_sys::core::PWSTR, lpcchgroupname: *mut u32) -> CLUSTER_RESOURCE_STATE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_TYPE_KEY = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsztypename: ::windows_sys::core::PCWSTR, samdesired: u32) -> super::super::System::Registry::HKEY>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_GET_NODE_CLUSTER_STATE = ::core::option::Option<unsafe extern "system" fn(lpsznodename: ::windows_sys::core::PCWSTR, pdwclusterstate: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_NOTIFY_EVENT_HANDLE_V2 = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_IS_FILE_ON_CLUSTER_SHARED_VOLUME = ::core::option::Option<unsafe extern "system" fn(lpszpathname: ::windows_sys::core::PCWSTR, pbfileisonsharedvolume: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_MOVE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OFFLINE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OFFLINE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ONLINE_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_ONLINE_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER = ::core::option::Option<unsafe extern "system" fn(lpszclustername: ::windows_sys::core::PCWSTR) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_EX = ::core::option::Option<unsafe extern "system" fn(lpszclustername: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HCLUSTER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_GROUP = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: ::windows_sys::core::PCWSTR) -> *mut _HGROUP>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HGROUP>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_GROUPSET = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupsetname: ::windows_sys::core::PCWSTR) -> *mut _HGROUPSET>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NETINTERFACE_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetinterfacename: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETINTERFACE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetworkname: ::windows_sys::core::PCWSTR) -> *mut _HNETWORK>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetworkname: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETWORK>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NET_INTERFACE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszinterfacename: ::windows_sys::core::PCWSTR) -> *mut _HNETINTERFACE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows_sys::core::PCWSTR) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, lpszresourcename: ::windows_sys::core::PCWSTR) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcename: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_OPEN_NODE_BY_ID = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, nodeid: u32) -> *mut _HNODE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_PAUSE_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_PAUSE_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, bdrainnode: super::super::Foundation::BOOL, dwpauseflags: u32, hnodedraintarget: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, dwfiltertype: u32, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY_V2 = ::core::option::Option<unsafe extern "system" fn(hchange: *const _HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_NAME_ACCOUNT = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hdependson: *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hnode: *mut _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_CROSS_CLUSTER_GROUPSET_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: ::windows_sys::core::PCWSTR, lpremotegroupsetname: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_REMOVE_RESOURCE_FROM_CLUSTER_SHARED_VOLUMES = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_RESTART_CLUSTER_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, dwflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_RESTORE_CLUSTER_DATABASE = ::core::option::Option<unsafe extern "system" fn(lpszpathname: ::windows_sys::core::PCWSTR, bforce: super::super::Foundation::BOOL, lpszquorumdriveletter: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_RESUME_CLUSTER_NODE = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_RESUME_CLUSTER_NODE_EX = ::core::option::Option<unsafe extern "system" fn(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, lpszdependencyexpression: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_GROUP_NAME = ::core::option::Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, lpszgroupname: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST = ::core::option::Option<unsafe extern "system" fn(hgroup: *const _HGROUP, nodecount: u32, nodelist: *const *const _HNODE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_NETWORK_NAME = ::core::option::Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, lpszname: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_NETWORK_PRIORITY_ORDER = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, networkcount: u32, networklist: *const *const _HNETWORK) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdevicename: ::windows_sys::core::PCWSTR, dwmaxquologsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdependencyexpression: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_NAME = ::core::option::Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, lpszresourcename: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_CLUSTER_SERVICE_ACCOUNT_PASSWORD = ::core::option::Option<unsafe extern "system" fn(lpszclustername: ::windows_sys::core::PCWSTR, lpsznewpassword: ::windows_sys::core::PCWSTR, dwflags: u32, lpreturnstatusbuffer: *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION = ::core::option::Option<unsafe extern "system" fn(hgroupset: *const _HGROUP, lpszdependencyexpression: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SHARED_VOLUME_SET_SNAPSHOT_STATE = ::core::option::Option<unsafe extern "system" fn(guidsnapshotset: ::windows_sys::core::GUID, lpszvolumename: ::windows_sys::core::PCWSTR, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSAPI_SetClusterName = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznewclustername: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_CLEAR_BACKUP_STATE_FOR_SHARED_VOLUME = ::core::option::Option<unsafe extern "system" fn(lpszvolumepathname: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_DECRYPT = ::core::option::Option<unsafe extern "system" fn(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_ENCRYPT = ::core::option::Option<unsafe extern "system" fn(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pdata: *const u8, cbdata: u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT = ::core::option::Option<unsafe extern "system" fn(lpszvolumemountpoint: ::windows_sys::core::PCWSTR, lpszvolumename: ::windows_sys::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_GET_VOLUME_PATH_NAME = ::core::option::Option<unsafe extern "system" fn(lpszfilename: ::windows_sys::core::PCWSTR, lpszvolumepathname: ::windows_sys::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_IS_PATH_ON_SHARED_VOLUME = ::core::option::Option<unsafe extern "system" fn(lpszpathname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_PREPARE_SHARED_VOLUME_FOR_BACKUP = ::core::option::Option<unsafe extern "system" fn(lpszfilename: ::windows_sys::core::PCWSTR, lpszvolumepathname: ::windows_sys::core::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: ::windows_sys::core::PWSTR, lpcchvolumename: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_BATCH_ADD_COMMAND = ::core::option::Option<unsafe extern "system" fn(hregbatch: *const _HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: ::windows_sys::core::PCWSTR, dwoptions: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_BATCH_CLOSE_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(hbatchnotification: *const _HREGBATCHNOTIFICATION) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_BATCH_READ_COMMAND = ::core::option::Option<unsafe extern "system" fn(hbatchnotification: *const _HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_REG_CLOSE_BATCH = ::core::option::Option<unsafe extern "system" fn(hregbatch: *const _HREGBATCH, bcommit: super::super::Foundation::BOOL, failedcommandnumber: *mut i32) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_CLOSE_BATCH_NOTIFY_PORT = ::core::option::Option<unsafe extern "system" fn(hbatchnotifyport: *const _HREGBATCHPORT) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_CLOSE_READ_BATCH = ::core::option::Option<unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_CLOSE_READ_BATCH_EX = ::core::option::Option<unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, flags: u32, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_CLOSE_READ_BATCH_REPLY = ::core::option::Option<unsafe extern "system" fn(hregreadbatchreply: *const _HREGREADBATCHREPLY) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_BATCH_NOTIFY_PORT = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phbatchnotifyport: *mut *mut _HREGBATCHPORT) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_READ_BATCH = ::core::option::Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregreadbatch: *mut *mut _HREGREADBATCH) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_GET_BATCH_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(hbatchnotify: *const _HREGBATCHPORT, phbatchnotification: *mut *mut _HREGBATCHNOTIFICATION) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_READ_BATCH_ADD_COMMAND = ::core::option::Option<unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, wzsubkeyname: ::windows_sys::core::PCWSTR, wzvaluename: ::windows_sys::core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_REG_READ_BATCH_REPLY_NEXT_COMMAND = ::core::option::Option<unsafe extern "system" fn(hregreadbatchreply: *const _HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_SETUP_PROGRESS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pvcallbackarg: *mut ::core::ffi::c_void, esetupphase: CLUSTER_SETUP_PHASE, ephasetype: CLUSTER_SETUP_PHASE_TYPE, ephaseseverity: CLUSTER_SETUP_PHASE_SEVERITY, dwpercentcomplete: u32, lpszobjectname: ::windows_sys::core::PCWSTR, dwstatus: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PCLUSTER_SET_ACCOUNT_ACCESS = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, szaccountsid: ::windows_sys::core::PCWSTR, dwaccess: u32, dwcontroltype: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_UPGRADE_PROGRESS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pvcallbackarg: *mut ::core::ffi::c_void, eupgradephase: CLUSTER_UPGRADE_PHASE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PEND_CONTROL_CALL = ::core::option::Option<unsafe extern "system" fn(context: i64, status: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PEND_TYPE_CONTROL_CALL = ::core::option::Option<unsafe extern "system" fn(context: i64, status: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PEXTEND_RES_CONTROL_CALL = ::core::option::Option<unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PEXTEND_RES_TYPE_CONTROL_CALL = ::core::option::Option<unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PFREE_CLUSTER_CRYPT = ::core::option::Option<unsafe extern "system" fn(pcryptinfo: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PIS_ALIVE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PLOG_EVENT_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, loglevel: LOG_LEVEL, formatstring: ::windows_sys::core::PCWSTR) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLOOKS_ALIVE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type POFFLINE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type POFFLINE_V2_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *const ::core::ffi::c_void, destinationnodename: ::windows_sys::core::PCWSTR, offlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PONLINE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PONLINE_V2_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *const ::core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE, onlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type POPEN_CLUSTER_CRYPT_PROVIDER = ::core::option::Option<unsafe extern "system" fn(lpszresource: ::windows_sys::core::PCWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type POPEN_CLUSTER_CRYPT_PROVIDEREX = ::core::option::Option<unsafe extern "system" fn(lpszresource: ::windows_sys::core::PCWSTR, lpszkeyname: ::windows_sys::core::PCWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type POPEN_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcename: ::windows_sys::core::PCWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type POPEN_V2_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcename: ::windows_sys::core::PCWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize, openflags: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PQUERY_APPINSTANCE_VERSION = ::core::option::Option<unsafe extern "system" fn(appinstanceid: *const ::windows_sys::core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PQUORUM_RESOURCE_LOST = ::core::option::Option<unsafe extern "system" fn(resource: isize) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRAISE_RES_TYPE_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(resourcetype: ::windows_sys::core::PCWSTR, ppayload: *const u8, payloadsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PREGISTER_APPINSTANCE = ::core::option::Option<unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, appinstanceid: *const ::windows_sys::core::GUID, childreninheritappinstance: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PREGISTER_APPINSTANCE_VERSION = ::core::option::Option<unsafe extern "system" fn(appinstanceid: *const ::windows_sys::core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRELEASE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PREQUEST_DUMP_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, dumpduetocallinprogress: super::super::Foundation::BOOL, dumpdelayinms: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESET_ALL_APPINSTANCE_VERSIONS = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESOURCE_CONTROL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESOURCE_TYPE_CONTROL_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetypename: ::windows_sys::core::PCWSTR, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_ADD_UNKNOWN_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_CREATE_DIRECTORY_TREE = ::core::option::Option<unsafe extern "system" fn(pszpath: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_DUP_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_DUP_STRING = ::core::option::Option<unsafe extern "system" fn(pszinstring: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::PWSTR>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_ENUM_PRIVATE_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszoutproperties: ::windows_sys::core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_ENUM_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: ::windows_sys::core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_ENUM_RESOURCES = ::core::option::Option<unsafe extern "system" fn(hself: *mut _HRESOURCE, lpszrestypename: ::windows_sys::core::PCWSTR, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_ENUM_RESOURCES_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: ::windows_sys::core::PCWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_ENUM_RESOURCES_EX2 = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: ::windows_sys::core::PCWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void, dwdesiredaccess: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_EXPAND_ENVIRONMENT_STRINGS = ::core::option::Option<unsafe extern "system" fn(pszsrc: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::PWSTR>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_BINARY_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows_sys::core::PCWSTR, pbpropertyvalue: *mut *mut u8, pcbpropertyvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_DEPENDENT_DISK_RESOURCE_DRIVE_LETTER = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, hresource: *const _HRESOURCE, pszdriveletter: ::windows_sys::core::PWSTR, pcchdriveletter: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_DWORD_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows_sys::core::PCWSTR, pdwpropertyvalue: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_EXPANDED_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows_sys::core::PCWSTR, pszpropertyvalue: *mut ::windows_sys::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_EXPAND_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows_sys::core::PCWSTR, pszpropertyvalue: *mut ::windows_sys::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_FILETIME_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows_sys::core::PCWSTR, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_LONG_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows_sys::core::PCWSTR, plpropertyvalue: *mut i32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_MULTI_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows_sys::core::PCWSTR, pszpropertyvalue: *mut ::windows_sys::core::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows_sys::core::PCWSTR, pszpropertyvalue: *mut ::windows_sys::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FIND_ULARGEINTEGER_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: ::windows_sys::core::PCWSTR, plpropertyvalue: *mut u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_FREE_ENVIRONMENT = ::core::option::Option<unsafe extern "system" fn(lpenvironment: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FREE_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_ALL_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_BINARY_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: *const u8, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_BINARY_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES = ::core::option::Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, phclusternameresource: *mut *mut _HRESOURCE, phclusteripaddressresource: *mut *mut _HRESOURCE, phclusterquorumresource: *mut *mut _HRESOURCE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES_EX = ::core::option::Option<unsafe extern "system" fn(hclusterin: *const _HCLUSTER, phclusternameresourceout: *mut *mut _HRESOURCE, phclusteripaddressresourceout: *mut *mut _HRESOURCE, phclusterquorumresourceout: *mut *mut _HRESOURCE, dwdesiredaccess: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_DWORD_PROPERTY = ::core::option::Option<unsafe extern "system" fn(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_DWORD_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_ENVIRONMENT_WITH_NET_NAME = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_EXPAND_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, bexpand: super::super::Foundation::BOOL) -> ::windows_sys::core::PWSTR>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_FILETIME_PROPERTY = ::core::option::Option<unsafe extern "system" fn(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_LONG_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_MULTI_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppszoutvalue: *mut ::windows_sys::core::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: ::windows_sys::core::PCWSTR, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PRIVATE_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTIES_TO_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: super::super::Foundation::BOOL, psznameofpropinerror: *mut ::windows_sys::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTY = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut ::core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_PROPERTY_FORMATS = ::core::option::Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut ::core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTY_SIZE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_QWORD_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: ::windows_sys::core::PCWSTR) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: ::windows_sys::core::PCWSTR, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME_EX = ::core::option::Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: ::windows_sys::core::PCWSTR, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_EX = ::core::option::Option<unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_RESOURCE_DEPENDENTIP_ADDRESS_PROPS = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, pszaddress: ::windows_sys::core::PWSTR, pcchaddress: *mut u32, pszsubnetmask: ::windows_sys::core::PWSTR, pcchsubnetmask: *mut u32, psznetwork: ::windows_sys::core::PWSTR, pcchnetwork: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_RESOURCE_NAME = ::core::option::Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, pszresourcename: ::windows_sys::core::PWSTR, pcchresourcenameinout: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY = ::core::option::Option<unsafe extern "system" fn(lpszresourcename: ::windows_sys::core::PCWSTR, lpszresourcetype: ::windows_sys::core::PCWSTR) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY_EX = ::core::option::Option<unsafe extern "system" fn(lpszresourcename: ::windows_sys::core::PCWSTR, lpszresourcetype: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_GET_SZ_PROPERTY = ::core::option::Option<unsafe extern "system" fn(ppszoutvalue: *mut ::windows_sys::core::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: ::windows_sys::core::PCWSTR, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::PWSTR>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_IS_PATH_VALID = ::core::option::Option<unsafe extern "system" fn(pszpath: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_IS_RESOURCE_CLASS_EQUAL = ::core::option::Option<unsafe extern "system" fn(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_PROPERTY_LIST_FROM_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_REMOVE_RESOURCE_SERVICE_ENVIRONMENT = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows_sys::core::PCWSTR, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_RESOURCES_EQUAL = ::core::option::Option<unsafe extern "system" fn(hself: *mut _HRESOURCE, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_RESOURCE_TYPES_EQUAL = ::core::option::Option<unsafe extern "system" fn(lpszresourcetypename: ::windows_sys::core::PCWSTR, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_BINARY_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, pbnewvalue: *const u8, cbnewvaluesize: u32, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_DWORD_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_EXPAND_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, psznewvalue: ::windows_sys::core::PCWSTR, ppszoutstring: *mut ::windows_sys::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_MULTI_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, psznewvalue: ::windows_sys::core::PCWSTR, cbnewvaluesize: u32, ppszoutvalue: *mut ::windows_sys::core::PWSTR, pcboutvaluesize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PRIVATE_PROPERTY_LIST = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK_EX = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_TABLE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_TABLE_EX = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_QWORD_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, qwnewvalue: u64, pqwoutvalue: *mut u64) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_SET_RESOURCE_SERVICE_ENVIRONMENT = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows_sys::core::PCWSTR, hresource: *mut _HRESOURCE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows_sys::core::PCWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS_EX = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows_sys::core::PCWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_SZ_VALUE = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: ::windows_sys::core::PCWSTR, psznewvalue: ::windows_sys::core::PCWSTR, ppszoutstring: *mut ::windows_sys::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_UNKNOWN_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_START_RESOURCE_SERVICE = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows_sys::core::PCWSTR, phservicehandle: *mut isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_STOP_RESOURCE_SERVICE = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_STOP_SERVICE = ::core::option::Option<unsafe extern "system" fn(hservicehandle: super::super::Security::SC_HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_TERMINATE_SERVICE_PROCESS_FROM_RES_DLL = ::core::option::Option<unsafe extern "system" fn(dwservicepid: u32, boffline: super::super::Foundation::BOOL, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_VERIFY_PRIVATE_PROPERTY_LIST = ::core::option::Option<unsafe extern "system" fn(pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_VERIFY_PROPERTY_TABLE = ::core::option::Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRESUTIL_VERIFY_RESOURCE_SERVICE = ::core::option::Option<unsafe extern "system" fn(pszservicename: ::windows_sys::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_VERIFY_SERVICE = ::core::option::Option<unsafe extern "system" fn(hservicehandle: super::super::Security::SC_HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PRES_UTIL_VERIFY_SHUTDOWN_SAFE = ::core::option::Option<unsafe extern "system" fn(flags: u32, reason: u32, presult: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_INTERNAL_STATE = ::core::option::Option<unsafe extern "system" fn(param0: isize, statetype: CLUSTER_RESOURCE_APPLICATION_STATE, active: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, propertylistbuffer: *const u8, propertylistbuffersize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: super::super::Foundation::BOOL, lockedmodereason: u32, lockedmodeflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_LOCKED_MODE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: super::super::Foundation::BOOL, lockedmodereason: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_STATUS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_STATUS_ROUTINE_EX = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS_EX) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PSIGNAL_FAILURE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcehandle: isize, failuretype: FAILURE_TYPE, applicationspecificerrorcode: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PSTARTUP_EX_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetype: ::windows_sys::core::PCWSTR, minversionsupported: u32, maxversionsupported: u32, monitorcallbackfunctions: *mut CLRES_CALLBACK_FUNCTION_TABLE, resourcedllinterfacefunctions: *mut *mut CLRES_FUNCTION_TABLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PSTARTUP_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resourcetype: ::windows_sys::core::PCWSTR, minversionsupported: u32, maxversionsupported: u32, setresourcestatus: PSET_RESOURCE_STATUS_ROUTINE, logevent: PLOG_EVENT_ROUTINE, functiontable: *mut *mut CLRES_FUNCTION_TABLE) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`*"]
pub type PTERMINATE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWORKER_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(pworker: *mut CLUS_WORKER, lpthreadparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Clustering\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SET_APP_INSTANCE_CSV_FLAGS = ::core::option::Option<unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, mask: u32, flags: u32) -> u32>;

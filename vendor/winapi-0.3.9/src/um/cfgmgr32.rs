// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! user APIs for the Configuration Manager
use shared::basetsd::{DWORD_PTR, ULONG32, ULONG64, ULONG_PTR};
use shared::cfg::PPNP_VETO_TYPE;
use shared::devpropdef::{DEVPROPKEY, DEVPROPTYPE};
use shared::guiddef::{GUID, LPGUID};
use shared::minwindef::{BOOL, BYTE, DWORD, MAX_PATH, PBOOL, PBYTE, PHKEY, PULONG, ULONG, WORD};
use um::winnt::{
    ANYSIZE_ARRAY, CHAR, DWORDLONG, HANDLE, LARGE_INTEGER, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR,
    PCHAR, PCSTR, PCWSTR, PDWORDLONG, PSTR, PVOID, PWCHAR, PWSTR, ULONGLONG, VOID, WCHAR
};
use um::winreg::REGSAM;
pub type PCVOID = *const VOID;
pub const MAX_DEVICE_ID_LEN: usize = 200;
pub const MAX_DEVNODE_ID_LEN: usize = MAX_DEVICE_ID_LEN;
pub const MAX_GUID_STRING_LEN: usize = 39;
pub const MAX_CLASS_NAME_LEN: usize = 32;
pub const MAX_PROFILE_LEN: usize = 80;
pub const MAX_CONFIG_VALUE: DWORD = 9999;
pub const MAX_INSTANCE_VALUE: DWORD = 9999;
pub const MAX_MEM_REGISTERS: DWORD = 9;
pub const MAX_IO_PORTS: DWORD = 20;
pub const MAX_IRQS: DWORD = 7;
pub const MAX_DMA_CHANNELS: DWORD = 7;
pub const DWORD_MAX: DWORD = 0xffffffff;
pub const DWORDLONG_MAX: DWORDLONG = 0xffffffffffffffff;
pub const CONFIGMG_VERSION: DWORD = 0x0400;
pub type RETURN_TYPE = DWORD;
pub type CONFIGRET = RETURN_TYPE;
pub type DEVNODE = DWORD;
pub type DEVINST = DWORD;
pub type PDEVNODE = *mut DEVNODE;
pub type PDEVINST = *mut DEVNODE;
pub type DEVNODEID_A = *mut CHAR;
pub type DEVINSTID_A = *mut CHAR;
pub type DEVNODEID_W = *mut WCHAR;
pub type DEVINSTID_W = *mut WCHAR;
pub type LOG_CONF = DWORD_PTR;
pub type PLOG_CONF = *mut LOG_CONF;
pub type RES_DES = DWORD_PTR;
pub type PRES_DES = *mut RES_DES;
pub type RESOURCEID = ULONG;
pub type PRESOURCEID = *mut RESOURCEID;
pub type PRIORITY = ULONG;
pub type PPRIORITY = *mut PRIORITY;
pub type RANGE_LIST = DWORD_PTR;
pub type PRANGE_LIST = *mut RANGE_LIST;
pub type RANGE_ELEMENT = DWORD_PTR;
pub type PRANGE_ELEMENT = *mut RANGE_ELEMENT;
pub type HMACHINE = HANDLE;
pub type PHMACHINE = *mut HMACHINE;
pub type CONFLICT_LIST = ULONG_PTR;
pub type PCONFLICT_LIST = *mut CONFLICT_LIST;
STRUCT!{struct CONFLICT_DETAILS_A {
    CD_ulSize: ULONG,
    CD_ulMask: ULONG,
    CD_dnDevInst: DEVINST,
    CD_rdResDes: RES_DES,
    CD_ulFlags: ULONG,
    CD_szDescription: [CHAR; MAX_PATH],
}}
pub type PCONFLICT_DETAILS_A = *mut CONFLICT_DETAILS_A;
STRUCT!{struct CONFLICT_DETAILS_W {
    CD_ulSize: ULONG,
    CD_ulMask: ULONG,
    CD_dnDevInst: DEVINST,
    CD_rdResDes: RES_DES,
    CD_ulFlags: ULONG,
    CD_szDescription: [WCHAR; MAX_PATH],
}}
pub type PCONFLICT_DETAILS_W = *mut CONFLICT_DETAILS_W;
pub const CM_CDMASK_DEVINST: ULONG = 0x00000001;
pub const CM_CDMASK_RESDES: ULONG = 0x00000002;
pub const CM_CDMASK_FLAGS: ULONG = 0x00000004;
pub const CM_CDMASK_DESCRIPTION: ULONG = 0x00000008;
pub const CM_CDMASK_VALID: ULONG = 0x0000000F;
pub const CM_CDFLAGS_DRIVER: ULONG = 0x00000001;
pub const CM_CDFLAGS_ROOT_OWNED: ULONG = 0x00000002;
pub const CM_CDFLAGS_RESERVED: ULONG = 0x00000004;
pub type REGDISPOSITION = ULONG;
pub const mMD_MemoryType: DWORD = 0x1;
pub const fMD_MemoryType: DWORD = mMD_MemoryType;
pub const fMD_ROM: DWORD = 0x0;
pub const fMD_RAM: DWORD = 0x1;
pub const mMD_32_24: DWORD = 0x2;
pub const fMD_32_24: DWORD = mMD_32_24;
pub const fMD_24: DWORD = 0x0;
pub const fMD_32: DWORD = 0x2;
pub const mMD_Prefetchable: DWORD = 0x4;
pub const fMD_Prefetchable: DWORD = mMD_Prefetchable;
pub const fMD_Pref: DWORD = mMD_Prefetchable;
pub const fMD_PrefetchDisallowed: DWORD = 0x0;
pub const fMD_PrefetchAllowed: DWORD = 0x4;
pub const mMD_Readable: DWORD = 0x8;
pub const fMD_Readable: DWORD = mMD_Readable;
pub const fMD_ReadAllowed: DWORD = 0x0;
pub const fMD_ReadDisallowed: DWORD = 0x8;
pub const mMD_CombinedWrite: DWORD = 0x10;
pub const fMD_CombinedWrite: DWORD = mMD_CombinedWrite;
pub const fMD_CombinedWriteDisallowed: DWORD = 0x0;
pub const fMD_CombinedWriteAllowed: DWORD = 0x10;
pub const mMD_Cacheable: DWORD = 0x20;
pub const fMD_NonCacheable: DWORD = 0x0;
pub const fMD_Cacheable: DWORD = 0x20;
pub const fMD_WINDOW_DECODE: DWORD = 0x40;
pub const fMD_MEMORY_BAR: DWORD = 0x80;
STRUCT!{#[repr(packed)] struct MEM_RANGE {
    MR_Align: DWORDLONG,
    MR_nBytes: ULONG,
    MR_Min: DWORDLONG,
    MR_Max: DWORDLONG,
    MR_Flags: DWORD,
    MR_Reserved: DWORD,
}}
pub type PMEM_RANGE = *mut MEM_RANGE;
STRUCT!{#[repr(packed)] struct MEM_DES {
    MD_Count: DWORD,
    MD_Type: DWORD,
    MD_Alloc_Base: DWORDLONG,
    MD_Alloc_End: DWORDLONG,
    MD_Flags: DWORD,
    MD_Reserved: DWORD,
}}
pub type PMEM_DES = *mut MEM_DES;
STRUCT!{#[repr(packed)] struct MEM_RESOURCE {
    MEM_Header: MEM_DES,
    MEM_Data: [MEM_RANGE; ANYSIZE_ARRAY],
}}
pub type PMEM_RESOURCE = *mut MEM_RESOURCE;
STRUCT!{#[repr(packed)] struct MEM_LARGE_RANGE {
    MLR_Align: DWORDLONG,
    MLR_nBytes: ULONGLONG,
    MLR_Min: DWORDLONG,
    MLR_Max: DWORDLONG,
    MLR_Flags: DWORD,
    MLR_Reserved: DWORD,
}}
pub type PMEM_LARGE_RANGE = *mut MEM_LARGE_RANGE;
STRUCT!{#[repr(packed)] struct MEM_LARGE_DES {
    MLD_Count: DWORD,
    MLD_Type: DWORD,
    MLD_Alloc_Base: DWORDLONG,
    MLD_Alloc_End: DWORDLONG,
    MLD_Flags: DWORD,
    MLD_Reserved: DWORD,
}}
pub type PMEM_LARGE_DES = *mut MEM_LARGE_DES;
STRUCT!{#[repr(packed)] struct MEM_LARGE_RESOURCE {
    MEM_LARGE_Header: MEM_LARGE_DES,
    MEM_LARGE_Data: [MEM_LARGE_RANGE; ANYSIZE_ARRAY],
}}
pub type PMEM_LARGE_RESOURCE = *mut MEM_LARGE_RESOURCE;
pub const fIOD_PortType: DWORD = 0x1;
pub const fIOD_Memory: DWORD = 0x0;
pub const fIOD_IO: DWORD = 0x1;
pub const fIOD_DECODE: DWORD = 0x00fc;
pub const fIOD_10_BIT_DECODE: DWORD = 0x0004;
pub const fIOD_12_BIT_DECODE: DWORD = 0x0008;
pub const fIOD_16_BIT_DECODE: DWORD = 0x0010;
pub const fIOD_POSITIVE_DECODE: DWORD = 0x0020;
pub const fIOD_PASSIVE_DECODE: DWORD = 0x0040;
pub const fIOD_WINDOW_DECODE: DWORD = 0x0080;
pub const fIOD_PORT_BAR: DWORD = 0x0100;
pub const IO_ALIAS_10_BIT_DECODE: DWORDLONG = 0x00000004;
pub const IO_ALIAS_12_BIT_DECODE: DWORDLONG = 0x00000010;
pub const IO_ALIAS_16_BIT_DECODE: DWORDLONG = 0x00000000;
pub const IO_ALIAS_POSITIVE_DECODE: DWORDLONG = 0x000000FF;
STRUCT!{#[repr(packed)] struct IO_RANGE {
    IOR_Align: DWORDLONG,
    IOR_nPorts: DWORD,
    IOR_Min: DWORDLONG,
    IOR_Max: DWORDLONG,
    IOR_RangeFlags: DWORD,
    IOR_Alias: DWORDLONG,
}}
pub type PIO_RANGE = *mut IO_RANGE;
STRUCT!{#[repr(packed)] struct IO_DES {
    IOD_Count: DWORD,
    IOD_Type: DWORD,
    IOD_Alloc_Base: DWORDLONG,
    IOD_Alloc_End: DWORDLONG,
    IOD_DesFlags: DWORD,
}}
pub type PIO_DES = *mut IO_DES;
STRUCT!{#[repr(packed)] struct IO_RESOURCE {
    IO_Header: IO_DES,
    IO_Data: [IO_RANGE; ANYSIZE_ARRAY],
}}
pub type PIO_RESOURCE = *mut IO_RESOURCE;
pub const mDD_Width: ULONG = 0x3;
pub const fDD_BYTE: ULONG = 0x0;
pub const fDD_WORD: ULONG = 0x1;
pub const fDD_DWORD: ULONG = 0x2;
pub const fDD_BYTE_AND_WORD: ULONG = 0x3;
pub const mDD_BusMaster: ULONG = 0x4;
pub const fDD_NoBusMaster: ULONG = 0x0;
pub const fDD_BusMaster: ULONG = 0x4;
pub const mDD_Type: ULONG = 0x18;
pub const fDD_TypeStandard: ULONG = 0x00;
pub const fDD_TypeA: ULONG = 0x08;
pub const fDD_TypeB: ULONG = 0x10;
pub const fDD_TypeF: ULONG = 0x18;
STRUCT!{#[repr(packed)] struct DMA_RANGE {
    DR_Min: ULONG,
    DR_Max: ULONG,
    DR_Flags: ULONG,
}}
pub type PDMA_RANGE = *mut DMA_RANGE;
STRUCT!{#[repr(packed)] struct DMA_DES {
    DD_Count: DWORD,
    DD_Type: DWORD,
    DD_Flags: DWORD,
    DD_Alloc_Chan: ULONG,
}}
pub type PDMA_DES = *mut DMA_DES;
STRUCT!{#[repr(packed)] struct DMA_RESOURCE {
    DMA_Header: DMA_DES,
    DMA_Data: [DMA_RANGE; ANYSIZE_ARRAY],
}}
pub type PDMA_RESOURCE = *mut DMA_RESOURCE;
pub const mIRQD_Share: ULONG = 0x1;
pub const fIRQD_Exclusive: ULONG = 0x0;
pub const fIRQD_Share: ULONG = 0x1;
pub const fIRQD_Share_Bit: ULONG = 0;
pub const fIRQD_Level_Bit: ULONG = 1;
pub const mIRQD_Edge_Level: ULONG = 0x2;
pub const fIRQD_Level: ULONG = 0x0;
pub const fIRQD_Edge: ULONG = 0x2;
STRUCT!{#[repr(packed)] struct IRQ_RANGE {
    IRQR_Min: ULONG,
    IRQR_Max: ULONG,
    IRQR_Flags: ULONG,
}}
pub type PIRQ_RANGE = *mut IRQ_RANGE;
STRUCT!{#[repr(packed)] struct IRQ_DES_32 {
    IRQD_Count: DWORD,
    IRQD_Type: DWORD,
    IRQD_Flags: DWORD,
    IRQD_Alloc_Num: ULONG,
    IRQD_Affinity: ULONG32,
}}
pub type PIRQ_DES_32 = *mut IRQ_DES_32;
STRUCT!{#[repr(packed)] struct IRQ_DES_64 {
    IRQD_Count: DWORD,
    IRQD_Type: DWORD,
    IRQD_Flags: DWORD,
    IRQD_Alloc_Num: ULONG,
    IRQD_Affinity: ULONG64,
}}
pub type PIRQ_DES_64 = *mut IRQ_DES_64;
STRUCT!{#[repr(packed)] struct IRQ_RESOURCE_32 {
    IRQ_Header: IRQ_DES_32,
    IRQ_Data: [IRQ_RANGE; ANYSIZE_ARRAY],
}}
pub type PIRQ_RESOURCE_32 = *mut IRQ_RESOURCE_32;
STRUCT!{#[repr(packed)] struct IRQ_RESOURCE_64 {
    IRQ_Header: IRQ_DES_64,
    IRQ_Data: [IRQ_RANGE; ANYSIZE_ARRAY],
}}
pub type PIRQ_RESOURCE_64 = *mut IRQ_RESOURCE_64;
STRUCT!{#[repr(packed)] struct DEVPRIVATE_RANGE {
    PR_Data1: DWORD,
    PR_Data2: DWORD,
    PR_Data3: DWORD,
}}
pub type PDEVPRIVATE_RANGE = *mut DEVPRIVATE_RANGE;
STRUCT!{#[repr(packed)] struct DEVPRIVATE_DES {
    PD_Count: DWORD,
    PD_Type: DWORD,
    PD_Data1: DWORD,
    PD_Data2: DWORD,
    PD_Data3: DWORD,
    PD_Flags: DWORD,
}}
pub type PDEVPRIVATE_DES = *mut DEVPRIVATE_DES;
STRUCT!{#[repr(packed)] struct DEVPRIVATE_RESOURCE {
    PRV_Header: DEVPRIVATE_DES,
    PRV_Data: [DEVPRIVATE_RANGE; ANYSIZE_ARRAY],
}}
pub type PDEVPRIVATE_RESOURCE = *mut DEVPRIVATE_RESOURCE;
STRUCT!{#[repr(packed)] struct CS_DES {
    CSD_SignatureLength: DWORD,
    CSD_LegacyDataOffset: DWORD,
    CSD_LegacyDataSize: DWORD,
    CSD_Flags: DWORD,
    CSD_ClassGuid: GUID,
    CSD_Signature: [BYTE; ANYSIZE_ARRAY],
}}
pub type PCS_DES = *mut CS_DES;
STRUCT!{#[repr(packed)] struct CS_RESOURCE {
    CS_Header: CS_DES,
}}
pub type PCS_RESOURCE = *mut CS_RESOURCE;
pub const mPCD_IO_8_16: DWORD = 0x1;
pub const fPCD_IO_8: DWORD = 0x0;
pub const fPCD_IO_16: DWORD = 0x1;
pub const mPCD_MEM_8_16: DWORD = 0x2;
pub const fPCD_MEM_8: DWORD = 0x0;
pub const fPCD_MEM_16: DWORD = 0x2;
pub const mPCD_MEM_A_C: DWORD = 0xC;
pub const fPCD_MEM1_A: DWORD = 0x4;
pub const fPCD_MEM2_A: DWORD = 0x8;
pub const fPCD_IO_ZW_8: DWORD = 0x10;
pub const fPCD_IO_SRC_16: DWORD = 0x20;
pub const fPCD_IO_WS_16: DWORD = 0x40;
pub const mPCD_MEM_WS: DWORD = 0x300;
pub const fPCD_MEM_WS_ONE: DWORD = 0x100;
pub const fPCD_MEM_WS_TWO: DWORD = 0x200;
pub const fPCD_MEM_WS_THREE: DWORD = 0x300;
pub const fPCD_MEM_A: DWORD = 0x4;
pub const fPCD_ATTRIBUTES_PER_WINDOW: DWORD = 0x8000;
pub const fPCD_IO1_16: DWORD = 0x00010000;
pub const fPCD_IO1_ZW_8: DWORD = 0x00020000;
pub const fPCD_IO1_SRC_16: DWORD = 0x00040000;
pub const fPCD_IO1_WS_16: DWORD = 0x00080000;
pub const fPCD_IO2_16: DWORD = 0x00100000;
pub const fPCD_IO2_ZW_8: DWORD = 0x00200000;
pub const fPCD_IO2_SRC_16: DWORD = 0x00400000;
pub const fPCD_IO2_WS_16: DWORD = 0x00800000;
pub const mPCD_MEM1_WS: DWORD = 0x03000000;
pub const fPCD_MEM1_WS_TWO: DWORD = 0x02000000;
pub const fPCD_MEM1_WS_THREE: DWORD = 0x03000000;
pub const fPCD_MEM1_16: DWORD = 0x04000000;
pub const mPCD_MEM2_WS: DWORD = 0x30000000;
pub const fPCD_MEM2_WS_ONE: DWORD = 0x10000000;
pub const fPCD_MEM2_WS_TWO: DWORD = 0x20000000;
pub const fPCD_MEM2_WS_THREE: DWORD = 0x30000000;
pub const fPCD_MEM2_16: DWORD = 0x40000000;
pub const PCD_MAX_MEMORY: usize = 2;
pub const PCD_MAX_IO: usize = 2;
STRUCT!{#[repr(packed)] struct PCCARD_DES {
    PCD_Count: DWORD,
    PCD_Type: DWORD,
    PCD_Flags: DWORD,
    PCD_ConfigIndex: BYTE,
    PCD_Reserved: [BYTE; 3],
    PCD_MemoryCardBase1: DWORD,
    PCD_MemoryCardBase2: DWORD,
    PCD_MemoryCardBase: [DWORD; PCD_MAX_MEMORY],
    PCD_MemoryFlags: [WORD; PCD_MAX_MEMORY],
    PCD_IoFlags: [BYTE; PCD_MAX_IO],
}}
pub type PPCCARD_DES = *mut PCCARD_DES;
STRUCT!{#[repr(packed)] struct PCCARD_RESOURCE {
    PcCard_Header: PCCARD_DES,
}}
pub type PPCCARD_RESOURCE = *mut PCCARD_RESOURCE;
pub const mPMF_AUDIO_ENABLE: DWORD = 0x8;
pub const fPMF_AUDIO_ENABLE: DWORD = 0x8;
STRUCT!{#[repr(packed)] struct MFCARD_DES {
    PMF_Count: DWORD,
    PMF_Type: DWORD,
    PMF_Flags: DWORD,
    PMF_ConfigOptions: BYTE,
    PMF_IoResourceIndex: BYTE,
    PMF_Reserved: [BYTE; 2],
    PMF_ConfigRegisterBase: DWORD,
}}
pub type PMFCARD_DES = *mut MFCARD_DES;
STRUCT!{#[repr(packed)] struct MFCARD_RESOURCE {
    MfCard_Header: MFCARD_DES,
}}
pub type PMFCARD_RESOURCE = *mut MFCARD_RESOURCE;
STRUCT!{#[repr(packed)] struct BUSNUMBER_RANGE {
    BUSR_Min: ULONG,
    BUSR_Max: ULONG,
    BUSR_nBusNumbers: ULONG,
    BUSR_Flags: ULONG,
}}
pub type PBUSNUMBER_RANGE = *mut BUSNUMBER_RANGE;
STRUCT!{#[repr(packed)] struct BUSNUMBER_DES {
    BUSD_Count: DWORD,
    BUSD_Type: DWORD,
    BUSD_Flags: DWORD,
    BUSD_Alloc_Base: ULONG,
    BUSD_Alloc_End: ULONG,
}}
pub type PBUSNUMBER_DES = *mut BUSNUMBER_DES;
STRUCT!{#[repr(packed)] struct BUSNUMBER_RESOURCE {
    BusNumber_Header: BUSNUMBER_DES,
    BusNumber_Data: [BUSNUMBER_RANGE; ANYSIZE_ARRAY],
}}
pub type PBUSNUMBER_RESOURCE = *mut BUSNUMBER_RESOURCE;
STRUCT!{#[repr(packed)] struct CONNECTION_DES {
    COND_Type: DWORD,
    COND_Flags: DWORD,
    COND_Class: BYTE,
    COND_ClassType: BYTE,
    COND_Reserved1: BYTE,
    COND_Reserved2: BYTE,
    COND_Id: LARGE_INTEGER,
}}
pub type PCONNECTION_DES = *mut CONNECTION_DES;
STRUCT!{#[repr(packed)] struct CONNECTION_RESOURCE {
    Connection_Header: CONNECTION_DES,
}}
pub type PCONNECTION_RESOURCE = *mut CONNECTION_RESOURCE;
pub const CM_HWPI_NOT_DOCKABLE: DWORD = 0x00000000;
pub const CM_HWPI_UNDOCKED: DWORD = 0x00000001;
pub const CM_HWPI_DOCKED: DWORD = 0x00000002;
STRUCT!{#[repr(packed)] struct HWPROFILEINFO_A {
    HWPI_ulHWProfile: ULONG,
    HWPI_szFriendlyName: [CHAR; MAX_PROFILE_LEN],
    HWPI_dwFlags: DWORD,
}}
pub type PHWPROFILEINFO_A = *mut HWPROFILEINFO_A;
STRUCT!{#[repr(packed)] struct HWPROFILEINFO_W {
    HWPI_ulHWProfile: ULONG,
    HWPI_szFriendlyName: [WCHAR; MAX_PROFILE_LEN],
    HWPI_dwFlags: DWORD,
}}
pub type PHWPROFILEINFO_W = *mut HWPROFILEINFO_W;
pub const ResType_All: RESOURCEID = 0x00000000;
pub const ResType_None: RESOURCEID = 0x00000000;
pub const ResType_Mem: RESOURCEID = 0x00000001;
pub const ResType_IO: RESOURCEID = 0x00000002;
pub const ResType_DMA: RESOURCEID = 0x00000003;
pub const ResType_IRQ: RESOURCEID = 0x00000004;
pub const ResType_DoNotUse: RESOURCEID = 0x00000005;
pub const ResType_BusNumber: RESOURCEID = 0x00000006;
pub const ResType_MemLarge: RESOURCEID = 0x00000007;
pub const ResType_MAX: RESOURCEID = 0x00000007;
pub const ResType_Ignored_Bit: RESOURCEID = 0x00008000;
pub const ResType_ClassSpecific: RESOURCEID = 0x0000FFFF;
pub const ResType_Reserved: RESOURCEID = 0x00008000;
pub const ResType_DevicePrivate: RESOURCEID = 0x00008001;
pub const ResType_PcCardConfig: RESOURCEID = 0x00008002;
pub const ResType_MfCardConfig: RESOURCEID = 0x00008003;
pub const ResType_Connection: RESOURCEID = 0x00008004;
pub const CM_ADD_RANGE_ADDIFCONFLICT: ULONG = 0x00000000;
pub const CM_ADD_RANGE_DONOTADDIFCONFLICT: ULONG = 0x00000001;
pub const CM_ADD_RANGE_BITS: ULONG = 0x00000001;
pub const BASIC_LOG_CONF: ULONG = 0x00000000;
pub const FILTERED_LOG_CONF: ULONG = 0x00000001;
pub const ALLOC_LOG_CONF: ULONG = 0x00000002;
pub const BOOT_LOG_CONF: ULONG = 0x00000003;
pub const FORCED_LOG_CONF: ULONG = 0x00000004;
pub const OVERRIDE_LOG_CONF: ULONG = 0x00000005;
pub const NUM_LOG_CONF: ULONG = 0x00000006;
pub const LOG_CONF_BITS: ULONG = 0x00000007;
pub const PRIORITY_EQUAL_FIRST: ULONG = 0x00000008;
pub const PRIORITY_EQUAL_LAST: ULONG = 0x00000000;
pub const PRIORITY_BIT: ULONG = 0x00000008;
pub const RegDisposition_OpenAlways: REGDISPOSITION = 0x00000000;
pub const RegDisposition_OpenExisting: REGDISPOSITION = 0x00000001;
pub const RegDisposition_Bits: REGDISPOSITION = 0x00000001;
pub const CM_ADD_ID_HARDWARE: ULONG = 0x00000000;
pub const CM_ADD_ID_COMPATIBLE: ULONG = 0x00000001;
pub const CM_ADD_ID_BITS: ULONG = 0x00000001;
pub const CM_CREATE_DEVNODE_NORMAL: ULONG = 0x00000000;
pub const CM_CREATE_DEVNODE_NO_WAIT_INSTALL: ULONG = 0x00000001;
pub const CM_CREATE_DEVNODE_PHANTOM: ULONG = 0x00000002;
pub const CM_CREATE_DEVNODE_GENERATE_ID: ULONG = 0x00000004;
pub const CM_CREATE_DEVNODE_DO_NOT_INSTALL: ULONG = 0x00000008;
pub const CM_CREATE_DEVNODE_BITS: ULONG = 0x0000000F;
pub const CM_CREATE_DEVINST_NORMAL: ULONG = CM_CREATE_DEVNODE_NORMAL;
pub const CM_CREATE_DEVINST_NO_WAIT_INSTALL: ULONG = CM_CREATE_DEVNODE_NO_WAIT_INSTALL;
pub const CM_CREATE_DEVINST_PHANTOM: ULONG = CM_CREATE_DEVNODE_PHANTOM;
pub const CM_CREATE_DEVINST_GENERATE_ID: ULONG = CM_CREATE_DEVNODE_GENERATE_ID;
pub const CM_CREATE_DEVINST_DO_NOT_INSTALL: ULONG = CM_CREATE_DEVNODE_DO_NOT_INSTALL;
pub const CM_CREATE_DEVINST_BITS: ULONG = CM_CREATE_DEVNODE_BITS;
pub const CM_DELETE_CLASS_ONLY: ULONG = 0x00000000;
pub const CM_DELETE_CLASS_SUBKEYS: ULONG = 0x00000001;
pub const CM_DELETE_CLASS_INTERFACE: ULONG = 0x00000002;
pub const CM_DELETE_CLASS_BITS: ULONG = 0x00000003;
pub const CM_ENUMERATE_CLASSES_INSTALLER: ULONG = 0x00000000;
pub const CM_ENUMERATE_CLASSES_INTERFACE: ULONG = 0x00000001;
pub const CM_ENUMERATE_CLASSES_BITS: ULONG = 0x00000001;
pub const CM_DETECT_NEW_PROFILE: ULONG = 0x00000001;
pub const CM_DETECT_CRASHED: ULONG = 0x00000002;
pub const CM_DETECT_HWPROF_FIRST_BOOT: ULONG = 0x00000004;
pub const CM_DETECT_RUN: ULONG = 0x80000000;
pub const CM_DETECT_BITS: ULONG = 0x80000007;
pub const CM_DISABLE_POLITE: ULONG = 0x00000000;
pub const CM_DISABLE_ABSOLUTE: ULONG = 0x00000001;
pub const CM_DISABLE_HARDWARE: ULONG = 0x00000002;
pub const CM_DISABLE_UI_NOT_OK: ULONG = 0x00000004;
pub const CM_DISABLE_BITS: ULONG = 0x00000007;
pub const CM_GETIDLIST_FILTER_NONE: ULONG = 0x00000000;
pub const CM_GETIDLIST_FILTER_ENUMERATOR: ULONG = 0x00000001;
pub const CM_GETIDLIST_FILTER_SERVICE: ULONG = 0x00000002;
pub const CM_GETIDLIST_FILTER_EJECTRELATIONS: ULONG = 0x00000004;
pub const CM_GETIDLIST_FILTER_REMOVALRELATIONS: ULONG = 0x00000008;
pub const CM_GETIDLIST_FILTER_POWERRELATIONS: ULONG = 0x00000010;
pub const CM_GETIDLIST_FILTER_BUSRELATIONS: ULONG = 0x00000020;
pub const CM_GETIDLIST_DONOTGENERATE: ULONG = 0x10000040;
pub const CM_GETIDLIST_FILTER_TRANSPORTRELATIONS: ULONG = 0x00000080;
pub const CM_GETIDLIST_FILTER_PRESENT: ULONG = 0x00000100;
pub const CM_GETIDLIST_FILTER_CLASS: ULONG = 0x00000200;
pub const CM_GETIDLIST_FILTER_BITS: ULONG = 0x100003FF;
pub const CM_GET_DEVICE_INTERFACE_LIST_PRESENT: ULONG = 0x00000000;
pub const CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES: ULONG = 0x00000001;
pub const CM_GET_DEVICE_INTERFACE_LIST_BITS: ULONG = 0x00000001;
pub const CM_DRP_DEVICEDESC: ULONG = 0x00000001;
pub const CM_DRP_HARDWAREID: ULONG = 0x00000002;
pub const CM_DRP_COMPATIBLEIDS: ULONG = 0x00000003;
pub const CM_DRP_UNUSED0: ULONG = 0x00000004;
pub const CM_DRP_SERVICE: ULONG = 0x00000005;
pub const CM_DRP_UNUSED1: ULONG = 0x00000006;
pub const CM_DRP_UNUSED2: ULONG = 0x00000007;
pub const CM_DRP_CLASS: ULONG = 0x00000008;
pub const CM_DRP_CLASSGUID: ULONG = 0x00000009;
pub const CM_DRP_DRIVER: ULONG = 0x0000000A;
pub const CM_DRP_CONFIGFLAGS: ULONG = 0x0000000B;
pub const CM_DRP_MFG: ULONG = 0x0000000C;
pub const CM_DRP_FRIENDLYNAME: ULONG = 0x0000000D;
pub const CM_DRP_LOCATION_INFORMATION: ULONG = 0x0000000E;
pub const CM_DRP_PHYSICAL_DEVICE_OBJECT_NAME: ULONG = 0x0000000F;
pub const CM_DRP_CAPABILITIES: ULONG = 0x00000010;
pub const CM_DRP_UI_NUMBER: ULONG = 0x00000011;
pub const CM_DRP_UPPERFILTERS: ULONG = 0x00000012;
pub const CM_CRP_UPPERFILTERS: ULONG = CM_DRP_UPPERFILTERS;
pub const CM_DRP_LOWERFILTERS: ULONG = 0x00000013;
pub const CM_CRP_LOWERFILTERS: ULONG = CM_DRP_LOWERFILTERS;
pub const CM_DRP_BUSTYPEGUID: ULONG = 0x00000014;
pub const CM_DRP_LEGACYBUSTYPE: ULONG = 0x00000015;
pub const CM_DRP_BUSNUMBER: ULONG = 0x00000016;
pub const CM_DRP_ENUMERATOR_NAME: ULONG = 0x00000017;
pub const CM_DRP_SECURITY: ULONG = 0x00000018;
pub const CM_CRP_SECURITY: ULONG = CM_DRP_SECURITY;
pub const CM_DRP_SECURITY_SDS: ULONG = 0x00000019;
pub const CM_CRP_SECURITY_SDS: ULONG = CM_DRP_SECURITY_SDS;
pub const CM_DRP_DEVTYPE: ULONG = 0x0000001A;
pub const CM_CRP_DEVTYPE: ULONG = CM_DRP_DEVTYPE;
pub const CM_DRP_EXCLUSIVE: ULONG = 0x0000001B;
pub const CM_CRP_EXCLUSIVE: ULONG = CM_DRP_EXCLUSIVE;
pub const CM_DRP_CHARACTERISTICS: ULONG = 0x0000001C;
pub const CM_CRP_CHARACTERISTICS: ULONG = CM_DRP_CHARACTERISTICS;
pub const CM_DRP_ADDRESS: ULONG = 0x0000001D;
pub const CM_DRP_UI_NUMBER_DESC_FORMAT: ULONG = 0x0000001E;
pub const CM_DRP_DEVICE_POWER_DATA: ULONG = 0x0000001F;
pub const CM_DRP_REMOVAL_POLICY: ULONG = 0x00000020;
pub const CM_DRP_REMOVAL_POLICY_HW_DEFAULT: ULONG = 0x00000021;
pub const CM_DRP_REMOVAL_POLICY_OVERRIDE: ULONG = 0x00000022;
pub const CM_DRP_INSTALL_STATE: ULONG = 0x00000023;
pub const CM_DRP_LOCATION_PATHS: ULONG = 0x00000024;
pub const CM_DRP_BASE_CONTAINERID: ULONG = 0x00000025;
pub const CM_DRP_MIN: ULONG = 0x00000001;
pub const CM_CRP_MIN: ULONG = CM_DRP_MIN;
pub const CM_DRP_MAX: ULONG = 0x00000025;
pub const CM_CRP_MAX: ULONG = CM_DRP_MAX;
pub const CM_DEVCAP_LOCKSUPPORTED: ULONG = 0x00000001;
pub const CM_DEVCAP_EJECTSUPPORTED: ULONG = 0x00000002;
pub const CM_DEVCAP_REMOVABLE: ULONG = 0x00000004;
pub const CM_DEVCAP_DOCKDEVICE: ULONG = 0x00000008;
pub const CM_DEVCAP_UNIQUEID: ULONG = 0x00000010;
pub const CM_DEVCAP_SILENTINSTALL: ULONG = 0x00000020;
pub const CM_DEVCAP_RAWDEVICEOK: ULONG = 0x00000040;
pub const CM_DEVCAP_SURPRISEREMOVALOK: ULONG = 0x00000080;
pub const CM_DEVCAP_HARDWAREDISABLED: ULONG = 0x00000100;
pub const CM_DEVCAP_NONDYNAMIC: ULONG = 0x00000200;
pub const CM_REMOVAL_POLICY_EXPECT_NO_REMOVAL: ULONG = 1;
pub const CM_REMOVAL_POLICY_EXPECT_ORDERLY_REMOVAL: ULONG = 2;
pub const CM_REMOVAL_POLICY_EXPECT_SURPRISE_REMOVAL: ULONG = 3;
pub const CM_INSTALL_STATE_INSTALLED: ULONG = 0;
pub const CM_INSTALL_STATE_NEEDS_REINSTALL: ULONG = 1;
pub const CM_INSTALL_STATE_FAILED_INSTALL: ULONG = 2;
pub const CM_INSTALL_STATE_FINISH_INSTALL: ULONG = 3;
pub const CM_LOCATE_DEVNODE_NORMAL: ULONG = 0x00000000;
pub const CM_LOCATE_DEVNODE_PHANTOM: ULONG = 0x00000001;
pub const CM_LOCATE_DEVNODE_CANCELREMOVE: ULONG = 0x00000002;
pub const CM_LOCATE_DEVNODE_NOVALIDATION: ULONG = 0x00000004;
pub const CM_LOCATE_DEVNODE_BITS: ULONG = 0x00000007;
pub const CM_LOCATE_DEVINST_NORMAL: ULONG = CM_LOCATE_DEVNODE_NORMAL;
pub const CM_LOCATE_DEVINST_PHANTOM: ULONG = CM_LOCATE_DEVNODE_PHANTOM;
pub const CM_LOCATE_DEVINST_CANCELREMOVE: ULONG = CM_LOCATE_DEVNODE_CANCELREMOVE;
pub const CM_LOCATE_DEVINST_NOVALIDATION: ULONG = CM_LOCATE_DEVNODE_NOVALIDATION;
pub const CM_LOCATE_DEVINST_BITS: ULONG = CM_LOCATE_DEVNODE_BITS;
pub const CM_OPEN_CLASS_KEY_INSTALLER: ULONG = 0x00000000;
pub const CM_OPEN_CLASS_KEY_INTERFACE: ULONG = 0x00000001;
pub const CM_OPEN_CLASS_KEY_BITS: ULONG = 0x00000001;
pub const CM_REMOVE_UI_OK: ULONG = 0x00000000;
pub const CM_REMOVE_UI_NOT_OK: ULONG = 0x00000001;
pub const CM_REMOVE_NO_RESTART: ULONG = 0x00000002;
pub const CM_REMOVE_BITS: ULONG = 0x00000003;
pub const CM_QUERY_REMOVE_UI_OK: ULONG = CM_REMOVE_UI_OK;
pub const CM_QUERY_REMOVE_UI_NOT_OK: ULONG = CM_REMOVE_UI_NOT_OK;
pub const CM_QUERY_REMOVE_BITS: ULONG = CM_QUERY_REMOVE_UI_OK | CM_QUERY_REMOVE_UI_NOT_OK;
pub const CM_REENUMERATE_NORMAL: ULONG = 0x00000000;
pub const CM_REENUMERATE_SYNCHRONOUS: ULONG = 0x00000001;
pub const CM_REENUMERATE_RETRY_INSTALLATION: ULONG = 0x00000002;
pub const CM_REENUMERATE_ASYNCHRONOUS: ULONG = 0x00000004;
pub const CM_REENUMERATE_BITS: ULONG = 0x00000007;
pub const CM_REGISTER_DEVICE_DRIVER_STATIC: ULONG = 0x00000000;
pub const CM_REGISTER_DEVICE_DRIVER_DISABLEABLE: ULONG = 0x00000001;
pub const CM_REGISTER_DEVICE_DRIVER_REMOVABLE: ULONG = 0x00000002;
pub const CM_REGISTER_DEVICE_DRIVER_BITS: ULONG = 0x00000003;
pub const CM_REGISTRY_HARDWARE: ULONG = 0x00000000;
pub const CM_REGISTRY_SOFTWARE: ULONG = 0x00000001;
pub const CM_REGISTRY_USER: ULONG = 0x00000100;
pub const CM_REGISTRY_CONFIG: ULONG = 0x00000200;
pub const CM_REGISTRY_BITS: ULONG = 0x00000301;
pub const CM_SET_DEVNODE_PROBLEM_NORMAL: ULONG = 0x00000000;
pub const CM_SET_DEVNODE_PROBLEM_OVERRIDE: ULONG = 0x00000001;
pub const CM_SET_DEVNODE_PROBLEM_BITS: ULONG = 0x00000001;
pub const CM_SET_DEVINST_PROBLEM_NORMAL: ULONG = CM_SET_DEVNODE_PROBLEM_NORMAL;
pub const CM_SET_DEVINST_PROBLEM_OVERRIDE: ULONG = CM_SET_DEVNODE_PROBLEM_OVERRIDE;
pub const CM_SET_DEVINST_PROBLEM_BITS: ULONG = CM_SET_DEVNODE_PROBLEM_BITS;
pub const CM_SET_HW_PROF_FLAGS_UI_NOT_OK: ULONG = 0x00000001;
pub const CM_SET_HW_PROF_FLAGS_BITS: ULONG = 0x00000001;
pub const CM_SETUP_DEVNODE_READY: ULONG = 0x00000000;
pub const CM_SETUP_DEVINST_READY: ULONG = CM_SETUP_DEVNODE_READY;
pub const CM_SETUP_DOWNLOAD: ULONG = 0x00000001;
pub const CM_SETUP_WRITE_LOG_CONFS: ULONG = 0x00000002;
pub const CM_SETUP_PROP_CHANGE: ULONG = 0x00000003;
pub const CM_SETUP_DEVNODE_RESET: ULONG = 0x00000004;
pub const CM_SETUP_DEVINST_RESET: ULONG = CM_SETUP_DEVNODE_RESET;
pub const CM_SETUP_DEVNODE_CONFIG: ULONG = 0x00000005;
pub const CM_SETUP_DEVINST_CONFIG: ULONG = CM_SETUP_DEVNODE_CONFIG;
pub const CM_SETUP_DEVNODE_CONFIG_CLASS: ULONG = 0x00000006;
pub const CM_SETUP_DEVINST_CONFIG_CLASS: ULONG = CM_SETUP_DEVNODE_CONFIG_CLASS;
pub const CM_SETUP_DEVNODE_CONFIG_EXTENSIONS: ULONG = 0x00000007;
pub const CM_SETUP_DEVINST_CONFIG_EXTENSIONS: ULONG = CM_SETUP_DEVNODE_CONFIG_EXTENSIONS;
pub const CM_SETUP_BITS: ULONG = 0x00000007;
pub const CM_QUERY_ARBITRATOR_RAW: ULONG = 0x00000000;
pub const CM_QUERY_ARBITRATOR_TRANSLATED: ULONG = 0x00000001;
pub const CM_QUERY_ARBITRATOR_BITS: ULONG = 0x00000001;
pub const CM_CUSTOMDEVPROP_MERGE_MULTISZ: ULONG = 0x00000001;
pub const CM_CUSTOMDEVPROP_BITS: ULONG = 0x00000001;
pub const CM_NAME_ATTRIBUTE_NAME_RETRIEVED_FROM_DEVICE: ULONG = 0x1;
pub const CM_NAME_ATTRIBUTE_USER_ASSIGNED_NAME: ULONG = 0x2;
pub const CM_CLASS_PROPERTY_INSTALLER: ULONG = 0x00000000;
pub const CM_CLASS_PROPERTY_INTERFACE: ULONG = 0x00000001;
pub const CM_CLASS_PROPERTY_BITS: ULONG = 0x00000001;
DECLARE_HANDLE!{HCMNOTIFICATION, HCMNOTIFICATION__}
pub type PHCMNOTIFICATION = *mut HCMNOTIFICATION;
pub const CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES: ULONG = 0x00000001;
pub const CM_NOTIFY_FILTER_FLAG_ALL_DEVICE_INSTANCES: ULONG = 0x00000002;
pub const CM_NOTIFY_FILTER_VALID_FLAGS: ULONG = CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES
    | CM_NOTIFY_FILTER_FLAG_ALL_DEVICE_INSTANCES;
ENUM!{enum CM_NOTIFY_FILTER_TYPE {
    CM_NOTIFY_FILTER_TYPE_DEVICEINTERFACE = 0,
    CM_NOTIFY_FILTER_TYPE_DEVICEHANDLE,
    CM_NOTIFY_FILTER_TYPE_DEVICEINSTANCE,
    CM_NOTIFY_FILTER_TYPE_MAX,
}}
pub type PCM_NOTIFY_FILTER_TYPE = *mut CM_NOTIFY_FILTER_TYPE;
STRUCT!{struct CM_NOTIFY_FILTER_DeviceInterface {
    ClassGuid: GUID,
}}
STRUCT!{struct CM_NOTIFY_FILTER_DeviceHandle {
    hTarget: HANDLE,
}}
STRUCT!{struct CM_NOTIFY_FILTER_DeviceInstance {
    InstanceId: [WCHAR; MAX_DEVICE_ID_LEN],
}}
UNION!{union CM_NOTIFY_FILTER_u {
    [u32; 100] [u64; 50],
    DeviceInterface DeviceInterface_mut: CM_NOTIFY_FILTER_DeviceInterface,
    DeviceHandle DeviceHandle_mut: CM_NOTIFY_FILTER_DeviceHandle,
    DeviceInstance DeviceInstance_mut: CM_NOTIFY_FILTER_DeviceInstance,
}}
STRUCT!{struct CM_NOTIFY_FILTER {
    cbSize: DWORD,
    Flags: DWORD,
    FilterType: CM_NOTIFY_FILTER_TYPE,
    Reserved: DWORD,
    u: CM_NOTIFY_FILTER_u,
}}
pub type PCM_NOTIFY_FILTER = *mut CM_NOTIFY_FILTER;
ENUM!{enum CM_NOTIFY_ACTION {
    CM_NOTIFY_ACTION_DEVICEINTERFACEARRIVAL = 0,
    CM_NOTIFY_ACTION_DEVICEINTERFACEREMOVAL,
    CM_NOTIFY_ACTION_DEVICEQUERYREMOVE,
    CM_NOTIFY_ACTION_DEVICEQUERYREMOVEFAILED,
    CM_NOTIFY_ACTION_DEVICEREMOVEPENDING,
    CM_NOTIFY_ACTION_DEVICEREMOVECOMPLETE,
    CM_NOTIFY_ACTION_DEVICECUSTOMEVENT,
    CM_NOTIFY_ACTION_DEVICEINSTANCEENUMERATED,
    CM_NOTIFY_ACTION_DEVICEINSTANCESTARTED,
    CM_NOTIFY_ACTION_DEVICEINSTANCEREMOVED,
    CM_NOTIFY_ACTION_MAX,
}}
pub type PCM_NOTIFY_ACTION = *mut CM_NOTIFY_ACTION;
STRUCT!{struct CM_NOTIFY_EVENT_DATA_DeviceInterface {
    ClassGuid: GUID,
    SymbolicLink: [WCHAR; ANYSIZE_ARRAY],
}}
STRUCT!{struct CM_NOTIFY_EVENT_DATA_DeviceHandle {
    EventGuid: GUID,
    NameOffset: LONG,
    DataSize: DWORD,
    Data: [BYTE; ANYSIZE_ARRAY],
}}
STRUCT!{struct CM_NOTIFY_EVENT_DATA_DeviceInstance {
    InstanceId: [WCHAR; ANYSIZE_ARRAY],
}}
UNION!{union CM_NOTIFY_EVENT_DATA_u {
    [u32; 7],
    DeviceInterface DeviceInterface_mut: CM_NOTIFY_EVENT_DATA_DeviceInterface,
    DeviceHandle DeviceHandle_mut: CM_NOTIFY_EVENT_DATA_DeviceHandle,
    DeviceInstance DeviceInstance_mut: CM_NOTIFY_EVENT_DATA_DeviceInstance,
}}
STRUCT!{struct CM_NOTIFY_EVENT_DATA {
    FilterType: CM_NOTIFY_FILTER_TYPE,
    Reserved: DWORD,
    u: CM_NOTIFY_EVENT_DATA_u,
}}
pub type PCM_NOTIFY_EVENT_DATA = *mut CM_NOTIFY_EVENT_DATA;
FN!{stdcall PCM_NOTIFY_CALLBACK(
    hNotify: HCMNOTIFICATION,
    Context: PVOID,
    Action: CM_NOTIFY_ACTION,
    EventData: PCM_NOTIFY_EVENT_DATA,
    EventDataSize: DWORD,
) -> DWORD}
extern "system" {
    pub fn CM_Add_Empty_Log_Conf(
        plcLogConf: PLOG_CONF,
        dnDevInst: DEVINST,
        Priority: PRIORITY,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Add_Empty_Log_Conf_Ex(
        plcLogConf: PLOG_CONF,
        dnDevInst: DEVINST,
        Priority: PRIORITY,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Add_IDA(
        dnDevInst: DEVINST,
        pszID: PSTR,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Add_IDW(
        dnDevInst: DEVINST,
        pszID: PWSTR,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Add_ID_ExA(
        dnDevInst: DEVINST,
        pszID: PSTR,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Add_ID_ExW(
        dnDevInst: DEVINST,
        pszID: PWSTR,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Add_Range(
        ullStartValue: DWORDLONG,
        ullEndValue: DWORDLONG,
        rlh: RANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Add_Res_Des(
        prdResDes: PRES_DES,
        lcLogConf: LOG_CONF,
        ResourceID: RESOURCEID,
        ResourceData: PCVOID,
        ResourceLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Add_Res_Des_Ex(
        prdResDes: PRES_DES,
        lcLogConf: LOG_CONF,
        ResourceID: RESOURCEID,
        ResourceData: PCVOID,
        ResourceLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Connect_MachineA(
        UNCServerName: PCSTR,
        phMachine: PHMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Connect_MachineW(
        UNCServerName: PCWSTR,
        phMachine: PHMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Create_DevNodeA(
        pdnDevInst: PDEVINST,
        pDeviceID: DEVINSTID_A,
        dnParent: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Create_DevNodeW(
        pdnDevInst: PDEVINST,
        pDeviceID: DEVINSTID_W,
        dnParent: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Create_DevNode_ExA(
        pdnDevInst: PDEVINST,
        pDeviceID: DEVINSTID_A,
        dnParent: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Create_DevNode_ExW(
        pdnDevInst: PDEVINST,
        pDeviceID: DEVINSTID_W,
        dnParent: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Create_Range_List(
        prlh: PRANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Delete_Class_Key(
        ClassGuid: LPGUID,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Delete_Class_Key_Ex(
        ClassGuid: LPGUID,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Delete_DevNode_Key(
        dnDevNode: DEVNODE,
        ulHardwareProfile: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Delete_DevNode_Key_Ex(
        dnDevNode: DEVNODE,
        ulHardwareProfile: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Delete_Range(
        ullStartValue: DWORDLONG,
        ullEndValue: DWORDLONG,
        rlh: RANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Detect_Resource_Conflict(
        dnDevInst: DEVINST,
        ResourceID: RESOURCEID,
        ResourceData: PCVOID,
        ResourceLen: ULONG,
        pbConflictDetected: PBOOL,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Detect_Resource_Conflict_Ex(
        dnDevInst: DEVINST,
        ResourceID: RESOURCEID,
        ResourceData: PCVOID,
        ResourceLen: ULONG,
        pbConflictDetected: PBOOL,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Disable_DevNode(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Disable_DevNode_Ex(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Disconnect_Machine(
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Dup_Range_List(
        rlhOld: RANGE_LIST,
        rlhNew: RANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Enable_DevNode(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Enable_DevNode_Ex(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_Classes(
        ulClassIndex: ULONG,
        ClassGuid: LPGUID,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_Classes_Ex(
        ulClassIndex: ULONG,
        ClassGuid: LPGUID,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_EnumeratorsA(
        ulEnumIndex: ULONG,
        Buffer: PSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_EnumeratorsW(
        ulEnumIndex: ULONG,
        Buffer: PWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_Enumerators_ExA(
        ulEnumIndex: ULONG,
        Buffer: PSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Enumerate_Enumerators_ExW(
        ulEnumIndex: ULONG,
        Buffer: PWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Find_Range(
        pullStart: PDWORDLONG,
        ullStart: DWORDLONG,
        ulLength: ULONG,
        ullAlignment: DWORDLONG,
        ullEnd: DWORDLONG,
        rlh: RANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_First_Range(
        rlh: RANGE_LIST,
        pullStart: PDWORDLONG,
        pullEnd: PDWORDLONG,
        preElement: PRANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Free_Log_Conf(
        lcLogConfToBeFreed: LOG_CONF,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Free_Log_Conf_Ex(
        lcLogConfToBeFreed: LOG_CONF,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Free_Log_Conf_Handle(
        lcLogConf: LOG_CONF,
    ) -> CONFIGRET;
    pub fn CM_Free_Range_List(
        rlh: RANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Free_Res_Des(
        prdResDes: PRES_DES,
        rdResDes: RES_DES,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Free_Res_Des_Ex(
        prdResDes: PRES_DES,
        rdResDes: RES_DES,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Free_Res_Des_Handle(
        rdResDes: RES_DES,
    ) -> CONFIGRET;
    pub fn CM_Get_Child(
        pdnDevInst: PDEVINST,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Child_Ex(
        pdnDevInst: PDEVINST,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Key_NameA(
        ClassGuid: LPGUID,
        pszKeyName: LPSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Key_NameW(
        ClassGuid: LPGUID,
        pszKeyName: LPWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Key_Name_ExA(
        ClassGuid: LPGUID,
        pszKeyName: LPSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Key_Name_ExW(
        ClassGuid: LPGUID,
        pszKeyName: LPWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_NameA(
        ClassGuid: LPGUID,
        Buffer: PSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_NameW(
        ClassGuid: LPGUID,
        Buffer: PWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Name_ExA(
        ClassGuid: LPGUID,
        Buffer: PSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Name_ExW(
        ClassGuid: LPGUID,
        Buffer: PWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Depth(
        pulDepth: PULONG,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Depth_Ex(
        pulDepth: PULONG,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_IDA(
        dnDevInst: DEVINST,
        Buffer: PSTR,
        BufferLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_IDW(
        dnDevInst: DEVINST,
        Buffer: PWSTR,
        BufferLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_ExA(
        dnDevInst: DEVINST,
        Buffer: PSTR,
        BufferLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_ExW(
        dnDevInst: DEVINST,
        Buffer: PWSTR,
        BufferLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_ListA(
        pszFilter: PCSTR,
        Buffer: PCHAR,
        BufferLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_ListW(
        pszFilter: PCWSTR,
        Buffer: PWCHAR,
        BufferLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_ExA(
        pszFilter: PCSTR,
        Buffer: PCHAR,
        BufferLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_ExW(
        pszFilter: PCWSTR,
        Buffer: PWCHAR,
        BufferLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_SizeA(
        pulLen: PULONG,
        pszFilter: PCSTR,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_SizeW(
        pulLen: PULONG,
        pszFilter: PCWSTR,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_Size_ExA(
        pulLen: PULONG,
        pszFilter: PCSTR,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_List_Size_ExW(
        pulLen: PULONG,
        pszFilter: PCWSTR,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_Size(
        pulLen: PULONG,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_ID_Size_Ex(
        pulLen: PULONG,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_PropertyW(
        dnDevInst: DEVINST,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_PropertyExW(
        dnDevInst: DEVINST,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Registry_PropertyA(
        dnDevInst: DEVINST,
        ulProperty: ULONG,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Registry_PropertyW(
        dnDevInst: DEVINST,
        ulProperty: ULONG,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Registry_Property_ExA(
        dnDevInst: DEVINST,
        ulProperty: ULONG,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Registry_Property_ExW(
        dnDevInst: DEVINST,
        ulProperty: ULONG,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Custom_PropertyA(
        dnDevInst: DEVINST,
        pszCustomPropertyName: PCSTR,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Custom_PropertyW(
        dnDevInst: DEVINST,
        pszCustomPropertyName: PCWSTR,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Custom_Property_ExA(
        dnDevInst: DEVINST,
        pszCustomPropertyName: PCSTR,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Custom_Property_ExW(
        dnDevInst: DEVINST,
        pszCustomPropertyName: PCWSTR,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Status(
        pulStatus: PULONG,
        pulProblemNumber: PULONG,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_DevNode_Status_Ex(
        pulStatus: PULONG,
        pulProblemNumber: PULONG,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_First_Log_Conf(
        plcLogConf: PLOG_CONF,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_First_Log_Conf_Ex(
        plcLogConf: PLOG_CONF,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Global_State(
        pulState: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Global_State_Ex(
        pulState: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Hardware_Profile_InfoA(
        ulIndex: ULONG,
        pHWProfileInfo: PHWPROFILEINFO_A,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Hardware_Profile_Info_ExA(
        ulIndex: ULONG,
        pHWProfileInfo: PHWPROFILEINFO_A,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Hardware_Profile_InfoW(
        ulIndex: ULONG,
        pHWProfileInfo: PHWPROFILEINFO_W,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Hardware_Profile_Info_ExW(
        ulIndex: ULONG,
        pHWProfileInfo: PHWPROFILEINFO_W,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_HW_Prof_FlagsA(
        pDeviceID: DEVINSTID_A,
        ulHardwareProfile: ULONG,
        pulValue: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_HW_Prof_FlagsW(
        pDeviceID: DEVINSTID_W,
        ulHardwareProfile: ULONG,
        pulValue: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_HW_Prof_Flags_ExA(
        pDeviceID: DEVINSTID_A,
        ulHardwareProfile: ULONG,
        pulValue: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_HW_Prof_Flags_ExW(
        pDeviceID: DEVINSTID_W,
        ulHardwareProfile: ULONG,
        pulValue: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_AliasA(
        pszDeviceInterface: LPCSTR,
        AliasInterfaceGuid: LPGUID,
        pszAliasDeviceInterface: LPSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_AliasW(
        pszDeviceInterface: LPCWSTR,
        AliasInterfaceGuid: LPGUID,
        pszAliasDeviceInterface: LPWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_Alias_ExA(
        pszDeviceInterface: LPCSTR,
        AliasInterfaceGuid: LPGUID,
        pszAliasDeviceInterface: LPSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_Alias_ExW(
        pszDeviceInterface: LPCWSTR,
        AliasInterfaceGuid: LPGUID,
        pszAliasDeviceInterface: LPWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_ListA(
        InterfaceClassGuid: LPGUID,
        pDeviceID: DEVINSTID_A,
        Buffer: PCHAR,
        BufferLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_ListW(
        InterfaceClassGuid: LPGUID,
        pDeviceID: DEVINSTID_W,
        Buffer: PWCHAR,
        BufferLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_ExA(
        InterfaceClassGuid: LPGUID,
        pDeviceID: DEVINSTID_A,
        Buffer: PCHAR,
        BufferLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_ExW(
        InterfaceClassGuid: LPGUID,
        pDeviceID: DEVINSTID_W,
        Buffer: PWCHAR,
        BufferLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_SizeA(
        pulLen: PULONG,
        InterfaceClassGuid: LPGUID,
        pDeviceID: DEVINSTID_A,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_SizeW(
        pulLen: PULONG,
        InterfaceClassGuid: LPGUID,
        pDeviceID: DEVINSTID_W,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_Size_ExA(
        pulLen: PULONG,
        InterfaceClassGuid: LPGUID,
        pDeviceID: DEVINSTID_A,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_List_Size_ExW(
        pulLen: PULONG,
        InterfaceClassGuid: LPGUID,
        pDeviceID: DEVINSTID_W,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_PropertyW(
        pszDeviceInterface: LPCWSTR,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Device_Interface_PropertyExW(
        pszDeviceInterface: LPCWSTR,
        PropertyKey: *const DEVPROPKEY,
        PropertyType: *mut DEVPROPTYPE,
        PropertyBuffer: PBYTE,
        PropertyBufferSize: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Log_Conf_Priority(
        lcLogConf: LOG_CONF,
        pPriority: PRIORITY,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Log_Conf_Priority_Ex(
        lcLogConf: LOG_CONF,
        pPriority: PRIORITY,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Next_Log_Conf(
        plcLogConf: PLOG_CONF,
        lcLogConf: LOG_CONF,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Next_Log_Conf_Ex(
        plcLogConf: PLOG_CONF,
        lcLogConf: LOG_CONF,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Parent(
        pdnDevInst: PDEVINST,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Parent_Ex(
        pdnDevInst: PDEVINST,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Res_Des_Data(
        rdResDes: RES_DES,
        Buffer: PVOID,
        BufferLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Res_Des_Data_Ex(
        rdResDes: RES_DES,
        Buffer: PVOID,
        BufferLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Res_Des_Data_Size(
        pulSize: PULONG,
        rdResDes: RES_DES,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Res_Des_Data_Size_Ex(
        pulSize: PULONG,
        rdResDes: RES_DES,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Sibling(
        pdnDevInst: PDEVINST,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Sibling_Ex(
        pdnDevInst: PDEVINST,
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Version() -> WORD;
    pub fn CM_Get_Version_Ex(
        hMachine: HMACHINE,
    ) -> WORD;
    pub fn CM_Is_Version_Available(
        wVersion: WORD,
    ) -> BOOL;
    pub fn CM_Is_Version_Available_Ex(
        wVersion: WORD,
        hMachine: HMACHINE,
    ) -> BOOL;
    pub fn CM_Intersect_Range_List(
        rlhOld1: RANGE_LIST,
        rlhOld2: RANGE_LIST,
        rlhNew: RANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Invert_Range_List(
        rlhOld: RANGE_LIST,
        rlhNew: RANGE_LIST,
        ullMaxValue: DWORDLONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Locate_DevNodeA(
        pdnDevInst: PDEVINST,
        pDeviceID: DEVINSTID_A,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Locate_DevNodeW(
        pdnDevInst: PDEVINST,
        pDeviceID: DEVINSTID_W,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Locate_DevNode_ExA(
        pdnDevInst: PDEVINST,
        pDeviceID: DEVINSTID_A,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Locate_DevNode_ExW(
        pdnDevInst: PDEVINST,
        pDeviceID: DEVINSTID_W,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Merge_Range_List(
        rlhOld1: RANGE_LIST,
        rlhOld2: RANGE_LIST,
        rlhNew: RANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Modify_Res_Des(
        prdResDes: PRES_DES,
        rdResDes: RES_DES,
        ResourceID: RESOURCEID,
        ResourceData: PCVOID,
        ResourceLen: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Modify_Res_Des_Ex(
        prdResDes: PRES_DES,
        rdResDes: RES_DES,
        ResourceID: RESOURCEID,
        ResourceData: PCVOID,
        ResourceLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Move_DevNode(
        dnFromDevInst: DEVINST,
        dnToDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Move_DevNode_Ex(
        dnFromDevInst: DEVINST,
        dnToDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Next_Range(
        preElement: PRANGE_LIST,
        pullStart: PDWORDLONG,
        pullEnd: PDWORDLONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Next_Res_Des(
        prdResDes: PRES_DES,
        rdResDes: RES_DES,
        ForResource: RESOURCEID,
        pResourceID: PRESOURCEID,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Next_Res_Des_Ex(
        prdResDes: PRES_DES,
        rdResDes: RES_DES,
        ForResource: RESOURCEID,
        pResourceID: PRESOURCEID,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Open_Class_KeyA(
        ClassGuid: LPGUID,
        pszClassName: LPCSTR,
        samDesired: REGSAM,
        Disposition: REGDISPOSITION,
        phkClass: PHKEY,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_Class_KeyW(
        ClassGuid: LPGUID,
        pszClassName: LPCWSTR,
        samDesired: REGSAM,
        Disposition: REGDISPOSITION,
        phkClass: PHKEY,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_Class_Key_ExA(
        ClassGuid: LPGUID,
        pszClassName: LPCSTR,
        samDesired: REGSAM,
        Disposition: REGDISPOSITION,
        phkClass: PHKEY,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Open_Class_Key_ExW(
        ClassGuid: LPGUID,
        pszClassName: LPCWSTR,
        samDesired: REGSAM,
        Disposition: REGDISPOSITION,
        phkClass: PHKEY,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Open_DevNode_Key(
        dnDevNode: DEVINST,
        samDesired: REGSAM,
        ulHardwareProfile: ULONG,
        Disposition: REGDISPOSITION,
        phkDevice: PHKEY,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_DevNode_Key_Ex(
        dnDevNode: DEVINST,
        samDesired: REGSAM,
        ulHardwareProfile: ULONG,
        Disposition: REGDISPOSITION,
        phkDevice: PHKEY,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Open_Device_Interface_KeyA(
        pszDeviceInterface: LPCSTR,
        samDesired: REGSAM,
        Disposition: REGDISPOSITION,
        phkDeviceInterface: PHKEY,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_Device_Interface_KeyW(
        pszDeviceInterface: LPCWSTR,
        samDesired: REGSAM,
        Disposition: REGDISPOSITION,
        phkDeviceInterface: PHKEY,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Open_Device_Interface_Key_ExA(
        pszDeviceInterface: LPCSTR,
        samDesired: REGSAM,
        Disposition: REGDISPOSITION,
        phkDeviceInterface: PHKEY,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Open_Device_Interface_Key_ExW(
        pszDeviceInterface: LPCWSTR,
        samDesired: REGSAM,
        Disposition: REGDISPOSITION,
        phkDeviceInterface: PHKEY,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Delete_Device_Interface_KeyA(
        pszDeviceInterface: LPCSTR,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Delete_Device_Interface_KeyW(
        pszDeviceInterface: LPCWSTR,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Delete_Device_Interface_Key_ExA(
        pszDeviceInterface: LPCSTR,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Delete_Device_Interface_Key_ExW(
        pszDeviceInterface: LPCWSTR,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_Arbitrator_Free_Data(
        pData: PVOID,
        DataLen: ULONG,
        dnDevInst: DEVINST,
        ResourceID: RESOURCEID,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Query_Arbitrator_Free_Data_Ex(
        pData: PVOID,
        DataLen: ULONG,
        dnDevInst: DEVINST,
        ResourceID: RESOURCEID,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_Arbitrator_Free_Size(
        pulSize: PULONG,
        dnDevInst: DEVINST,
        ResourceID: RESOURCEID,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Query_Arbitrator_Free_Size_Ex(
        pulSize: PULONG,
        dnDevInst: DEVINST,
        ResourceID: RESOURCEID,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_Remove_SubTree(
        dnAncestor: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Query_Remove_SubTree_Ex(
        dnAncestor: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_And_Remove_SubTreeA(
        dnAncestor: DEVINST,
        pVetoType: PPNP_VETO_TYPE,
        pszVetoName: LPSTR,
        ulNameLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Query_And_Remove_SubTree_ExA(
        dnAncestor: DEVINST,
        pVetoType: PPNP_VETO_TYPE,
        pszVetoName: LPSTR,
        ulNameLength: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_And_Remove_SubTreeW(
        dnAncestor: DEVINST,
        pVetoType: PPNP_VETO_TYPE,
        pszVetoName: LPWSTR,
        ulNameLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Query_And_Remove_SubTree_ExW(
        dnAncestor: DEVINST,
        pVetoType: PPNP_VETO_TYPE,
        pszVetoName: LPWSTR,
        ulNameLength: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Request_Device_EjectA(
        dnDevInst: DEVINST,
        pVetoType: PPNP_VETO_TYPE,
        pszVetoName: LPSTR,
        ulNameLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Request_Device_Eject_ExA(
        dnDevInst: DEVINST,
        pVetoType: PPNP_VETO_TYPE,
        pszVetoName: LPSTR,
        ulNameLength: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Request_Device_EjectW(
        dnDevInst: DEVINST,
        pVetoType: PPNP_VETO_TYPE,
        pszVetoName: LPWSTR,
        ulNameLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Request_Device_Eject_ExW(
        dnDevInst: DEVINST,
        pVetoType: PPNP_VETO_TYPE,
        pszVetoName: LPWSTR,
        ulNameLength: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Reenumerate_DevNode(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Reenumerate_DevNode_Ex(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_InterfaceA(
        dnDevInst: DEVINST,
        InterfaceClassGuid: LPGUID,
        pszReference: LPCSTR,
        pszDeviceInterface: LPSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_InterfaceW(
        dnDevInst: DEVINST,
        InterfaceClassGuid: LPGUID,
        pszReference: LPCWSTR,
        pszDeviceInterface: LPWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_Interface_ExA(
        dnDevInst: DEVINST,
        InterfaceClassGuid: LPGUID,
        pszReference: LPCSTR,
        pszDeviceInterface: LPSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_Interface_ExW(
        dnDevInst: DEVINST,
        InterfaceClassGuid: LPGUID,
        pszReference: LPCWSTR,
        pszDeviceInterface: LPWSTR,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Problem_Ex(
        dnDevInst: DEVINST,
        ulProblem: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Problem(
        dnDevInst: DEVINST,
        ulProblem: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Unregister_Device_InterfaceA(
        pszDeviceInterface: LPCSTR,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Unregister_Device_InterfaceW(
        pszDeviceInterface: LPCWSTR,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Unregister_Device_Interface_ExA(
        pszDeviceInterface: LPCSTR,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Unregister_Device_Interface_ExW(
        pszDeviceInterface: LPCWSTR,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_Driver(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Register_Device_Driver_Ex(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Remove_SubTree(
        dnAncestor: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Remove_SubTree_Ex(
        dnAncestor: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Registry_PropertyA(
        dnDevInst: DEVINST,
        ulProperty: ULONG,
        Buffer: PCVOID,
        ulLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Registry_PropertyW(
        dnDevInst: DEVINST,
        ulProperty: ULONG,
        Buffer: PCVOID,
        ulLength: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Registry_Property_ExA(
        dnDevInst: DEVINST,
        ulProperty: ULONG,
        Buffer: PCVOID,
        ulLength: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_DevNode_Registry_Property_ExW(
        dnDevInst: DEVINST,
        ulProperty: ULONG,
        Buffer: PCVOID,
        ulLength: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Is_Dock_Station_Present(
        pbPresent: PBOOL,
    ) -> CONFIGRET;
    pub fn CM_Is_Dock_Station_Present_Ex(
        pbPresent: PBOOL,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Request_Eject_PC() -> CONFIGRET;
    pub fn CM_Request_Eject_PC_Ex(
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_FlagsA(
        pDeviceID: DEVINSTID_A,
        ulConfig: ULONG,
        ulValue: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_FlagsW(
        pDeviceID: DEVINSTID_W,
        ulConfig: ULONG,
        ulValue: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_Flags_ExA(
        pDeviceID: DEVINSTID_A,
        ulConfig: ULONG,
        ulValue: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_Flags_ExW(
        pDeviceID: DEVINSTID_A,
        ulConfig: ULONG,
        ulValue: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Setup_DevNode(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Setup_DevNode_Ex(
        dnDevInst: DEVINST,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Test_Range_Available(
        ullStartValue: DWORDLONG,
        ullEndValue: DWORDLONG,
        rlh: RANGE_LIST,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Uninstall_DevNode(
        dnDevInst: DEVNODE,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Uninstall_DevNode_Ex(
        dnDevInst: DEVNODE,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Run_Detection(
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Run_Detection_Ex(
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof(
        ulHardwareProfile: ULONG,
        ulFlags: ULONG,
    ) -> CONFIGRET;
    pub fn CM_Set_HW_Prof_Ex(
        ulHardwareProfile: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Query_Resource_Conflict_List(
        pclConflictList: PCONFLICT_LIST,
        dnDevInst: DEVINST,
        ResourceID: RESOURCEID,
        ResourceData: PCVOID,
        ResourceLen: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Free_Resource_Conflict_Handle(
        clConflictList: CONFLICT_LIST,
    ) -> CONFIGRET;
    pub fn CM_Get_Resource_Conflict_Count(
        clConflictList: CONFLICT_LIST,
        pulCount: PULONG,
    ) -> CONFIGRET;
    pub fn CM_Get_Resource_Conflict_DetailsA(
        clConflictList: CONFLICT_LIST,
        ulIndex: ULONG,
        pConflictDetails: PCONFLICT_DETAILS_A,
    ) -> CONFIGRET;
    pub fn CM_Get_Resource_Conflict_DetailsW(
        clConflictList: CONFLICT_LIST,
        ulIndex: ULONG,
        pConflictDetails: PCONFLICT_DETAILS_W,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Registry_PropertyW(
        ClassGuid: LPGUID,
        ulProperty: ULONG,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_Class_Registry_PropertyW(
        ClassGuid: LPGUID,
        ulProperty: ULONG,
        Buffer: PCVOID,
        ulLength: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Get_Class_Registry_PropertyA(
        ClassGuid: LPGUID,
        ulProperty: ULONG,
        pulRegDataType: PULONG,
        Buffer: PVOID,
        pulLength: PULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CM_Set_Class_Registry_PropertyA(
        ClassGuid: LPGUID,
        ulProperty: ULONG,
        Buffer: PCVOID,
        ulLength: ULONG,
        ulFlags: ULONG,
        hMachine: HMACHINE,
    ) -> CONFIGRET;
    pub fn CMP_WaitNoPendingInstallEvents(
        dwTimeout: DWORD,
    ) -> DWORD;
}
pub const CR_SUCCESS: CONFIGRET = 0x00000000;
pub const CR_DEFAULT: CONFIGRET = 0x00000001;
pub const CR_OUT_OF_MEMORY: CONFIGRET = 0x00000002;
pub const CR_INVALID_POINTER: CONFIGRET = 0x00000003;
pub const CR_INVALID_FLAG: CONFIGRET = 0x00000004;
pub const CR_INVALID_DEVNODE: CONFIGRET = 0x00000005;
pub const CR_INVALID_DEVINST: CONFIGRET = CR_INVALID_DEVNODE;
pub const CR_INVALID_RES_DES: CONFIGRET = 0x00000006;
pub const CR_INVALID_LOG_CONF: CONFIGRET = 0x00000007;
pub const CR_INVALID_ARBITRATOR: CONFIGRET = 0x00000008;
pub const CR_INVALID_NODELIST: CONFIGRET = 0x00000009;
pub const CR_DEVNODE_HAS_REQS: CONFIGRET = 0x0000000A;
pub const CR_DEVINST_HAS_REQS: CONFIGRET = CR_DEVNODE_HAS_REQS;
pub const CR_INVALID_RESOURCEID: CONFIGRET = 0x0000000B;
pub const CR_DLVXD_NOT_FOUND: CONFIGRET = 0x0000000C;
pub const CR_NO_SUCH_DEVNODE: CONFIGRET = 0x0000000D;
pub const CR_NO_SUCH_DEVINST: CONFIGRET = CR_NO_SUCH_DEVNODE;
pub const CR_NO_MORE_LOG_CONF: CONFIGRET = 0x0000000E;
pub const CR_NO_MORE_RES_DES: CONFIGRET = 0x0000000F;
pub const CR_ALREADY_SUCH_DEVNODE: CONFIGRET = 0x00000010;
pub const CR_ALREADY_SUCH_DEVINST: CONFIGRET = CR_ALREADY_SUCH_DEVNODE;
pub const CR_INVALID_RANGE_LIST: CONFIGRET = 0x00000011;
pub const CR_INVALID_RANGE: CONFIGRET = 0x00000012;
pub const CR_FAILURE: CONFIGRET = 0x00000013;
pub const CR_NO_SUCH_LOGICAL_DEV: CONFIGRET = 0x00000014;
pub const CR_CREATE_BLOCKED: CONFIGRET = 0x00000015;
pub const CR_NOT_SYSTEM_VM: CONFIGRET = 0x00000016;
pub const CR_REMOVE_VETOED: CONFIGRET = 0x00000017;
pub const CR_APM_VETOED: CONFIGRET = 0x00000018;
pub const CR_INVALID_LOAD_TYPE: CONFIGRET = 0x00000019;
pub const CR_BUFFER_SMALL: CONFIGRET = 0x0000001A;
pub const CR_NO_ARBITRATOR: CONFIGRET = 0x0000001B;
pub const CR_NO_REGISTRY_HANDLE: CONFIGRET = 0x0000001C;
pub const CR_REGISTRY_ERROR: CONFIGRET = 0x0000001D;
pub const CR_INVALID_DEVICE_ID: CONFIGRET = 0x0000001E;
pub const CR_INVALID_DATA: CONFIGRET = 0x0000001F;
pub const CR_INVALID_API: CONFIGRET = 0x00000020;
pub const CR_DEVLOADER_NOT_READY: CONFIGRET = 0x00000021;
pub const CR_NEED_RESTART: CONFIGRET = 0x00000022;
pub const CR_NO_MORE_HW_PROFILES: CONFIGRET = 0x00000023;
pub const CR_DEVICE_NOT_THERE: CONFIGRET = 0x00000024;
pub const CR_NO_SUCH_VALUE: CONFIGRET = 0x00000025;
pub const CR_WRONG_TYPE: CONFIGRET = 0x00000026;
pub const CR_INVALID_PRIORITY: CONFIGRET = 0x00000027;
pub const CR_NOT_DISABLEABLE: CONFIGRET = 0x00000028;
pub const CR_FREE_RESOURCES: CONFIGRET = 0x00000029;
pub const CR_QUERY_VETOED: CONFIGRET = 0x0000002A;
pub const CR_CANT_SHARE_IRQ: CONFIGRET = 0x0000002B;
pub const CR_NO_DEPENDENT: CONFIGRET = 0x0000002C;
pub const CR_SAME_RESOURCES: CONFIGRET = 0x0000002D;
pub const CR_NO_SUCH_REGISTRY_KEY: CONFIGRET = 0x0000002E;
pub const CR_INVALID_MACHINENAME: CONFIGRET = 0x0000002F;
pub const CR_REMOTE_COMM_FAILURE: CONFIGRET = 0x00000030;
pub const CR_MACHINE_UNAVAILABLE: CONFIGRET = 0x00000031;
pub const CR_NO_CM_SERVICES: CONFIGRET = 0x00000032;
pub const CR_ACCESS_DENIED: CONFIGRET = 0x00000033;
pub const CR_CALL_NOT_IMPLEMENTED: CONFIGRET = 0x00000034;
pub const CR_INVALID_PROPERTY: CONFIGRET = 0x00000035;
pub const CR_DEVICE_INTERFACE_ACTIVE: CONFIGRET = 0x00000036;
pub const CR_NO_SUCH_DEVICE_INTERFACE: CONFIGRET = 0x00000037;
pub const CR_INVALID_REFERENCE_STRING: CONFIGRET = 0x00000038;
pub const CR_INVALID_CONFLICT_LIST: CONFIGRET = 0x00000039;
pub const CR_INVALID_INDEX: CONFIGRET = 0x0000003A;
pub const CR_INVALID_STRUCTURE_SIZE: CONFIGRET = 0x0000003B;
pub const NUM_CR_RESULTS: CONFIGRET = 0x0000003C;

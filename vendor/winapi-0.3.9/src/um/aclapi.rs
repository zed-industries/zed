// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, DWORD, PULONG, ULONG, USHORT};
use um::accctrl::{
    ACCESS_MODE, MULTIPLE_TRUSTEE_OPERATION, PEXPLICIT_ACCESS_A, PEXPLICIT_ACCESS_W,
    PFN_OBJECT_MGR_FUNCTS, PINHERITED_FROMA, PINHERITED_FROMW, POBJECTS_AND_NAME_A,
    POBJECTS_AND_NAME_W, POBJECTS_AND_SID, PPROG_INVOKE_SETTING, PROG_INVOKE_SETTING, PTRUSTEE_A,
    PTRUSTEE_W, SE_OBJECT_TYPE, TRUSTEE_FORM, TRUSTEE_TYPE
};
use um::winnt::{
    HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PACCESS_MASK, PACL, PGENERIC_MAPPING,
    PSECURITY_DESCRIPTOR, PSID, PVOID, SECURITY_INFORMATION
};
FN!{cdecl FN_PROGRESS(
    pObjectName: LPWSTR,
    Status: DWORD,
    pInvokeSetting: PPROG_INVOKE_SETTING,
    Args: PVOID,
    SecuritySet: BOOL,
) -> ()}
extern "system" {
    pub fn SetEntriesInAclA(
        cCountOfExplicitEntries: ULONG,
        pListOfExplicitEntries: PEXPLICIT_ACCESS_A,
        OldAcl: PACL,
        NewAcl: *mut PACL,
    ) -> DWORD;
    pub fn SetEntriesInAclW(
        cCountOfExplicitEntries: ULONG,
        pListOfExplicitEntries: PEXPLICIT_ACCESS_W,
        OldAcl: PACL,
        NewAcl: *mut PACL,
    ) -> DWORD;
    pub fn GetExplicitEntriesFromAclA(
        pacl: PACL,
        pcCountOfExplicitEntries: PULONG,
        pListOfExplicitEntries: *mut PEXPLICIT_ACCESS_A,
    ) -> DWORD;
    pub fn GetExplicitEntriesFromAclW(
        pacl: PACL,
        pcCountOfExplicitEntries: PULONG,
        pListOfExplicitEntries: *mut PEXPLICIT_ACCESS_W,
    ) -> DWORD;
    pub fn GetEffectiveRightsFromAclA(
        pacl: PACL,
        pTrustee: PTRUSTEE_A,
        pAccessRight: PACCESS_MASK,
    ) -> DWORD;
    pub fn GetEffectiveRightsFromAclW(
        pacl: PACL,
        pTrustee: PTRUSTEE_W,
        pAccessRight: PACCESS_MASK,
    ) -> DWORD;
    pub fn GetAuditedPermissionsFromAclA(
        pAcl: PACL,
        pTrustee: PTRUSTEE_A,
        pSuccessfulAuditedRights: PACCESS_MASK,
        pFailedAuditRights: PACCESS_MASK,
    ) -> DWORD;
    pub fn GetAuditedPermissionsFromAclW(
        pAcl: PACL,
        pTrustee: PTRUSTEE_W,
        pSuccessfulAuditedRights: PACCESS_MASK,
        pFailedAuditRights: PACCESS_MASK,
    ) -> DWORD;
    pub fn GetNamedSecurityInfoA(
        pObjectName: LPCSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        ppsidOwner: *mut PSID,
        ppsidGroup: *mut PSID,
        ppDacl: *mut PACL,
        ppSacl: *mut PACL,
        ppSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> DWORD;
    pub fn GetNamedSecurityInfoW(
        pObjectName: LPCWSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        ppsidOwner: *mut PSID,
        ppsidGroup: *mut PSID,
        ppDacl: *mut PACL,
        ppSacl: *mut PACL,
        ppSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> DWORD;
    pub fn GetSecurityInfo(
        handle: HANDLE,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        ppsidOwner: *mut PSID,
        ppsidGroup: *mut PSID,
        ppDacl: *mut PACL,
        ppSacl: *mut PACL,
        ppSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> DWORD;
    pub fn SetNamedSecurityInfoA(
        pObjectame: LPSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        psidOwner: PSID,
        psidGroup: PSID,
        pDacl: PACL,
        pSacl: PACL,
    ) -> DWORD;
    pub fn SetNamedSecurityInfoW(
        pObjectame: LPWSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        psidOwner: PSID,
        psidGroup: PSID,
        pDacl: PACL,
        pSacl: PACL,
    ) -> DWORD;
    pub fn SetSecurityInfo(
        handle: HANDLE,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        psidOwner: PSID,
        psidGroup: PSID,
        pDacl: PACL,
        pSacl: PACL,
    ) -> DWORD;
    pub fn GetInheritanceSourceA(
        pObjectName: LPSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        Container: BOOL,
        pObjectClassGuids: *mut *mut GUID,
        GuidCount: DWORD,
        pAcl: PACL,
        pfnArray: PFN_OBJECT_MGR_FUNCTS,
        pGenericMapping: PGENERIC_MAPPING,
        pInheritArray: PINHERITED_FROMA,
    ) -> DWORD;
    pub fn GetInheritanceSourceW(
        pObjectName: LPWSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        Container: BOOL,
        pObjectClassGuids: *mut *mut GUID,
        GuidCount: DWORD,
        pAcl: PACL,
        pfnArray: PFN_OBJECT_MGR_FUNCTS,
        pGenericMapping: PGENERIC_MAPPING,
        pInheritArray: PINHERITED_FROMW,
    ) -> DWORD;
    pub fn FreeInheritedFromArray(
        pInheritArray: PINHERITED_FROMW,
        AceCnt: USHORT,
        pfnArray: PFN_OBJECT_MGR_FUNCTS,
    ) -> DWORD;
    pub fn TreeResetNamedSecurityInfoA(
        pObjectName: LPSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        pOwner: PSID,
        pGroup: PSID,
        pDacl: PACL,
        pSacl: PACL,
        KeepExplicit: BOOL,
        fnProgress: FN_PROGRESS,
        ProgressInvokeSetting: PROG_INVOKE_SETTING,
        Args: PVOID,
    ) -> DWORD;
    pub fn TreeResetNamedSecurityInfoW(
        pObjectName: LPWSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        pOwner: PSID,
        pGroup: PSID,
        pDacl: PACL,
        pSacl: PACL,
        KeepExplicit: BOOL,
        fnProgress: FN_PROGRESS,
        ProgressInvokeSetting: PROG_INVOKE_SETTING,
        Args: PVOID,
    ) -> DWORD;
    pub fn TreeSetNamedSecurityInfoA(
        pObjectName: LPSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        pOwner: PSID,
        pGroup: PSID,
        pDacl: PACL,
        pSacl: PACL,
        dwAction: DWORD,
        fnProgress: FN_PROGRESS,
        ProgressInvokeSetting: PROG_INVOKE_SETTING,
        Args: PVOID,
    ) -> DWORD;
    pub fn TreeSetNamedSecurityInfoW(
        pObjectName: LPWSTR,
        ObjectType: SE_OBJECT_TYPE,
        SecurityInfo: SECURITY_INFORMATION,
        pOwner: PSID,
        pGroup: PSID,
        pDacl: PACL,
        pSacl: PACL,
        dwAction: DWORD,
        fnProgress: FN_PROGRESS,
        ProgressInvokeSetting: PROG_INVOKE_SETTING,
        Args: PVOID,
    ) -> DWORD;
    pub fn BuildSecurityDescriptorA(
        pOwner: PTRUSTEE_A,
        pGroup: PTRUSTEE_A,
        cCountOfAccessEntries: ULONG,
        pListOfAccessEntries: PEXPLICIT_ACCESS_A,
        cCountOfAuditEntries: ULONG,
        pListOfAuditEntries: PEXPLICIT_ACCESS_A,
        pOldSD: PSECURITY_DESCRIPTOR,
        pSizeNewSD: PULONG,
        pNewSD: *mut PSECURITY_DESCRIPTOR,
    ) -> DWORD;
    pub fn BuildSecurityDescriptorW(
        pOwner: PTRUSTEE_W,
        pGroup: PTRUSTEE_W,
        cCountOfAccessEntries: ULONG,
        pListOfAccessEntries: PEXPLICIT_ACCESS_W,
        cCountOfAuditEntries: ULONG,
        pListOfAuditEntries: PEXPLICIT_ACCESS_W,
        pOldSD: PSECURITY_DESCRIPTOR,
        pSizeNewSD: PULONG,
        pNewSD: *mut PSECURITY_DESCRIPTOR,
    ) -> DWORD;
    pub fn LookupSecurityDescriptorPartsA(
        ppOwner: *mut PTRUSTEE_A,
        ppGroup: *mut PTRUSTEE_A,
        pcCountOfAccessEntries: PULONG,
        ppListOfAccessEntries: *mut PEXPLICIT_ACCESS_A,
        pcCountOfAuditEntries: PULONG,
        ppListOfAuditEntries: *mut PEXPLICIT_ACCESS_A,
        pSD: PSECURITY_DESCRIPTOR,
    ) -> DWORD;
    pub fn LookupSecurityDescriptorPartsW(
        ppOwner: *mut PTRUSTEE_W,
        ppGroup: *mut PTRUSTEE_W,
        pcCountOfAccessEntries: PULONG,
        ppListOfAccessEntries: *mut PEXPLICIT_ACCESS_W,
        pcCountOfAuditEntries: PULONG,
        ppListOfAuditEntries: *mut PEXPLICIT_ACCESS_W,
        pSD: PSECURITY_DESCRIPTOR,
    ) -> DWORD;
    pub fn BuildExplicitAccessWithNameA(
        pExplicitAccess: PEXPLICIT_ACCESS_A,
        pTrusteeName: LPSTR,
        AccessPermissions: DWORD,
        AccessMode: ACCESS_MODE,
        Inheritance: DWORD,
    );
    pub fn BuildExplicitAccessWithNameW(
        pExplicitAccess: PEXPLICIT_ACCESS_W,
        pTrusteeName: LPWSTR,
        AccessPermissions: DWORD,
        AccessMode: ACCESS_MODE,
        Inheritance: DWORD,
    );
    pub fn BuildImpersonateExplicitAccessWithNameA(
        pExplicitAccess: PEXPLICIT_ACCESS_A,
        pTrusteeName: LPSTR,
        pTrustee: PTRUSTEE_A,
        AccessPermissions: DWORD,
        AccessMode: ACCESS_MODE,
        Inheritance: DWORD,
    );
    pub fn BuildImpersonateExplicitAccessWithNameW(
        pExplicitAccess: PEXPLICIT_ACCESS_W,
        pTrusteeName: LPWSTR,
        pTrustee: PTRUSTEE_W,
        AccessPermissions: DWORD,
        AccessMode: ACCESS_MODE,
        Inheritance: DWORD,
    );
    pub fn BuildTrusteeWithNameA(
        pTrustee: PTRUSTEE_A,
        pName: LPSTR,
    );
    pub fn BuildTrusteeWithNameW(
        pTrustee: PTRUSTEE_W,
        pName: LPWSTR,
    );
    pub fn BuildImpersonateTrusteeA(
        pTrustee: PTRUSTEE_A,
        pImpersonateTrustee: PTRUSTEE_A,
    );
    pub fn BuildImpersonateTrusteeW(
        pTrustee: PTRUSTEE_W,
        pImpersonateTrustee: PTRUSTEE_W,
    );
    pub fn BuildTrusteeWithSidA(
        pTrustee: PTRUSTEE_A,
        pSid: PSID,
    );
    pub fn BuildTrusteeWithSidW(
        pTrustee: PTRUSTEE_W,
        pSid: PSID,
    );
    pub fn BuildTrusteeWithObjectsAndSidA(
        pTrustee: PTRUSTEE_A,
        pObjSid: POBJECTS_AND_SID,
        pObjectGuid: *mut GUID,
        pInheritedObjectGuid: *mut GUID,
        pSid: PSID,
    );
    pub fn BuildTrusteeWithObjectsAndSidW(
        pTrustee: PTRUSTEE_W,
        pObjSid: POBJECTS_AND_SID,
        pObjectGuid: *mut GUID,
        pInheritedObjectGuid: *mut GUID,
        pSid: PSID,
    );
    pub fn BuildTrusteeWithObjectsAndNameA(
        pTrustee: PTRUSTEE_A,
        pObjName: POBJECTS_AND_NAME_A,
        ObjectType: SE_OBJECT_TYPE,
        ObjectTypeName: LPSTR,
        InheritedObjectTypeName: LPSTR,
        Name: LPSTR,
    );
    pub fn BuildTrusteeWithObjectsAndNameW(
        pTrustee: PTRUSTEE_W,
        pObjName: POBJECTS_AND_NAME_W,
        ObjectType: SE_OBJECT_TYPE,
        ObjectTypeName: LPWSTR,
        InheritedObjectTypeName: LPWSTR,
        Name: LPWSTR,
    );
    pub fn GetTrusteeNameA(
        pTrustee: PTRUSTEE_A,
    ) -> LPSTR;
    pub fn GetTrusteeNameW(
        pTrustee: PTRUSTEE_W,
    ) -> LPWSTR;
    pub fn GetTrusteeTypeA(
        pTrustee: PTRUSTEE_A,
    ) -> TRUSTEE_TYPE;
    pub fn GetTrusteeTypeW(
        pTrustee: PTRUSTEE_W,
    ) -> TRUSTEE_TYPE;
    pub fn GetTrusteeFormA(
        pTrustee: PTRUSTEE_A,
    ) -> TRUSTEE_FORM;
    pub fn GetTrusteeFormW(
        pTrustee: PTRUSTEE_W,
    ) -> TRUSTEE_FORM;
    pub fn GetMultipleTrusteeOperationA(
        pTrustee: PTRUSTEE_A,
    ) -> MULTIPLE_TRUSTEE_OPERATION;
    pub fn GetMultipleTrusteeOperationW(
        pTrustee: PTRUSTEE_W,
    ) -> MULTIPLE_TRUSTEE_OPERATION;
    pub fn GetMultipleTrusteeA(
        pTrustee: PTRUSTEE_A,
    ) -> PTRUSTEE_A;
    pub fn GetMultipleTrusteeW(
        pTrustee: PTRUSTEE_W,
    ) -> PTRUSTEE_W;
}

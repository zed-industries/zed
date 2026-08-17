// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! FFI bindings to psapi.
use shared::guiddef::GUID;
use shared::minwindef::{
    BOOL, BYTE, DWORD, LPBOOL, LPDWORD, LPVOID, PBOOL, PDWORD, PUCHAR, PULONG, UCHAR, ULONG
};
use um::minwinbase::LPSECURITY_ATTRIBUTES;
use um::winnt::{
    ACL_INFORMATION_CLASS, AUDIT_EVENT_TYPE, BOOLEAN, HANDLE, LONG, LPCWSTR, LPWSTR, PACL,
    PCLAIM_SECURITY_ATTRIBUTES_INFORMATION, PCWSTR, PGENERIC_MAPPING, PHANDLE, PLUID,
    PLUID_AND_ATTRIBUTES, POBJECT_TYPE_LIST, PPRIVILEGE_SET, PSECURITY_DESCRIPTOR,
    PSECURITY_DESCRIPTOR_CONTROL, PSID, PSID_AND_ATTRIBUTES, PSID_IDENTIFIER_AUTHORITY,
    PTOKEN_GROUPS, PTOKEN_PRIVILEGES, PVOID, SECURITY_DESCRIPTOR_CONTROL,
    SECURITY_IMPERSONATION_LEVEL, SECURITY_INFORMATION, TOKEN_INFORMATION_CLASS, TOKEN_TYPE,
    WELL_KNOWN_SID_TYPE
};
extern "system" {
    pub fn AccessCheck(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: DWORD,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: LPDWORD,
        GrantedAccess: LPDWORD,
        AccessStatus: LPBOOL,
    ) -> BOOL;
    pub fn AccessCheckAndAuditAlarmW(
        SubsystemName: LPCWSTR,
        HandleId: LPVOID,
        ObjectTypeName: LPWSTR,
        ObjectName: LPWSTR,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        DesiredAccess: DWORD,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOL,
        GrantedAccess: LPDWORD,
        AccessStatus: LPBOOL,
        pfGenerateOnClose: LPBOOL,
    ) -> BOOL;
    pub fn AccessCheckByType(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: DWORD,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: DWORD,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: LPDWORD,
        GrantedAccess: LPDWORD,
        AccessStatus: LPBOOL,
    ) -> BOOL;
    pub fn AccessCheckByTypeResultList(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        ClientToken: HANDLE,
        DesiredAccess: DWORD,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: DWORD,
        GenericMapping: PGENERIC_MAPPING,
        PrivilegeSet: PPRIVILEGE_SET,
        PrivilegeSetLength: LPDWORD,
        GrantedAccessList: LPDWORD,
        AccessStatusList: LPDWORD,
        ) -> BOOL;
    pub fn AccessCheckByTypeAndAuditAlarmW(
        SubsystemName: LPCWSTR,
        HandleId: LPVOID,
        ObjectTypeName: LPWSTR,
        ObjectName: LPCWSTR,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: DWORD,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: DWORD,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: DWORD,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOL,
        GrantedAccess: LPDWORD,
        AccessStatus: LPBOOL,
        pfGenerateOnClose: LPBOOL,
    ) -> BOOL;
    pub fn AccessCheckByTypeResultListAndAuditAlarmW(
        SubsystemName: LPCWSTR,
        HandleId: LPVOID,
        ObjectTypeName: LPCWSTR,
        ObjectName: LPCWSTR,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: DWORD,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: DWORD,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: DWORD,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOL,
        GrantedAccess: LPDWORD,
        AccessStatusList: LPDWORD,
        pfGenerateOnClose: LPBOOL,
    ) -> BOOL;
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(
        SubsystemName: LPCWSTR,
        HandleId: LPVOID,
        ClientToken: HANDLE,
        ObjectTypeName: LPCWSTR,
        ObjectName: LPCWSTR,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        PrincipalSelfSid: PSID,
        DesiredAccess: DWORD,
        AuditType: AUDIT_EVENT_TYPE,
        Flags: DWORD,
        ObjectTypeList: POBJECT_TYPE_LIST,
        ObjectTypeListLength: DWORD,
        GenericMapping: PGENERIC_MAPPING,
        ObjectCreation: BOOL,
        GrantedAccess: LPDWORD,
        AccessStatusList: LPDWORD,
        pfGenerateOnClose: LPBOOL,
    ) -> BOOL;
    pub fn AddAccessAllowedAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AccessMask: DWORD,
        pSid: PSID,
    ) -> BOOL;
    pub fn AddAccessAllowedAceEx(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AceFlags: DWORD,
        AccessMask: DWORD,
        pSid: PSID,
    ) -> BOOL;
    pub fn AddAccessAllowedObjectAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AceFlags: DWORD,
        AccessMask: DWORD,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        pSid: PSID,
    ) -> BOOL;
    pub fn AddAccessDeniedAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AccessMask: DWORD,
        pSid: PSID,
    ) -> BOOL;
    pub fn AddAccessDeniedAceEx(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AceFlags: DWORD,
        AccessMask: DWORD,
        pSid: PSID,
    ) -> BOOL;
    pub fn AddAccessDeniedObjectAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AceFlags: DWORD,
        AccessMask: DWORD,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        pSid: PSID,
    ) -> BOOL;
    pub fn AddAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        dwStartingAceIndex: DWORD,
        pAceList: LPVOID,
        nAceListLength: DWORD,
    ) -> BOOL;
    pub fn AddAuditAccessAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        dwAccessMask: DWORD,
        pSid: PSID,
        bAuditSuccess: BOOL,
        bAuditFailure: BOOL,
    ) -> BOOL;
    pub fn AddAuditAccessAceEx(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AceFlags: DWORD,
        dwAccessMask: DWORD,
        pSid: PSID,
        bAuditSuccess: BOOL,
        bAuditFailure: BOOL,
    ) -> BOOL;
    pub fn AddAuditAccessObjectAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AceFlags: DWORD,
        AccessMask: DWORD,
        ObjectTypeGuid: *mut GUID,
        InheritedObjectTypeGuid: *mut GUID,
        pSid: PSID,
        bAuditSuccess: BOOL,
        bAuditFailure: BOOL,
    ) -> BOOL;
    pub fn AddMandatoryAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AceFlags: DWORD,
        MandatoryPolicy: DWORD,
        pLabelSid: PSID,
    ) -> BOOL;
    pub fn AddResourceAttributeAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AceFlags: DWORD,
        AccessMask: DWORD,
        pSid: PSID,
        pAttributeInfo: PCLAIM_SECURITY_ATTRIBUTES_INFORMATION,
        pReturnLength: PDWORD,
    ) -> BOOL;
    pub fn AddScopedPolicyIDAce(
        pAcl: PACL,
        dwAceRevision: DWORD,
        AceFlags: DWORD,
        AccessMask: DWORD,
        pSid: PSID,
    ) -> BOOL;
    pub fn AdjustTokenGroups(
        TokenHandle: HANDLE,
        ResetToDefault: BOOL,
        NewState: PTOKEN_GROUPS,
        BufferLength: DWORD,
        PreviousState: PTOKEN_GROUPS,
        ReturnLength: PDWORD,
    ) -> BOOL;
    pub fn AdjustTokenPrivileges(
        TokenHandle: HANDLE,
        DisableAllPrivileges: BOOL,
        NewState: PTOKEN_PRIVILEGES,
        BufferLength: DWORD,
        PreviousState: PTOKEN_PRIVILEGES,
        ReturnLength: PDWORD,
    ) -> BOOL;
    pub fn AllocateAndInitializeSid(
        pIdentifierAuthoirity: PSID_IDENTIFIER_AUTHORITY,
        nSubAuthorityCount: BYTE,
        dwSubAuthority0: DWORD,
        dwSubAuthority1: DWORD,
        dwSubAuthority2: DWORD,
        dwSubAuthority3: DWORD,
        dwSubAuthority4: DWORD,
        dwSubAuthority5: DWORD,
        dwSubAuthority6: DWORD,
        dwSubAuthority7: DWORD,
        pSid: *mut PSID,
    ) -> BOOL;
    pub fn AllocateLocallyUniqueId(
        Luid: PLUID,
    ) -> BOOL;
    pub fn AreAllAccessesGranted(
        GrantedAccess: DWORD,
        DesiredAccess: DWORD,
    ) -> BOOL;
    pub fn AreAnyAccessesGranted(
        GrantedAccess: DWORD,
        DesiredAccess: DWORD,
    ) -> BOOL;
    pub fn CheckTokenMembership(
        TokenHandle: HANDLE,
        SidToCheck: PSID,
        IsMember: PBOOL,
    ) -> BOOL;
    pub fn CheckTokenCapability(
        TokenHandle: HANDLE,
        CapabilitySidToCheck: PSID,
        HasCapability: PBOOL,
    ) -> BOOL;
    pub fn GetAppContainerAce(
        Acl: PACL,
        StartingAceIndex: DWORD,
        AppContainerAce: *mut PVOID,
        AppContainerAceIndex: *mut DWORD,
    ) -> BOOL;
    pub fn CheckTokenMembershipEx(
        TokenHandle: HANDLE,
        SidToCheck: PSID,
        Flags: DWORD,
        IsMember: PBOOL,
    ) -> BOOL;
    pub fn ConvertToAutoInheritPrivateObjectSecurity(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CurrentSecurityDescriptor: PSECURITY_DESCRIPTOR,
        NewSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        ObjectType: *mut GUID,
        IsDirectoryObject: BOOLEAN,
        GenericMapping: PGENERIC_MAPPING,
    ) -> BOOL;
    pub fn CopySid(
        nDestinationSidLength: DWORD,
        pDestinationSid: PSID,
        pSourceSid: PSID,
    ) -> BOOL;
    pub fn CreatePrivateObjectSecurity(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewDescriptor: *mut PSECURITY_DESCRIPTOR,
        IsDirectoryObject: BOOL,
        Token: HANDLE,
        GenericMapping: PGENERIC_MAPPING,
    ) -> BOOL;
    pub fn CreatePrivateObjectSecurityEx(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        ObjectType: *mut GUID,
        IsContainerObject: BOOL,
        AutoInheritFlags: ULONG,
        Token: HANDLE,
        GenericMapping: PGENERIC_MAPPING,
    ) -> BOOL;
    pub fn CreatePrivateObjectSecurityWithMultipleInheritance(
        ParentDescriptor: PSECURITY_DESCRIPTOR,
        CreatorDescriptor: PSECURITY_DESCRIPTOR,
        NewSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        ObjectTypes: *mut *mut GUID,
        GuidCount: ULONG,
        IsContainerObject: BOOL,
        AutoInheritFlags: ULONG,
        Token: HANDLE,
        GenericMapping: PGENERIC_MAPPING,
    ) -> BOOL;
    pub fn CreateRestrictedToken(
        ExistingTokenHandle: HANDLE,
        Flags: DWORD,
        DisableSidCount: DWORD,
        SidsToDisable: PSID_AND_ATTRIBUTES,
        DeletePrivilegeCount: DWORD,
        PrivilegesToDelete: PLUID_AND_ATTRIBUTES,
        RestrictedSidCount: DWORD,
        SidsToRestrict: PSID_AND_ATTRIBUTES,
        NewTokenHandle: PHANDLE,
    ) -> BOOL;
    pub fn CreateWellKnownSid(
        WellKnownSidType: WELL_KNOWN_SID_TYPE,
        DomainSid: PSID,
        pSid: PSID,
        cbSid: *mut DWORD,
    ) -> BOOL;
    pub fn EqualDomainSid(
        pSid1: PSID,
        pSid2: PSID,
        pfEqual: *mut BOOL,
    ) -> BOOL;
    pub fn DeleteAce(
        pAcl: PACL,
        dwAceIndex: DWORD,
    ) -> BOOL;
    pub fn DestroyPrivateObjectSecurity(
        ObjectDescriptor: *mut PSECURITY_DESCRIPTOR,
    ) -> BOOL;
    pub fn DuplicateToken(
        ExistingTokenHandle: HANDLE,
        ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
        DuplicateTokenHandle: PHANDLE,
    ) -> BOOL;
    pub fn DuplicateTokenEx(
        hExistingToken: HANDLE,
        dwDesiredAccess: DWORD,
        lpTokenAttributes: LPSECURITY_ATTRIBUTES,
        ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
        TokenType: TOKEN_TYPE,
        phNewToken: PHANDLE,
    ) -> BOOL;
    pub fn EqualPrefixSid(
        pSid1: PSID,
        pSid2: PSID,
    ) -> BOOL;
    pub fn EqualSid(
        pSid1: PSID,
        pSid2: PSID,
    ) -> BOOL;
    pub fn FindFirstFreeAce(
        pAcl: PACL,
        pAce: *mut LPVOID,
    ) -> BOOL;
    pub fn FreeSid(
        pSid: PSID,
    ) -> PVOID;
    pub fn GetAce(
        pAcl: PACL,
        dwAceIndex: DWORD,
        pAce: *mut LPVOID,
    ) -> BOOL;
    pub fn GetAclInformation(
        pAcl: PACL,
        pAclInformtion: LPVOID,
        nAclInformationLength: DWORD,
        dwAclInformationClass: ACL_INFORMATION_CLASS,
    ) -> BOOL;
    pub fn GetFileSecurityW(
        lpFileName: LPCWSTR,
        RequestedInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        nLength: DWORD,
        lpnLengthNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetKernelObjectSecurity(
        Handle: HANDLE,
        RequestedInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        nLength: DWORD,
        lpnLengthNeeded: LPDWORD,
    ) -> BOOL;
    pub fn GetLengthSid(
        pSid: PSID,
    ) -> DWORD;
    pub fn GetPrivateObjectSecurity(
        ObjectDescriptor: PSECURITY_DESCRIPTOR,
        SecurityInformation: SECURITY_INFORMATION,
        ResultantDescriptor: PSECURITY_DESCRIPTOR,
        DescriptorLength: DWORD,
        ReturnLength: PDWORD,
    ) -> BOOL;
    pub fn GetSecurityDescriptorControl(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        pControl: PSECURITY_DESCRIPTOR_CONTROL,
        lpdwRevision: LPDWORD,
    ) -> BOOL;
    pub fn GetSecurityDescriptorDacl(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        lpbDaclPresent: LPBOOL,
        pDacl: *mut PACL,
        lpbDaclDefaulted: LPBOOL,
    ) -> BOOL;
    pub fn GetSecurityDescriptorGroup(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        pGroup: *mut PSID,
        lpbGroupDefaulted: LPBOOL,
    ) -> BOOL;
    pub fn GetSecurityDescriptorLength(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> DWORD;
    pub fn GetSecurityDescriptorOwner(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        pOwner: *mut PSID,
        lpbOwnerDefaulted: LPBOOL,
    ) -> BOOL;
    pub fn GetSecurityDescriptorRMControl(
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        RMControl: PUCHAR,
    ) -> DWORD;
    pub fn GetSecurityDescriptorSacl(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        lpbSaclPresent: LPBOOL,
        pSacl: *mut PACL,
        lpbSaclDefaulted: LPBOOL,
    ) -> BOOL;
    pub fn GetSidIdentifierAuthority(
        pSid: PSID,
    ) -> PSID_IDENTIFIER_AUTHORITY;
    pub fn GetSidLengthRequired(
        nSubAuthorityCount: UCHAR,
    ) -> DWORD;
    pub fn GetSidSubAuthority(
        pSid: PSID,
        nSubAuthority: DWORD,
    ) -> PDWORD;
    pub fn GetSidSubAuthorityCount(
        pSid: PSID,
    ) -> PUCHAR;
    pub fn GetTokenInformation(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: LPVOID,
        TokenInformationLength: DWORD,
        ReturnLength: PDWORD,
    ) -> BOOL;
    pub fn GetWindowsAccountDomainSid(
        pSid: PSID,
        pDomainSid: PSID,
        cbDomainSid: *mut DWORD,
    ) -> BOOL;
    pub fn ImpersonateAnonymousToken(
        ThreadHandle: HANDLE,
    ) -> BOOL;
    pub fn ImpersonateLoggedOnUser(
        hToken: HANDLE,
    ) -> BOOL;
    pub fn ImpersonateSelf(
        ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    ) -> BOOL;
    pub fn InitializeAcl(
        pAcl: PACL,
        nAclLength: DWORD,
        dwAclRevision: DWORD,
    ) -> BOOL;
    pub fn InitializeSecurityDescriptor(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        dwRevision: DWORD,
    ) -> BOOL;
    pub fn InitializeSid(
        Sid: PSID,
        pIdentifierAuthority: PSID_IDENTIFIER_AUTHORITY,
        nSubAuthorityCount: BYTE,
    ) -> BOOL;
    pub fn IsTokenRestricted(
        TokenHandle: HANDLE,
    ) -> BOOL;
    pub fn IsValidAcl(
        pAcl: PACL,
    ) -> BOOL;
    pub fn IsValidSecurityDescriptor(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> BOOL;
    pub fn IsValidSid(
        pSid: PSID,
    ) -> BOOL;
    pub fn IsWellKnownSid(
        pSid: PSID,
        WellKnownSidType: WELL_KNOWN_SID_TYPE,
    ) -> BOOL;
    pub fn MakeAbsoluteSD(
        pSelfRelativeSD: PSECURITY_DESCRIPTOR,
        pAbsoluteSD: PSECURITY_DESCRIPTOR,
        lpdwAbsoluteSDSize: LPDWORD,
        pDacl: PACL,
        lpdwDaclSize: LPDWORD,
        pSacl: PACL,
        lpdwSaclSize: LPDWORD,
        pOwner: PSID,
        lpdwOwnerSize: LPDWORD,
        pPrimaryGroup: PSID,
        lpdwPrimaryGroupSize: LPDWORD,
    ) -> BOOL;
    pub fn MakeSelfRelativeSD(
        pAbsoluteSD: PSECURITY_DESCRIPTOR,
        pSelfRelativeSD: PSECURITY_DESCRIPTOR,
        lpdwBufferLength: LPDWORD,
    ) -> BOOL;
    pub fn MapGenericMask(
        AccessMask: PDWORD,
        GenericMapping: PGENERIC_MAPPING,
    );
    pub fn ObjectCloseAuditAlarmW(
        SubsystemName: LPCWSTR,
        HandleId: LPVOID,
        GenerateOnClose: BOOL,
    ) -> BOOL;
    pub fn ObjectDeleteAuditAlarmW(
        SubsystemName: LPCWSTR,
        HandleId: LPVOID,
        GenerateOnClose: BOOL,
    ) -> BOOL;
    pub fn ObjectOpenAuditAlarmW(
        SubsystemName: LPCWSTR,
        HandleId: LPVOID,
        ObjectTypeName: LPWSTR,
        ObjectName: LPWSTR,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ClientToken: HANDLE,
        DesiredAccess: DWORD,
        GrantedAccess: DWORD,
        Privileges: PPRIVILEGE_SET,
        ObjectCreation: BOOL,
        AccessGranted: BOOL,
        GenerateOnClose: LPBOOL,
    ) -> BOOL;
    pub fn ObjectPrivilegeAuditAlarmW(
        SubsystemName: LPCWSTR,
        HandleId: LPVOID,
        ClientToken: HANDLE,
        DesiredAccess: DWORD,
        Privileges: PPRIVILEGE_SET,
        AccessGranted: BOOL,
    ) -> BOOL;
    pub fn PrivilegeCheck(
        ClientToken: HANDLE,
        RequiredPrivileges: PPRIVILEGE_SET,
        pfResult: LPBOOL,
    ) -> BOOL;
    pub fn PrivilegedServiceAuditAlarmW(
        SubsystemName: LPCWSTR,
        ServiceName: LPCWSTR,
        ClientToken: HANDLE,
        Privileges: PPRIVILEGE_SET,
        AccessGranted: BOOL,
    ) -> BOOL;
    pub fn QuerySecurityAccessMask(
        SecurityInformation: SECURITY_INFORMATION,
        DesiredAccess: LPDWORD,
    );
    pub fn RevertToSelf() -> BOOL;
    pub fn SetAclInformation(
        pAcl: PACL,
        pAclInformation: LPVOID,
        nAclInformationLength: DWORD,
        dwAclInfomrationClass: ACL_INFORMATION_CLASS,
    ) -> BOOL;
    pub fn SetFileSecurityW(
        lpFileName: LPCWSTR,
        SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> BOOL;
    pub fn SetKernelObjectSecurity(
        Handle: HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> BOOL;
    pub fn SetPrivateObjectSecurity(
        SecurityInformation: SECURITY_INFORMATION,
        ModificationDescriptor: PSECURITY_DESCRIPTOR,
        ObjectsSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        GenericMapping: PGENERIC_MAPPING,
        Token: HANDLE,
    ) -> BOOL;
    pub fn SetPrivateObjectSecurityEx(
        SecurityInformation: SECURITY_INFORMATION,
        ModificationDescriptor: PSECURITY_DESCRIPTOR,
        ObjectsSecurityDescriptor: *mut PSECURITY_DESCRIPTOR,
        AutoInheritFlags: ULONG,
        GenericMapping: PGENERIC_MAPPING,
        Token: HANDLE,
    ) -> BOOL;
    pub fn SetSecurityAccessMask(
        SecurityInformation: SECURITY_INFORMATION,
        DesiredAccess: LPDWORD,
    );
    pub fn SetSecurityDescriptorControl(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        ControlBitsOfInterest: SECURITY_DESCRIPTOR_CONTROL,
        ControlBitsToSet: SECURITY_DESCRIPTOR_CONTROL,
    ) -> BOOL;
    pub fn SetSecurityDescriptorDacl(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        bDaclPresent: BOOL,
        pDacl: PACL,
        bDaclDefaulted: BOOL,
    ) -> BOOL;
    pub fn SetSecurityDescriptorGroup(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        pGroup: PSID,
        bGroupDefaulted: BOOL,
    ) -> BOOL;
    pub fn SetSecurityDescriptorOwner(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        pOwner: PSID,
        bOwnerDefaulted: BOOL,
    ) -> BOOL;
    pub fn SetSecurityDescriptorRMControl(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        RMControl: PUCHAR,
    ) -> DWORD;
    pub fn SetSecurityDescriptorSacl(
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        bSaclPresent: BOOL,
        pSacl: PACL,
        bSaclDefaulted: BOOL,
    ) -> BOOL;
    pub fn SetTokenInformation(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: LPVOID,
        TokenInformationLength: DWORD,
    ) -> BOOL;
    pub fn SetCachedSigningLevel(
        SourceFiles: PHANDLE,
        SourceFileCount: ULONG,
        Flags: ULONG,
        TargetFile: HANDLE,
    ) -> BOOL;
    pub fn GetCachedSigningLevel(
        File: HANDLE,
        Flags: PULONG,
        SigningLevel: PULONG,
        Thumbprint: PUCHAR,
        ThumbprintSize: PULONG,
        ThumbprintAlgorithm: PULONG,
    ) -> BOOL;
    pub fn CveEventWrite(
        CveId: PCWSTR,
        AdditionalDetails: PCWSTR,
    ) -> LONG;
    pub fn DeriveCapabilitySidsFromName(
        CapName: LPCWSTR,
        CapabilityGroupSids: *mut *mut PSID,
        CapabilityGroupSidCount: *mut DWORD,
        CapabilitySids: *mut *mut PSID,
        CapabilitySidCount: *mut DWORD,
    ) -> BOOL;
}

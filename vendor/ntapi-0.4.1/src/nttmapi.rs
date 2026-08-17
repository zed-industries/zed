use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::guiddef::LPGUID;
use winapi::shared::ktmtypes::{NOTIFICATION_MASK, PCRM_PROTOCOL_ID, PTRANSACTION_NOTIFICATION};
use winapi::shared::ntdef::{
    BOOLEAN, HANDLE, NTSTATUS, PHANDLE, PLARGE_INTEGER, POBJECT_ATTRIBUTES, PULONG,
    PUNICODE_STRING, PVOID, ULONG,
};
use winapi::um::winnt::{
    ACCESS_MASK, ENLISTMENT_INFORMATION_CLASS, KTMOBJECT_TYPE, PKTMOBJECT_CURSOR,
    RESOURCEMANAGER_INFORMATION_CLASS, TRANSACTIONMANAGER_INFORMATION_CLASS,
    TRANSACTION_INFORMATION_CLASS,
};
EXTERN!{extern "system" {
    fn NtCreateTransactionManager(
        TmHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        LogFileName: PUNICODE_STRING,
        CreateOptions: ULONG,
        CommitStrength: ULONG,
    ) -> NTSTATUS;
    fn NtOpenTransactionManager(
        TmHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        LogFileName: PUNICODE_STRING,
        TmIdentity: LPGUID,
        OpenOptions: ULONG,
    ) -> NTSTATUS;
    fn NtRenameTransactionManager(
        LogFileName: PUNICODE_STRING,
        ExistingTransactionManagerGuid: LPGUID,
    ) -> NTSTATUS;
    fn NtRollforwardTransactionManager(
        TransactionManagerHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtRecoverTransactionManager(
        TransactionManagerHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtQueryInformationTransactionManager(
        TransactionManagerHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: PVOID,
        TransactionManagerInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationTransactionManager(
        TmHandle: HANDLE,
        TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS,
        TransactionManagerInformation: PVOID,
        TransactionManagerInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtEnumerateTransactionObject(
        RootObjectHandle: HANDLE,
        QueryType: KTMOBJECT_TYPE,
        ObjectCursor: PKTMOBJECT_CURSOR,
        ObjectCursorLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtCreateTransaction(
        TransactionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Uow: LPGUID,
        TmHandle: HANDLE,
        CreateOptions: ULONG,
        IsolationLevel: ULONG,
        IsolationFlags: ULONG,
        Timeout: PLARGE_INTEGER,
        Description: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn NtOpenTransaction(
        TransactionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        Uow: LPGUID,
        TmHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtQueryInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: PVOID,
        TransactionInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationTransaction(
        TransactionHandle: HANDLE,
        TransactionInformationClass: TRANSACTION_INFORMATION_CLASS,
        TransactionInformation: PVOID,
        TransactionInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtCommitTransaction(
        TransactionHandle: HANDLE,
        Wait: BOOLEAN,
    ) -> NTSTATUS;
    fn NtRollbackTransaction(
        TransactionHandle: HANDLE,
        Wait: BOOLEAN,
    ) -> NTSTATUS;
    fn NtCreateEnlistment(
        EnlistmentHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ResourceManagerHandle: HANDLE,
        TransactionHandle: HANDLE,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        CreateOptions: ULONG,
        NotificationMask: NOTIFICATION_MASK,
        EnlistmentKey: PVOID,
    ) -> NTSTATUS;
    fn NtOpenEnlistment(
        EnlistmentHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ResourceManagerHandle: HANDLE,
        EnlistmentGuid: LPGUID,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtQueryInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: PVOID,
        EnlistmentInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS,
        EnlistmentInformation: PVOID,
        EnlistmentInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtRecoverEnlistment(
        EnlistmentHandle: HANDLE,
        EnlistmentKey: PVOID,
    ) -> NTSTATUS;
    fn NtPrePrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtPrepareEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtCommitEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtRollbackEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtPrePrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtPrepareComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtCommitComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtReadOnlyEnlistment(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtRollbackComplete(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtSinglePhaseReject(
        EnlistmentHandle: HANDLE,
        TmVirtualClock: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtCreateResourceManager(
        ResourceManagerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        TmHandle: HANDLE,
        RmGuid: LPGUID,
        ObjectAttributes: POBJECT_ATTRIBUTES,
        CreateOptions: ULONG,
        Description: PUNICODE_STRING,
    ) -> NTSTATUS;
    fn NtOpenResourceManager(
        ResourceManagerHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        TmHandle: HANDLE,
        ResourceManagerGuid: LPGUID,
        ObjectAttributes: POBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
    fn NtRecoverResourceManager(
        ResourceManagerHandle: HANDLE,
    ) -> NTSTATUS;
    fn NtGetNotificationResourceManager(
        ResourceManagerHandle: HANDLE,
        TransactionNotification: PTRANSACTION_NOTIFICATION,
        NotificationLength: ULONG,
        Timeout: PLARGE_INTEGER,
        ReturnLength: PULONG,
        Asynchronous: ULONG,
        AsynchronousContext: ULONG_PTR,
    ) -> NTSTATUS;
    fn NtQueryInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: PVOID,
        ResourceManagerInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
    fn NtSetInformationResourceManager(
        ResourceManagerHandle: HANDLE,
        ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS,
        ResourceManagerInformation: PVOID,
        ResourceManagerInformationLength: ULONG,
    ) -> NTSTATUS;
    fn NtRegisterProtocolAddressInformation(
        ResourceManager: HANDLE,
        ProtocolId: PCRM_PROTOCOL_ID,
        ProtocolInformationSize: ULONG,
        ProtocolInformation: PVOID,
        CreateOptions: ULONG,
    ) -> NTSTATUS;
    fn NtPropagationComplete(
        ResourceManagerHandle: HANDLE,
        RequestCookie: ULONG,
        BufferLength: ULONG,
        Buffer: PVOID,
    ) -> NTSTATUS;
    fn NtPropagationFailed(
        ResourceManagerHandle: HANDLE,
        RequestCookie: ULONG,
        PropStatus: NTSTATUS,
    ) -> NTSTATUS;
    fn NtFreezeTransactions(
        FreezeTimeout: PLARGE_INTEGER,
        ThawTimeout: PLARGE_INTEGER,
    ) -> NTSTATUS;
    fn NtThawTransactions() -> NTSTATUS;
}}

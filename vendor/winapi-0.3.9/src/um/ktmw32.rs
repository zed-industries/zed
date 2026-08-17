// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! FFI bindings to ktmw32.
use shared::guiddef::LPGUID;
use shared::minwindef::{BOOL, DWORD};
use um::minwinbase::LPSECURITY_ATTRIBUTES;
use um::winnt::{HANDLE, LPWSTR};
extern "system" {
    pub fn CreateTransaction(
        lpTransactionAttributes: LPSECURITY_ATTRIBUTES,
        UOW: LPGUID,
        CreateOptions: DWORD,
        IsolationLevel: DWORD,
        IsolationFlags: DWORD,
        Timeout: DWORD,
        Description: LPWSTR,
    ) -> HANDLE;
    // pub fn OpenTransaction();
    pub fn CommitTransaction(
        TransactionHandle: HANDLE,
    ) -> BOOL;
    // pub fn CommitTransactionAsync();
    pub fn RollbackTransaction(
        TransactionHandle: HANDLE,
    ) -> BOOL;
    // pub fn RollbackTransactionAsync();
    // pub fn GetTransactionId();
    // pub fn GetTransactionInformation();
    // pub fn SetTransactionInformation();
    // pub fn CreateTransactionManager();
    // pub fn OpenTransactionManager();
    // pub fn OpenTransactionManagerById();
    // pub fn RenameTransactionManager();
    // pub fn RollforwardTransactionManager();
    // pub fn RecoverTransactionManager();
    // pub fn GetCurrentClockTransactionManager();
    // pub fn GetTransactionManagerId();
    // pub fn CreateResourceManager();
    // pub fn OpenResourceManager();
    // pub fn RecoverResourceManager();
    // pub fn GetNotificationResourceManager();
    // pub fn GetNotificationResourceManagerAsync();
    // pub fn SetResourceManagerCompletionPort();
    // pub fn CreateEnlistment();
    // pub fn OpenEnlistment();
    // pub fn RecoverEnlistment();
    // pub fn GetEnlistmentRecoveryInformation();
    // pub fn GetEnlistmentId();
    // pub fn SetEnlistmentRecoveryInformation();
    // pub fn PrepareEnlistment();
    // pub fn PrePrepareEnlistment();
    // pub fn CommitEnlistment();
    // pub fn RollbackEnlistment();
    // pub fn PrePrepareComplete();
    // pub fn PrepareComplete();
    // pub fn ReadOnlyEnlistment();
    // pub fn CommitComplete();
    // pub fn RollbackComplete();
    // pub fn SinglePhaseReject();
}

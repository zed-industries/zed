use winapi::shared::ntdef::{BOOLEAN, NTSTATUS, PSTR, PVOID, ULONG};
use winapi::um::winnt::{PCONTEXT, PEXCEPTION_RECORD};
EXTERN!{extern "system" {
    fn RtlDispatchException(
        ExceptionRecord: PEXCEPTION_RECORD,
        ContextRecord: PCONTEXT,
    ) -> BOOLEAN;
    fn RtlRaiseStatus(
        Status: NTSTATUS,
    );
    fn RtlRaiseException(
        ExceptionRecord: PEXCEPTION_RECORD,
    );
    fn NtContinue(
        ContextRecord: PCONTEXT,
        TestAlert: BOOLEAN,
    ) -> NTSTATUS;
    fn NtRaiseException(
        ExceptionRecord: PEXCEPTION_RECORD,
        ContextRecord: PCONTEXT,
        FirstChance: BOOLEAN,
    ) -> NTSTATUS;
    fn RtlAssert(
        VoidFailedAssertion: PVOID,
        VoidFileName: PVOID,
        LineNumber: ULONG,
        MutableMessage: PSTR,
    );
}}

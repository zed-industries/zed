#[inline]
pub unsafe fn WSManCloseCommand(commandhandle: Option<WSMAN_COMMAND_HANDLE>, flags: u32, r#async: *const WSMAN_SHELL_ASYNC) {
    windows_core::link!("wsmsvc.dll" "system" fn WSManCloseCommand(commandhandle : WSMAN_COMMAND_HANDLE, flags : u32, r#async : *const WSMAN_SHELL_ASYNC));
    unsafe { WSManCloseCommand(commandhandle.unwrap_or(core::mem::zeroed()) as _, flags, r#async) }
}
#[inline]
pub unsafe fn WSManCloseOperation(operationhandle: Option<WSMAN_OPERATION_HANDLE>, flags: u32) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManCloseOperation(operationhandle : WSMAN_OPERATION_HANDLE, flags : u32) -> u32);
    unsafe { WSManCloseOperation(operationhandle.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn WSManCloseSession(session: Option<WSMAN_SESSION_HANDLE>, flags: u32) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManCloseSession(session : WSMAN_SESSION_HANDLE, flags : u32) -> u32);
    unsafe { WSManCloseSession(session.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn WSManCloseShell(shellhandle: Option<WSMAN_SHELL_HANDLE>, flags: u32, r#async: *const WSMAN_SHELL_ASYNC) {
    windows_core::link!("wsmsvc.dll" "system" fn WSManCloseShell(shellhandle : WSMAN_SHELL_HANDLE, flags : u32, r#async : *const WSMAN_SHELL_ASYNC));
    unsafe { WSManCloseShell(shellhandle.unwrap_or(core::mem::zeroed()) as _, flags, r#async) }
}
#[inline]
pub unsafe fn WSManConnectShell<P2, P3>(session: WSMAN_SESSION_HANDLE, flags: u32, resourceuri: P2, shellid: P3, options: Option<*const WSMAN_OPTION_SET>, connectxml: Option<*const WSMAN_DATA>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_SHELL_HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManConnectShell(session : WSMAN_SESSION_HANDLE, flags : u32, resourceuri : windows_core::PCWSTR, shellid : windows_core::PCWSTR, options : *const WSMAN_OPTION_SET, connectxml : *const WSMAN_DATA, r#async : *const WSMAN_SHELL_ASYNC, shell : *mut WSMAN_SHELL_HANDLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSManConnectShell(session, flags, resourceuri.param().abi(), shellid.param().abi(), options.unwrap_or(core::mem::zeroed()) as _, connectxml.unwrap_or(core::mem::zeroed()) as _, r#async, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn WSManConnectShellCommand<P2>(shell: WSMAN_SHELL_HANDLE, flags: u32, commandid: P2, options: Option<*const WSMAN_OPTION_SET>, connectxml: Option<*const WSMAN_DATA>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_COMMAND_HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManConnectShellCommand(shell : WSMAN_SHELL_HANDLE, flags : u32, commandid : windows_core::PCWSTR, options : *const WSMAN_OPTION_SET, connectxml : *const WSMAN_DATA, r#async : *const WSMAN_SHELL_ASYNC, command : *mut WSMAN_COMMAND_HANDLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSManConnectShellCommand(shell, flags, commandid.param().abi(), options.unwrap_or(core::mem::zeroed()) as _, connectxml.unwrap_or(core::mem::zeroed()) as _, r#async, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn WSManCreateSession<P1>(apihandle: WSMAN_API_HANDLE, connection: P1, flags: u32, serverauthenticationcredentials: Option<*const WSMAN_AUTHENTICATION_CREDENTIALS>, proxyinfo: Option<*const WSMAN_PROXY_INFO>, session: *mut WSMAN_SESSION_HANDLE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManCreateSession(apihandle : WSMAN_API_HANDLE, connection : windows_core::PCWSTR, flags : u32, serverauthenticationcredentials : *const WSMAN_AUTHENTICATION_CREDENTIALS, proxyinfo : *const WSMAN_PROXY_INFO, session : *mut WSMAN_SESSION_HANDLE) -> u32);
    unsafe { WSManCreateSession(apihandle, connection.param().abi(), flags, serverauthenticationcredentials.unwrap_or(core::mem::zeroed()) as _, proxyinfo.unwrap_or(core::mem::zeroed()) as _, session as _) }
}
#[inline]
pub unsafe fn WSManCreateShell<P2>(session: WSMAN_SESSION_HANDLE, flags: u32, resourceuri: P2, startupinfo: Option<*const WSMAN_SHELL_STARTUP_INFO_V11>, options: Option<*const WSMAN_OPTION_SET>, createxml: Option<*const WSMAN_DATA>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_SHELL_HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManCreateShell(session : WSMAN_SESSION_HANDLE, flags : u32, resourceuri : windows_core::PCWSTR, startupinfo : *const WSMAN_SHELL_STARTUP_INFO_V11, options : *const WSMAN_OPTION_SET, createxml : *const WSMAN_DATA, r#async : *const WSMAN_SHELL_ASYNC, shell : *mut WSMAN_SHELL_HANDLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSManCreateShell(session, flags, resourceuri.param().abi(), startupinfo.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, createxml.unwrap_or(core::mem::zeroed()) as _, r#async, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn WSManCreateShellEx<P2, P3>(session: WSMAN_SESSION_HANDLE, flags: u32, resourceuri: P2, shellid: P3, startupinfo: Option<*const WSMAN_SHELL_STARTUP_INFO_V11>, options: Option<*const WSMAN_OPTION_SET>, createxml: Option<*const WSMAN_DATA>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_SHELL_HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManCreateShellEx(session : WSMAN_SESSION_HANDLE, flags : u32, resourceuri : windows_core::PCWSTR, shellid : windows_core::PCWSTR, startupinfo : *const WSMAN_SHELL_STARTUP_INFO_V11, options : *const WSMAN_OPTION_SET, createxml : *const WSMAN_DATA, r#async : *const WSMAN_SHELL_ASYNC, shell : *mut WSMAN_SHELL_HANDLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSManCreateShellEx(session, flags, resourceuri.param().abi(), shellid.param().abi(), startupinfo.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, createxml.unwrap_or(core::mem::zeroed()) as _, r#async, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn WSManDeinitialize(apihandle: Option<WSMAN_API_HANDLE>, flags: u32) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManDeinitialize(apihandle : WSMAN_API_HANDLE, flags : u32) -> u32);
    unsafe { WSManDeinitialize(apihandle.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn WSManDisconnectShell(shell: WSMAN_SHELL_HANDLE, flags: u32, disconnectinfo: *const WSMAN_SHELL_DISCONNECT_INFO, r#async: *const WSMAN_SHELL_ASYNC) {
    windows_core::link!("wsmsvc.dll" "system" fn WSManDisconnectShell(shell : WSMAN_SHELL_HANDLE, flags : u32, disconnectinfo : *const WSMAN_SHELL_DISCONNECT_INFO, r#async : *const WSMAN_SHELL_ASYNC));
    unsafe { WSManDisconnectShell(shell, flags, disconnectinfo, r#async) }
}
#[inline]
pub unsafe fn WSManGetErrorMessage<P2>(apihandle: WSMAN_API_HANDLE, flags: Option<u32>, languagecode: P2, errorcode: u32, message: Option<&mut [u16]>, messagelengthused: *mut u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManGetErrorMessage(apihandle : WSMAN_API_HANDLE, flags : u32, languagecode : windows_core::PCWSTR, errorcode : u32, messagelength : u32, message : windows_core::PWSTR, messagelengthused : *mut u32) -> u32);
    unsafe { WSManGetErrorMessage(apihandle, flags.unwrap_or(core::mem::zeroed()) as _, languagecode.param().abi(), errorcode, message.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(message.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), messagelengthused as _) }
}
#[inline]
pub unsafe fn WSManGetSessionOptionAsDword(session: WSMAN_SESSION_HANDLE, option: WSManSessionOption, value: *mut u32) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManGetSessionOptionAsDword(session : WSMAN_SESSION_HANDLE, option : WSManSessionOption, value : *mut u32) -> u32);
    unsafe { WSManGetSessionOptionAsDword(session, option, value as _) }
}
#[inline]
pub unsafe fn WSManGetSessionOptionAsString(session: WSMAN_SESSION_HANDLE, option: WSManSessionOption, string: Option<&mut [u16]>, stringlengthused: *mut u32) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManGetSessionOptionAsString(session : WSMAN_SESSION_HANDLE, option : WSManSessionOption, stringlength : u32, string : windows_core::PWSTR, stringlengthused : *mut u32) -> u32);
    unsafe { WSManGetSessionOptionAsString(session, option, string.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(string.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), stringlengthused as _) }
}
#[inline]
pub unsafe fn WSManInitialize(flags: u32, apihandle: *mut WSMAN_API_HANDLE) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManInitialize(flags : u32, apihandle : *mut WSMAN_API_HANDLE) -> u32);
    unsafe { WSManInitialize(flags, apihandle as _) }
}
#[inline]
pub unsafe fn WSManPluginAuthzOperationComplete<P4>(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, userauthorizationcontext: Option<*const core::ffi::c_void>, errorcode: u32, extendederrorinformation: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginAuthzOperationComplete(senderdetails : *const WSMAN_SENDER_DETAILS, flags : u32, userauthorizationcontext : *const core::ffi::c_void, errorcode : u32, extendederrorinformation : windows_core::PCWSTR) -> u32);
    unsafe { WSManPluginAuthzOperationComplete(senderdetails, flags, userauthorizationcontext.unwrap_or(core::mem::zeroed()) as _, errorcode, extendederrorinformation.param().abi()) }
}
#[inline]
pub unsafe fn WSManPluginAuthzQueryQuotaComplete<P4>(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, quota: Option<*const WSMAN_AUTHZ_QUOTA>, errorcode: u32, extendederrorinformation: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginAuthzQueryQuotaComplete(senderdetails : *const WSMAN_SENDER_DETAILS, flags : u32, quota : *const WSMAN_AUTHZ_QUOTA, errorcode : u32, extendederrorinformation : windows_core::PCWSTR) -> u32);
    unsafe { WSManPluginAuthzQueryQuotaComplete(senderdetails, flags, quota.unwrap_or(core::mem::zeroed()) as _, errorcode, extendederrorinformation.param().abi()) }
}
#[inline]
pub unsafe fn WSManPluginAuthzUserComplete<P6>(senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, userauthorizationcontext: Option<*const core::ffi::c_void>, impersonationtoken: Option<super::super::Foundation::HANDLE>, userisadministrator: bool, errorcode: u32, extendederrorinformation: P6) -> u32
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginAuthzUserComplete(senderdetails : *const WSMAN_SENDER_DETAILS, flags : u32, userauthorizationcontext : *const core::ffi::c_void, impersonationtoken : super::super::Foundation:: HANDLE, userisadministrator : windows_core::BOOL, errorcode : u32, extendederrorinformation : windows_core::PCWSTR) -> u32);
    unsafe { WSManPluginAuthzUserComplete(senderdetails, flags, userauthorizationcontext.unwrap_or(core::mem::zeroed()) as _, impersonationtoken.unwrap_or(core::mem::zeroed()) as _, userisadministrator.into(), errorcode, extendederrorinformation.param().abi()) }
}
#[inline]
pub unsafe fn WSManPluginFreeRequestDetails(requestdetails: *const WSMAN_PLUGIN_REQUEST) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginFreeRequestDetails(requestdetails : *const WSMAN_PLUGIN_REQUEST) -> u32);
    unsafe { WSManPluginFreeRequestDetails(requestdetails) }
}
#[inline]
pub unsafe fn WSManPluginGetConfiguration(plugincontext: *const core::ffi::c_void, flags: u32, data: *mut WSMAN_DATA) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginGetConfiguration(plugincontext : *const core::ffi::c_void, flags : u32, data : *mut WSMAN_DATA) -> u32);
    unsafe { WSManPluginGetConfiguration(plugincontext, flags, data as _) }
}
#[inline]
pub unsafe fn WSManPluginGetOperationParameters(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, data: *mut WSMAN_DATA) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginGetOperationParameters(requestdetails : *const WSMAN_PLUGIN_REQUEST, flags : u32, data : *mut WSMAN_DATA) -> u32);
    unsafe { WSManPluginGetOperationParameters(requestdetails, flags, data as _) }
}
#[inline]
pub unsafe fn WSManPluginOperationComplete<P3>(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, errorcode: u32, extendedinformation: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginOperationComplete(requestdetails : *const WSMAN_PLUGIN_REQUEST, flags : u32, errorcode : u32, extendedinformation : windows_core::PCWSTR) -> u32);
    unsafe { WSManPluginOperationComplete(requestdetails, flags, errorcode, extendedinformation.param().abi()) }
}
#[inline]
pub unsafe fn WSManPluginReceiveResult<P2, P4>(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, stream: P2, streamresult: Option<*const WSMAN_DATA>, commandstate: P4, exitcode: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginReceiveResult(requestdetails : *const WSMAN_PLUGIN_REQUEST, flags : u32, stream : windows_core::PCWSTR, streamresult : *const WSMAN_DATA, commandstate : windows_core::PCWSTR, exitcode : u32) -> u32);
    unsafe { WSManPluginReceiveResult(requestdetails, flags, stream.param().abi(), streamresult.unwrap_or(core::mem::zeroed()) as _, commandstate.param().abi(), exitcode) }
}
#[inline]
pub unsafe fn WSManPluginReportCompletion(plugincontext: *const core::ffi::c_void, flags: u32) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginReportCompletion(plugincontext : *const core::ffi::c_void, flags : u32) -> u32);
    unsafe { WSManPluginReportCompletion(plugincontext, flags) }
}
#[inline]
pub unsafe fn WSManPluginReportContext(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, context: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManPluginReportContext(requestdetails : *const WSMAN_PLUGIN_REQUEST, flags : u32, context : *const core::ffi::c_void) -> u32);
    unsafe { WSManPluginReportContext(requestdetails, flags, context) }
}
#[inline]
pub unsafe fn WSManReceiveShellOutput(shell: WSMAN_SHELL_HANDLE, command: Option<WSMAN_COMMAND_HANDLE>, flags: u32, desiredstreamset: Option<*const WSMAN_STREAM_ID_SET>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_OPERATION_HANDLE {
    windows_core::link!("wsmsvc.dll" "system" fn WSManReceiveShellOutput(shell : WSMAN_SHELL_HANDLE, command : WSMAN_COMMAND_HANDLE, flags : u32, desiredstreamset : *const WSMAN_STREAM_ID_SET, r#async : *const WSMAN_SHELL_ASYNC, receiveoperation : *mut WSMAN_OPERATION_HANDLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSManReceiveShellOutput(shell, command.unwrap_or(core::mem::zeroed()) as _, flags, desiredstreamset.unwrap_or(core::mem::zeroed()) as _, r#async, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn WSManReconnectShell(shell: WSMAN_SHELL_HANDLE, flags: u32, r#async: *const WSMAN_SHELL_ASYNC) {
    windows_core::link!("wsmsvc.dll" "system" fn WSManReconnectShell(shell : WSMAN_SHELL_HANDLE, flags : u32, r#async : *const WSMAN_SHELL_ASYNC));
    unsafe { WSManReconnectShell(shell, flags, r#async) }
}
#[inline]
pub unsafe fn WSManReconnectShellCommand(commandhandle: WSMAN_COMMAND_HANDLE, flags: u32, r#async: *const WSMAN_SHELL_ASYNC) {
    windows_core::link!("wsmsvc.dll" "system" fn WSManReconnectShellCommand(commandhandle : WSMAN_COMMAND_HANDLE, flags : u32, r#async : *const WSMAN_SHELL_ASYNC));
    unsafe { WSManReconnectShellCommand(commandhandle, flags, r#async) }
}
#[inline]
pub unsafe fn WSManRunShellCommand<P2>(shell: WSMAN_SHELL_HANDLE, flags: u32, commandline: P2, args: Option<*const WSMAN_COMMAND_ARG_SET>, options: Option<*const WSMAN_OPTION_SET>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_COMMAND_HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManRunShellCommand(shell : WSMAN_SHELL_HANDLE, flags : u32, commandline : windows_core::PCWSTR, args : *const WSMAN_COMMAND_ARG_SET, options : *const WSMAN_OPTION_SET, r#async : *const WSMAN_SHELL_ASYNC, command : *mut WSMAN_COMMAND_HANDLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSManRunShellCommand(shell, flags, commandline.param().abi(), args.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, r#async, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn WSManRunShellCommandEx<P2, P3>(shell: WSMAN_SHELL_HANDLE, flags: u32, commandid: P2, commandline: P3, args: Option<*const WSMAN_COMMAND_ARG_SET>, options: Option<*const WSMAN_OPTION_SET>, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_COMMAND_HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManRunShellCommandEx(shell : WSMAN_SHELL_HANDLE, flags : u32, commandid : windows_core::PCWSTR, commandline : windows_core::PCWSTR, args : *const WSMAN_COMMAND_ARG_SET, options : *const WSMAN_OPTION_SET, r#async : *const WSMAN_SHELL_ASYNC, command : *mut WSMAN_COMMAND_HANDLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSManRunShellCommandEx(shell, flags, commandid.param().abi(), commandline.param().abi(), args.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, r#async, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn WSManSendShellInput<P3>(shell: WSMAN_SHELL_HANDLE, command: Option<WSMAN_COMMAND_HANDLE>, flags: u32, streamid: P3, streamdata: *const WSMAN_DATA, endofstream: bool, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_OPERATION_HANDLE
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManSendShellInput(shell : WSMAN_SHELL_HANDLE, command : WSMAN_COMMAND_HANDLE, flags : u32, streamid : windows_core::PCWSTR, streamdata : *const WSMAN_DATA, endofstream : windows_core::BOOL, r#async : *const WSMAN_SHELL_ASYNC, sendoperation : *mut WSMAN_OPERATION_HANDLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSManSendShellInput(shell, command.unwrap_or(core::mem::zeroed()) as _, flags, streamid.param().abi(), streamdata, endofstream.into(), r#async, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn WSManSetSessionOption(session: WSMAN_SESSION_HANDLE, option: WSManSessionOption, data: *const WSMAN_DATA) -> u32 {
    windows_core::link!("wsmsvc.dll" "system" fn WSManSetSessionOption(session : WSMAN_SESSION_HANDLE, option : WSManSessionOption, data : *const WSMAN_DATA) -> u32);
    unsafe { WSManSetSessionOption(session, option, data) }
}
#[inline]
pub unsafe fn WSManSignalShell<P3>(shell: WSMAN_SHELL_HANDLE, command: Option<WSMAN_COMMAND_HANDLE>, flags: u32, code: P3, r#async: *const WSMAN_SHELL_ASYNC) -> WSMAN_OPERATION_HANDLE
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsmsvc.dll" "system" fn WSManSignalShell(shell : WSMAN_SHELL_HANDLE, command : WSMAN_COMMAND_HANDLE, flags : u32, code : windows_core::PCWSTR, r#async : *const WSMAN_SHELL_ASYNC, signaloperation : *mut WSMAN_OPERATION_HANDLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSManSignalShell(shell, command.unwrap_or(core::mem::zeroed()) as _, flags, code.param().abi(), r#async, &mut result__);
        result__
    }
}
pub const ERROR_REDIRECT_LOCATION_INVALID: u32 = 2150859191u32;
pub const ERROR_REDIRECT_LOCATION_TOO_LONG: u32 = 2150859190u32;
pub const ERROR_SERVICE_CBT_HARDENING_INVALID: u32 = 2150859192u32;
pub const ERROR_WINRS_CLIENT_CLOSERECEIVEHANDLE_NULL_PARAM: u32 = 2150859058u32;
pub const ERROR_WINRS_CLIENT_CLOSESENDHANDLE_NULL_PARAM: u32 = 2150859061u32;
pub const ERROR_WINRS_CLIENT_CLOSESHELL_NULL_PARAM: u32 = 2150859050u32;
pub const ERROR_WINRS_CLIENT_CREATESHELL_NULL_PARAM: u32 = 2150859049u32;
pub const ERROR_WINRS_CLIENT_FREECREATESHELLRESULT_NULL_PARAM: u32 = 2150859051u32;
pub const ERROR_WINRS_CLIENT_FREEPULLRESULT_NULL_PARAM: u32 = 2150859056u32;
pub const ERROR_WINRS_CLIENT_FREERUNCOMMANDRESULT_NULL_PARAM: u32 = 2150859053u32;
pub const ERROR_WINRS_CLIENT_GET_NULL_PARAM: u32 = 2150859062u32;
pub const ERROR_WINRS_CLIENT_INVALID_FLAG: u32 = 2150859040u32;
pub const ERROR_WINRS_CLIENT_NULL_PARAM: u32 = 2150859041u32;
pub const ERROR_WINRS_CLIENT_PULL_NULL_PARAM: u32 = 2150859057u32;
pub const ERROR_WINRS_CLIENT_PUSH_NULL_PARAM: u32 = 2150859060u32;
pub const ERROR_WINRS_CLIENT_RECEIVE_NULL_PARAM: u32 = 2150859055u32;
pub const ERROR_WINRS_CLIENT_RUNCOMMAND_NULL_PARAM: u32 = 2150859052u32;
pub const ERROR_WINRS_CLIENT_SEND_NULL_PARAM: u32 = 2150859059u32;
pub const ERROR_WINRS_CLIENT_SIGNAL_NULL_PARAM: u32 = 2150859054u32;
pub const ERROR_WINRS_CODE_PAGE_NOT_SUPPORTED: u32 = 2150859072u32;
pub const ERROR_WINRS_CONNECT_RESPONSE_BAD_BODY: u32 = 2150859211u32;
pub const ERROR_WINRS_IDLETIMEOUT_OUTOFBOUNDS: u32 = 2150859250u32;
pub const ERROR_WINRS_RECEIVE_IN_PROGRESS: u32 = 2150859047u32;
pub const ERROR_WINRS_RECEIVE_NO_RESPONSE_DATA: u32 = 2150859048u32;
pub const ERROR_WINRS_SHELLCOMMAND_CLIENTID_NOT_VALID: u32 = 2150859220u32;
pub const ERROR_WINRS_SHELLCOMMAND_CLIENTID_RESOURCE_CONFLICT: u32 = 2150859222u32;
pub const ERROR_WINRS_SHELLCOMMAND_DISCONNECT_OPERATION_NOT_VALID: u32 = 2150859224u32;
pub const ERROR_WINRS_SHELLCOMMAND_RECONNECT_OPERATION_NOT_VALID: u32 = 2150859219u32;
pub const ERROR_WINRS_SHELL_CLIENTID_NOT_VALID: u32 = 2150859221u32;
pub const ERROR_WINRS_SHELL_CLIENTID_RESOURCE_CONFLICT: u32 = 2150859223u32;
pub const ERROR_WINRS_SHELL_CLIENTSESSIONID_MISMATCH: u32 = 2150859206u32;
pub const ERROR_WINRS_SHELL_CONNECTED_TO_DIFFERENT_CLIENT: u32 = 2150859213u32;
pub const ERROR_WINRS_SHELL_DISCONNECTED: u32 = 2150859204u32;
pub const ERROR_WINRS_SHELL_DISCONNECT_NOT_SUPPORTED: u32 = 2150859205u32;
pub const ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_GRACEFUL: u32 = 2150859214u32;
pub const ERROR_WINRS_SHELL_DISCONNECT_OPERATION_NOT_VALID: u32 = 2150859215u32;
pub const ERROR_WINRS_SHELL_RECONNECT_OPERATION_NOT_VALID: u32 = 2150859216u32;
pub const ERROR_WINRS_SHELL_URI_INVALID: u32 = 2150859099u32;
pub const ERROR_WSMAN_ACK_NOT_SUPPORTED: u32 = 2150858853u32;
pub const ERROR_WSMAN_ACTION_MISMATCH: u32 = 2150858801u32;
pub const ERROR_WSMAN_ACTION_NOT_SUPPORTED: u32 = 2150858771u32;
pub const ERROR_WSMAN_ADDOBJECT_MISSING_EPR: u32 = 2150859045u32;
pub const ERROR_WSMAN_ADDOBJECT_MISSING_OBJECT: u32 = 2150859044u32;
pub const ERROR_WSMAN_ALREADY_EXISTS: u32 = 2150858803u32;
pub const ERROR_WSMAN_AMBIGUOUS_SELECTORS: u32 = 2150858846u32;
pub const ERROR_WSMAN_AUTHENTICATION_INVALID_FLAG: u32 = 2150859077u32;
pub const ERROR_WSMAN_AUTHORIZATION_MODE_NOT_SUPPORTED: u32 = 2150858852u32;
pub const ERROR_WSMAN_BAD_METHOD: u32 = 2150858868u32;
pub const ERROR_WSMAN_BATCHSIZE_TOO_SMALL: u32 = 2150858919u32;
pub const ERROR_WSMAN_BATCH_COMPLETE: u32 = 2150858756u32;
pub const ERROR_WSMAN_BOOKMARKS_NOT_SUPPORTED: u32 = 2150858859u32;
pub const ERROR_WSMAN_BOOKMARK_EXPIRED: u32 = 2150858832u32;
pub const ERROR_WSMAN_CANNOT_CHANGE_KEYS: u32 = 2150858989u32;
pub const ERROR_WSMAN_CANNOT_DECRYPT: u32 = 2150859001u32;
pub const ERROR_WSMAN_CANNOT_PROCESS_FILTER: u32 = 2150859042u32;
pub const ERROR_WSMAN_CANNOT_USE_ALLOW_NEGOTIATE_IMPLICIT_CREDENTIALS_FOR_HTTP: u32 = 2150859184u32;
pub const ERROR_WSMAN_CANNOT_USE_CERTIFICATES_FOR_HTTP: u32 = 2150858968u32;
pub const ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_CREDSSP: u32 = 2150859187u32;
pub const ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_HTTP: u32 = 2150859185u32;
pub const ERROR_WSMAN_CANNOT_USE_PROXY_SETTINGS_FOR_KERBEROS: u32 = 2150859186u32;
pub const ERROR_WSMAN_CERTMAPPING_CONFIGLIMIT_EXCEEDED: u32 = 2150859091u32;
pub const ERROR_WSMAN_CERTMAPPING_CREDENTIAL_MANAGEMENT_FAILIED: u32 = 2150859262u32;
pub const ERROR_WSMAN_CERTMAPPING_INVALIDISSUERKEY: u32 = 2150859106u32;
pub const ERROR_WSMAN_CERTMAPPING_INVALIDSUBJECTKEY: u32 = 2150859105u32;
pub const ERROR_WSMAN_CERTMAPPING_INVALIDUSERCREDENTIALS: u32 = 2150859092u32;
pub const ERROR_WSMAN_CERTMAPPING_PASSWORDBLANK: u32 = 2150859115u32;
pub const ERROR_WSMAN_CERTMAPPING_PASSWORDTOOLONG: u32 = 2150859114u32;
pub const ERROR_WSMAN_CERTMAPPING_PASSWORDUSERTUPLE: u32 = 2150859116u32;
pub const ERROR_WSMAN_CERT_INVALID_USAGE: u32 = 2150858990u32;
pub const ERROR_WSMAN_CERT_INVALID_USAGE_CLIENT: u32 = 2150859093u32;
pub const ERROR_WSMAN_CERT_MISSING_AUTH_FLAG: u32 = 2150859094u32;
pub const ERROR_WSMAN_CERT_MULTIPLE_CREDENTIALS_FLAG: u32 = 2150859095u32;
pub const ERROR_WSMAN_CERT_NOT_FOUND: u32 = 2150858882u32;
pub const ERROR_WSMAN_CERT_THUMBPRINT_BLANK: u32 = 2150858983u32;
pub const ERROR_WSMAN_CERT_THUMBPRINT_NOT_BLANK: u32 = 2150858982u32;
pub const ERROR_WSMAN_CHARACTER_SET: u32 = 2150858828u32;
pub const ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS: u32 = 2150859171u32;
pub const ERROR_WSMAN_CLIENT_ALLOWFRESHCREDENTIALS_NTLMONLY: u32 = 2150859172u32;
pub const ERROR_WSMAN_CLIENT_BASIC_AUTHENTICATION_DISABLED: u32 = 2150858975u32;
pub const ERROR_WSMAN_CLIENT_BATCH_ITEMS_TOO_SMALL: u32 = 2150858946u32;
pub const ERROR_WSMAN_CLIENT_BLANK_ACTION_URI: u32 = 2150858948u32;
pub const ERROR_WSMAN_CLIENT_BLANK_INPUT_XML: u32 = 2150858945u32;
pub const ERROR_WSMAN_CLIENT_BLANK_URI: u32 = 2150858943u32;
pub const ERROR_WSMAN_CLIENT_CERTIFICATES_AUTHENTICATION_DISABLED: u32 = 2150858979u32;
pub const ERROR_WSMAN_CLIENT_CERT_NEEDED: u32 = 2150858932u32;
pub const ERROR_WSMAN_CLIENT_CERT_UNKNOWN_LOCATION: u32 = 2150858934u32;
pub const ERROR_WSMAN_CLIENT_CERT_UNKNOWN_TYPE: u32 = 2150858933u32;
pub const ERROR_WSMAN_CLIENT_CERT_UNNEEDED_CREDS: u32 = 2150858927u32;
pub const ERROR_WSMAN_CLIENT_CERT_UNNEEDED_USERNAME: u32 = 2150858929u32;
pub const ERROR_WSMAN_CLIENT_CLOSECOMMAND_NULL_PARAM: u32 = 2150859135u32;
pub const ERROR_WSMAN_CLIENT_CLOSESHELL_NULL_PARAM: u32 = 2150859134u32;
pub const ERROR_WSMAN_CLIENT_COMPRESSION_INVALID_OPTION: u32 = 2150858957u32;
pub const ERROR_WSMAN_CLIENT_CONNECTCOMMAND_NULL_PARAM: u32 = 2150859210u32;
pub const ERROR_WSMAN_CLIENT_CONNECTSHELL_NULL_PARAM: u32 = 2150859209u32;
pub const ERROR_WSMAN_CLIENT_CONSTRUCTERROR_NULL_PARAM: u32 = 2150858965u32;
pub const ERROR_WSMAN_CLIENT_CREATESESSION_NULL_PARAM: u32 = 2150858938u32;
pub const ERROR_WSMAN_CLIENT_CREATESHELL_NAME_INVALID: u32 = 2150859202u32;
pub const ERROR_WSMAN_CLIENT_CREATESHELL_NULL_PARAM: u32 = 2150859130u32;
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_FLAG_NEEDED: u32 = 2150858931u32;
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_DEFAULT_AUTHENTICATION: u32 = 2150859078u32;
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_FOR_PROXY_AUTHENTICATION: u32 = 2150859163u32;
pub const ERROR_WSMAN_CLIENT_CREDENTIALS_NEEDED: u32 = 2150858930u32;
pub const ERROR_WSMAN_CLIENT_CREDSSP_AUTHENTICATION_DISABLED: u32 = 2150859170u32;
pub const ERROR_WSMAN_CLIENT_DECODEOBJECT_NULL_PARAM: u32 = 2150858961u32;
pub const ERROR_WSMAN_CLIENT_DELIVERENDSUBSCRIPTION_NULL_PARAM: u32 = 2150858958u32;
pub const ERROR_WSMAN_CLIENT_DELIVEREVENTS_NULL_PARAM: u32 = 2150858959u32;
pub const ERROR_WSMAN_CLIENT_DIGEST_AUTHENTICATION_DISABLED: u32 = 2150858976u32;
pub const ERROR_WSMAN_CLIENT_DISABLE_LOOPBACK_WITH_EXPLICIT_CREDENTIALS: u32 = 2150859073u32;
pub const ERROR_WSMAN_CLIENT_DISCONNECTSHELL_NULL_PARAM: u32 = 2150859207u32;
pub const ERROR_WSMAN_CLIENT_ENCODEOBJECT_NULL_PARAM: u32 = 2150858962u32;
pub const ERROR_WSMAN_CLIENT_ENUMERATE_NULL_PARAM: u32 = 2150858939u32;
pub const ERROR_WSMAN_CLIENT_ENUMERATORADDEVENT_NULL_PARAM: u32 = 2150859043u32;
pub const ERROR_WSMAN_CLIENT_ENUMERATORADDOBJECT_NULL_PARAM: u32 = 2150858963u32;
pub const ERROR_WSMAN_CLIENT_ENUMERATORNEXTOBJECT_NULL_PARAM: u32 = 2150858964u32;
pub const ERROR_WSMAN_CLIENT_ENUM_RECEIVED_TOO_MANY_ITEMS: u32 = 2150859075u32;
pub const ERROR_WSMAN_CLIENT_GETBOOKMARK_NULL_PARAM: u32 = 2150858960u32;
pub const ERROR_WSMAN_CLIENT_GETERRORMESSAGE_NULL_PARAM: u32 = 2150859158u32;
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_INVALID_PARAM: u32 = 2150859167u32;
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_DWORD_NULL_PARAM: u32 = 2150859166u32;
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_INVALID_PARAM: u32 = 2150859129u32;
pub const ERROR_WSMAN_CLIENT_GETSESSIONOPTION_STRING_INVALID_PARAM: u32 = 2150859168u32;
pub const ERROR_WSMAN_CLIENT_INITIALIZE_NULL_PARAM: u32 = 2150859124u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CERT: u32 = 2150858935u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CERT_DNS_OR_UPN: u32 = 2150859080u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CLOSE_COMMAND_FLAG: u32 = 2150859133u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CLOSE_SHELL_FLAG: u32 = 2150859132u32;
pub const ERROR_WSMAN_CLIENT_INVALID_CREATE_SHELL_FLAG: u32 = 2150859131u32;
pub const ERROR_WSMAN_CLIENT_INVALID_DEINIT_APPLICATION_FLAG: u32 = 2150859126u32;
pub const ERROR_WSMAN_CLIENT_INVALID_DELIVERY_RETRY: u32 = 2150859108u32;
pub const ERROR_WSMAN_CLIENT_INVALID_DISABLE_LOOPBACK: u32 = 2150859074u32;
pub const ERROR_WSMAN_CLIENT_INVALID_DISCONNECT_SHELL_FLAG: u32 = 2150859226u32;
pub const ERROR_WSMAN_CLIENT_INVALID_FLAG: u32 = 2150858924u32;
pub const ERROR_WSMAN_CLIENT_INVALID_GETERRORMESSAGE_FLAG: u32 = 2150859160u32;
pub const ERROR_WSMAN_CLIENT_INVALID_INIT_APPLICATION_FLAG: u32 = 2150859125u32;
pub const ERROR_WSMAN_CLIENT_INVALID_LANGUAGE_CODE: u32 = 2150859159u32;
pub const ERROR_WSMAN_CLIENT_INVALID_LOCALE: u32 = 2150859156u32;
pub const ERROR_WSMAN_CLIENT_INVALID_RECEIVE_SHELL_FLAG: u32 = 2150859150u32;
pub const ERROR_WSMAN_CLIENT_INVALID_RESOURCE_LOCATOR: u32 = 2150858944u32;
pub const ERROR_WSMAN_CLIENT_INVALID_RUNCOMMAND_FLAG: u32 = 2150859137u32;
pub const ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_FLAG: u32 = 2150859145u32;
pub const ERROR_WSMAN_CLIENT_INVALID_SEND_SHELL_PARAMETER: u32 = 2150859146u32;
pub const ERROR_WSMAN_CLIENT_INVALID_SHELL_COMMAND_PAIR: u32 = 2150859227u32;
pub const ERROR_WSMAN_CLIENT_INVALID_SIGNAL_SHELL_FLAG: u32 = 2150859143u32;
pub const ERROR_WSMAN_CLIENT_INVALID_UI_LANGUAGE: u32 = 2150859157u32;
pub const ERROR_WSMAN_CLIENT_KERBEROS_AUTHENTICATION_DISABLED: u32 = 2150858978u32;
pub const ERROR_WSMAN_CLIENT_LOCAL_INVALID_CONNECTION_OPTIONS: u32 = 2150858937u32;
pub const ERROR_WSMAN_CLIENT_LOCAL_INVALID_CREDS: u32 = 2150858936u32;
pub const ERROR_WSMAN_CLIENT_MAX_CHARS_TOO_SMALL: u32 = 2150858947u32;
pub const ERROR_WSMAN_CLIENT_MISSING_EXPIRATION: u32 = 2150858953u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_AUTH_FLAGS: u32 = 2150858925u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_DELIVERY_MODES: u32 = 2150858950u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_ENUM_MODE_FLAGS: u32 = 2150859039u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_ENVELOPE_POLICIES: u32 = 2150858951u32;
pub const ERROR_WSMAN_CLIENT_MULTIPLE_PROXY_AUTH_FLAGS: u32 = 2150859188u32;
pub const ERROR_WSMAN_CLIENT_NEGOTIATE_AUTHENTICATION_DISABLED: u32 = 2150858977u32;
pub const ERROR_WSMAN_CLIENT_NO_HANDLE: u32 = 2150858942u32;
pub const ERROR_WSMAN_CLIENT_NO_SOURCES: u32 = 2150859111u32;
pub const ERROR_WSMAN_CLIENT_NULL_ISSUERS: u32 = 2150859110u32;
pub const ERROR_WSMAN_CLIENT_NULL_PUBLISHERS: u32 = 2150859109u32;
pub const ERROR_WSMAN_CLIENT_NULL_RESULT_PARAM: u32 = 2150858941u32;
pub const ERROR_WSMAN_CLIENT_PULL_INVALID_FLAGS: u32 = 2150858954u32;
pub const ERROR_WSMAN_CLIENT_PUSH_HOST_TOO_LONG: u32 = 2150858956u32;
pub const ERROR_WSMAN_CLIENT_PUSH_UNSUPPORTED_TRANSPORT: u32 = 2150858955u32;
pub const ERROR_WSMAN_CLIENT_RECEIVE_NULL_PARAM: u32 = 2150859148u32;
pub const ERROR_WSMAN_CLIENT_RECONNECTSHELLCOMMAND_NULL_PARAM: u32 = 2150859218u32;
pub const ERROR_WSMAN_CLIENT_RECONNECTSHELL_NULL_PARAM: u32 = 2150859208u32;
pub const ERROR_WSMAN_CLIENT_RUNCOMMAND_NOTCOMPLETED: u32 = 2150859138u32;
pub const ERROR_WSMAN_CLIENT_RUNCOMMAND_NULL_PARAM: u32 = 2150859136u32;
pub const ERROR_WSMAN_CLIENT_SEND_NULL_PARAM: u32 = 2150859144u32;
pub const ERROR_WSMAN_CLIENT_SESSION_UNUSABLE: u32 = 2150859258u32;
pub const ERROR_WSMAN_CLIENT_SETSESSIONOPTION_INVALID_PARAM: u32 = 2150859128u32;
pub const ERROR_WSMAN_CLIENT_SETSESSIONOPTION_NULL_PARAM: u32 = 2150859127u32;
pub const ERROR_WSMAN_CLIENT_SIGNAL_NULL_PARAM: u32 = 2150859142u32;
pub const ERROR_WSMAN_CLIENT_SPN_WRONG_AUTH: u32 = 2150858926u32;
pub const ERROR_WSMAN_CLIENT_SUBSCRIBE_NULL_PARAM: u32 = 2150858940u32;
pub const ERROR_WSMAN_CLIENT_UNENCRYPTED_DISABLED: u32 = 2150858974u32;
pub const ERROR_WSMAN_CLIENT_UNENCRYPTED_HTTP_ONLY: u32 = 2150858967u32;
pub const ERROR_WSMAN_CLIENT_UNKNOWN_EXPIRATION_TYPE: u32 = 2150858952u32;
pub const ERROR_WSMAN_CLIENT_USERNAME_AND_PASSWORD_NEEDED: u32 = 2150859079u32;
pub const ERROR_WSMAN_CLIENT_USERNAME_PASSWORD_NEEDED: u32 = 2150858928u32;
pub const ERROR_WSMAN_CLIENT_WORKGROUP_NO_KERBEROS: u32 = 2150859020u32;
pub const ERROR_WSMAN_CLIENT_ZERO_HEARTBEAT: u32 = 2150858949u32;
pub const ERROR_WSMAN_COMMAND_ALREADY_CLOSED: u32 = 2150859087u32;
pub const ERROR_WSMAN_COMMAND_TERMINATED: u32 = 2150859212u32;
pub const ERROR_WSMAN_CONCURRENCY: u32 = 2150858802u32;
pub const ERROR_WSMAN_CONFIG_CANNOT_CHANGE_CERTMAPPING_KEYS: u32 = 2150859122u32;
pub const ERROR_WSMAN_CONFIG_CANNOT_CHANGE_GPO_CONTROLLED_SETTING: u32 = 2150858890u32;
pub const ERROR_WSMAN_CONFIG_CANNOT_CHANGE_MUTUAL: u32 = 2150858885u32;
pub const ERROR_WSMAN_CONFIG_CANNOT_SHARE_SSL_CONFIG: u32 = 2150858984u32;
pub const ERROR_WSMAN_CONFIG_CERT_CN_DOES_NOT_MATCH_HOSTNAME: u32 = 2150858985u32;
pub const ERROR_WSMAN_CONFIG_CORRUPTED: u32 = 2150858757u32;
pub const ERROR_WSMAN_CONFIG_GROUP_POLICY_CHANGE_NOTIFICATION_SUBSCRIPTION_FAILED: u32 = 2150859217u32;
pub const ERROR_WSMAN_CONFIG_HOSTNAME_CHANGE_WITHOUT_CERT: u32 = 2150858986u32;
pub const ERROR_WSMAN_CONFIG_PORT_INVALID: u32 = 2150858972u32;
pub const ERROR_WSMAN_CONFIG_READONLY_PROPERTY: u32 = 2150859071u32;
pub const ERROR_WSMAN_CONFIG_SHELLURI_INVALID_OPERATION_ON_KEY: u32 = 2150859119u32;
pub const ERROR_WSMAN_CONFIG_SHELLURI_INVALID_PROCESSPATH: u32 = 2150859098u32;
pub const ERROR_WSMAN_CONFIG_SHELL_URI_CMDSHELLURI_NOTPERMITTED: u32 = 2150859097u32;
pub const ERROR_WSMAN_CONFIG_SHELL_URI_INVALID: u32 = 2150859096u32;
pub const ERROR_WSMAN_CONFIG_THUMBPRINT_SHOULD_BE_EMPTY: u32 = 2150858987u32;
pub const ERROR_WSMAN_CONNECTIONSTR_INVALID: u32 = 2150858969u32;
pub const ERROR_WSMAN_CONNECTOR_GET: u32 = 2150858873u32;
pub const ERROR_WSMAN_CREATESHELL_NULL_ENVIRONMENT_VARIABLE_NAME: u32 = 2150859081u32;
pub const ERROR_WSMAN_CREATESHELL_NULL_STREAMID: u32 = 2150859083u32;
pub const ERROR_WSMAN_CREATESHELL_RUNAS_FAILED: u32 = 2150859231u32;
pub const ERROR_WSMAN_CREATE_RESPONSE_NO_EPR: u32 = 2150858992u32;
pub const ERROR_WSMAN_CREDSSP_USERNAME_PASSWORD_NEEDED: u32 = 2150859169u32;
pub const ERROR_WSMAN_CREDS_PASSED_WITH_NO_AUTH_FLAG: u32 = 2150858923u32;
pub const ERROR_WSMAN_CUSTOMREMOTESHELL_DEPRECATED: u32 = 2150859196u32;
pub const ERROR_WSMAN_DEFAULTAUTH_IPADDRESS: u32 = 2150859195u32;
pub const ERROR_WSMAN_DELIVERY_REFUSED: u32 = 2150858804u32;
pub const ERROR_WSMAN_DELIVERY_RETRIES_NOT_SUPPORTED: u32 = 2150858857u32;
pub const ERROR_WSMAN_DELIVER_IN_PROGRESS: u32 = 2150858821u32;
pub const ERROR_WSMAN_DEPRECATED_CONFIG_SETTING: u32 = 2150859182u32;
pub const ERROR_WSMAN_DESERIALIZE_CLASS: u32 = 2150859244u32;
pub const ERROR_WSMAN_DESTINATION_INVALID: u32 = 2150859256u32;
pub const ERROR_WSMAN_DESTINATION_UNREACHABLE: u32 = 2150858770u32;
pub const ERROR_WSMAN_DIFFERENT_AUTHZ_TOKEN: u32 = 2150859177u32;
pub const ERROR_WSMAN_DIFFERENT_CIM_SELECTOR: u32 = 2150859067u32;
pub const ERROR_WSMAN_DUPLICATE_SELECTORS: u32 = 2150858847u32;
pub const ERROR_WSMAN_ENCODING_LIMIT: u32 = 2150858805u32;
pub const ERROR_WSMAN_ENCODING_TYPE: u32 = 2150859033u32;
pub const ERROR_WSMAN_ENDPOINT_UNAVAILABLE: u32 = 2150858772u32;
pub const ERROR_WSMAN_ENDPOINT_UNAVAILABLE_INVALID_VALUE: u32 = 2150859034u32;
pub const ERROR_WSMAN_ENUMERATE_CANNOT_PROCESS_FILTER: u32 = 2150858778u32;
pub const ERROR_WSMAN_ENUMERATE_FILTERING_NOT_SUPPORTED: u32 = 2150858776u32;
pub const ERROR_WSMAN_ENUMERATE_FILTER_DIALECT_REQUESTED_UNAVAILABLE: u32 = 2150858777u32;
pub const ERROR_WSMAN_ENUMERATE_INVALID_ENUMERATION_CONTEXT: u32 = 2150858779u32;
pub const ERROR_WSMAN_ENUMERATE_INVALID_EXPIRATION_TIME: u32 = 2150858774u32;
pub const ERROR_WSMAN_ENUMERATE_SHELLCOMAMNDS_FILTER_EXPECTED: u32 = 2150859200u32;
pub const ERROR_WSMAN_ENUMERATE_SHELLCOMMANDS_EPRS_NOTSUPPORTED: u32 = 2150859201u32;
pub const ERROR_WSMAN_ENUMERATE_TIMED_OUT: u32 = 2150858780u32;
pub const ERROR_WSMAN_ENUMERATE_UNABLE_TO_RENEW: u32 = 2150858781u32;
pub const ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TIME: u32 = 2150858775u32;
pub const ERROR_WSMAN_ENUMERATE_UNSUPPORTED_EXPIRATION_TYPE: u32 = 2150859036u32;
pub const ERROR_WSMAN_ENUMERATE_WMI_INVALID_KEY: u32 = 2150859016u32;
pub const ERROR_WSMAN_ENUMERATION_CLOSED: u32 = 2150858759u32;
pub const ERROR_WSMAN_ENUMERATION_INITIALIZING: u32 = 2150858872u32;
pub const ERROR_WSMAN_ENUMERATION_INVALID: u32 = 2150858884u32;
pub const ERROR_WSMAN_ENUMERATION_MODE_UNSUPPORTED: u32 = 2150858886u32;
pub const ERROR_WSMAN_ENVELOPE_TOO_LARGE: u32 = 2150858790u32;
pub const ERROR_WSMAN_EPR_NESTING_EXCEEDED: u32 = 2150858879u32;
pub const ERROR_WSMAN_EVENTING_CONCURRENT_CLIENT_RECEIVE: u32 = 2150858891u32;
pub const ERROR_WSMAN_EVENTING_DELIVERYFAILED_FROMSOURCE: u32 = 2150858908u32;
pub const ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_INVALID: u32 = 2150858920u32;
pub const ERROR_WSMAN_EVENTING_DELIVERY_MODE_REQUESTED_UNAVAILABLE: u32 = 2150858782u32;
pub const ERROR_WSMAN_EVENTING_FAST_SENDER: u32 = 2150858892u32;
pub const ERROR_WSMAN_EVENTING_FILTERING_NOT_SUPPORTED: u32 = 2150858785u32;
pub const ERROR_WSMAN_EVENTING_FILTERING_REQUESTED_UNAVAILABLE: u32 = 2150858786u32;
pub const ERROR_WSMAN_EVENTING_INCOMPATIBLE_BATCHPARAMS_AND_DELIVERYMODE: u32 = 2150858900u32;
pub const ERROR_WSMAN_EVENTING_INSECURE_PUSHSUBSCRIPTION_CONNECTION: u32 = 2150858893u32;
pub const ERROR_WSMAN_EVENTING_INVALID_ENCODING_IN_DELIVERY: u32 = 2150859255u32;
pub const ERROR_WSMAN_EVENTING_INVALID_ENDTO_ADDRESSS: u32 = 2150858902u32;
pub const ERROR_WSMAN_EVENTING_INVALID_EVENTSOURCE: u32 = 2150858894u32;
pub const ERROR_WSMAN_EVENTING_INVALID_EXPIRATION_TIME: u32 = 2150858783u32;
pub const ERROR_WSMAN_EVENTING_INVALID_HEARTBEAT: u32 = 2150858916u32;
pub const ERROR_WSMAN_EVENTING_INVALID_INCOMING_EVENT_PACKET_HEADER: u32 = 2150858903u32;
pub const ERROR_WSMAN_EVENTING_INVALID_LOCALE_IN_DELIVERY: u32 = 2150858915u32;
pub const ERROR_WSMAN_EVENTING_INVALID_MESSAGE: u32 = 2150858789u32;
pub const ERROR_WSMAN_EVENTING_INVALID_NOTIFYTO_ADDRESSS: u32 = 2150858914u32;
pub const ERROR_WSMAN_EVENTING_LOOPBACK_TESTFAILED: u32 = 2150858901u32;
pub const ERROR_WSMAN_EVENTING_MISSING_LOCALE_IN_DELIVERY: u32 = 2150859028u32;
pub const ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO: u32 = 2150858912u32;
pub const ERROR_WSMAN_EVENTING_MISSING_NOTIFYTO_ADDRESSS: u32 = 2150858913u32;
pub const ERROR_WSMAN_EVENTING_NOMATCHING_LISTENER: u32 = 2150858895u32;
pub const ERROR_WSMAN_EVENTING_NONDOMAINJOINED_COLLECTOR: u32 = 2150859070u32;
pub const ERROR_WSMAN_EVENTING_NONDOMAINJOINED_PUBLISHER: u32 = 2150859069u32;
pub const ERROR_WSMAN_EVENTING_PUSH_SUBSCRIPTION_NOACTIVATE_EVENTSOURCE: u32 = 2150859263u32;
pub const ERROR_WSMAN_EVENTING_SOURCE_UNABLE_TO_PROCESS: u32 = 2150858787u32;
pub const ERROR_WSMAN_EVENTING_SUBSCRIPTIONCLOSED_BYREMOTESERVICE: u32 = 2150858907u32;
pub const ERROR_WSMAN_EVENTING_SUBSCRIPTION_CANCELLED_BYSOURCE: u32 = 2150858910u32;
pub const ERROR_WSMAN_EVENTING_UNABLE_TO_RENEW: u32 = 2150858788u32;
pub const ERROR_WSMAN_EVENTING_UNSUPPORTED_EXPIRATION_TYPE: u32 = 2150858784u32;
pub const ERROR_WSMAN_EXPIRATION_TIME_NOT_SUPPORTED: u32 = 2150858856u32;
pub const ERROR_WSMAN_EXPLICIT_CREDENTIALS_REQUIRED: u32 = 2150858981u32;
pub const ERROR_WSMAN_FAILED_AUTHENTICATION: u32 = 2150858806u32;
pub const ERROR_WSMAN_FEATURE_DEPRECATED: u32 = 2150859197u32;
pub const ERROR_WSMAN_FILE_NOT_PRESENT: u32 = 2150859154u32;
pub const ERROR_WSMAN_FILTERING_REQUIRED: u32 = 2150858831u32;
pub const ERROR_WSMAN_FILTERING_REQUIRED_NOT_SUPPORTED: u32 = 2150858864u32;
pub const ERROR_WSMAN_FORMAT_MISMATCH_NOT_SUPPORTED: u32 = 2150858866u32;
pub const ERROR_WSMAN_FORMAT_SECURITY_TOKEN_NOT_SUPPORTED: u32 = 2150858867u32;
pub const ERROR_WSMAN_FRAGMENT_DIALECT_REQUESTED_UNAVAILABLE: u32 = 2150858896u32;
pub const ERROR_WSMAN_FRAGMENT_TRANSFER_NOT_SUPPORTED: u32 = 2150858871u32;
pub const ERROR_WSMAN_GETCLASS: u32 = 2150859245u32;
pub const ERROR_WSMAN_HEARTBEATS_NOT_SUPPORTED: u32 = 2150858858u32;
pub const ERROR_WSMAN_HTML_ERROR: u32 = 2150859123u32;
pub const ERROR_WSMAN_HTTP_CONTENT_TYPE_MISSMATCH_RESPONSE_DATA: u32 = 2150859000u32;
pub const ERROR_WSMAN_HTTP_INVALID_CONTENT_TYPE_IN_RESPONSE_DATA: u32 = 2150858999u32;
pub const ERROR_WSMAN_HTTP_NOT_FOUND_STATUS: u32 = 2150859027u32;
pub const ERROR_WSMAN_HTTP_NO_RESPONSE_DATA: u32 = 2150858997u32;
pub const ERROR_WSMAN_HTTP_REQUEST_TOO_LARGE_STATUS: u32 = 2150859025u32;
pub const ERROR_WSMAN_HTTP_SERVICE_UNAVAILABLE_STATUS: u32 = 2150859026u32;
pub const ERROR_WSMAN_HTTP_STATUS_BAD_REQUEST: u32 = 2150859121u32;
pub const ERROR_WSMAN_HTTP_STATUS_SERVER_ERROR: u32 = 2150859120u32;
pub const ERROR_WSMAN_IISCONFIGURATION_READ_FAILED: u32 = 2150859155u32;
pub const ERROR_WSMAN_INCOMPATIBLE_EPR: u32 = 2150858807u32;
pub const ERROR_WSMAN_INEXISTENT_MAC_ADDRESS: u32 = 2150858875u32;
pub const ERROR_WSMAN_INSECURE_ADDRESS_NOT_SUPPORTED: u32 = 2150858865u32;
pub const ERROR_WSMAN_INSUFFCIENT_SELECTORS: u32 = 2150858842u32;
pub const ERROR_WSMAN_INSUFFICIENT_METADATA_FOR_BASIC: u32 = 2150859251u32;
pub const ERROR_WSMAN_INVALID_ACTIONURI: u32 = 2150858753u32;
pub const ERROR_WSMAN_INVALID_BATCH_PARAMETER: u32 = 2150858799u32;
pub const ERROR_WSMAN_INVALID_BATCH_SETTINGS_PARAMETER: u32 = 2150859021u32;
pub const ERROR_WSMAN_INVALID_BOOKMARK: u32 = 2150858808u32;
pub const ERROR_WSMAN_INVALID_CHARACTERS_IN_RESPONSE: u32 = 2150859018u32;
pub const ERROR_WSMAN_INVALID_CONFIGSDDL_URL: u32 = 2150859199u32;
pub const ERROR_WSMAN_INVALID_CONNECTIONRETRY: u32 = 2150859103u32;
pub const ERROR_WSMAN_INVALID_FILEPATH: u32 = 2150859153u32;
pub const ERROR_WSMAN_INVALID_FILTER_XML: u32 = 2150859015u32;
pub const ERROR_WSMAN_INVALID_FRAGMENT_DIALECT: u32 = 2150858898u32;
pub const ERROR_WSMAN_INVALID_FRAGMENT_PATH: u32 = 2150858899u32;
pub const ERROR_WSMAN_INVALID_FRAGMENT_PATH_BLANK: u32 = 2150859017u32;
pub const ERROR_WSMAN_INVALID_HEADER: u32 = 2150859035u32;
pub const ERROR_WSMAN_INVALID_HOSTNAME_PATTERN: u32 = 2150858911u32;
pub const ERROR_WSMAN_INVALID_IPFILTER: u32 = 2150858988u32;
pub const ERROR_WSMAN_INVALID_KEY: u32 = 2150858820u32;
pub const ERROR_WSMAN_INVALID_LITERAL_URI: u32 = 2150859252u32;
pub const ERROR_WSMAN_INVALID_MESSAGE_INFORMATION_HEADER: u32 = 2150858767u32;
pub const ERROR_WSMAN_INVALID_OPTIONS: u32 = 2150858809u32;
pub const ERROR_WSMAN_INVALID_OPTIONSET: u32 = 2150859140u32;
pub const ERROR_WSMAN_INVALID_OPTION_NO_PROXY_SERVER: u32 = 2150859165u32;
pub const ERROR_WSMAN_INVALID_PARAMETER: u32 = 2150858810u32;
pub const ERROR_WSMAN_INVALID_PARAMETER_NAME: u32 = 2150858837u32;
pub const ERROR_WSMAN_INVALID_PROPOSED_ID: u32 = 2150858798u32;
pub const ERROR_WSMAN_INVALID_PROVIDER_RESPONSE: u32 = 2150859117u32;
pub const ERROR_WSMAN_INVALID_PUBLISHERS_TYPE: u32 = 2150859107u32;
pub const ERROR_WSMAN_INVALID_REDIRECT_ERROR: u32 = 2150859189u32;
pub const ERROR_WSMAN_INVALID_REPRESENTATION: u32 = 2150858773u32;
pub const ERROR_WSMAN_INVALID_RESOURCE_URI: u32 = 2150858811u32;
pub const ERROR_WSMAN_INVALID_RESUMPTION_CONTEXT: u32 = 2150858792u32;
pub const ERROR_WSMAN_INVALID_SECURITY_DESCRIPTOR: u32 = 2150859100u32;
pub const ERROR_WSMAN_INVALID_SELECTORS: u32 = 2150858813u32;
pub const ERROR_WSMAN_INVALID_SELECTOR_NAME: u32 = 2150859032u32;
pub const ERROR_WSMAN_INVALID_SELECTOR_VALUE: u32 = 2150858845u32;
pub const ERROR_WSMAN_INVALID_SOAP_BODY: u32 = 2150858791u32;
pub const ERROR_WSMAN_INVALID_SUBSCRIBE_OBJECT: u32 = 2150859112u32;
pub const ERROR_WSMAN_INVALID_SUBSCRIPTION_MANAGER: u32 = 2150859006u32;
pub const ERROR_WSMAN_INVALID_SYSTEM: u32 = 2150858812u32;
pub const ERROR_WSMAN_INVALID_TARGET_RESOURCEURI: u32 = 2150858849u32;
pub const ERROR_WSMAN_INVALID_TARGET_SELECTORS: u32 = 2150858848u32;
pub const ERROR_WSMAN_INVALID_TARGET_SYSTEM: u32 = 2150858850u32;
pub const ERROR_WSMAN_INVALID_TIMEOUT_HEADER: u32 = 2150858881u32;
pub const ERROR_WSMAN_INVALID_URI: u32 = 2150858754u32;
pub const ERROR_WSMAN_INVALID_URI_WMI_ENUM_WQL: u32 = 2150859003u32;
pub const ERROR_WSMAN_INVALID_URI_WMI_SINGLETON: u32 = 2150859002u32;
pub const ERROR_WSMAN_INVALID_USESSL_PARAM: u32 = 2150859198u32;
pub const ERROR_WSMAN_INVALID_XML: u32 = 2150858819u32;
pub const ERROR_WSMAN_INVALID_XML_FRAGMENT: u32 = 2150858841u32;
pub const ERROR_WSMAN_INVALID_XML_MISSING_VALUES: u32 = 2150858839u32;
pub const ERROR_WSMAN_INVALID_XML_NAMESPACE: u32 = 2150858840u32;
pub const ERROR_WSMAN_INVALID_XML_RUNAS_DISABLED: u32 = 2150859232u32;
pub const ERROR_WSMAN_INVALID_XML_VALUES: u32 = 2150858838u32;
pub const ERROR_WSMAN_KERBEROS_IPADDRESS: u32 = 2150859019u32;
pub const ERROR_WSMAN_LISTENER_ADDRESS_INVALID: u32 = 2150858889u32;
pub const ERROR_WSMAN_LOCALE_NOT_SUPPORTED: u32 = 2150858855u32;
pub const ERROR_WSMAN_MACHINE_OPTION_REQUIRED: u32 = 2150858917u32;
pub const ERROR_WSMAN_MAXENVELOPE_POLICY_NOT_SUPPORTED: u32 = 2150858863u32;
pub const ERROR_WSMAN_MAXENVELOPE_SIZE_NOT_SUPPORTED: u32 = 2150858862u32;
pub const ERROR_WSMAN_MAXITEMS_NOT_SUPPORTED: u32 = 2150858860u32;
pub const ERROR_WSMAN_MAXTIME_NOT_SUPPORTED: u32 = 2150858861u32;
pub const ERROR_WSMAN_MAX_ELEMENTS_NOT_SUPPORTED: u32 = 2150859037u32;
pub const ERROR_WSMAN_MAX_ENVELOPE_SIZE: u32 = 2150858823u32;
pub const ERROR_WSMAN_MAX_ENVELOPE_SIZE_EXCEEDED: u32 = 2150858824u32;
pub const ERROR_WSMAN_MESSAGE_INFORMATION_HEADER_REQUIRED: u32 = 2150858769u32;
pub const ERROR_WSMAN_METADATA_REDIRECT: u32 = 2150858814u32;
pub const ERROR_WSMAN_MIN_ENVELOPE_SIZE: u32 = 2150858878u32;
pub const ERROR_WSMAN_MISSING_CLASSNAME: u32 = 2150859254u32;
pub const ERROR_WSMAN_MISSING_FRAGMENT_PATH: u32 = 2150858897u32;
pub const ERROR_WSMAN_MULTIPLE_CREDENTIALS: u32 = 2150859076u32;
pub const ERROR_WSMAN_MUSTUNDERSTAND_ON_LOCALE_UNSUPPORTED: u32 = 2150858887u32;
pub const ERROR_WSMAN_MUTUAL_AUTH_FAILED: u32 = 2150859248u32;
pub const ERROR_WSMAN_NAME_NOT_RESOLVED: u32 = 2150859193u32;
pub const ERROR_WSMAN_NETWORK_TIMEDOUT: u32 = 2150859046u32;
pub const ERROR_WSMAN_NEW_DESERIALIZER: u32 = 2150859243u32;
pub const ERROR_WSMAN_NEW_SESSION: u32 = 2150859246u32;
pub const ERROR_WSMAN_NON_PULL_SUBSCRIPTION_NOT_SUPPORTED: u32 = 2150859007u32;
pub const ERROR_WSMAN_NO_ACK: u32 = 2150858800u32;
pub const ERROR_WSMAN_NO_CERTMAPPING_OPERATION_FOR_LOCAL_SESSION: u32 = 2150859090u32;
pub const ERROR_WSMAN_NO_COMMANDID: u32 = 2150859141u32;
pub const ERROR_WSMAN_NO_COMMAND_RESPONSE: u32 = 2150859139u32;
pub const ERROR_WSMAN_NO_DHCP_ADDRESSES: u32 = 2150858877u32;
pub const ERROR_WSMAN_NO_IDENTIFY_FOR_LOCAL_SESSION: u32 = 2150859004u32;
pub const ERROR_WSMAN_NO_PUSH_SUBSCRIPTION_FOR_LOCAL_SESSION: u32 = 2150859005u32;
pub const ERROR_WSMAN_NO_RECEIVE_RESPONSE: u32 = 2150859151u32;
pub const ERROR_WSMAN_NO_UNICAST_ADDRESSES: u32 = 2150858876u32;
pub const ERROR_WSMAN_NULL_KEY: u32 = 2150859247u32;
pub const ERROR_WSMAN_OBJECTONLY_INVALID: u32 = 2150859253u32;
pub const ERROR_WSMAN_OPERATION_TIMEDOUT: u32 = 2150858793u32;
pub const ERROR_WSMAN_OPERATION_TIMEOUT_NOT_SUPPORTED: u32 = 2150858854u32;
pub const ERROR_WSMAN_OPTIONS_INVALID_NAME: u32 = 2150858834u32;
pub const ERROR_WSMAN_OPTIONS_INVALID_VALUE: u32 = 2150858835u32;
pub const ERROR_WSMAN_OPTIONS_NOT_SUPPORTED: u32 = 2150858833u32;
pub const ERROR_WSMAN_OPTION_LIMIT: u32 = 2150858827u32;
pub const ERROR_WSMAN_PARAMETER_TYPE_MISMATCH: u32 = 2150858836u32;
pub const ERROR_WSMAN_PLUGIN_CONFIGURATION_CORRUPTED: u32 = 2150859152u32;
pub const ERROR_WSMAN_PLUGIN_FAILED: u32 = 2150858883u32;
pub const ERROR_WSMAN_POLICY_CANNOT_COMPLY: u32 = 2150859102u32;
pub const ERROR_WSMAN_POLICY_CORRUPTED: u32 = 2150858888u32;
pub const ERROR_WSMAN_POLICY_TOO_COMPLEX: u32 = 2150859101u32;
pub const ERROR_WSMAN_POLYMORPHISM_MODE_UNSUPPORTED: u32 = 2150859063u32;
pub const ERROR_WSMAN_PORT_INVALID: u32 = 2150858971u32;
pub const ERROR_WSMAN_PROVIDER_FAILURE: u32 = 2150858755u32;
pub const ERROR_WSMAN_PROVIDER_LOAD_FAILED: u32 = 2150858906u32;
pub const ERROR_WSMAN_PROVSYS_NOT_SUPPORTED: u32 = 2150858921u32;
pub const ERROR_WSMAN_PROXY_ACCESS_TYPE: u32 = 2150859164u32;
pub const ERROR_WSMAN_PROXY_AUTHENTICATION_INVALID_FLAG: u32 = 2150859162u32;
pub const ERROR_WSMAN_PUBLIC_FIREWALL_PROFILE_ACTIVE: u32 = 2150859113u32;
pub const ERROR_WSMAN_PULL_IN_PROGRESS: u32 = 2150858758u32;
pub const ERROR_WSMAN_PULL_PARAMS_NOT_SAME_AS_ENUM: u32 = 2150859181u32;
pub const ERROR_WSMAN_PUSHSUBSCRIPTION_INVALIDUSERACCOUNT: u32 = 2150859068u32;
pub const ERROR_WSMAN_PUSH_SUBSCRIPTION_CONFIG_INVALID: u32 = 2150858922u32;
pub const ERROR_WSMAN_QUICK_CONFIG_FAILED_CERT_REQUIRED: u32 = 2150859029u32;
pub const ERROR_WSMAN_QUICK_CONFIG_FIREWALL_EXCEPTIONS_DISALLOWED: u32 = 2150859030u32;
pub const ERROR_WSMAN_QUICK_CONFIG_LOCAL_POLICY_CHANGE_DISALLOWED: u32 = 2150859031u32;
pub const ERROR_WSMAN_QUOTA_LIMIT: u32 = 2150858815u32;
pub const ERROR_WSMAN_QUOTA_MAX_COMMANDS_PER_SHELL_PPQ: u32 = 2150859241u32;
pub const ERROR_WSMAN_QUOTA_MAX_OPERATIONS: u32 = 2150859174u32;
pub const ERROR_WSMAN_QUOTA_MAX_OPERATIONS_USER_PPQ: u32 = 2150859240u32;
pub const ERROR_WSMAN_QUOTA_MAX_PLUGINOPERATIONS_PPQ: u32 = 2150859239u32;
pub const ERROR_WSMAN_QUOTA_MAX_PLUGINSHELLS_PPQ: u32 = 2150859238u32;
pub const ERROR_WSMAN_QUOTA_MAX_SHELLS: u32 = 2150859173u32;
pub const ERROR_WSMAN_QUOTA_MAX_SHELLS_PPQ: u32 = 2150859236u32;
pub const ERROR_WSMAN_QUOTA_MAX_SHELLUSERS: u32 = 2150859179u32;
pub const ERROR_WSMAN_QUOTA_MAX_USERS_PPQ: u32 = 2150859237u32;
pub const ERROR_WSMAN_QUOTA_MIN_REQUIREMENT_NOT_AVAILABLE_PPQ: u32 = 2150859242u32;
pub const ERROR_WSMAN_QUOTA_SYSTEM: u32 = 2150859176u32;
pub const ERROR_WSMAN_QUOTA_USER: u32 = 2150859175u32;
pub const ERROR_WSMAN_REDIRECT_LOCATION_NOT_AVAILABLE: u32 = 2150859178u32;
pub const ERROR_WSMAN_REDIRECT_REQUESTED: u32 = 2150859161u32;
pub const ERROR_WSMAN_REMOTESHELLS_NOT_ALLOWED: u32 = 2150859180u32;
pub const ERROR_WSMAN_REMOTE_CIMPATH_NOT_SUPPORTED: u32 = 2150859009u32;
pub const ERROR_WSMAN_REMOTE_CONNECTION_NOT_ALLOWED: u32 = 2150859235u32;
pub const ERROR_WSMAN_RENAME_FAILURE: u32 = 2150858816u32;
pub const ERROR_WSMAN_REQUEST_INIT_ERROR: u32 = 2150858880u32;
pub const ERROR_WSMAN_REQUEST_NOT_SUPPORTED_AT_SERVICE: u32 = 2150859064u32;
pub const ERROR_WSMAN_RESOURCE_NOT_FOUND: u32 = 2150858752u32;
pub const ERROR_WSMAN_RESPONSE_INVALID_ENUMERATION_CONTEXT: u32 = 2150858993u32;
pub const ERROR_WSMAN_RESPONSE_INVALID_MESSAGE_INFORMATION_HEADER: u32 = 2150858995u32;
pub const ERROR_WSMAN_RESPONSE_INVALID_SOAP_FAULT: u32 = 2150858998u32;
pub const ERROR_WSMAN_RESPONSE_NO_RESULTS: u32 = 2150858991u32;
pub const ERROR_WSMAN_RESPONSE_NO_SOAP_HEADER_BODY: u32 = 2150858996u32;
pub const ERROR_WSMAN_RESPONSE_NO_XML_FRAGMENT_WRAPPER: u32 = 2150858994u32;
pub const ERROR_WSMAN_RESUMPTION_NOT_SUPPORTED: u32 = 2150858794u32;
pub const ERROR_WSMAN_RESUMPTION_TYPE_NOT_SUPPORTED: u32 = 2150858795u32;
pub const ERROR_WSMAN_RUNASUSER_MANAGEDACCOUNT_LOGON_FAILED: u32 = 2150859261u32;
pub const ERROR_WSMAN_RUNAS_INVALIDUSERCREDENTIALS: u32 = 2150859203u32;
pub const ERROR_WSMAN_RUNSHELLCOMMAND_NULL_ARGUMENT: u32 = 2150859086u32;
pub const ERROR_WSMAN_SCHEMA_VALIDATION_ERROR: u32 = 2150858817u32;
pub const ERROR_WSMAN_SECURITY_UNMAPPED: u32 = 2150858909u32;
pub const ERROR_WSMAN_SELECTOR_LIMIT: u32 = 2150858826u32;
pub const ERROR_WSMAN_SELECTOR_TYPEMISMATCH: u32 = 2150858844u32;
pub const ERROR_WSMAN_SEMANTICCALLBACK_TIMEDOUT: u32 = 2150859228u32;
pub const ERROR_WSMAN_SENDHEARBEAT_EMPTY_ENUMERATOR: u32 = 2150858973u32;
pub const ERROR_WSMAN_SENDSHELLINPUT_INVALID_STREAMID_INDEX: u32 = 2150859088u32;
pub const ERROR_WSMAN_SERVER_DESTINATION_LOCALHOST: u32 = 2150859022u32;
pub const ERROR_WSMAN_SERVER_ENVELOPE_LIMIT: u32 = 2150858825u32;
pub const ERROR_WSMAN_SERVER_NONPULLSUBSCRIBE_NULL_PARAM: u32 = 2150858966u32;
pub const ERROR_WSMAN_SERVER_NOT_TRUSTED: u32 = 2150858980u32;
pub const ERROR_WSMAN_SERVICE_REMOTE_ACCESS_DISABLED: u32 = 2150859229u32;
pub const ERROR_WSMAN_SERVICE_STREAM_DISCONNECTED: u32 = 2150859230u32;
pub const ERROR_WSMAN_SESSION_ALREADY_CLOSED: u32 = 2150858904u32;
pub const ERROR_WSMAN_SHELL_ALREADY_CLOSED: u32 = 2150859082u32;
pub const ERROR_WSMAN_SHELL_INVALID_COMMAND_HANDLE: u32 = 2150859085u32;
pub const ERROR_WSMAN_SHELL_INVALID_DESIRED_STREAMS: u32 = 2150859149u32;
pub const ERROR_WSMAN_SHELL_INVALID_INPUT_STREAM: u32 = 2150859147u32;
pub const ERROR_WSMAN_SHELL_INVALID_SHELL_HANDLE: u32 = 2150859084u32;
pub const ERROR_WSMAN_SHELL_NOT_INITIALIZED: u32 = 2150859118u32;
pub const ERROR_WSMAN_SHELL_SYNCHRONOUS_NOT_SUPPORTED: u32 = 2150859089u32;
pub const ERROR_WSMAN_SOAP_DATA_ENCODING_UNKNOWN: u32 = 2150858766u32;
pub const ERROR_WSMAN_SOAP_FAULT_MUST_UNDERSTAND: u32 = 2150858768u32;
pub const ERROR_WSMAN_SOAP_VERSION_MISMATCH: u32 = 2150858765u32;
pub const ERROR_WSMAN_SSL_CONNECTION_ABORTED: u32 = 2150859194u32;
pub const ERROR_WSMAN_SUBSCRIBE_WMI_INVALID_KEY: u32 = 2150859225u32;
pub const ERROR_WSMAN_SUBSCRIPTION_CLIENT_DID_NOT_CALL_WITHIN_HEARTBEAT: u32 = 2150858762u32;
pub const ERROR_WSMAN_SUBSCRIPTION_CLOSED: u32 = 2150858760u32;
pub const ERROR_WSMAN_SUBSCRIPTION_CLOSE_IN_PROGRESS: u32 = 2150858761u32;
pub const ERROR_WSMAN_SUBSCRIPTION_LISTENER_NOLONGERVALID: u32 = 2150858905u32;
pub const ERROR_WSMAN_SUBSCRIPTION_NO_HEARTBEAT: u32 = 2150858763u32;
pub const ERROR_WSMAN_SYSTEM_NOT_FOUND: u32 = 2150858822u32;
pub const ERROR_WSMAN_TARGET_ALREADY_EXISTS: u32 = 2150858851u32;
pub const ERROR_WSMAN_TRANSPORT_NOT_SUPPORTED: u32 = 2150858970u32;
pub const ERROR_WSMAN_UNEXPECTED_SELECTORS: u32 = 2150858843u32;
pub const ERROR_WSMAN_UNKNOWN_HTTP_STATUS_RETURNED: u32 = 2150859023u32;
pub const ERROR_WSMAN_UNREPORTABLE_SUCCESS: u32 = 2150858829u32;
pub const ERROR_WSMAN_UNSUPPORTED_ADDRESSING_MODE: u32 = 2150858870u32;
pub const ERROR_WSMAN_UNSUPPORTED_ENCODING: u32 = 2150858796u32;
pub const ERROR_WSMAN_UNSUPPORTED_FEATURE: u32 = 2150858818u32;
pub const ERROR_WSMAN_UNSUPPORTED_FEATURE_IDENTIFY: u32 = 2150859257u32;
pub const ERROR_WSMAN_UNSUPPORTED_FEATURE_OPTIONS: u32 = 2150858918u32;
pub const ERROR_WSMAN_UNSUPPORTED_HTTP_STATUS_REDIRECT: u32 = 2150859024u32;
pub const ERROR_WSMAN_UNSUPPORTED_MEDIA: u32 = 2150858869u32;
pub const ERROR_WSMAN_UNSUPPORTED_OCTETTYPE: u32 = 2150859249u32;
pub const ERROR_WSMAN_UNSUPPORTED_TIMEOUT: u32 = 2150858764u32;
pub const ERROR_WSMAN_UNSUPPORTED_TYPE: u32 = 2150859234u32;
pub const ERROR_WSMAN_URISECURITY_INVALIDURIKEY: u32 = 2150859104u32;
pub const ERROR_WSMAN_URI_LIMIT: u32 = 2150858797u32;
pub const ERROR_WSMAN_URI_NON_DMTF_CLASS: u32 = 2150859065u32;
pub const ERROR_WSMAN_URI_QUERY_STRING_SYNTAX_ERROR: u32 = 2150858874u32;
pub const ERROR_WSMAN_URI_SECURITY_URI: u32 = 2150859183u32;
pub const ERROR_WSMAN_URI_WRONG_DMTF_VERSION: u32 = 2150859066u32;
pub const ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED: u32 = 2150859259u32;
pub const ERROR_WSMAN_VIRTUALACCOUNT_NOTSUPPORTED_DOWNLEVEL: u32 = 2150859260u32;
pub const ERROR_WSMAN_WHITESPACE: u32 = 2150858830u32;
pub const ERROR_WSMAN_WMI_CANNOT_CONNECT_ACCESS_DENIED: u32 = 2150859014u32;
pub const ERROR_WSMAN_WMI_INVALID_VALUE: u32 = 2150859011u32;
pub const ERROR_WSMAN_WMI_MAX_NESTED: u32 = 2150859008u32;
pub const ERROR_WSMAN_WMI_PROVIDER_ACCESS_DENIED: u32 = 2150859013u32;
pub const ERROR_WSMAN_WMI_PROVIDER_INVALID_PARAMETER: u32 = 2150859038u32;
pub const ERROR_WSMAN_WMI_PROVIDER_NOT_CAPABLE: u32 = 2150859010u32;
pub const ERROR_WSMAN_WMI_SVC_ACCESS_DENIED: u32 = 2150859012u32;
pub const ERROR_WSMAN_WRONG_METADATA: u32 = 2150859233u32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSMan, IWSMan_Vtbl, 0x190d8637_5cd3_496d_ad24_69636bb5a3b5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSMan {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSMan, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWSMan {
    pub unsafe fn CreateSession<P2>(&self, connection: &windows_core::BSTR, flags: i32, connectionoptions: P2) -> windows_core::Result<super::Com::IDispatch>
    where
        P2: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSession)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(connection), flags, connectionoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateConnectionOptions(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateConnectionOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommandLine(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommandLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSMan_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateConnectionOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommandLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSMan_Impl: super::Com::IDispatch_Impl {
    fn CreateSession(&self, connection: &windows_core::BSTR, flags: i32, connectionoptions: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<super::Com::IDispatch>;
    fn CreateConnectionOptions(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn CommandLine(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Error(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSMan_Vtbl {
    pub const fn new<Identity: IWSMan_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSession<Identity: IWSMan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connection: *mut core::ffi::c_void, flags: i32, connectionoptions: *mut core::ffi::c_void, session: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSMan_Impl::CreateSession(this, core::mem::transmute(&connection), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&connectionoptions)) {
                    Ok(ok__) => {
                        session.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateConnectionOptions<Identity: IWSMan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSMan_Impl::CreateConnectionOptions(this) {
                    Ok(ok__) => {
                        connectionoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommandLine<Identity: IWSMan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSMan_Impl::CommandLine(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Error<Identity: IWSMan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSMan_Impl::Error(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateSession: CreateSession::<Identity, OFFSET>,
            CreateConnectionOptions: CreateConnectionOptions::<Identity, OFFSET>,
            CommandLine: CommandLine::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSMan as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSMan {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManConnectionOptions, IWSManConnectionOptions_Vtbl, 0xf704e861_9e52_464f_b786_da5eb2320fdd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManConnectionOptions {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManConnectionOptions, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptions {
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetUserName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn SetPassword(&self, password: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(password)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManConnectionOptions_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManConnectionOptions_Impl: super::Com::IDispatch_Impl {
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUserName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPassword(&self, password: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManConnectionOptions_Vtbl {
    pub const fn new<Identity: IWSManConnectionOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UserName<Identity: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptions_Impl::UserName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUserName<Identity: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManConnectionOptions_Impl::SetUserName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn SetPassword<Identity: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, password: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManConnectionOptions_Impl::SetPassword(this, core::mem::transmute(&password)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            UserName: UserName::<Identity, OFFSET>,
            SetUserName: SetUserName::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManConnectionOptions as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManConnectionOptions {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManConnectionOptionsEx, IWSManConnectionOptionsEx_Vtbl, 0xef43edf7_2a48_4d93_9526_8bd6ab6d4a6b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManConnectionOptionsEx {
    type Target = IWSManConnectionOptions;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManConnectionOptionsEx, windows_core::IUnknown, super::Com::IDispatch, IWSManConnectionOptions);
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptionsEx {
    pub unsafe fn CertificateThumbprint(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CertificateThumbprint)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCertificateThumbprint(&self, thumbprint: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCertificateThumbprint)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(thumbprint)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManConnectionOptionsEx_Vtbl {
    pub base__: IWSManConnectionOptions_Vtbl,
    pub CertificateThumbprint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCertificateThumbprint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManConnectionOptionsEx_Impl: IWSManConnectionOptions_Impl {
    fn CertificateThumbprint(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCertificateThumbprint(&self, thumbprint: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManConnectionOptionsEx_Vtbl {
    pub const fn new<Identity: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CertificateThumbprint<Identity: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thumbprint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx_Impl::CertificateThumbprint(this) {
                    Ok(ok__) => {
                        thumbprint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCertificateThumbprint<Identity: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thumbprint: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManConnectionOptionsEx_Impl::SetCertificateThumbprint(this, core::mem::transmute(&thumbprint)).into()
            }
        }
        Self {
            base__: IWSManConnectionOptions_Vtbl::new::<Identity, OFFSET>(),
            CertificateThumbprint: CertificateThumbprint::<Identity, OFFSET>,
            SetCertificateThumbprint: SetCertificateThumbprint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManConnectionOptionsEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWSManConnectionOptions as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManConnectionOptionsEx {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManConnectionOptionsEx2, IWSManConnectionOptionsEx2_Vtbl, 0xf500c9ec_24ee_48ab_b38d_fc9a164c658e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManConnectionOptionsEx2 {
    type Target = IWSManConnectionOptionsEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManConnectionOptionsEx2, windows_core::IUnknown, super::Com::IDispatch, IWSManConnectionOptions, IWSManConnectionOptionsEx);
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptionsEx2 {
    pub unsafe fn SetProxy(&self, accesstype: i32, authenticationmechanism: i32, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProxy)(windows_core::Interface::as_raw(self), accesstype, authenticationmechanism, core::mem::transmute_copy(username), core::mem::transmute_copy(password)).ok() }
    }
    pub unsafe fn ProxyIEConfig(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyIEConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyWinHttpConfig(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyWinHttpConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyAutoDetect(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyAutoDetect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyNoProxyServer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyNoProxyServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyAuthenticationUseNegotiate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyAuthenticationUseNegotiate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyAuthenticationUseBasic(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyAuthenticationUseBasic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProxyAuthenticationUseDigest(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyAuthenticationUseDigest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManConnectionOptionsEx2_Vtbl {
    pub base__: IWSManConnectionOptionsEx_Vtbl,
    pub SetProxy: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProxyIEConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyWinHttpConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyAutoDetect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyNoProxyServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyAuthenticationUseNegotiate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyAuthenticationUseBasic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProxyAuthenticationUseDigest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManConnectionOptionsEx2_Impl: IWSManConnectionOptionsEx_Impl {
    fn SetProxy(&self, accesstype: i32, authenticationmechanism: i32, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProxyIEConfig(&self) -> windows_core::Result<i32>;
    fn ProxyWinHttpConfig(&self) -> windows_core::Result<i32>;
    fn ProxyAutoDetect(&self) -> windows_core::Result<i32>;
    fn ProxyNoProxyServer(&self) -> windows_core::Result<i32>;
    fn ProxyAuthenticationUseNegotiate(&self) -> windows_core::Result<i32>;
    fn ProxyAuthenticationUseBasic(&self) -> windows_core::Result<i32>;
    fn ProxyAuthenticationUseDigest(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManConnectionOptionsEx2_Vtbl {
    pub const fn new<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProxy<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accesstype: i32, authenticationmechanism: i32, username: *mut core::ffi::c_void, password: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManConnectionOptionsEx2_Impl::SetProxy(this, core::mem::transmute_copy(&accesstype), core::mem::transmute_copy(&authenticationmechanism), core::mem::transmute(&username), core::mem::transmute(&password)).into()
            }
        }
        unsafe extern "system" fn ProxyIEConfig<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyIEConfig(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyWinHttpConfig<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyWinHttpConfig(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyAutoDetect<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyAutoDetect(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyNoProxyServer<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyNoProxyServer(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseNegotiate<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyAuthenticationUseNegotiate(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseBasic<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyAuthenticationUseBasic(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseDigest<Identity: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManConnectionOptionsEx2_Impl::ProxyAuthenticationUseDigest(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSManConnectionOptionsEx_Vtbl::new::<Identity, OFFSET>(),
            SetProxy: SetProxy::<Identity, OFFSET>,
            ProxyIEConfig: ProxyIEConfig::<Identity, OFFSET>,
            ProxyWinHttpConfig: ProxyWinHttpConfig::<Identity, OFFSET>,
            ProxyAutoDetect: ProxyAutoDetect::<Identity, OFFSET>,
            ProxyNoProxyServer: ProxyNoProxyServer::<Identity, OFFSET>,
            ProxyAuthenticationUseNegotiate: ProxyAuthenticationUseNegotiate::<Identity, OFFSET>,
            ProxyAuthenticationUseBasic: ProxyAuthenticationUseBasic::<Identity, OFFSET>,
            ProxyAuthenticationUseDigest: ProxyAuthenticationUseDigest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManConnectionOptionsEx2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWSManConnectionOptions as windows_core::Interface>::IID || iid == &<IWSManConnectionOptionsEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManConnectionOptionsEx2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManEnumerator, IWSManEnumerator_Vtbl, 0xf3457ca9_abb9_4fa5_b850_90e8ca300e7f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManEnumerator {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManEnumerator, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWSManEnumerator {
    pub unsafe fn ReadItem(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadItem)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AtEndOfStream(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AtEndOfStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEnumerator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ReadItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AtEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManEnumerator_Impl: super::Com::IDispatch_Impl {
    fn ReadItem(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AtEndOfStream(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Error(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManEnumerator_Vtbl {
    pub const fn new<Identity: IWSManEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadItem<Identity: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEnumerator_Impl::ReadItem(this) {
                    Ok(ok__) => {
                        resource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AtEndOfStream<Identity: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eos: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEnumerator_Impl::AtEndOfStream(this) {
                    Ok(ok__) => {
                        eos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Error<Identity: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEnumerator_Impl::Error(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ReadItem: ReadItem::<Identity, OFFSET>,
            AtEndOfStream: AtEndOfStream::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManEnumerator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManEnumerator {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManEx, IWSManEx_Vtbl, 0x2d53bdaa_798e_49e6_a1aa_74d01256f411);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManEx {
    type Target = IWSMan;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManEx, windows_core::IUnknown, super::Com::IDispatch, IWSMan);
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx {
    pub unsafe fn CreateResourceLocator(&self, strresourcelocator: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateResourceLocator)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strresourcelocator), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SessionFlagUTF8(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUTF8)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagCredUsernamePassword(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagCredUsernamePassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagSkipCACheck(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagSkipCACheck)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagSkipCNCheck(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagSkipCNCheck)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseDigest(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseDigest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseNegotiate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseNegotiate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseBasic(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseBasic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseKerberos(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseKerberos)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagNoEncryption(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagNoEncryption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagEnableSPNServerPort(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagEnableSPNServerPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseNoAuthentication(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseNoAuthentication)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagNonXmlText(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagNonXmlText)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagReturnEPR(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagReturnEPR)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagReturnObjectAndEPR(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagReturnObjectAndEPR)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetErrorMessage(&self, errornumber: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorMessage)(windows_core::Interface::as_raw(self), errornumber, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EnumerationFlagHierarchyDeep(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagHierarchyDeep)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagHierarchyShallow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagHierarchyShallow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagHierarchyDeepBasePropsOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagReturnObject(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagReturnObject)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEx_Vtbl {
    pub base__: IWSMan_Vtbl,
    pub CreateResourceLocator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionFlagUTF8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagCredUsernamePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagSkipCACheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagSkipCNCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseDigest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseNegotiate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseBasic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseKerberos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagNoEncryption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagEnableSPNServerPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseNoAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagNonXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagReturnEPR: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagReturnObjectAndEPR: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetErrorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerationFlagHierarchyDeep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagHierarchyShallow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagHierarchyDeepBasePropsOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagReturnObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManEx_Impl: IWSMan_Impl {
    fn CreateResourceLocator(&self, strresourcelocator: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn SessionFlagUTF8(&self) -> windows_core::Result<i32>;
    fn SessionFlagCredUsernamePassword(&self) -> windows_core::Result<i32>;
    fn SessionFlagSkipCACheck(&self) -> windows_core::Result<i32>;
    fn SessionFlagSkipCNCheck(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseDigest(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseNegotiate(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseBasic(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseKerberos(&self) -> windows_core::Result<i32>;
    fn SessionFlagNoEncryption(&self) -> windows_core::Result<i32>;
    fn SessionFlagEnableSPNServerPort(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseNoAuthentication(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagNonXmlText(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagReturnEPR(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagReturnObjectAndEPR(&self) -> windows_core::Result<i32>;
    fn GetErrorMessage(&self, errornumber: u32) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerationFlagHierarchyDeep(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagHierarchyShallow(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagReturnObject(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManEx_Vtbl {
    pub const fn new<Identity: IWSManEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateResourceLocator<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresourcelocator: *mut core::ffi::c_void, newresourcelocator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::CreateResourceLocator(this, core::mem::transmute(&strresourcelocator)) {
                    Ok(ok__) => {
                        newresourcelocator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUTF8<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUTF8(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagCredUsernamePassword<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagCredUsernamePassword(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagSkipCACheck<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagSkipCACheck(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagSkipCNCheck<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagSkipCNCheck(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseDigest<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseDigest(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseNegotiate<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseNegotiate(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseBasic<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseBasic(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseKerberos<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseKerberos(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagNoEncryption<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagNoEncryption(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagEnableSPNServerPort<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagEnableSPNServerPort(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseNoAuthentication<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::SessionFlagUseNoAuthentication(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagNonXmlText<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagNonXmlText(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnEPR<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagReturnEPR(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObjectAndEPR<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagReturnObjectAndEPR(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorMessage<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errornumber: u32, errormessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::GetErrorMessage(this, core::mem::transmute_copy(&errornumber)) {
                    Ok(ok__) => {
                        errormessage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeep<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagHierarchyDeep(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyShallow<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagHierarchyShallow(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeepBasePropsOnly<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagHierarchyDeepBasePropsOnly(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObject<Identity: IWSManEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx_Impl::EnumerationFlagReturnObject(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSMan_Vtbl::new::<Identity, OFFSET>(),
            CreateResourceLocator: CreateResourceLocator::<Identity, OFFSET>,
            SessionFlagUTF8: SessionFlagUTF8::<Identity, OFFSET>,
            SessionFlagCredUsernamePassword: SessionFlagCredUsernamePassword::<Identity, OFFSET>,
            SessionFlagSkipCACheck: SessionFlagSkipCACheck::<Identity, OFFSET>,
            SessionFlagSkipCNCheck: SessionFlagSkipCNCheck::<Identity, OFFSET>,
            SessionFlagUseDigest: SessionFlagUseDigest::<Identity, OFFSET>,
            SessionFlagUseNegotiate: SessionFlagUseNegotiate::<Identity, OFFSET>,
            SessionFlagUseBasic: SessionFlagUseBasic::<Identity, OFFSET>,
            SessionFlagUseKerberos: SessionFlagUseKerberos::<Identity, OFFSET>,
            SessionFlagNoEncryption: SessionFlagNoEncryption::<Identity, OFFSET>,
            SessionFlagEnableSPNServerPort: SessionFlagEnableSPNServerPort::<Identity, OFFSET>,
            SessionFlagUseNoAuthentication: SessionFlagUseNoAuthentication::<Identity, OFFSET>,
            EnumerationFlagNonXmlText: EnumerationFlagNonXmlText::<Identity, OFFSET>,
            EnumerationFlagReturnEPR: EnumerationFlagReturnEPR::<Identity, OFFSET>,
            EnumerationFlagReturnObjectAndEPR: EnumerationFlagReturnObjectAndEPR::<Identity, OFFSET>,
            GetErrorMessage: GetErrorMessage::<Identity, OFFSET>,
            EnumerationFlagHierarchyDeep: EnumerationFlagHierarchyDeep::<Identity, OFFSET>,
            EnumerationFlagHierarchyShallow: EnumerationFlagHierarchyShallow::<Identity, OFFSET>,
            EnumerationFlagHierarchyDeepBasePropsOnly: EnumerationFlagHierarchyDeepBasePropsOnly::<Identity, OFFSET>,
            EnumerationFlagReturnObject: EnumerationFlagReturnObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWSMan as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManEx {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManEx2, IWSManEx2_Vtbl, 0x1d1b5ae0_42d9_4021_8261_3987619512e9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManEx2 {
    type Target = IWSManEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManEx2, windows_core::IUnknown, super::Com::IDispatch, IWSMan, IWSManEx);
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx2 {
    pub unsafe fn SessionFlagUseClientCertificate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseClientCertificate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEx2_Vtbl {
    pub base__: IWSManEx_Vtbl,
    pub SessionFlagUseClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManEx2_Impl: IWSManEx_Impl {
    fn SessionFlagUseClientCertificate(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManEx2_Vtbl {
    pub const fn new<Identity: IWSManEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SessionFlagUseClientCertificate<Identity: IWSManEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx2_Impl::SessionFlagUseClientCertificate(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWSManEx_Vtbl::new::<Identity, OFFSET>(), SessionFlagUseClientCertificate: SessionFlagUseClientCertificate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManEx2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWSMan as windows_core::Interface>::IID || iid == &<IWSManEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManEx2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManEx3, IWSManEx3_Vtbl, 0x6400e966_011d_4eac_8474_049e0848afad);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManEx3 {
    type Target = IWSManEx2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManEx3, windows_core::IUnknown, super::Com::IDispatch, IWSMan, IWSManEx, IWSManEx2);
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx3 {
    pub unsafe fn SessionFlagUTF16(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUTF16)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseCredSsp(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseCredSsp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagAssociationInstance(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagAssociationInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerationFlagAssociatedInstance(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerationFlagAssociatedInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagSkipRevocationCheck(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagSkipRevocationCheck)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagAllowNegotiateImplicitCredentials(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagAllowNegotiateImplicitCredentials)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SessionFlagUseSsl(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionFlagUseSsl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManEx3_Vtbl {
    pub base__: IWSManEx2_Vtbl,
    pub SessionFlagUTF16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseCredSsp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagAssociationInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerationFlagAssociatedInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagSkipRevocationCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagAllowNegotiateImplicitCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SessionFlagUseSsl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManEx3_Impl: IWSManEx2_Impl {
    fn SessionFlagUTF16(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseCredSsp(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagAssociationInstance(&self) -> windows_core::Result<i32>;
    fn EnumerationFlagAssociatedInstance(&self) -> windows_core::Result<i32>;
    fn SessionFlagSkipRevocationCheck(&self) -> windows_core::Result<i32>;
    fn SessionFlagAllowNegotiateImplicitCredentials(&self) -> windows_core::Result<i32>;
    fn SessionFlagUseSsl(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManEx3_Vtbl {
    pub const fn new<Identity: IWSManEx3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SessionFlagUTF16<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagUTF16(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseCredSsp<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagUseCredSsp(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociationInstance<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::EnumerationFlagAssociationInstance(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociatedInstance<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::EnumerationFlagAssociatedInstance(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagSkipRevocationCheck<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagSkipRevocationCheck(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagAllowNegotiateImplicitCredentials<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagAllowNegotiateImplicitCredentials(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionFlagUseSsl<Identity: IWSManEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManEx3_Impl::SessionFlagUseSsl(this) {
                    Ok(ok__) => {
                        flags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSManEx2_Vtbl::new::<Identity, OFFSET>(),
            SessionFlagUTF16: SessionFlagUTF16::<Identity, OFFSET>,
            SessionFlagUseCredSsp: SessionFlagUseCredSsp::<Identity, OFFSET>,
            EnumerationFlagAssociationInstance: EnumerationFlagAssociationInstance::<Identity, OFFSET>,
            EnumerationFlagAssociatedInstance: EnumerationFlagAssociatedInstance::<Identity, OFFSET>,
            SessionFlagSkipRevocationCheck: SessionFlagSkipRevocationCheck::<Identity, OFFSET>,
            SessionFlagAllowNegotiateImplicitCredentials: SessionFlagAllowNegotiateImplicitCredentials::<Identity, OFFSET>,
            SessionFlagUseSsl: SessionFlagUseSsl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManEx3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWSMan as windows_core::Interface>::IID || iid == &<IWSManEx as windows_core::Interface>::IID || iid == &<IWSManEx2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManEx3 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManInternal, IWSManInternal_Vtbl, 0x04ae2b1d_9954_4d99_94a9_a961e72c3a13);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManInternal {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManInternal, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWSManInternal {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ConfigSDDL<P0>(&self, session: P0, resourceuri: &super::Variant::VARIANT, flags: i32) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConfigSDDL)(windows_core::Interface::as_raw(self), session.param().abi(), core::mem::transmute_copy(resourceuri), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManInternal_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ConfigSDDL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ConfigSDDL: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManInternal_Impl: super::Com::IDispatch_Impl {
    fn ConfigSDDL(&self, session: windows_core::Ref<super::Com::IDispatch>, resourceuri: &super::Variant::VARIANT, flags: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManInternal_Vtbl {
    pub const fn new<Identity: IWSManInternal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConfigSDDL<Identity: IWSManInternal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, session: *mut core::ffi::c_void, resourceuri: super::Variant::VARIANT, flags: i32, resource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManInternal_Impl::ConfigSDDL(this, core::mem::transmute_copy(&session), core::mem::transmute(&resourceuri), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        resource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ConfigSDDL: ConfigSDDL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManInternal as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManInternal {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManResourceLocator, IWSManResourceLocator_Vtbl, 0xa7a1ba28_de41_466a_ad0a_c4059ead7428);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManResourceLocator {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManResourceLocator, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWSManResourceLocator {
    pub unsafe fn SetResourceURI(&self, uri: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetResourceURI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(uri)).ok() }
    }
    pub unsafe fn ResourceURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddSelector(&self, resourceselname: &windows_core::BSTR, selvalue: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddSelector)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceselname), core::mem::transmute_copy(selvalue)).ok() }
    }
    pub unsafe fn ClearSelectors(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearSelectors)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn FragmentPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FragmentPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFragmentPath(&self, text: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFragmentPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)).ok() }
    }
    pub unsafe fn FragmentDialect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FragmentDialect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFragmentDialect(&self, text: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFragmentDialect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddOption(&self, optionname: &windows_core::BSTR, optionvalue: &super::Variant::VARIANT, mustcomply: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddOption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(optionname), core::mem::transmute_copy(optionvalue), mustcomply.into()).ok() }
    }
    pub unsafe fn SetMustUnderstandOptions(&self, mustunderstand: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMustUnderstandOptions)(windows_core::Interface::as_raw(self), mustunderstand.into()).ok() }
    }
    pub unsafe fn MustUnderstandOptions(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MustUnderstandOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ClearOptions(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearOptions)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManResourceLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetResourceURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddSelector: usize,
    pub ClearSelectors: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FragmentPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFragmentPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FragmentDialect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFragmentDialect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddOption: usize,
    pub SetMustUnderstandOptions: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub MustUnderstandOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ClearOptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManResourceLocator_Impl: super::Com::IDispatch_Impl {
    fn SetResourceURI(&self, uri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ResourceURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AddSelector(&self, resourceselname: &windows_core::BSTR, selvalue: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ClearSelectors(&self) -> windows_core::Result<()>;
    fn FragmentPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFragmentPath(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FragmentDialect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFragmentDialect(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddOption(&self, optionname: &windows_core::BSTR, optionvalue: &super::Variant::VARIANT, mustcomply: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetMustUnderstandOptions(&self, mustunderstand: windows_core::BOOL) -> windows_core::Result<()>;
    fn MustUnderstandOptions(&self) -> windows_core::Result<windows_core::BOOL>;
    fn ClearOptions(&self) -> windows_core::Result<()>;
    fn Error(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManResourceLocator_Vtbl {
    pub const fn new<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetResourceURI<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::SetResourceURI(this, core::mem::transmute(&uri)).into()
            }
        }
        unsafe extern "system" fn ResourceURI<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::ResourceURI(this) {
                    Ok(ok__) => {
                        uri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddSelector<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceselname: *mut core::ffi::c_void, selvalue: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::AddSelector(this, core::mem::transmute(&resourceselname), core::mem::transmute(&selvalue)).into()
            }
        }
        unsafe extern "system" fn ClearSelectors<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::ClearSelectors(this).into()
            }
        }
        unsafe extern "system" fn FragmentPath<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::FragmentPath(this) {
                    Ok(ok__) => {
                        text.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFragmentPath<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::SetFragmentPath(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn FragmentDialect<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::FragmentDialect(this) {
                    Ok(ok__) => {
                        text.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFragmentDialect<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::SetFragmentDialect(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn AddOption<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionname: *mut core::ffi::c_void, optionvalue: super::Variant::VARIANT, mustcomply: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::AddOption(this, core::mem::transmute(&optionname), core::mem::transmute(&optionvalue), core::mem::transmute_copy(&mustcomply)).into()
            }
        }
        unsafe extern "system" fn SetMustUnderstandOptions<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mustunderstand: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::SetMustUnderstandOptions(this, core::mem::transmute_copy(&mustunderstand)).into()
            }
        }
        unsafe extern "system" fn MustUnderstandOptions<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mustunderstand: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::MustUnderstandOptions(this) {
                    Ok(ok__) => {
                        mustunderstand.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClearOptions<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManResourceLocator_Impl::ClearOptions(this).into()
            }
        }
        unsafe extern "system" fn Error<Identity: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManResourceLocator_Impl::Error(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetResourceURI: SetResourceURI::<Identity, OFFSET>,
            ResourceURI: ResourceURI::<Identity, OFFSET>,
            AddSelector: AddSelector::<Identity, OFFSET>,
            ClearSelectors: ClearSelectors::<Identity, OFFSET>,
            FragmentPath: FragmentPath::<Identity, OFFSET>,
            SetFragmentPath: SetFragmentPath::<Identity, OFFSET>,
            FragmentDialect: FragmentDialect::<Identity, OFFSET>,
            SetFragmentDialect: SetFragmentDialect::<Identity, OFFSET>,
            AddOption: AddOption::<Identity, OFFSET>,
            SetMustUnderstandOptions: SetMustUnderstandOptions::<Identity, OFFSET>,
            MustUnderstandOptions: MustUnderstandOptions::<Identity, OFFSET>,
            ClearOptions: ClearOptions::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManResourceLocator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManResourceLocator {}
windows_core::imp::define_interface!(IWSManResourceLocatorInternal, IWSManResourceLocatorInternal_Vtbl, 0xeffaead7_7ec8_4716_b9be_f2e7e9fb4adb);
windows_core::imp::interface_hierarchy!(IWSManResourceLocatorInternal, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IWSManResourceLocatorInternal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IWSManResourceLocatorInternal_Impl: windows_core::IUnknownImpl {}
impl IWSManResourceLocatorInternal_Vtbl {
    pub const fn new<Identity: IWSManResourceLocatorInternal_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManResourceLocatorInternal as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSManResourceLocatorInternal {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWSManSession, IWSManSession_Vtbl, 0xfc84fc58_1286_40c4_9da0_c8ef6ec241e0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWSManSession {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWSManSession, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWSManSession {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get(&self, resourceuri: &super::Variant::VARIANT, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put(&self, resourceuri: &super::Variant::VARIANT, resource: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), core::mem::transmute_copy(resource), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Create(&self, resourceuri: &super::Variant::VARIANT, resource: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), core::mem::transmute_copy(resource), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Delete(&self, resourceuri: &super::Variant::VARIANT, flags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), flags).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Invoke(&self, actionuri: &windows_core::BSTR, resourceuri: &super::Variant::VARIANT, parameters: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(actionuri), core::mem::transmute_copy(resourceuri), core::mem::transmute_copy(parameters), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Enumerate(&self, resourceuri: &super::Variant::VARIANT, filter: &windows_core::BSTR, dialect: &windows_core::BSTR, flags: i32) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enumerate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(resourceuri), core::mem::transmute_copy(filter), core::mem::transmute_copy(dialect), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Identify(&self, flags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Identify)(windows_core::Interface::as_raw(self), flags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Error(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BatchItems(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BatchItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBatchItems(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBatchItems)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn Timeout(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTimeout(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimeout)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSManSession_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Get: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Put: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Create: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Delete: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Invoke: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Enumerate: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Enumerate: usize,
    pub Identify: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BatchItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBatchItems: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Timeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWSManSession_Impl: super::Com::IDispatch_Impl {
    fn Get(&self, resourceuri: &super::Variant::VARIANT, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Put(&self, resourceuri: &super::Variant::VARIANT, resource: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Create(&self, resourceuri: &super::Variant::VARIANT, resource: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Delete(&self, resourceuri: &super::Variant::VARIANT, flags: i32) -> windows_core::Result<()>;
    fn Invoke(&self, actionuri: &windows_core::BSTR, resourceuri: &super::Variant::VARIANT, parameters: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Enumerate(&self, resourceuri: &super::Variant::VARIANT, filter: &windows_core::BSTR, dialect: &windows_core::BSTR, flags: i32) -> windows_core::Result<super::Com::IDispatch>;
    fn Identify(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Error(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BatchItems(&self) -> windows_core::Result<i32>;
    fn SetBatchItems(&self, value: i32) -> windows_core::Result<()>;
    fn Timeout(&self) -> windows_core::Result<i32>;
    fn SetTimeout(&self, value: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWSManSession_Vtbl {
    pub const fn new<Identity: IWSManSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::Variant::VARIANT, flags: i32, resource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Get(this, core::mem::transmute(&resourceuri), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        resource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Put<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::Variant::VARIANT, resource: *mut core::ffi::c_void, flags: i32, resultresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Put(this, core::mem::transmute(&resourceuri), core::mem::transmute(&resource), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        resultresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::Variant::VARIANT, resource: *mut core::ffi::c_void, flags: i32, newuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Create(this, core::mem::transmute(&resourceuri), core::mem::transmute(&resource), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        newuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::Variant::VARIANT, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManSession_Impl::Delete(this, core::mem::transmute(&resourceuri), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Invoke<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actionuri: *mut core::ffi::c_void, resourceuri: super::Variant::VARIANT, parameters: *mut core::ffi::c_void, flags: i32, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Invoke(this, core::mem::transmute(&actionuri), core::mem::transmute(&resourceuri), core::mem::transmute(&parameters), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enumerate<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceuri: super::Variant::VARIANT, filter: *mut core::ffi::c_void, dialect: *mut core::ffi::c_void, flags: i32, resultset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Enumerate(this, core::mem::transmute(&resourceuri), core::mem::transmute(&filter), core::mem::transmute(&dialect), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        resultset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Identify<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Identify(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Error<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Error(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BatchItems<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::BatchItems(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBatchItems<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManSession_Impl::SetBatchItems(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Timeout<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSManSession_Impl::Timeout(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTimeout<Identity: IWSManSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSManSession_Impl::SetTimeout(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            Put: Put::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
            Enumerate: Enumerate::<Identity, OFFSET>,
            Identify: Identify::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
            BatchItems: BatchItems::<Identity, OFFSET>,
            SetBatchItems: SetBatchItems::<Identity, OFFSET>,
            Timeout: Timeout::<Identity, OFFSET>,
            SetTimeout: SetTimeout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSManSession as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWSManSession {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WSMAN_API_HANDLE(pub isize);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSMAN_AUTHENTICATION_CREDENTIALS {
    pub authenticationMechanism: u32,
    pub Anonymous: WSMAN_AUTHENTICATION_CREDENTIALS_0,
}
impl Default for WSMAN_AUTHENTICATION_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WSMAN_AUTHENTICATION_CREDENTIALS_0 {
    pub userAccount: WSMAN_USERNAME_PASSWORD_CREDS,
    pub certificateThumbprint: windows_core::PCWSTR,
}
impl Default for WSMAN_AUTHENTICATION_CREDENTIALS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_AUTHZ_QUOTA {
    pub maxAllowedConcurrentShells: u32,
    pub maxAllowedConcurrentOperations: u32,
    pub timeslotSize: u32,
    pub maxAllowedOperationsPerTimeslot: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_CERTIFICATE_DETAILS {
    pub subject: windows_core::PCWSTR,
    pub issuerName: windows_core::PCWSTR,
    pub issuerThumbprint: windows_core::PCWSTR,
    pub subjectName: windows_core::PCWSTR,
}
pub const WSMAN_CMDSHELL_OPTION_CODEPAGE: windows_core::PCWSTR = windows_core::w!("WINRS_CODEPAGE");
pub const WSMAN_CMDSHELL_OPTION_CONSOLEMODE_STDIN: windows_core::PCWSTR = windows_core::w!("WINRS_CONSOLEMODE_STDIN");
pub const WSMAN_CMDSHELL_OPTION_SKIP_CMD_SHELL: windows_core::PCWSTR = windows_core::w!("WINRS_SKIP_CMD_SHELL");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_COMMAND_ARG_SET {
    pub argsCount: u32,
    pub args: *const windows_core::PCWSTR,
}
impl Default for WSMAN_COMMAND_ARG_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WSMAN_COMMAND_HANDLE(pub isize);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSMAN_CONNECT_DATA {
    pub data: WSMAN_DATA,
}
impl Default for WSMAN_CONNECT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSMAN_CREATE_SHELL_DATA {
    pub data: WSMAN_DATA,
}
impl Default for WSMAN_CREATE_SHELL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSMAN_DATA {
    pub r#type: WSManDataType,
    pub Anonymous: WSMAN_DATA_0,
}
impl Default for WSMAN_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WSMAN_DATA_0 {
    pub text: WSMAN_DATA_TEXT,
    pub binaryData: WSMAN_DATA_BINARY,
    pub number: u32,
}
impl Default for WSMAN_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_DATA_BINARY {
    pub dataLength: u32,
    pub data: *mut u8,
}
impl Default for WSMAN_DATA_BINARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSMAN_DATA_NONE: WSManDataType = WSManDataType(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_DATA_TEXT {
    pub bufferLength: u32,
    pub buffer: windows_core::PCWSTR,
}
pub const WSMAN_DATA_TYPE_BINARY: WSManDataType = WSManDataType(2i32);
pub const WSMAN_DATA_TYPE_DWORD: WSManDataType = WSManDataType(4i32);
pub const WSMAN_DATA_TYPE_TEXT: WSManDataType = WSManDataType(1i32);
pub const WSMAN_DEFAULT_TIMEOUT_MS: u32 = 60000u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_ENVIRONMENT_VARIABLE {
    pub name: windows_core::PCWSTR,
    pub value: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_ENVIRONMENT_VARIABLE_SET {
    pub varsCount: u32,
    pub vars: *mut WSMAN_ENVIRONMENT_VARIABLE,
}
impl Default for WSMAN_ENVIRONMENT_VARIABLE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_ERROR {
    pub code: u32,
    pub errorDetail: windows_core::PCWSTR,
    pub language: windows_core::PCWSTR,
    pub machineName: windows_core::PCWSTR,
    pub pluginName: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_FILTER {
    pub filter: windows_core::PCWSTR,
    pub dialect: windows_core::PCWSTR,
}
pub const WSMAN_FLAG_AUTH_BASIC: WSManAuthenticationFlags = WSManAuthenticationFlags(8i32);
pub const WSMAN_FLAG_AUTH_CLIENT_CERTIFICATE: WSManAuthenticationFlags = WSManAuthenticationFlags(32i32);
pub const WSMAN_FLAG_AUTH_CREDSSP: WSManAuthenticationFlags = WSManAuthenticationFlags(128i32);
pub const WSMAN_FLAG_AUTH_DIGEST: WSManAuthenticationFlags = WSManAuthenticationFlags(2i32);
pub const WSMAN_FLAG_AUTH_KERBEROS: WSManAuthenticationFlags = WSManAuthenticationFlags(16i32);
pub const WSMAN_FLAG_AUTH_NEGOTIATE: WSManAuthenticationFlags = WSManAuthenticationFlags(4i32);
pub const WSMAN_FLAG_CALLBACK_END_OF_OPERATION: WSManCallbackFlags = WSManCallbackFlags(1i32);
pub const WSMAN_FLAG_CALLBACK_END_OF_STREAM: WSManCallbackFlags = WSManCallbackFlags(8i32);
pub const WSMAN_FLAG_CALLBACK_NETWORK_FAILURE_DETECTED: WSManCallbackFlags = WSManCallbackFlags(256i32);
pub const WSMAN_FLAG_CALLBACK_RECEIVE_DELAY_STREAM_REQUEST_PROCESSED: WSManCallbackFlags = WSManCallbackFlags(8192i32);
pub const WSMAN_FLAG_CALLBACK_RECONNECTED_AFTER_NETWORK_FAILURE: WSManCallbackFlags = WSManCallbackFlags(1024i32);
pub const WSMAN_FLAG_CALLBACK_RETRYING_AFTER_NETWORK_FAILURE: WSManCallbackFlags = WSManCallbackFlags(512i32);
pub const WSMAN_FLAG_CALLBACK_RETRY_ABORTED_DUE_TO_INTERNAL_ERROR: WSManCallbackFlags = WSManCallbackFlags(4096i32);
pub const WSMAN_FLAG_CALLBACK_SHELL_AUTODISCONNECTED: WSManCallbackFlags = WSManCallbackFlags(64i32);
pub const WSMAN_FLAG_CALLBACK_SHELL_AUTODISCONNECTING: WSManCallbackFlags = WSManCallbackFlags(2048i32);
pub const WSMAN_FLAG_CALLBACK_SHELL_SUPPORTS_DISCONNECT: WSManCallbackFlags = WSManCallbackFlags(32i32);
pub const WSMAN_FLAG_DEFAULT_AUTHENTICATION: WSManAuthenticationFlags = WSManAuthenticationFlags(0i32);
pub const WSMAN_FLAG_DELETE_SERVER_SESSION: WSManShellFlag = WSManShellFlag(2i32);
pub const WSMAN_FLAG_NO_AUTHENTICATION: WSManAuthenticationFlags = WSManAuthenticationFlags(1i32);
pub const WSMAN_FLAG_NO_COMPRESSION: WSManShellFlag = WSManShellFlag(1i32);
pub const WSMAN_FLAG_RECEIVE_DELAY_OUTPUT_STREAM: WSManShellFlag = WSManShellFlag(16i32);
pub const WSMAN_FLAG_RECEIVE_FLUSH: u32 = 2u32;
pub const WSMAN_FLAG_RECEIVE_RESULT_DATA_BOUNDARY: u32 = 4u32;
pub const WSMAN_FLAG_RECEIVE_RESULT_NO_MORE_DATA: u32 = 1u32;
pub const WSMAN_FLAG_REQUESTED_API_VERSION_1_0: u32 = 0u32;
pub const WSMAN_FLAG_REQUESTED_API_VERSION_1_1: u32 = 1u32;
pub const WSMAN_FLAG_SEND_NO_MORE_DATA: u32 = 1u32;
pub const WSMAN_FLAG_SERVER_BUFFERING_MODE_BLOCK: WSManShellFlag = WSManShellFlag(8i32);
pub const WSMAN_FLAG_SERVER_BUFFERING_MODE_DROP: WSManShellFlag = WSManShellFlag(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_FRAGMENT {
    pub path: windows_core::PCWSTR,
    pub dialect: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_KEY {
    pub key: windows_core::PCWSTR,
    pub value: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WSMAN_OPERATION_HANDLE(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_OPERATION_INFO {
    pub fragment: WSMAN_FRAGMENT,
    pub filter: WSMAN_FILTER,
    pub selectorSet: WSMAN_SELECTOR_SET,
    pub optionSet: WSMAN_OPTION_SET,
    pub reserved: *mut core::ffi::c_void,
    pub version: u32,
}
impl Default for WSMAN_OPERATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_OPERATION_INFOEX {
    pub fragment: WSMAN_FRAGMENT,
    pub filter: WSMAN_FILTER,
    pub selectorSet: WSMAN_SELECTOR_SET,
    pub optionSet: WSMAN_OPTION_SETEX,
    pub version: u32,
    pub uiLocale: windows_core::PCWSTR,
    pub dataLocale: windows_core::PCWSTR,
}
pub const WSMAN_OPERATION_INFOV1: u32 = 0u32;
pub const WSMAN_OPERATION_INFOV2: u32 = 2864434397u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_OPTION {
    pub name: windows_core::PCWSTR,
    pub value: windows_core::PCWSTR,
    pub mustComply: windows_core::BOOL,
}
pub const WSMAN_OPTION_ALLOW_NEGOTIATE_IMPLICIT_CREDENTIALS: WSManSessionOption = WSManSessionOption(32i32);
pub const WSMAN_OPTION_DEFAULT_OPERATION_TIMEOUTMS: WSManSessionOption = WSManSessionOption(1i32);
pub const WSMAN_OPTION_ENABLE_SPN_SERVER_PORT: WSManSessionOption = WSManSessionOption(22i32);
pub const WSMAN_OPTION_LOCALE: WSManSessionOption = WSManSessionOption(25i32);
pub const WSMAN_OPTION_MACHINE_ID: WSManSessionOption = WSManSessionOption(23i32);
pub const WSMAN_OPTION_MAX_ENVELOPE_SIZE_KB: WSManSessionOption = WSManSessionOption(28i32);
pub const WSMAN_OPTION_MAX_RETRY_TIME: WSManSessionOption = WSManSessionOption(11i32);
pub const WSMAN_OPTION_PROXY_AUTO_DETECT: WSManProxyAccessType = WSManProxyAccessType(4i32);
pub const WSMAN_OPTION_PROXY_IE_PROXY_CONFIG: WSManProxyAccessType = WSManProxyAccessType(1i32);
pub const WSMAN_OPTION_PROXY_NO_PROXY_SERVER: WSManProxyAccessType = WSManProxyAccessType(8i32);
pub const WSMAN_OPTION_PROXY_WINHTTP_PROXY_CONFIG: WSManProxyAccessType = WSManProxyAccessType(2i32);
pub const WSMAN_OPTION_REDIRECT_LOCATION: WSManSessionOption = WSManSessionOption(30i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_OPTION_SET {
    pub optionsCount: u32,
    pub options: *mut WSMAN_OPTION,
    pub optionsMustUnderstand: windows_core::BOOL,
}
impl Default for WSMAN_OPTION_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_OPTION_SETEX {
    pub optionsCount: u32,
    pub options: *mut WSMAN_OPTION,
    pub optionsMustUnderstand: windows_core::BOOL,
    pub optionTypes: *const windows_core::PCWSTR,
}
impl Default for WSMAN_OPTION_SETEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSMAN_OPTION_SHELL_MAX_DATA_SIZE_PER_MESSAGE_KB: WSManSessionOption = WSManSessionOption(29i32);
pub const WSMAN_OPTION_SKIP_CA_CHECK: WSManSessionOption = WSManSessionOption(18i32);
pub const WSMAN_OPTION_SKIP_CN_CHECK: WSManSessionOption = WSManSessionOption(19i32);
pub const WSMAN_OPTION_SKIP_REVOCATION_CHECK: WSManSessionOption = WSManSessionOption(31i32);
pub const WSMAN_OPTION_TIMEOUTMS_CLOSE_SHELL: WSManSessionOption = WSManSessionOption(17i32);
pub const WSMAN_OPTION_TIMEOUTMS_CREATE_SHELL: WSManSessionOption = WSManSessionOption(12i32);
pub const WSMAN_OPTION_TIMEOUTMS_RECEIVE_SHELL_OUTPUT: WSManSessionOption = WSManSessionOption(14i32);
pub const WSMAN_OPTION_TIMEOUTMS_RUN_SHELL_COMMAND: WSManSessionOption = WSManSessionOption(13i32);
pub const WSMAN_OPTION_TIMEOUTMS_SEND_SHELL_INPUT: WSManSessionOption = WSManSessionOption(15i32);
pub const WSMAN_OPTION_TIMEOUTMS_SIGNAL_SHELL: WSManSessionOption = WSManSessionOption(16i32);
pub const WSMAN_OPTION_UI_LANGUAGE: WSManSessionOption = WSManSessionOption(26i32);
pub const WSMAN_OPTION_UNENCRYPTED_MESSAGES: WSManSessionOption = WSManSessionOption(20i32);
pub const WSMAN_OPTION_USE_INTEARACTIVE_TOKEN: WSManSessionOption = WSManSessionOption(34i32);
pub const WSMAN_OPTION_USE_SSL: WSManSessionOption = WSManSessionOption(33i32);
pub const WSMAN_OPTION_UTF16: WSManSessionOption = WSManSessionOption(21i32);
pub type WSMAN_PLUGIN_AUTHORIZE_OPERATION = Option<unsafe extern "system" fn(plugincontext: *const core::ffi::c_void, senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32, operation: u32, action: windows_core::PCWSTR, resourceuri: windows_core::PCWSTR)>;
pub type WSMAN_PLUGIN_AUTHORIZE_QUERY_QUOTA = Option<unsafe extern "system" fn(plugincontext: *const core::ffi::c_void, senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32)>;
pub type WSMAN_PLUGIN_AUTHORIZE_RELEASE_CONTEXT = Option<unsafe extern "system" fn(userauthorizationcontext: *const core::ffi::c_void)>;
pub type WSMAN_PLUGIN_AUTHORIZE_USER = Option<unsafe extern "system" fn(plugincontext: *const core::ffi::c_void, senderdetails: *const WSMAN_SENDER_DETAILS, flags: u32)>;
pub type WSMAN_PLUGIN_COMMAND = Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const core::ffi::c_void, commandline: windows_core::PCWSTR, arguments: *const WSMAN_COMMAND_ARG_SET)>;
pub type WSMAN_PLUGIN_CONNECT = Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const core::ffi::c_void, commandcontext: *const core::ffi::c_void, inboundconnectinformation: *const WSMAN_DATA)>;
pub const WSMAN_PLUGIN_PARAMS_AUTORESTART: u32 = 3u32;
pub const WSMAN_PLUGIN_PARAMS_GET_REQUESTED_DATA_LOCALE: u32 = 6u32;
pub const WSMAN_PLUGIN_PARAMS_GET_REQUESTED_LOCALE: u32 = 5u32;
pub const WSMAN_PLUGIN_PARAMS_HOSTIDLETIMEOUTSECONDS: u32 = 4u32;
pub const WSMAN_PLUGIN_PARAMS_LARGEST_RESULT_SIZE: u32 = 4u32;
pub const WSMAN_PLUGIN_PARAMS_MAX_ENVELOPE_SIZE: u32 = 1u32;
pub const WSMAN_PLUGIN_PARAMS_NAME: u32 = 5u32;
pub const WSMAN_PLUGIN_PARAMS_REMAINING_RESULT_SIZE: u32 = 3u32;
pub const WSMAN_PLUGIN_PARAMS_RUNAS_USER: u32 = 2u32;
pub const WSMAN_PLUGIN_PARAMS_SHAREDHOST: u32 = 1u32;
pub const WSMAN_PLUGIN_PARAMS_TIMEOUT: u32 = 2u32;
pub type WSMAN_PLUGIN_RECEIVE = Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const core::ffi::c_void, commandcontext: *const core::ffi::c_void, streamset: *const WSMAN_STREAM_ID_SET)>;
pub type WSMAN_PLUGIN_RELEASE_COMMAND_CONTEXT = Option<unsafe extern "system" fn(shellcontext: *const core::ffi::c_void, commandcontext: *const core::ffi::c_void)>;
pub type WSMAN_PLUGIN_RELEASE_SHELL_CONTEXT = Option<unsafe extern "system" fn(shellcontext: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_PLUGIN_REQUEST {
    pub senderDetails: *mut WSMAN_SENDER_DETAILS,
    pub locale: windows_core::PCWSTR,
    pub resourceUri: windows_core::PCWSTR,
    pub operationInfo: *mut WSMAN_OPERATION_INFO,
    pub shutdownNotification: windows_core::BOOL,
    pub shutdownNotificationHandle: super::super::Foundation::HANDLE,
    pub dataLocale: windows_core::PCWSTR,
}
impl Default for WSMAN_PLUGIN_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSMAN_PLUGIN_SEND = Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const core::ffi::c_void, commandcontext: *const core::ffi::c_void, stream: windows_core::PCWSTR, inbounddata: *const WSMAN_DATA)>;
pub type WSMAN_PLUGIN_SHELL = Option<unsafe extern "system" fn(plugincontext: *const core::ffi::c_void, requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, startupinfo: *const WSMAN_SHELL_STARTUP_INFO_V11, inboundshellinformation: *const WSMAN_DATA)>;
pub type WSMAN_PLUGIN_SHUTDOWN = Option<unsafe extern "system" fn(plugincontext: *const core::ffi::c_void, flags: u32, reason: u32) -> u32>;
pub const WSMAN_PLUGIN_SHUTDOWN_IDLETIMEOUT_ELAPSED: u32 = 4u32;
pub const WSMAN_PLUGIN_SHUTDOWN_IISHOST: u32 = 3u32;
pub const WSMAN_PLUGIN_SHUTDOWN_SERVICE: u32 = 2u32;
pub const WSMAN_PLUGIN_SHUTDOWN_SYSTEM: u32 = 1u32;
pub type WSMAN_PLUGIN_SIGNAL = Option<unsafe extern "system" fn(requestdetails: *const WSMAN_PLUGIN_REQUEST, flags: u32, shellcontext: *const core::ffi::c_void, commandcontext: *const core::ffi::c_void, code: windows_core::PCWSTR)>;
pub type WSMAN_PLUGIN_STARTUP = Option<unsafe extern "system" fn(flags: u32, applicationidentification: windows_core::PCWSTR, extrainfo: windows_core::PCWSTR, plugincontext: *mut *mut core::ffi::c_void) -> u32>;
pub const WSMAN_PLUGIN_STARTUP_AUTORESTARTED_CRASH: u32 = 2u32;
pub const WSMAN_PLUGIN_STARTUP_AUTORESTARTED_REBOOT: u32 = 1u32;
pub const WSMAN_PLUGIN_STARTUP_REQUEST_RECEIVED: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSMAN_PROXY_INFO {
    pub accessType: u32,
    pub authenticationCredentials: WSMAN_AUTHENTICATION_CREDENTIALS,
}
impl Default for WSMAN_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSMAN_RECEIVE_DATA_RESULT {
    pub streamId: windows_core::PCWSTR,
    pub streamData: WSMAN_DATA,
    pub commandState: windows_core::PCWSTR,
    pub exitCode: u32,
}
impl Default for WSMAN_RECEIVE_DATA_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WSMAN_RESPONSE_DATA {
    pub receiveData: WSMAN_RECEIVE_DATA_RESULT,
    pub connectData: WSMAN_CONNECT_DATA,
    pub createData: WSMAN_CREATE_SHELL_DATA,
}
impl Default for WSMAN_RESPONSE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_SELECTOR_SET {
    pub numberKeys: u32,
    pub keys: *mut WSMAN_KEY,
}
impl Default for WSMAN_SELECTOR_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_SENDER_DETAILS {
    pub senderName: windows_core::PCWSTR,
    pub authenticationMechanism: windows_core::PCWSTR,
    pub certificateDetails: *mut WSMAN_CERTIFICATE_DETAILS,
    pub clientToken: super::super::Foundation::HANDLE,
    pub httpURL: windows_core::PCWSTR,
}
impl Default for WSMAN_SENDER_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WSMAN_SESSION_HANDLE(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WSMAN_SHELL_ASYNC {
    pub operationContext: *mut core::ffi::c_void,
    pub completionFunction: WSMAN_SHELL_COMPLETION_FUNCTION,
}
impl Default for WSMAN_SHELL_ASYNC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSMAN_SHELL_COMPLETION_FUNCTION = Option<unsafe extern "system" fn(operationcontext: *const core::ffi::c_void, flags: u32, error: *const WSMAN_ERROR, shell: WSMAN_SHELL_HANDLE, command: WSMAN_COMMAND_HANDLE, operationhandle: WSMAN_OPERATION_HANDLE, data: *const WSMAN_RESPONSE_DATA)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_SHELL_DISCONNECT_INFO {
    pub idleTimeoutMs: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WSMAN_SHELL_HANDLE(pub isize);
pub const WSMAN_SHELL_NS: windows_core::PCWSTR = windows_core::w!("http://schemas.microsoft.com/wbem/wsman/1/windows/shell");
pub const WSMAN_SHELL_OPTION_NOPROFILE: windows_core::PCWSTR = windows_core::w!("WINRS_NOPROFILE");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_SHELL_STARTUP_INFO_V10 {
    pub inputStreamSet: *mut WSMAN_STREAM_ID_SET,
    pub outputStreamSet: *mut WSMAN_STREAM_ID_SET,
    pub idleTimeoutMs: u32,
    pub workingDirectory: windows_core::PCWSTR,
    pub variableSet: *mut WSMAN_ENVIRONMENT_VARIABLE_SET,
}
impl Default for WSMAN_SHELL_STARTUP_INFO_V10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_SHELL_STARTUP_INFO_V11 {
    pub Base: WSMAN_SHELL_STARTUP_INFO_V10,
    pub name: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSMAN_STREAM_ID_SET {
    pub streamIDsCount: u32,
    pub streamIDs: *const windows_core::PCWSTR,
}
impl Default for WSMAN_STREAM_ID_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSMAN_STREAM_ID_STDERR: windows_core::PCWSTR = windows_core::w!("stderr");
pub const WSMAN_STREAM_ID_STDIN: windows_core::PCWSTR = windows_core::w!("stdin");
pub const WSMAN_STREAM_ID_STDOUT: windows_core::PCWSTR = windows_core::w!("stdout");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSMAN_USERNAME_PASSWORD_CREDS {
    pub username: windows_core::PCWSTR,
    pub password: windows_core::PCWSTR,
}
pub const WSMan: windows_core::GUID = windows_core::GUID::from_u128(0xbced617b_ec03_420b_8508_977dc7a686bd);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManAuthenticationFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManCallbackFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManDataType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManEnumFlags(pub i32);
pub const WSManFlagAllowNegotiateImplicitCredentials: WSManSessionFlags = WSManSessionFlags(67108864i32);
pub const WSManFlagAssociatedInstance: WSManEnumFlags = WSManEnumFlags(0i32);
pub const WSManFlagAssociationInstance: WSManEnumFlags = WSManEnumFlags(128i32);
pub const WSManFlagCredUsernamePassword: WSManSessionFlags = WSManSessionFlags(4096i32);
pub const WSManFlagEnableSPNServerPort: WSManSessionFlags = WSManSessionFlags(4194304i32);
pub const WSManFlagHierarchyDeep: WSManEnumFlags = WSManEnumFlags(0i32);
pub const WSManFlagHierarchyDeepBasePropsOnly: WSManEnumFlags = WSManEnumFlags(64i32);
pub const WSManFlagHierarchyShallow: WSManEnumFlags = WSManEnumFlags(32i32);
pub const WSManFlagNoEncryption: WSManSessionFlags = WSManSessionFlags(1048576i32);
pub const WSManFlagNonXmlText: WSManEnumFlags = WSManEnumFlags(1i32);
pub const WSManFlagProxyAuthenticationUseBasic: WSManProxyAuthenticationFlags = WSManProxyAuthenticationFlags(2i32);
pub const WSManFlagProxyAuthenticationUseDigest: WSManProxyAuthenticationFlags = WSManProxyAuthenticationFlags(4i32);
pub const WSManFlagProxyAuthenticationUseNegotiate: WSManProxyAuthenticationFlags = WSManProxyAuthenticationFlags(1i32);
pub const WSManFlagReturnEPR: WSManEnumFlags = WSManEnumFlags(2i32);
pub const WSManFlagReturnObject: WSManEnumFlags = WSManEnumFlags(0i32);
pub const WSManFlagReturnObjectAndEPR: WSManEnumFlags = WSManEnumFlags(4i32);
pub const WSManFlagSkipCACheck: WSManSessionFlags = WSManSessionFlags(8192i32);
pub const WSManFlagSkipCNCheck: WSManSessionFlags = WSManSessionFlags(16384i32);
pub const WSManFlagSkipRevocationCheck: WSManSessionFlags = WSManSessionFlags(33554432i32);
pub const WSManFlagUTF16: WSManSessionFlags = WSManSessionFlags(8388608i32);
pub const WSManFlagUTF8: WSManSessionFlags = WSManSessionFlags(1i32);
pub const WSManFlagUseBasic: WSManSessionFlags = WSManSessionFlags(262144i32);
pub const WSManFlagUseClientCertificate: WSManSessionFlags = WSManSessionFlags(2097152i32);
pub const WSManFlagUseCredSsp: WSManSessionFlags = WSManSessionFlags(16777216i32);
pub const WSManFlagUseDigest: WSManSessionFlags = WSManSessionFlags(65536i32);
pub const WSManFlagUseKerberos: WSManSessionFlags = WSManSessionFlags(524288i32);
pub const WSManFlagUseNegotiate: WSManSessionFlags = WSManSessionFlags(131072i32);
pub const WSManFlagUseNoAuthentication: WSManSessionFlags = WSManSessionFlags(32768i32);
pub const WSManFlagUseSsl: WSManSessionFlags = WSManSessionFlags(134217728i32);
pub const WSManInternal: windows_core::GUID = windows_core::GUID::from_u128(0x7de087a5_5dcb_4df7_bb12_0924ad8fbd9a);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManProxyAccessType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManProxyAccessTypeFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManProxyAuthenticationFlags(pub i32);
pub const WSManProxyAutoDetect: WSManProxyAccessTypeFlags = WSManProxyAccessTypeFlags(4i32);
pub const WSManProxyIEConfig: WSManProxyAccessTypeFlags = WSManProxyAccessTypeFlags(1i32);
pub const WSManProxyNoProxyServer: WSManProxyAccessTypeFlags = WSManProxyAccessTypeFlags(8i32);
pub const WSManProxyWinHttpConfig: WSManProxyAccessTypeFlags = WSManProxyAccessTypeFlags(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManSessionFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManSessionOption(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSManShellFlag(pub i32);

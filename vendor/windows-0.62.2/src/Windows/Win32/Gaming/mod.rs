#[inline]
pub unsafe fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: &windows_core::HSTRING, policy: &windows_core::HSTRING) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-1.dll" "system" fn CheckGamingPrivilegeSilently(privilegeid : u32, scope : * mut core::ffi::c_void, policy : * mut core::ffi::c_void, hasprivilege : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CheckGamingPrivilegeSilently(privilegeid, core::mem::transmute_copy(scope), core::mem::transmute_copy(policy), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CheckGamingPrivilegeSilentlyForUser<P0>(user: P0, privilegeid: u32, scope: &windows_core::HSTRING, policy: &windows_core::HSTRING) -> windows_core::Result<windows_core::BOOL>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn CheckGamingPrivilegeSilentlyForUser(user : * mut core::ffi::c_void, privilegeid : u32, scope : * mut core::ffi::c_void, policy : * mut core::ffi::c_void, hasprivilege : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CheckGamingPrivilegeSilentlyForUser(user.param().abi(), privilegeid, core::mem::transmute_copy(scope), core::mem::transmute_copy(policy), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: &windows_core::HSTRING, policy: &windows_core::HSTRING, friendlymessage: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-1.dll" "system" fn CheckGamingPrivilegeWithUI(privilegeid : u32, scope : * mut core::ffi::c_void, policy : * mut core::ffi::c_void, friendlymessage : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CheckGamingPrivilegeWithUI(privilegeid, core::mem::transmute_copy(scope), core::mem::transmute_copy(policy), core::mem::transmute_copy(friendlymessage), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUIForUser<P0>(user: P0, privilegeid: u32, scope: &windows_core::HSTRING, policy: &windows_core::HSTRING, friendlymessage: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn CheckGamingPrivilegeWithUIForUser(user : * mut core::ffi::c_void, privilegeid : u32, scope : * mut core::ffi::c_void, policy : * mut core::ffi::c_void, friendlymessage : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CheckGamingPrivilegeWithUIForUser(user.param().abi(), privilegeid, core::mem::transmute_copy(scope), core::mem::transmute_copy(policy), core::mem::transmute_copy(friendlymessage), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetExpandedResourceExclusiveCpuCount() -> windows_core::Result<u32> {
    windows_core::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetExpandedResourceExclusiveCpuCount(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetGamingDeviceModelInformation() -> windows_core::Result<GAMING_DEVICE_MODEL_INFORMATION> {
    windows_core::link!("api-ms-win-gaming-deviceinformation-l1-1-0.dll" "system" fn GetGamingDeviceModelInformation(information : *mut GAMING_DEVICE_MODEL_INFORMATION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetGamingDeviceModelInformation(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HasExpandedResources() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn HasExpandedResources(hasexpandedresources : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HasExpandedResources(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ProcessPendingGameUI(waitforcompletion: bool) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ProcessPendingGameUI(waitforcompletion : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { ProcessPendingGameUI(waitforcompletion.into()).ok() }
}
#[inline]
pub unsafe fn ReleaseExclusiveCpuSets() -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn ReleaseExclusiveCpuSets() -> windows_core::HRESULT);
    unsafe { ReleaseExclusiveCpuSets().ok() }
}
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUI(targetuserxuid: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowChangeFriendRelationshipUI(targetuserxuid : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowChangeFriendRelationshipUI(core::mem::transmute_copy(targetuserxuid), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUIForUser<P0>(user: P0, targetuserxuid: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowChangeFriendRelationshipUIForUser(user : * mut core::ffi::c_void, targetuserxuid : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowChangeFriendRelationshipUIForUser(user.param().abi(), core::mem::transmute_copy(targetuserxuid), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowCustomizeUserProfileUI(completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowCustomizeUserProfileUI(completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowCustomizeUserProfileUIForUser<P0>(user: P0, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowCustomizeUserProfileUIForUser(user : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowCustomizeUserProfileUIForUser(user.param().abi(), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowFindFriendsUI(completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowFindFriendsUI(completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowFindFriendsUIForUser<P0>(user: P0, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowFindFriendsUIForUser(user : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowFindFriendsUIForUser(user.param().abi(), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowGameInfoUI(titleid : u32, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowGameInfoUI(titleid, completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowGameInfoUIForUser<P0>(user: P0, titleid: u32, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowGameInfoUIForUser(user : * mut core::ffi::c_void, titleid : u32, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowGameInfoUIForUser(user.param().abi(), titleid, completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowGameInviteUI(serviceconfigurationid: &windows_core::HSTRING, sessiontemplatename: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, invitationdisplaytext: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowGameInviteUI(serviceconfigurationid : * mut core::ffi::c_void, sessiontemplatename : * mut core::ffi::c_void, sessionid : * mut core::ffi::c_void, invitationdisplaytext : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowGameInviteUI(core::mem::transmute_copy(serviceconfigurationid), core::mem::transmute_copy(sessiontemplatename), core::mem::transmute_copy(sessionid), core::mem::transmute_copy(invitationdisplaytext), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowGameInviteUIForUser<P0>(user: P0, serviceconfigurationid: &windows_core::HSTRING, sessiontemplatename: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, invitationdisplaytext: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowGameInviteUIForUser(user : * mut core::ffi::c_void, serviceconfigurationid : * mut core::ffi::c_void, sessiontemplatename : * mut core::ffi::c_void, sessionid : * mut core::ffi::c_void, invitationdisplaytext : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowGameInviteUIForUser(user.param().abi(), core::mem::transmute_copy(serviceconfigurationid), core::mem::transmute_copy(sessiontemplatename), core::mem::transmute_copy(sessionid), core::mem::transmute_copy(invitationdisplaytext), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowGameInviteUIWithContext(serviceconfigurationid: &windows_core::HSTRING, sessiontemplatename: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, invitationdisplaytext: &windows_core::HSTRING, customactivationcontext: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-3.dll" "system" fn ShowGameInviteUIWithContext(serviceconfigurationid : * mut core::ffi::c_void, sessiontemplatename : * mut core::ffi::c_void, sessionid : * mut core::ffi::c_void, invitationdisplaytext : * mut core::ffi::c_void, customactivationcontext : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowGameInviteUIWithContext(core::mem::transmute_copy(serviceconfigurationid), core::mem::transmute_copy(sessiontemplatename), core::mem::transmute_copy(sessionid), core::mem::transmute_copy(invitationdisplaytext), core::mem::transmute_copy(customactivationcontext), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowGameInviteUIWithContextForUser<P0>(user: P0, serviceconfigurationid: &windows_core::HSTRING, sessiontemplatename: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, invitationdisplaytext: &windows_core::HSTRING, customactivationcontext: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-3.dll" "system" fn ShowGameInviteUIWithContextForUser(user : * mut core::ffi::c_void, serviceconfigurationid : * mut core::ffi::c_void, sessiontemplatename : * mut core::ffi::c_void, sessionid : * mut core::ffi::c_void, invitationdisplaytext : * mut core::ffi::c_void, customactivationcontext : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowGameInviteUIWithContextForUser(user.param().abi(), core::mem::transmute_copy(serviceconfigurationid), core::mem::transmute_copy(sessiontemplatename), core::mem::transmute_copy(sessionid), core::mem::transmute_copy(invitationdisplaytext), core::mem::transmute_copy(customactivationcontext), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowPlayerPickerUI(promptdisplaytext: &windows_core::HSTRING, xuids: &[windows_core::HSTRING], preselectedxuids: Option<&[windows_core::HSTRING]>, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowPlayerPickerUI(promptdisplaytext : * mut core::ffi::c_void, xuids : *const * mut core::ffi::c_void, xuidscount : usize, preselectedxuids : *const * mut core::ffi::c_void, preselectedxuidscount : usize, minselectioncount : usize, maxselectioncount : usize, completionroutine : PlayerPickerUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowPlayerPickerUI(core::mem::transmute_copy(promptdisplaytext), core::mem::transmute(xuids.as_ptr()), xuids.len().try_into().unwrap(), core::mem::transmute(preselectedxuids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), preselectedxuids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), minselectioncount, maxselectioncount, completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowPlayerPickerUIForUser<P0>(user: P0, promptdisplaytext: &windows_core::HSTRING, xuids: &[windows_core::HSTRING], preselectedxuids: Option<&[windows_core::HSTRING]>, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowPlayerPickerUIForUser(user : * mut core::ffi::c_void, promptdisplaytext : * mut core::ffi::c_void, xuids : *const * mut core::ffi::c_void, xuidscount : usize, preselectedxuids : *const * mut core::ffi::c_void, preselectedxuidscount : usize, minselectioncount : usize, maxselectioncount : usize, completionroutine : PlayerPickerUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowPlayerPickerUIForUser(user.param().abi(), core::mem::transmute_copy(promptdisplaytext), core::mem::transmute(xuids.as_ptr()), xuids.len().try_into().unwrap(), core::mem::transmute(preselectedxuids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), preselectedxuids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), minselectioncount, maxselectioncount, completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowProfileCardUI(targetuserxuid: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowProfileCardUI(targetuserxuid : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowProfileCardUI(core::mem::transmute_copy(targetuserxuid), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowProfileCardUIForUser<P0>(user: P0, targetuserxuid: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowProfileCardUIForUser(user : * mut core::ffi::c_void, targetuserxuid : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowProfileCardUIForUser(user.param().abi(), core::mem::transmute_copy(targetuserxuid), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowTitleAchievementsUI(titleid : u32, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowTitleAchievementsUI(titleid, completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowTitleAchievementsUIForUser<P0>(user: P0, titleid: u32, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowTitleAchievementsUIForUser(user : * mut core::ffi::c_void, titleid : u32, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowTitleAchievementsUIForUser(user.param().abi(), titleid, completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowUserSettingsUI(completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowUserSettingsUI(completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ShowUserSettingsUIForUser<P0>(user: P0, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowUserSettingsUIForUser(user : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ShowUserSettingsUIForUser(user.param().abi(), completionroutine, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn TryCancelPendingGameUI() -> windows_core::BOOL {
    windows_core::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn TryCancelPendingGameUI() -> windows_core::BOOL);
    unsafe { TryCancelPendingGameUI() }
}
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(0i32);
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(1i32);
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(1i32);
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GAMESTATS_OPEN_RESULT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GAMESTATS_OPEN_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GAME_INSTALL_SCOPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GAMING_DEVICE_DEVICE_ID(pub i32);
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(0i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1988865574i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(712204761i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1523980231i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(284675555i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_SERIES_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(489159355i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_SERIES_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(796540415i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_SERIES_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(-561359263i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GAMING_DEVICE_VENDOR_ID(pub i32);
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(-1024700366i32);
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(0i32);
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(3i32);
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(2i32);
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(1i32);
pub const GameExplorer: windows_core::GUID = windows_core::GUID::from_u128(0x9a5ea990_3034_4d6f_9128_01f3c61022bc);
pub const GameStatistics: windows_core::GUID = windows_core::GUID::from_u128(0xdbc85a2c_c0dc_4961_b6e2_d28b62c11ad4);
pub type GameUICompletionRoutine = Option<unsafe extern "system" fn(returncode: windows_core::HRESULT, context: *const core::ffi::c_void)>;
pub const ID_GDF_THUMBNAIL_STR: windows_core::PCWSTR = windows_core::w!("__GDF_THUMBNAIL");
pub const ID_GDF_XML_STR: windows_core::PCWSTR = windows_core::w!("__GDF_XML");
windows_core::imp::define_interface!(IGameExplorer, IGameExplorer_Vtbl, 0xe7b2fb72_d728_49b3_a5f2_18ebf5f1349e);
windows_core::imp::interface_hierarchy!(IGameExplorer, windows_core::IUnknown);
impl IGameExplorer {
    pub unsafe fn AddGame(&self, bstrgdfbinarypath: &windows_core::BSTR, bstrgameinstalldirectory: &windows_core::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddGame)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgdfbinarypath), core::mem::transmute_copy(bstrgameinstalldirectory), installscope, pguidinstanceid as _).ok() }
    }
    pub unsafe fn RemoveGame(&self, guidinstanceid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveGame)(windows_core::Interface::as_raw(self), core::mem::transmute(guidinstanceid)).ok() }
    }
    pub unsafe fn UpdateGame(&self, guidinstanceid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateGame)(windows_core::Interface::as_raw(self), core::mem::transmute(guidinstanceid)).ok() }
    }
    pub unsafe fn VerifyAccess(&self, bstrgdfbinarypath: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerifyAccess)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgdfbinarypath), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddGame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, GAME_INSTALL_SCOPE, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RemoveGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub UpdateGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub VerifyAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IGameExplorer_Impl: windows_core::IUnknownImpl {
    fn AddGame(&self, bstrgdfbinarypath: &windows_core::BSTR, bstrgameinstalldirectory: &windows_core::BSTR, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RemoveGame(&self, guidinstanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn UpdateGame(&self, guidinstanceid: &windows_core::GUID) -> windows_core::Result<()>;
    fn VerifyAccess(&self, bstrgdfbinarypath: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL>;
}
impl IGameExplorer_Vtbl {
    pub const fn new<Identity: IGameExplorer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddGame<Identity: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgdfbinarypath: *mut core::ffi::c_void, bstrgameinstalldirectory: *mut core::ffi::c_void, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer_Impl::AddGame(this, core::mem::transmute(&bstrgdfbinarypath), core::mem::transmute(&bstrgameinstalldirectory), core::mem::transmute_copy(&installscope), core::mem::transmute_copy(&pguidinstanceid)).into()
            }
        }
        unsafe extern "system" fn RemoveGame<Identity: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer_Impl::RemoveGame(this, core::mem::transmute(&guidinstanceid)).into()
            }
        }
        unsafe extern "system" fn UpdateGame<Identity: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidinstanceid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer_Impl::UpdateGame(this, core::mem::transmute(&guidinstanceid)).into()
            }
        }
        unsafe extern "system" fn VerifyAccess<Identity: IGameExplorer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgdfbinarypath: *mut core::ffi::c_void, pfhasaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameExplorer_Impl::VerifyAccess(this, core::mem::transmute(&bstrgdfbinarypath)) {
                    Ok(ok__) => {
                        pfhasaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddGame: AddGame::<Identity, OFFSET>,
            RemoveGame: RemoveGame::<Identity, OFFSET>,
            UpdateGame: UpdateGame::<Identity, OFFSET>,
            VerifyAccess: VerifyAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameExplorer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameExplorer {}
windows_core::imp::define_interface!(IGameExplorer2, IGameExplorer2_Vtbl, 0x86874aa7_a1ed_450d_a7eb_b89e20b2fff3);
windows_core::imp::interface_hierarchy!(IGameExplorer2, windows_core::IUnknown);
impl IGameExplorer2 {
    pub unsafe fn InstallGame<P0, P1>(&self, binarygdfpath: P0, installdirectory: P1, installscope: GAME_INSTALL_SCOPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InstallGame)(windows_core::Interface::as_raw(self), binarygdfpath.param().abi(), installdirectory.param().abi(), installscope).ok() }
    }
    pub unsafe fn UninstallGame<P0>(&self, binarygdfpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UninstallGame)(windows_core::Interface::as_raw(self), binarygdfpath.param().abi()).ok() }
    }
    pub unsafe fn CheckAccess<P0>(&self, binarygdfpath: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckAccess)(windows_core::Interface::as_raw(self), binarygdfpath.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InstallGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, GAME_INSTALL_SCOPE) -> windows_core::HRESULT,
    pub UninstallGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IGameExplorer2_Impl: windows_core::IUnknownImpl {
    fn InstallGame(&self, binarygdfpath: &windows_core::PCWSTR, installdirectory: &windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> windows_core::Result<()>;
    fn UninstallGame(&self, binarygdfpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CheckAccess(&self, binarygdfpath: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
impl IGameExplorer2_Vtbl {
    pub const fn new<Identity: IGameExplorer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InstallGame<Identity: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR, installdirectory: windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer2_Impl::InstallGame(this, core::mem::transmute(&binarygdfpath), core::mem::transmute(&installdirectory), core::mem::transmute_copy(&installscope)).into()
            }
        }
        unsafe extern "system" fn UninstallGame<Identity: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameExplorer2_Impl::UninstallGame(this, core::mem::transmute(&binarygdfpath)).into()
            }
        }
        unsafe extern "system" fn CheckAccess<Identity: IGameExplorer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binarygdfpath: windows_core::PCWSTR, phasaccess: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameExplorer2_Impl::CheckAccess(this, core::mem::transmute(&binarygdfpath)) {
                    Ok(ok__) => {
                        phasaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InstallGame: InstallGame::<Identity, OFFSET>,
            UninstallGame: UninstallGame::<Identity, OFFSET>,
            CheckAccess: CheckAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameExplorer2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameExplorer2 {}
windows_core::imp::define_interface!(IGameStatistics, IGameStatistics_Vtbl, 0x3887c9ca_04a0_42ae_bc4c_5fa6c7721145);
windows_core::imp::interface_hierarchy!(IGameStatistics, windows_core::IUnknown);
impl IGameStatistics {
    pub unsafe fn GetMaxCategoryLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxCategoryLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxNameLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxNameLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxValueLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxValueLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxCategories(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxCategories)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxStatsPerCategory(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxStatsPerCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCategoryTitle<P1>(&self, categoryindex: u16, title: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCategoryTitle)(windows_core::Interface::as_raw(self), categoryindex, title.param().abi()).ok() }
    }
    pub unsafe fn GetCategoryTitle(&self, categoryindex: u16) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategoryTitle)(windows_core::Interface::as_raw(self), categoryindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: Option<*mut windows_core::PWSTR>, pvalue: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatistic)(windows_core::Interface::as_raw(self), categoryindex, statindex, pname.unwrap_or(core::mem::zeroed()) as _, pvalue.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetStatistic<P2, P3>(&self, categoryindex: u16, statindex: u16, name: P2, value: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStatistic)(windows_core::Interface::as_raw(self), categoryindex, statindex, name.param().abi(), value.param().abi()).ok() }
    }
    pub unsafe fn Save(&self, trackchanges: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), trackchanges.into()).ok() }
    }
    pub unsafe fn SetLastPlayedCategory(&self, categoryindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLastPlayedCategory)(windows_core::Interface::as_raw(self), categoryindex).ok() }
    }
    pub unsafe fn GetLastPlayedCategory(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPlayedCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatistics_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMaxCategoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaxNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaxValueLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaxCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetMaxStatsPerCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetCategoryTitle: unsafe extern "system" fn(*mut core::ffi::c_void, u16, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCategoryTitle: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetStatistic: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetStatistic: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetLastPlayedCategory: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetLastPlayedCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IGameStatistics_Impl: windows_core::IUnknownImpl {
    fn GetMaxCategoryLength(&self) -> windows_core::Result<u32>;
    fn GetMaxNameLength(&self) -> windows_core::Result<u32>;
    fn GetMaxValueLength(&self) -> windows_core::Result<u32>;
    fn GetMaxCategories(&self) -> windows_core::Result<u16>;
    fn GetMaxStatsPerCategory(&self) -> windows_core::Result<u16>;
    fn SetCategoryTitle(&self, categoryindex: u16, title: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCategoryTitle(&self, categoryindex: u16) -> windows_core::Result<windows_core::PWSTR>;
    fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut windows_core::PWSTR, pvalue: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetStatistic(&self, categoryindex: u16, statindex: u16, name: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Save(&self, trackchanges: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetLastPlayedCategory(&self, categoryindex: u32) -> windows_core::Result<()>;
    fn GetLastPlayedCategory(&self) -> windows_core::Result<u32>;
}
impl IGameStatistics_Vtbl {
    pub const fn new<Identity: IGameStatistics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaxCategoryLength<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxCategoryLength(this) {
                    Ok(ok__) => {
                        cch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxNameLength<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxNameLength(this) {
                    Ok(ok__) => {
                        cch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxValueLength<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxValueLength(this) {
                    Ok(ok__) => {
                        cch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxCategories<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmax: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxCategories(this) {
                    Ok(ok__) => {
                        pmax.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxStatsPerCategory<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmax: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetMaxStatsPerCategory(this) {
                    Ok(ok__) => {
                        pmax.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCategoryTitle<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, title: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::SetCategoryTitle(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute(&title)).into()
            }
        }
        unsafe extern "system" fn GetCategoryTitle<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, ptitle: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetCategoryTitle(this, core::mem::transmute_copy(&categoryindex)) {
                    Ok(ok__) => {
                        ptitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatistic<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut windows_core::PWSTR, pvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::GetStatistic(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute_copy(&statindex), core::mem::transmute_copy(&pname), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetStatistic<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u16, statindex: u16, name: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::SetStatistic(this, core::mem::transmute_copy(&categoryindex), core::mem::transmute_copy(&statindex), core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackchanges: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::Save(this, core::mem::transmute_copy(&trackchanges)).into()
            }
        }
        unsafe extern "system" fn SetLastPlayedCategory<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, categoryindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatistics_Impl::SetLastPlayedCategory(this, core::mem::transmute_copy(&categoryindex)).into()
            }
        }
        unsafe extern "system" fn GetLastPlayedCategory<Identity: IGameStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategoryindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameStatistics_Impl::GetLastPlayedCategory(this) {
                    Ok(ok__) => {
                        pcategoryindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxCategoryLength: GetMaxCategoryLength::<Identity, OFFSET>,
            GetMaxNameLength: GetMaxNameLength::<Identity, OFFSET>,
            GetMaxValueLength: GetMaxValueLength::<Identity, OFFSET>,
            GetMaxCategories: GetMaxCategories::<Identity, OFFSET>,
            GetMaxStatsPerCategory: GetMaxStatsPerCategory::<Identity, OFFSET>,
            SetCategoryTitle: SetCategoryTitle::<Identity, OFFSET>,
            GetCategoryTitle: GetCategoryTitle::<Identity, OFFSET>,
            GetStatistic: GetStatistic::<Identity, OFFSET>,
            SetStatistic: SetStatistic::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SetLastPlayedCategory: SetLastPlayedCategory::<Identity, OFFSET>,
            GetLastPlayedCategory: GetLastPlayedCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameStatistics as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameStatistics {}
windows_core::imp::define_interface!(IGameStatisticsMgr, IGameStatisticsMgr_Vtbl, 0xaff3ea11_e70e_407d_95dd_35e612c41ce2);
windows_core::imp::interface_hierarchy!(IGameStatisticsMgr, windows_core::IUnknown);
impl IGameStatisticsMgr {
    pub unsafe fn GetGameStatistics<P0>(&self, gdfbinarypath: P0, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut Option<IGameStatistics>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetGameStatistics)(windows_core::Interface::as_raw(self), gdfbinarypath.param().abi(), opentype, popenresult as _, core::mem::transmute(ppistats)).ok() }
    }
    pub unsafe fn RemoveGameStatistics<P0>(&self, gdfbinarypath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveGameStatistics)(windows_core::Interface::as_raw(self), gdfbinarypath.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatisticsMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, GAMESTATS_OPEN_TYPE, *mut GAMESTATS_OPEN_RESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveGameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IGameStatisticsMgr_Impl: windows_core::IUnknownImpl {
    fn GetGameStatistics(&self, gdfbinarypath: &windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: windows_core::OutRef<IGameStatistics>) -> windows_core::Result<()>;
    fn RemoveGameStatistics(&self, gdfbinarypath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IGameStatisticsMgr_Vtbl {
    pub const fn new<Identity: IGameStatisticsMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGameStatistics<Identity: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdfbinarypath: windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatisticsMgr_Impl::GetGameStatistics(this, core::mem::transmute(&gdfbinarypath), core::mem::transmute_copy(&opentype), core::mem::transmute_copy(&popenresult), core::mem::transmute_copy(&ppistats)).into()
            }
        }
        unsafe extern "system" fn RemoveGameStatistics<Identity: IGameStatisticsMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdfbinarypath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGameStatisticsMgr_Impl::RemoveGameStatistics(this, core::mem::transmute(&gdfbinarypath)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGameStatistics: GetGameStatistics::<Identity, OFFSET>,
            RemoveGameStatistics: RemoveGameStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameStatisticsMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGameStatisticsMgr {}
windows_core::imp::define_interface!(IXblIdpAuthManager, IXblIdpAuthManager_Vtbl, 0xeb5ddb08_8bbf_449b_ac21_b02ddeb3b136);
windows_core::imp::interface_hierarchy!(IXblIdpAuthManager, windows_core::IUnknown);
impl IXblIdpAuthManager {
    pub unsafe fn SetGamerAccount<P0, P1>(&self, msaaccountid: P0, xuid: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGamerAccount)(windows_core::Interface::as_raw(self), msaaccountid.param().abi(), xuid.param().abi()).ok() }
    }
    pub unsafe fn GetGamerAccount(&self, msaaccountid: *mut windows_core::PWSTR, xuid: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGamerAccount)(windows_core::Interface::as_raw(self), msaaccountid as _, xuid as _).ok() }
    }
    pub unsafe fn SetAppViewInitialized<P0, P1>(&self, appsid: P0, msaaccountid: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAppViewInitialized)(windows_core::Interface::as_raw(self), appsid.param().abi(), msaaccountid.param().abi()).ok() }
    }
    pub unsafe fn GetEnvironment(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnvironment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSandbox(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSandbox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTokenAndSignatureWithTokenResult<P0, P1, P2, P3, P4, P5, P6>(&self, msaaccountid: P0, appsid: P1, msatarget: P2, msapolicy: P3, httpmethod: P4, uri: P5, headers: P6, body: &[u8], forcerefresh: bool) -> windows_core::Result<IXblIdpAuthTokenResult>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTokenAndSignatureWithTokenResult)(windows_core::Interface::as_raw(self), msaaccountid.param().abi(), appsid.param().abi(), msatarget.param().abi(), msapolicy.param().abi(), httpmethod.param().abi(), uri.param().abi(), headers.param().abi(), core::mem::transmute(body.as_ptr()), body.len().try_into().unwrap(), forcerefresh.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGamerAccount: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetGamerAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAppViewInitialized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetTokenAndSignatureWithTokenResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXblIdpAuthManager_Impl: windows_core::IUnknownImpl {
    fn SetGamerAccount(&self, msaaccountid: &windows_core::PCWSTR, xuid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetGamerAccount(&self, msaaccountid: *mut windows_core::PWSTR, xuid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetAppViewInitialized(&self, appsid: &windows_core::PCWSTR, msaaccountid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetEnvironment(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSandbox(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTokenAndSignatureWithTokenResult(&self, msaaccountid: &windows_core::PCWSTR, appsid: &windows_core::PCWSTR, msatarget: &windows_core::PCWSTR, msapolicy: &windows_core::PCWSTR, httpmethod: &windows_core::PCWSTR, uri: &windows_core::PCWSTR, headers: &windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: windows_core::BOOL) -> windows_core::Result<IXblIdpAuthTokenResult>;
}
impl IXblIdpAuthManager_Vtbl {
    pub const fn new<Identity: IXblIdpAuthManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGamerAccount<Identity: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: windows_core::PCWSTR, xuid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXblIdpAuthManager_Impl::SetGamerAccount(this, core::mem::transmute(&msaaccountid), core::mem::transmute(&xuid)).into()
            }
        }
        unsafe extern "system" fn GetGamerAccount<Identity: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: *mut windows_core::PWSTR, xuid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXblIdpAuthManager_Impl::GetGamerAccount(this, core::mem::transmute_copy(&msaaccountid), core::mem::transmute_copy(&xuid)).into()
            }
        }
        unsafe extern "system" fn SetAppViewInitialized<Identity: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appsid: windows_core::PCWSTR, msaaccountid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXblIdpAuthManager_Impl::SetAppViewInitialized(this, core::mem::transmute(&appsid), core::mem::transmute(&msaaccountid)).into()
            }
        }
        unsafe extern "system" fn GetEnvironment<Identity: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthManager_Impl::GetEnvironment(this) {
                    Ok(ok__) => {
                        environment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sandbox: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthManager_Impl::GetSandbox(this) {
                    Ok(ok__) => {
                        sandbox.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTokenAndSignatureWithTokenResult<Identity: IXblIdpAuthManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: windows_core::PCWSTR, appsid: windows_core::PCWSTR, msatarget: windows_core::PCWSTR, msapolicy: windows_core::PCWSTR, httpmethod: windows_core::PCWSTR, uri: windows_core::PCWSTR, headers: windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: windows_core::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthManager_Impl::GetTokenAndSignatureWithTokenResult(this, core::mem::transmute(&msaaccountid), core::mem::transmute(&appsid), core::mem::transmute(&msatarget), core::mem::transmute(&msapolicy), core::mem::transmute(&httpmethod), core::mem::transmute(&uri), core::mem::transmute(&headers), core::mem::transmute_copy(&body), core::mem::transmute_copy(&bodysize), core::mem::transmute_copy(&forcerefresh)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGamerAccount: SetGamerAccount::<Identity, OFFSET>,
            GetGamerAccount: GetGamerAccount::<Identity, OFFSET>,
            SetAppViewInitialized: SetAppViewInitialized::<Identity, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, OFFSET>,
            GetSandbox: GetSandbox::<Identity, OFFSET>,
            GetTokenAndSignatureWithTokenResult: GetTokenAndSignatureWithTokenResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXblIdpAuthManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXblIdpAuthManager {}
windows_core::imp::define_interface!(IXblIdpAuthManager2, IXblIdpAuthManager2_Vtbl, 0xbf8c0950_8389_43dd_9a76_a19728ec5dc5);
windows_core::imp::interface_hierarchy!(IXblIdpAuthManager2, windows_core::IUnknown);
impl IXblIdpAuthManager2 {
    pub unsafe fn GetUserlessTokenAndSignatureWithTokenResult<P0, P1, P2, P3, P4, P5>(&self, appsid: P0, msatarget: P1, msapolicy: P2, httpmethod: P3, uri: P4, headers: P5, body: &[u8], forcerefresh: bool) -> windows_core::Result<IXblIdpAuthTokenResult>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserlessTokenAndSignatureWithTokenResult)(windows_core::Interface::as_raw(self), appsid.param().abi(), msatarget.param().abi(), msapolicy.param().abi(), httpmethod.param().abi(), uri.param().abi(), headers.param().abi(), core::mem::transmute(body.as_ptr()), body.len().try_into().unwrap(), forcerefresh.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthManager2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUserlessTokenAndSignatureWithTokenResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXblIdpAuthManager2_Impl: windows_core::IUnknownImpl {
    fn GetUserlessTokenAndSignatureWithTokenResult(&self, appsid: &windows_core::PCWSTR, msatarget: &windows_core::PCWSTR, msapolicy: &windows_core::PCWSTR, httpmethod: &windows_core::PCWSTR, uri: &windows_core::PCWSTR, headers: &windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: windows_core::BOOL) -> windows_core::Result<IXblIdpAuthTokenResult>;
}
impl IXblIdpAuthManager2_Vtbl {
    pub const fn new<Identity: IXblIdpAuthManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUserlessTokenAndSignatureWithTokenResult<Identity: IXblIdpAuthManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appsid: windows_core::PCWSTR, msatarget: windows_core::PCWSTR, msapolicy: windows_core::PCWSTR, httpmethod: windows_core::PCWSTR, uri: windows_core::PCWSTR, headers: windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: windows_core::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthManager2_Impl::GetUserlessTokenAndSignatureWithTokenResult(this, core::mem::transmute(&appsid), core::mem::transmute(&msatarget), core::mem::transmute(&msapolicy), core::mem::transmute(&httpmethod), core::mem::transmute(&uri), core::mem::transmute(&headers), core::mem::transmute_copy(&body), core::mem::transmute_copy(&bodysize), core::mem::transmute_copy(&forcerefresh)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUserlessTokenAndSignatureWithTokenResult: GetUserlessTokenAndSignatureWithTokenResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXblIdpAuthManager2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXblIdpAuthManager2 {}
windows_core::imp::define_interface!(IXblIdpAuthTokenResult, IXblIdpAuthTokenResult_Vtbl, 0x46ce0225_f267_4d68_b299_b2762552dec1);
windows_core::imp::interface_hierarchy!(IXblIdpAuthTokenResult, windows_core::IUnknown);
impl IXblIdpAuthTokenResult {
    pub unsafe fn GetStatus(&self) -> windows_core::Result<XBL_IDP_AUTH_TOKEN_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetToken(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetToken)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSignature(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignature)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSandbox(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSandbox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnvironment(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnvironment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMsaAccountId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMsaAccountId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetXuid(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGamertag(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGamertag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAgeGroup(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAgeGroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPrivileges(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrivileges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMsaTarget(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMsaTarget)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMsaPolicy(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMsaPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMsaAppId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMsaAppId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRedirect(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRedirect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMessage(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHelpId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelpId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnforcementBans(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnforcementBans)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRestrictions(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTitleRestrictions(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTitleRestrictions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XBL_IDP_AUTH_TOKEN_STATUS) -> windows_core::HRESULT,
    pub GetErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMsaAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetXuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetGamertag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetAgeGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPrivileges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMsaTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMsaPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMsaAppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetHelpId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetEnforcementBans: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetTitleRestrictions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IXblIdpAuthTokenResult_Impl: windows_core::IUnknownImpl {
    fn GetStatus(&self) -> windows_core::Result<XBL_IDP_AUTH_TOKEN_STATUS>;
    fn GetErrorCode(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetToken(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignature(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSandbox(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetEnvironment(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMsaAccountId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetXuid(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetGamertag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetAgeGroup(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPrivileges(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMsaTarget(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMsaPolicy(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMsaAppId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetRedirect(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMessage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHelpId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetEnforcementBans(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetRestrictions(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTitleRestrictions(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IXblIdpAuthTokenResult_Vtbl {
    pub const fn new<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStatus<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        status.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetErrorCode(this) {
                    Ok(ok__) => {
                        errorcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetToken<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetToken(this) {
                    Ok(ok__) => {
                        token.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignature<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, signature: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetSignature(this) {
                    Ok(ok__) => {
                        signature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSandbox<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sandbox: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetSandbox(this) {
                    Ok(ok__) => {
                        sandbox.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnvironment<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetEnvironment(this) {
                    Ok(ok__) => {
                        environment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMsaAccountId<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaaccountid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetMsaAccountId(this) {
                    Ok(ok__) => {
                        msaaccountid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXuid<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xuid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetXuid(this) {
                    Ok(ok__) => {
                        xuid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGamertag<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gamertag: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetGamertag(this) {
                    Ok(ok__) => {
                        gamertag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAgeGroup<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, agegroup: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetAgeGroup(this) {
                    Ok(ok__) => {
                        agegroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrivileges<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, privileges: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetPrivileges(this) {
                    Ok(ok__) => {
                        privileges.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMsaTarget<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msatarget: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetMsaTarget(this) {
                    Ok(ok__) => {
                        msatarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMsaPolicy<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msapolicy: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetMsaPolicy(this) {
                    Ok(ok__) => {
                        msapolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMsaAppId<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msaappid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetMsaAppId(this) {
                    Ok(ok__) => {
                        msaappid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRedirect<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, redirect: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetRedirect(this) {
                    Ok(ok__) => {
                        redirect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMessage<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetMessage(this) {
                    Ok(ok__) => {
                        message.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHelpId<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, helpid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetHelpId(this) {
                    Ok(ok__) => {
                        helpid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnforcementBans<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enforcementbans: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetEnforcementBans(this) {
                    Ok(ok__) => {
                        enforcementbans.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestrictions<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictions: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetRestrictions(this) {
                    Ok(ok__) => {
                        restrictions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTitleRestrictions<Identity: IXblIdpAuthTokenResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, titlerestrictions: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult_Impl::GetTitleRestrictions(this) {
                    Ok(ok__) => {
                        titlerestrictions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, OFFSET>,
            GetToken: GetToken::<Identity, OFFSET>,
            GetSignature: GetSignature::<Identity, OFFSET>,
            GetSandbox: GetSandbox::<Identity, OFFSET>,
            GetEnvironment: GetEnvironment::<Identity, OFFSET>,
            GetMsaAccountId: GetMsaAccountId::<Identity, OFFSET>,
            GetXuid: GetXuid::<Identity, OFFSET>,
            GetGamertag: GetGamertag::<Identity, OFFSET>,
            GetAgeGroup: GetAgeGroup::<Identity, OFFSET>,
            GetPrivileges: GetPrivileges::<Identity, OFFSET>,
            GetMsaTarget: GetMsaTarget::<Identity, OFFSET>,
            GetMsaPolicy: GetMsaPolicy::<Identity, OFFSET>,
            GetMsaAppId: GetMsaAppId::<Identity, OFFSET>,
            GetRedirect: GetRedirect::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
            GetHelpId: GetHelpId::<Identity, OFFSET>,
            GetEnforcementBans: GetEnforcementBans::<Identity, OFFSET>,
            GetRestrictions: GetRestrictions::<Identity, OFFSET>,
            GetTitleRestrictions: GetTitleRestrictions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXblIdpAuthTokenResult {}
windows_core::imp::define_interface!(IXblIdpAuthTokenResult2, IXblIdpAuthTokenResult2_Vtbl, 0x75d760b0_60b9_412d_994f_26b2cd5f7812);
windows_core::imp::interface_hierarchy!(IXblIdpAuthTokenResult2, windows_core::IUnknown);
impl IXblIdpAuthTokenResult2 {
    pub unsafe fn GetModernGamertag(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModernGamertag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetModernGamertagSuffix(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModernGamertagSuffix)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUniqueModernGamertag(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUniqueModernGamertag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetModernGamertag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetModernGamertagSuffix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetUniqueModernGamertag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IXblIdpAuthTokenResult2_Impl: windows_core::IUnknownImpl {
    fn GetModernGamertag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetModernGamertagSuffix(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetUniqueModernGamertag(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IXblIdpAuthTokenResult2_Vtbl {
    pub const fn new<Identity: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetModernGamertag<Identity: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult2_Impl::GetModernGamertag(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetModernGamertagSuffix<Identity: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult2_Impl::GetModernGamertagSuffix(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUniqueModernGamertag<Identity: IXblIdpAuthTokenResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXblIdpAuthTokenResult2_Impl::GetUniqueModernGamertag(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetModernGamertag: GetModernGamertag::<Identity, OFFSET>,
            GetModernGamertagSuffix: GetModernGamertagSuffix::<Identity, OFFSET>,
            GetUniqueModernGamertag: GetUniqueModernGamertag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXblIdpAuthTokenResult2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXblIdpAuthTokenResult2 {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KnownGamingPrivileges(pub i32);
pub type PlayerPickerUICompletionRoutine = Option<unsafe extern "system" fn(returncode: windows_core::HRESULT, context: *const core::ffi::c_void, selectedxuids: *const windows_core::HSTRING, selectedxuidscount: usize)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XBL_IDP_AUTH_TOKEN_STATUS(pub i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(3i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(5i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(2i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(6i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(1i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(0i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(-1i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(7i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(4i32);
pub const XPRIVILEGE_ADD_FRIEND: KnownGamingPrivileges = KnownGamingPrivileges(255i32);
pub const XPRIVILEGE_BROADCAST: KnownGamingPrivileges = KnownGamingPrivileges(190i32);
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(208i32);
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(207i32);
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: KnownGamingPrivileges = KnownGamingPrivileges(209i32);
pub const XPRIVILEGE_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(252i32);
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: KnownGamingPrivileges = KnownGamingPrivileges(205i32);
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: KnownGamingPrivileges = KnownGamingPrivileges(206i32);
pub const XPRIVILEGE_GAME_DVR: KnownGamingPrivileges = KnownGamingPrivileges(198i32);
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: KnownGamingPrivileges = KnownGamingPrivileges(203i32);
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: KnownGamingPrivileges = KnownGamingPrivileges(254i32);
pub const XPRIVILEGE_PREMIUM_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(214i32);
pub const XPRIVILEGE_PREMIUM_VIDEO: KnownGamingPrivileges = KnownGamingPrivileges(224i32);
pub const XPRIVILEGE_PROFILE_VIEWING: KnownGamingPrivileges = KnownGamingPrivileges(249i32);
pub const XPRIVILEGE_PURCHASE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(245i32);
pub const XPRIVILEGE_SHARE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(211i32);
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(199i32);
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: KnownGamingPrivileges = KnownGamingPrivileges(220i32);
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(219i32);
pub const XPRIVILEGE_USER_CREATED_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(247i32);
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(235i32);
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: KnownGamingPrivileges = KnownGamingPrivileges(197i32);
pub const XblIdpAuthManager: windows_core::GUID = windows_core::GUID::from_u128(0xce23534b_56d8_4978_86a2_7ee570640468);
pub const XblIdpAuthTokenResult: windows_core::GUID = windows_core::GUID::from_u128(0x9f493441_744a_410c_ae2b_9a22f7c7731f);

#[inline]
pub unsafe fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: &windows_core::HSTRING, policy: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::BOOL> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-1.dll" "system" fn CheckGamingPrivilegeSilently(privilegeid : u32, scope : core::mem::MaybeUninit < windows_core::HSTRING >, policy : core::mem::MaybeUninit < windows_core::HSTRING >, hasprivilege : *mut super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CheckGamingPrivilegeSilently(privilegeid, core::mem::transmute_copy(scope), core::mem::transmute_copy(policy), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CheckGamingPrivilegeSilentlyForUser<P0>(user: P0, privilegeid: u32, scope: &windows_core::HSTRING, policy: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::BOOL>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn CheckGamingPrivilegeSilentlyForUser(user : * mut core::ffi::c_void, privilegeid : u32, scope : core::mem::MaybeUninit < windows_core::HSTRING >, policy : core::mem::MaybeUninit < windows_core::HSTRING >, hasprivilege : *mut super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CheckGamingPrivilegeSilentlyForUser(user.param().abi(), privilegeid, core::mem::transmute_copy(scope), core::mem::transmute_copy(policy), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: &windows_core::HSTRING, policy: &windows_core::HSTRING, friendlymessage: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-1.dll" "system" fn CheckGamingPrivilegeWithUI(privilegeid : u32, scope : core::mem::MaybeUninit < windows_core::HSTRING >, policy : core::mem::MaybeUninit < windows_core::HSTRING >, friendlymessage : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    CheckGamingPrivilegeWithUI(privilegeid, core::mem::transmute_copy(scope), core::mem::transmute_copy(policy), core::mem::transmute_copy(friendlymessage), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUIForUser<P0>(user: P0, privilegeid: u32, scope: &windows_core::HSTRING, policy: &windows_core::HSTRING, friendlymessage: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn CheckGamingPrivilegeWithUIForUser(user : * mut core::ffi::c_void, privilegeid : u32, scope : core::mem::MaybeUninit < windows_core::HSTRING >, policy : core::mem::MaybeUninit < windows_core::HSTRING >, friendlymessage : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    CheckGamingPrivilegeWithUIForUser(user.param().abi(), privilegeid, core::mem::transmute_copy(scope), core::mem::transmute_copy(policy), core::mem::transmute_copy(friendlymessage), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn GetExpandedResourceExclusiveCpuCount() -> windows_core::Result<u32> {
    windows_targets::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetExpandedResourceExclusiveCpuCount(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetGamingDeviceModelInformation() -> windows_core::Result<GAMING_DEVICE_MODEL_INFORMATION> {
    windows_targets::link!("api-ms-win-gaming-deviceinformation-l1-1-0.dll" "system" fn GetGamingDeviceModelInformation(information : *mut GAMING_DEVICE_MODEL_INFORMATION) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetGamingDeviceModelInformation(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn HasExpandedResources() -> windows_core::Result<super::Foundation::BOOL> {
    windows_targets::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn HasExpandedResources(hasexpandedresources : *mut super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    HasExpandedResources(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn ProcessPendingGameUI<P0>(waitforcompletion: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Foundation::BOOL>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ProcessPendingGameUI(waitforcompletion : super::Foundation:: BOOL) -> windows_core::HRESULT);
    ProcessPendingGameUI(waitforcompletion.param().abi()).ok()
}
#[inline]
pub unsafe fn ReleaseExclusiveCpuSets() -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-expandedresources-l1-1-0.dll" "system" fn ReleaseExclusiveCpuSets() -> windows_core::HRESULT);
    ReleaseExclusiveCpuSets().ok()
}
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUI(targetuserxuid: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowChangeFriendRelationshipUI(targetuserxuid : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowChangeFriendRelationshipUI(core::mem::transmute_copy(targetuserxuid), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUIForUser<P0>(user: P0, targetuserxuid: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowChangeFriendRelationshipUIForUser(user : * mut core::ffi::c_void, targetuserxuid : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowChangeFriendRelationshipUIForUser(user.param().abi(), core::mem::transmute_copy(targetuserxuid), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowCustomizeUserProfileUI(completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowCustomizeUserProfileUI(completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowCustomizeUserProfileUIForUser<P0>(user: P0, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowCustomizeUserProfileUIForUser(user : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowCustomizeUserProfileUIForUser(user.param().abi(), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowFindFriendsUI(completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowFindFriendsUI(completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowFindFriendsUIForUser<P0>(user: P0, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowFindFriendsUIForUser(user : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowFindFriendsUIForUser(user.param().abi(), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowGameInfoUI(titleid : u32, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowGameInfoUI(titleid, completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowGameInfoUIForUser<P0>(user: P0, titleid: u32, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowGameInfoUIForUser(user : * mut core::ffi::c_void, titleid : u32, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowGameInfoUIForUser(user.param().abi(), titleid, completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowGameInviteUI(serviceconfigurationid: &windows_core::HSTRING, sessiontemplatename: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, invitationdisplaytext: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowGameInviteUI(serviceconfigurationid : core::mem::MaybeUninit < windows_core::HSTRING >, sessiontemplatename : core::mem::MaybeUninit < windows_core::HSTRING >, sessionid : core::mem::MaybeUninit < windows_core::HSTRING >, invitationdisplaytext : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowGameInviteUI(core::mem::transmute_copy(serviceconfigurationid), core::mem::transmute_copy(sessiontemplatename), core::mem::transmute_copy(sessionid), core::mem::transmute_copy(invitationdisplaytext), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowGameInviteUIForUser<P0>(user: P0, serviceconfigurationid: &windows_core::HSTRING, sessiontemplatename: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, invitationdisplaytext: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowGameInviteUIForUser(user : * mut core::ffi::c_void, serviceconfigurationid : core::mem::MaybeUninit < windows_core::HSTRING >, sessiontemplatename : core::mem::MaybeUninit < windows_core::HSTRING >, sessionid : core::mem::MaybeUninit < windows_core::HSTRING >, invitationdisplaytext : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowGameInviteUIForUser(user.param().abi(), core::mem::transmute_copy(serviceconfigurationid), core::mem::transmute_copy(sessiontemplatename), core::mem::transmute_copy(sessionid), core::mem::transmute_copy(invitationdisplaytext), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowGameInviteUIWithContext(serviceconfigurationid: &windows_core::HSTRING, sessiontemplatename: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, invitationdisplaytext: &windows_core::HSTRING, customactivationcontext: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-3.dll" "system" fn ShowGameInviteUIWithContext(serviceconfigurationid : core::mem::MaybeUninit < windows_core::HSTRING >, sessiontemplatename : core::mem::MaybeUninit < windows_core::HSTRING >, sessionid : core::mem::MaybeUninit < windows_core::HSTRING >, invitationdisplaytext : core::mem::MaybeUninit < windows_core::HSTRING >, customactivationcontext : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowGameInviteUIWithContext(core::mem::transmute_copy(serviceconfigurationid), core::mem::transmute_copy(sessiontemplatename), core::mem::transmute_copy(sessionid), core::mem::transmute_copy(invitationdisplaytext), core::mem::transmute_copy(customactivationcontext), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowGameInviteUIWithContextForUser<P0>(user: P0, serviceconfigurationid: &windows_core::HSTRING, sessiontemplatename: &windows_core::HSTRING, sessionid: &windows_core::HSTRING, invitationdisplaytext: &windows_core::HSTRING, customactivationcontext: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-3.dll" "system" fn ShowGameInviteUIWithContextForUser(user : * mut core::ffi::c_void, serviceconfigurationid : core::mem::MaybeUninit < windows_core::HSTRING >, sessiontemplatename : core::mem::MaybeUninit < windows_core::HSTRING >, sessionid : core::mem::MaybeUninit < windows_core::HSTRING >, invitationdisplaytext : core::mem::MaybeUninit < windows_core::HSTRING >, customactivationcontext : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowGameInviteUIWithContextForUser(user.param().abi(), core::mem::transmute_copy(serviceconfigurationid), core::mem::transmute_copy(sessiontemplatename), core::mem::transmute_copy(sessionid), core::mem::transmute_copy(invitationdisplaytext), core::mem::transmute_copy(customactivationcontext), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowPlayerPickerUI(promptdisplaytext: &windows_core::HSTRING, xuids: &[windows_core::HSTRING], preselectedxuids: Option<&[windows_core::HSTRING]>, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowPlayerPickerUI(promptdisplaytext : core::mem::MaybeUninit < windows_core::HSTRING >, xuids : *const core::mem::MaybeUninit < windows_core::HSTRING >, xuidscount : usize, preselectedxuids : *const core::mem::MaybeUninit < windows_core::HSTRING >, preselectedxuidscount : usize, minselectioncount : usize, maxselectioncount : usize, completionroutine : PlayerPickerUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowPlayerPickerUI(core::mem::transmute_copy(promptdisplaytext), core::mem::transmute(xuids.as_ptr()), xuids.len().try_into().unwrap(), core::mem::transmute(preselectedxuids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), preselectedxuids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), minselectioncount, maxselectioncount, completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowPlayerPickerUIForUser<P0>(user: P0, promptdisplaytext: &windows_core::HSTRING, xuids: &[windows_core::HSTRING], preselectedxuids: Option<&[windows_core::HSTRING]>, minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowPlayerPickerUIForUser(user : * mut core::ffi::c_void, promptdisplaytext : core::mem::MaybeUninit < windows_core::HSTRING >, xuids : *const core::mem::MaybeUninit < windows_core::HSTRING >, xuidscount : usize, preselectedxuids : *const core::mem::MaybeUninit < windows_core::HSTRING >, preselectedxuidscount : usize, minselectioncount : usize, maxselectioncount : usize, completionroutine : PlayerPickerUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowPlayerPickerUIForUser(user.param().abi(), core::mem::transmute_copy(promptdisplaytext), core::mem::transmute(xuids.as_ptr()), xuids.len().try_into().unwrap(), core::mem::transmute(preselectedxuids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), preselectedxuids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), minselectioncount, maxselectioncount, completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowProfileCardUI(targetuserxuid: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowProfileCardUI(targetuserxuid : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowProfileCardUI(core::mem::transmute_copy(targetuserxuid), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowProfileCardUIForUser<P0>(user: P0, targetuserxuid: &windows_core::HSTRING, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowProfileCardUIForUser(user : * mut core::ffi::c_void, targetuserxuid : core::mem::MaybeUninit < windows_core::HSTRING >, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowProfileCardUIForUser(user.param().abi(), core::mem::transmute_copy(targetuserxuid), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn ShowTitleAchievementsUI(titleid : u32, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowTitleAchievementsUI(titleid, completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowTitleAchievementsUIForUser<P0>(user: P0, titleid: u32, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-2.dll" "system" fn ShowTitleAchievementsUIForUser(user : * mut core::ffi::c_void, titleid : u32, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowTitleAchievementsUIForUser(user.param().abi(), titleid, completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowUserSettingsUI(completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowUserSettingsUI(completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn ShowUserSettingsUIForUser<P0>(user: P0, completionroutine: GameUICompletionRoutine, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IInspectable>,
{
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-4.dll" "system" fn ShowUserSettingsUIForUser(user : * mut core::ffi::c_void, completionroutine : GameUICompletionRoutine, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    ShowUserSettingsUIForUser(user.param().abi(), completionroutine, core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn TryCancelPendingGameUI() -> super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-gaming-tcui-l1-1-0.dll" "system" fn TryCancelPendingGameUI() -> super::Foundation:: BOOL);
    TryCancelPendingGameUI()
}
windows_core::imp::define_interface!(IGameExplorer, IGameExplorer_Vtbl, 0xe7b2fb72_d728_49b3_a5f2_18ebf5f1349e);
impl core::ops::Deref for IGameExplorer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGameExplorer, windows_core::IUnknown);
impl IGameExplorer {
    pub unsafe fn AddGame<P0, P1>(&self, bstrgdfbinarypath: P0, bstrgameinstalldirectory: P1, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddGame)(windows_core::Interface::as_raw(self), bstrgdfbinarypath.param().abi(), bstrgameinstalldirectory.param().abi(), installscope, pguidinstanceid).ok()
    }
    pub unsafe fn RemoveGame(&self, guidinstanceid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveGame)(windows_core::Interface::as_raw(self), core::mem::transmute(guidinstanceid)).ok()
    }
    pub unsafe fn UpdateGame(&self, guidinstanceid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateGame)(windows_core::Interface::as_raw(self), core::mem::transmute(guidinstanceid)).ok()
    }
    pub unsafe fn VerifyAccess<P0>(&self, bstrgdfbinarypath: P0) -> windows_core::Result<super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VerifyAccess)(windows_core::Interface::as_raw(self), bstrgdfbinarypath.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IGameExplorer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddGame: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, GAME_INSTALL_SCOPE, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RemoveGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub UpdateGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub VerifyAccess: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameExplorer2, IGameExplorer2_Vtbl, 0x86874aa7_a1ed_450d_a7eb_b89e20b2fff3);
impl core::ops::Deref for IGameExplorer2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGameExplorer2, windows_core::IUnknown);
impl IGameExplorer2 {
    pub unsafe fn InstallGame<P0, P1>(&self, binarygdfpath: P0, installdirectory: P1, installscope: GAME_INSTALL_SCOPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InstallGame)(windows_core::Interface::as_raw(self), binarygdfpath.param().abi(), installdirectory.param().abi(), installscope).ok()
    }
    pub unsafe fn UninstallGame<P0>(&self, binarygdfpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UninstallGame)(windows_core::Interface::as_raw(self), binarygdfpath.param().abi()).ok()
    }
    pub unsafe fn CheckAccess<P0>(&self, binarygdfpath: P0) -> windows_core::Result<super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckAccess)(windows_core::Interface::as_raw(self), binarygdfpath.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IGameExplorer2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InstallGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, GAME_INSTALL_SCOPE) -> windows_core::HRESULT,
    pub UninstallGame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameStatistics, IGameStatistics_Vtbl, 0x3887c9ca_04a0_42ae_bc4c_5fa6c7721145);
impl core::ops::Deref for IGameStatistics {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGameStatistics, windows_core::IUnknown);
impl IGameStatistics {
    pub unsafe fn GetMaxCategoryLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxCategoryLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxNameLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxNameLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxValueLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxValueLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxCategories(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxCategories)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxStatsPerCategory(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxStatsPerCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCategoryTitle<P0>(&self, categoryindex: u16, title: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCategoryTitle)(windows_core::Interface::as_raw(self), categoryindex, title.param().abi()).ok()
    }
    pub unsafe fn GetCategoryTitle(&self, categoryindex: u16) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCategoryTitle)(windows_core::Interface::as_raw(self), categoryindex, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: Option<*mut windows_core::PWSTR>, pvalue: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatistic)(windows_core::Interface::as_raw(self), categoryindex, statindex, core::mem::transmute(pname.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pvalue.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetStatistic<P0, P1>(&self, categoryindex: u16, statindex: u16, name: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetStatistic)(windows_core::Interface::as_raw(self), categoryindex, statindex, name.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn Save<P0>(&self, trackchanges: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), trackchanges.param().abi()).ok()
    }
    pub unsafe fn SetLastPlayedCategory(&self, categoryindex: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLastPlayedCategory)(windows_core::Interface::as_raw(self), categoryindex).ok()
    }
    pub unsafe fn GetLastPlayedCategory(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastPlayedCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetLastPlayedCategory: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetLastPlayedCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameStatisticsMgr, IGameStatisticsMgr_Vtbl, 0xaff3ea11_e70e_407d_95dd_35e612c41ce2);
impl core::ops::Deref for IGameStatisticsMgr {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGameStatisticsMgr, windows_core::IUnknown);
impl IGameStatisticsMgr {
    pub unsafe fn GetGameStatistics<P0>(&self, gdfbinarypath: P0, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut Option<IGameStatistics>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetGameStatistics)(windows_core::Interface::as_raw(self), gdfbinarypath.param().abi(), opentype, popenresult, core::mem::transmute(ppistats)).ok()
    }
    pub unsafe fn RemoveGameStatistics<P0>(&self, gdfbinarypath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveGameStatistics)(windows_core::Interface::as_raw(self), gdfbinarypath.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IGameStatisticsMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, GAMESTATS_OPEN_TYPE, *mut GAMESTATS_OPEN_RESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveGameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXblIdpAuthManager, IXblIdpAuthManager_Vtbl, 0xeb5ddb08_8bbf_449b_ac21_b02ddeb3b136);
impl core::ops::Deref for IXblIdpAuthManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXblIdpAuthManager, windows_core::IUnknown);
impl IXblIdpAuthManager {
    pub unsafe fn SetGamerAccount<P0, P1>(&self, msaaccountid: P0, xuid: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetGamerAccount)(windows_core::Interface::as_raw(self), msaaccountid.param().abi(), xuid.param().abi()).ok()
    }
    pub unsafe fn GetGamerAccount(&self, msaaccountid: *mut windows_core::PWSTR, xuid: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGamerAccount)(windows_core::Interface::as_raw(self), msaaccountid, xuid).ok()
    }
    pub unsafe fn SetAppViewInitialized<P0, P1>(&self, appsid: P0, msaaccountid: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAppViewInitialized)(windows_core::Interface::as_raw(self), appsid.param().abi(), msaaccountid.param().abi()).ok()
    }
    pub unsafe fn GetEnvironment(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnvironment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSandbox(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSandbox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTokenAndSignatureWithTokenResult<P0, P1, P2, P3, P4, P5, P6, P7>(&self, msaaccountid: P0, appsid: P1, msatarget: P2, msapolicy: P3, httpmethod: P4, uri: P5, headers: P6, body: &[u8], forcerefresh: P7) -> windows_core::Result<IXblIdpAuthTokenResult>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTokenAndSignatureWithTokenResult)(windows_core::Interface::as_raw(self), msaaccountid.param().abi(), appsid.param().abi(), msatarget.param().abi(), msapolicy.param().abi(), httpmethod.param().abi(), uri.param().abi(), headers.param().abi(), core::mem::transmute(body.as_ptr()), body.len().try_into().unwrap(), forcerefresh.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXblIdpAuthManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGamerAccount: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetGamerAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAppViewInitialized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetTokenAndSignatureWithTokenResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXblIdpAuthManager2, IXblIdpAuthManager2_Vtbl, 0xbf8c0950_8389_43dd_9a76_a19728ec5dc5);
impl core::ops::Deref for IXblIdpAuthManager2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXblIdpAuthManager2, windows_core::IUnknown);
impl IXblIdpAuthManager2 {
    pub unsafe fn GetUserlessTokenAndSignatureWithTokenResult<P0, P1, P2, P3, P4, P5, P6>(&self, appsid: P0, msatarget: P1, msapolicy: P2, httpmethod: P3, uri: P4, headers: P5, body: &[u8], forcerefresh: P6) -> windows_core::Result<IXblIdpAuthTokenResult>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUserlessTokenAndSignatureWithTokenResult)(windows_core::Interface::as_raw(self), appsid.param().abi(), msatarget.param().abi(), msapolicy.param().abi(), httpmethod.param().abi(), uri.param().abi(), headers.param().abi(), core::mem::transmute(body.as_ptr()), body.len().try_into().unwrap(), forcerefresh.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IXblIdpAuthManager2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUserlessTokenAndSignatureWithTokenResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXblIdpAuthTokenResult, IXblIdpAuthTokenResult_Vtbl, 0x46ce0225_f267_4d68_b299_b2762552dec1);
impl core::ops::Deref for IXblIdpAuthTokenResult {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXblIdpAuthTokenResult, windows_core::IUnknown);
impl IXblIdpAuthTokenResult {
    pub unsafe fn GetStatus(&self) -> windows_core::Result<XBL_IDP_AUTH_TOKEN_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetToken(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetToken)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSignature(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignature)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSandbox(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSandbox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEnvironment(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnvironment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMsaAccountId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMsaAccountId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetXuid(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGamertag(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGamertag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAgeGroup(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAgeGroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPrivileges(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrivileges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMsaTarget(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMsaTarget)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMsaPolicy(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMsaPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMsaAppId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMsaAppId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRedirect(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRedirect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMessage(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHelpId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHelpId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEnforcementBans(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnforcementBans)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRestrictions(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRestrictions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTitleRestrictions(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTitleRestrictions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IXblIdpAuthTokenResult2, IXblIdpAuthTokenResult2_Vtbl, 0x75d760b0_60b9_412d_994f_26b2cd5f7812);
impl core::ops::Deref for IXblIdpAuthTokenResult2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXblIdpAuthTokenResult2, windows_core::IUnknown);
impl IXblIdpAuthTokenResult2 {
    pub unsafe fn GetModernGamertag(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetModernGamertag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetModernGamertagSuffix(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetModernGamertagSuffix)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetUniqueModernGamertag(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUniqueModernGamertag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IXblIdpAuthTokenResult2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetModernGamertag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetModernGamertagSuffix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetUniqueModernGamertag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(0i32);
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(1i32);
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(1i32);
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(0i32);
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(0i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1988865574i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(712204761i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1523980231i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(284675555i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_SERIES_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(489159355i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_SERIES_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(796540415i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_SERIES_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(-561359263i32);
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(-1024700366i32);
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(0i32);
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(3i32);
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(2i32);
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(1i32);
pub const ID_GDF_THUMBNAIL_STR: windows_core::PCWSTR = windows_core::w!("__GDF_THUMBNAIL");
pub const ID_GDF_XML_STR: windows_core::PCWSTR = windows_core::w!("__GDF_XML");
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GAMESTATS_OPEN_RESULT(pub i32);
impl windows_core::TypeKind for GAMESTATS_OPEN_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GAMESTATS_OPEN_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GAMESTATS_OPEN_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GAMESTATS_OPEN_TYPE(pub i32);
impl windows_core::TypeKind for GAMESTATS_OPEN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GAMESTATS_OPEN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GAMESTATS_OPEN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GAME_INSTALL_SCOPE(pub i32);
impl windows_core::TypeKind for GAME_INSTALL_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GAME_INSTALL_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GAME_INSTALL_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GAMING_DEVICE_DEVICE_ID(pub i32);
impl windows_core::TypeKind for GAMING_DEVICE_DEVICE_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GAMING_DEVICE_DEVICE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GAMING_DEVICE_DEVICE_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GAMING_DEVICE_VENDOR_ID(pub i32);
impl windows_core::TypeKind for GAMING_DEVICE_VENDOR_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GAMING_DEVICE_VENDOR_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GAMING_DEVICE_VENDOR_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KnownGamingPrivileges(pub i32);
impl windows_core::TypeKind for KnownGamingPrivileges {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KnownGamingPrivileges {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KnownGamingPrivileges").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XBL_IDP_AUTH_TOKEN_STATUS(pub i32);
impl windows_core::TypeKind for XBL_IDP_AUTH_TOKEN_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XBL_IDP_AUTH_TOKEN_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XBL_IDP_AUTH_TOKEN_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
impl windows_core::TypeKind for GAMING_DEVICE_MODEL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for GAMING_DEVICE_MODEL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GameExplorer: windows_core::GUID = windows_core::GUID::from_u128(0x9a5ea990_3034_4d6f_9128_01f3c61022bc);
pub const GameStatistics: windows_core::GUID = windows_core::GUID::from_u128(0xdbc85a2c_c0dc_4961_b6e2_d28b62c11ad4);
pub const XblIdpAuthManager: windows_core::GUID = windows_core::GUID::from_u128(0xce23534b_56d8_4978_86a2_7ee570640468);
pub const XblIdpAuthTokenResult: windows_core::GUID = windows_core::GUID::from_u128(0x9f493441_744a_410c_ae2b_9a22f7c7731f);
pub type GameUICompletionRoutine = Option<unsafe extern "system" fn(returncode: windows_core::HRESULT, context: *const core::ffi::c_void)>;
pub type PlayerPickerUICompletionRoutine = Option<unsafe extern "system" fn(returncode: windows_core::HRESULT, context: *const core::ffi::c_void, selectedxuids: *const windows_core::HSTRING, selectedxuidscount: usize)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");

# Simplified Chinese (zh-CN) catalog for the collaboration UI (collab panel,
# channels, calls, notifications and call diagnostics).
#
# Product names (Zed, GitHub, WebRTC), protocol terms (CLA, ID) and URLs are
# deliberately left untranslated. Keys already defined by another catalog
# (cancel and open-action in common.ftl; close, dismiss, untitled,
# failed-to-join-channel and failed-to-join-project in workspace.ftl; remove in
# recent_projects.ftl; rename in project_panel.ftl; delete in git_ui.ftl;
# network and copy-link in settings_ui.ftl; disconnected, leave-call,
# click-to-follow, follow-login, signing-in, excellent, good, poor, lost,
# latency, jitter, packet-loss and input-lag in title_bar.ftl) are reused, not
# redefined here.

## Panel section headers

current-call = 当前通话
favorites = 收藏
requests = 请求
contacts = 联系人
channels = 频道
invites = 邀请
online = 在线
offline = 离线

## Call participants

calling = 正在呼叫…
guest = 访客
member = 成员
admin = 管理员
you = 你
invited = 已邀请
mic-only = 仅麦克风
mute = 静音
revoke-access = 撤销访问权限
grant-mic-access = 授予麦克风权限
grant-write-access = 授予写入权限
screen = 屏幕
open-shared-screen = 打开共享屏幕
open-name = 打开 { $name }

## Channel notes

notes = 笔记
open-notes = 打开笔记
open-channel-notes = 打开频道笔记
copy-link-to-section = 复制到此章节的链接
link-copied-to-clipboard = 已将链接复制到剪贴板

## Channel tab status

read-only = 只读
# Explicit key: `unknown` belongs to git_ui's "Unknown", and this placeholder
# keeps its angle brackets.
unknown-channel = <未知>

## Channels

join-channel = 加入频道
create-channel = 新建频道
new-subchannel = 新建子频道
expand-subchannels = 展开子频道
collapse-subchannels = 折叠子频道
move-this-channel = 移动此频道
move-channel-here = 将「#{ $channel }」移动到此处
make-channel-public = 设为公开频道
make-channel-private = 设为私有频道
public = 公开
copy-channel-link = 复制频道链接
copy-channel-notes-link = 复制频道笔记链接
add-to-favorites = 添加到收藏
remove-from-favorites = 从收藏中移除
leave-channel = 离开频道
search-channels = 搜索频道…
clear-filter = 清除筛选
show-all-channels = 显示所有频道
show-occupied-channels = 仅显示有人的频道

## Channel members

manage-members = 管理成员
invite-members = 邀请成员
promote-to-member = 提升为成员
promote-to-admin = 提升为管理员
demote-to-member = 降级为成员
demote-to-guest = 降级为访客
remove-from-channel = 从频道中移除

## Contacts

add-a-contact = 添加联系人
remove-contact = 移除联系人
search-for-new-contact = 搜索新联系人
search-collaborator-by-username = 按用户名搜索协作者…
invite-new-contacts = 邀请新联系人
accept-invite = 接受邀请
decline-invite = 拒绝邀请
cancel-invite = 取消邀请
call-login = 呼叫 { $login }
invite-login-to-join = 邀请 { $login } 加入
invite-login-to-join-call = 邀请 { $login } 加入通话
login-is-offline = { $login } 当前离线
login-is-on-a-call = { $login } 正在通话中

## Notifications

accept = 接受
decline = 拒绝
login-wants-to-add-you-as-a-contact = { $login } 想把你添加为联系人
login-accepted-your-contact-request = { $login } 已接受你的联系人请求
login-invited-you-to-join-the-channel-channel = { $login } 邀请你加入 #{ $channel } 频道
login-is-sharing-a-project-in-zed = { $login } 正在 Zed 中共享一个项目
login-is-sharing-a-project-with-you = { $login } 正在与你共享项目

## Confirmation prompts

leave = 离开
are-you-sure-you-want-to-leave-channel = 确定要离开「#{ $channel }」吗？
are-you-sure-you-want-to-remove-the-channel-channel = 确定要删除频道「{ $channel }」吗？
are-you-sure-you-want-to-remove-login-from-your-contacts = 确定要将「{ $login }」从联系人中移除吗？

## Sign-in and empty states

work-with-your-team-in-realtime-with-collaborative-editing-voice-shared-notes-and-more = 与团队实时协作：协同编辑、语音通话、共享笔记等等。
connect = 连接
connecting = 正在连接…
sign-in-with-github = 使用 GitHub 登录
collaboration-is-disabled-for-this-organization = 你所在的组织已停用协作功能。

## Call actions

auto-watch-screens = 自动观看屏幕
auto-watch-screens-paused-while-sharing = 自动观看屏幕（共享期间已暂停）
stop-auto-watching-screens = 停止自动观看屏幕
room-id-copied-to-clipboard = 已将房间 ID 复制到剪贴板
there-s-no-active-call-join-one-first = 当前没有进行中的通话，请先加入一个。

## Call diagnostics

call-diagnostics = 通话诊断
showing-diagnostics-from-the-most-recent-call = 显示最近一次通话的诊断信息
no-call-diagnostics-available = 暂无通话诊断信息
samples-samples-retained-s-retained-intervals-affected-intervals-in-the-last-60s = { $samples } 个样本 · 已保留 { $retained } 秒 · 最近 60 秒内有 { $intervals } 个受影响的区间
time-for-data-to-travel-to-the-server = 数据传输到服务器所需的时间
variance-or-fluctuation-in-latency = 延迟的波动幅度
amount-of-data-lost-during-transfer = 传输过程中丢失的数据量
delay-from-audio-capture-to-webrtc = 从音频采集到 WebRTC 的延迟
normal = 正常
high = 偏高
inbound-audio = 入向音频
waiting-for-inbound-audio-statistics = 正在等待入向音频统计数据
affected = 受影响
healthy = 正常
loss-loss-jitter-jitter-ms-jitter-buffer-buffer = 丢包 { $loss } · 抖动 { $jitter }ms · 抖动缓冲 { $buffer }
webrtc-repaired-duration-in-count-event = WebRTC 在 { $count } 次事件中修复了 { $duration }
webrtc-repaired-duration-in-count-events = WebRTC 在 { $count } 次事件中修复了 { $duration }
local-playback-starved-for-starved-dropped-dropped-buffered-buffered-peak-peak = 本地播放欠载 { $starved } · 丢弃 { $dropped } · 缓冲 { $buffered }（峰值 { $peak }）
copy-report = 复制报告
save-report = 保存报告…

## Errors

failed-to-hang-up = 挂断失败
call-failed = 呼叫失败
failed-to-create-channel = 创建频道失败
failed-to-leave-channel = 离开频道失败
failed-to-move-channel = 移动频道失败
failed-to-move-channel-up = 上移频道失败
failed-to-move-channel-down = 下移频道失败
failed-to-set-channel-visibility = 设置频道可见性失败
failed-to-grant-mic-access = 授予麦克风权限失败
failed-to-grant-write-access = 授予写入权限失败
failed-to-revoke-access = 撤销访问权限失败
failed-to-update-role = 更新角色失败
failed-to-invite-member = 邀请成员失败
failed-to-remove-member = 移除成员失败
failed-to-remove-contact = 移除联系人失败
failed-to-respond-to-contact-request = 响应联系人请求失败
public-channels-must-have-public-parents = 公开频道的父频道也必须是公开的
you-cannot-move-a-channel-into-itself = 不能将频道移动到其自身内部
you-cannot-move-a-channel-into-a-different-root-channel = 不能将频道移动到另一个根频道下
to-make-a-channel-public-its-parent-channel-must-be-public = 要将频道设为公开，其父频道必须是公开的。
to-make-a-channel-private-all-of-its-subchannels-must-be-private = 要将频道设为私有，其所有子频道都必须是私有的。
this-user-has-not-yet-signed-the-cla-at-https-zed-dev-cla = 该用户尚未在 https://zed.dev/cla 签署 CLA。

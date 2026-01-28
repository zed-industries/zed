# CollabOverlay Implementation Plan

## Overview

This document outlines the plan to consolidate all active call/voice functionality into the `CollabOverlay` component. This includes moving participant display, call controls, and following functionality from the title bar and collab panel into a unified overlay that appears below whichever dock contains the `CollabPanel`.

## Current State

### What Exists Today
- **Title Bar** (`crates/title_bar/src/collab.rs`): Renders participant avatars with mute/speaking indicators, following facepile, and call controls (mute, deafen, screen share, leave)
- **Collab Panel** (`crates/collab_ui/src/collab_panel.rs`): Shows `ActiveCall` section with participants, their shared projects, and shared screens
- **CollabOverlay UI Components** (`crates/ui/src/components/collab/`): Pure presentation/layout components with no business logic - these serve as the **visual skeleton/reference**

### What Will Change
- Title bar will have **no call-related UI** during active calls
- CollabOverlay becomes the **single source of truth** for call UI
- A **micro version** appears in the status bar when all docks are closed

---

## Existing Code to Reuse

This section documents existing functions, patterns, and logic that should be **reused directly** rather than recreated. The goal is to give existing functionality a new home in the CollabOverlay UI.

### Call Control Functions (title_bar/src/collab.rs)

These standalone functions can be called directly from the new overlay:

| Function | Location | Purpose |
|----------|----------|---------|
| `toggle_screen_sharing(screen, window, cx)` | L25-79 | Handles screen share toggle with telemetry, error handling |
| `toggle_mute(cx)` | L81-99 | Toggles microphone mute via `Room::toggle_mute()` with telemetry |
| `toggle_deafen(cx)` | L101-105 | Toggles audio output via `Room::toggle_deafen()` |

**Usage pattern:**
```rust
// These are already pub functions, just call them:
use title_bar::collab::{toggle_mute, toggle_deafen, toggle_screen_sharing};

// In button handler:
.on_click(move |_, _, cx| toggle_mute(cx))
```

### Leave Call Pattern

From `collab_panel.rs` L981-984 and `title_bar/collab.rs` L364-368:
```rust
ActiveCall::global(cx)
    .update(cx, |call, cx| call.hang_up(cx))
    .detach_and_log_err(cx);
```

### Follow/Unfollow Pattern

From `collab_panel.rs` L1001-1004 and `title_bar/collab.rs` L220-226:
```rust
// Follow
workspace.update(cx, |workspace, cx| workspace.follow(peer_id, window, cx)).ok();

// Unfollow
workspace.update(cx, |workspace, cx| workspace.unfollow(peer_id, window, cx)).ok();

// Check if following
let is_following = workspace.read(cx).is_being_followed(peer_id);
```

### Open Channel Notes

From `collab_panel.rs` L2027-2035:
```rust
fn open_channel_notes(channel_id: ChannelId, workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut App) {
    if let Some(workspace) = workspace.upgrade() {
        ChannelView::open(channel_id, None, workspace, window, cx).detach();
    }
}
```

### Join Shared Project

From `collab_panel.rs` L1035-1046:
```rust
workspace.update(cx, |workspace, cx| {
    let app_state = workspace.app_state().clone();
    workspace::join_in_room_project(project_id, host_user_id, app_state, cx)
        .detach_and_prompt_err("Failed to join project", window, cx, |_, _, _| None);
}).ok();
```

### Open Shared Screen

From `collab_panel.rs` L1076-1079:
```rust
workspace.update(cx, |workspace, cx| {
    workspace.open_shared_screen(peer_id, window, cx)
}).ok();
```

### Accessing Room State

From `title_bar/collab.rs` L137-165 - pattern for getting room and participant data:
```rust
let room = ActiveCall::global(cx).read(cx).room().cloned();
if let Some(room) = room {
    let room = room.read(cx);

    // Local participant
    let local = room.local_participant();
    let is_muted = room.is_muted();
    let is_speaking = room.is_speaking();
    let is_deafened = room.is_deafened().unwrap_or(false);
    let is_screen_sharing = room.is_sharing_screen();
    let can_use_microphone = room.can_use_microphone();

    // Remote participants (sorted by index)
    let mut remote_participants = room.remote_participants().values().collect::<Vec<_>>();
    remote_participants.sort_by_key(|p| p.participant_index.0);

    for participant in remote_participants {
        let user = &participant.user;
        let peer_id = participant.peer_id;
        let is_muted = participant.muted;
        let is_speaking = participant.speaking;
        let has_screen_share = participant.has_video_tracks();
        let projects = &participant.projects;
        let role = participant.role;
    }

    // Channel info
    let channel_id = room.channel_id();
    let channel_name = /* get from ChannelStore */;
}
```

### Getting Current User

From `collab_panel.rs` L517:
```rust
let current_user = self.user_store.read(cx).current_user();
```

### Player Colors for Following Indicator

From `title_bar/collab.rs` L153-156, L188-194:
```rust
let player_colors = cx.theme().players();

// Local user color
let local_color = player_colors.local().cursor;

// Remote participant color (for following highlight)
let player_color = player_colors.color_for_participant(participant.participant_index.0);
let selection_color = player_color.selection; // Use for following background
```

### Role Checking

From `collab_panel.rs` L968-970:
```rust
let is_call_admin = ActiveCall::global(cx).read(cx).room().is_some_and(|room| {
    room.read(cx).local_participant().role == proto::ChannelRole::Admin
});

// Check if user is guest (to hide from collaborator list)
if room.role_for_user(user.id) == Some(proto::ChannelRole::Guest) {
    // Don't show in main list
}
```

### Avatar with Mute Indicator

From `title_bar/collab.rs` L271-291:
```rust
Avatar::new(user.avatar_uri.clone())
    .grayscale(!is_present)  // Gray out if not in same project
    .border_color(if is_speaking {
        cx.theme().status().info
    } else {
        gpui::transparent_black()
    })
    .when(is_muted, |avatar| {
        avatar.indicator(
            AvatarAudioStatusIndicator::new(ui::AudioStatus::Muted)
                .tooltip(Tooltip::text(format!("{} is muted", user.github_login)))
        )
    })
```

### Facepile for Followers

From `title_bar/collab.rs` L256-315 - showing who is following a participant:
```rust
const FACEPILE_LIMIT: usize = 3;
let followers = project_id.map_or(&[] as &[_], |id| room.followers_for(peer_id, id));
let extra_count = followers.len().saturating_sub(FACEPILE_LIMIT);

Facepile::empty()
    .child(/* main avatar */)
    .children(followers.iter().take(FACEPILE_LIMIT).filter_map(|follower_peer_id| {
        // Get follower user and render small avatar
    }))
    .children(extra_count > 0).then(|| Label::new(format!("+{extra_count}")))
```

### Tooltip Patterns

```rust
// Simple tooltip
.tooltip(Tooltip::text("Leave Call"))

// Tooltip with meta info
.tooltip(move |_window, cx| {
    if is_muted {
        if is_deafened {
            Tooltip::with_meta("Unmute Microphone", None, "Audio will be unmuted", cx)
        } else {
            Tooltip::simple("Unmute Microphone", cx)
        }
    } else {
        Tooltip::simple("Mute Microphone", cx)
    }
})
```

### Icon Button Toggle States

From `title_bar/collab.rs` L418-449:
```rust
IconButton::new("mute-microphone", if is_muted { IconName::MicMute } else { IconName::Mic })
    .style(ButtonStyle::Subtle)
    .icon_size(IconSize::Small)
    .toggle_state(is_muted)
    .selected_style(ButtonStyle::Tinted(TintColor::Error))  // Red when muted
    .on_click(move |_, _window, cx| toggle_mute(cx))

// For screen share (accent color when active)
.selected_style(ButtonStyle::Tinted(TintColor::Accent))
```

---

## Architecture

### Component Layers

We maintain a clear separation between **presentation** (ui crate) and **business logic** (collab_ui crate):

```
┌─────────────────────────────────────────────────────────────────┐
│                        collab_ui crate                          │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  CollabOverlayPanel (Entity)                              │  │
│  │  - Subscribes to ActiveCall/Room events                   │  │
│  │  - Manages collapsed/expanded state                       │  │
│  │  - Wires up action callbacks (mute, follow, leave, etc.)  │  │
│  │  - Composes UI components with real data                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              │ uses                             │
│                              ▼                                  │
├─────────────────────────────────────────────────────────────────┤
│                          ui crate                               │
│  ┌─────────────────┐ ┌─────────────────┐ ┌──────────────────┐   │
│  │ CollabOverlay   │ │CollabOverlay    │ │ CollabOverlay    │   │
│  │ (layout)        │ │Header           │ │ Controls         │   │
│  └─────────────────┘ └─────────────────┘ └──────────────────┘   │
│  ┌─────────────────┐ ┌─────────────────┐ ┌──────────────────┐   │
│  │ ParticipantItem │ │ParticipantProj  │ │ ParticipantScreen│   │
│  └─────────────────┘ └─────────────────┘ └──────────────────┘   │
│                                                                 │
│  These are pure RenderOnce components - visual skeleton only    │
│  No dependencies on call, workspace, or other business crates   │
└─────────────────────────────────────────────────────────────────┘
```

### Workspace Layout

```
┌─────────────────────────────────────────────────────────────────┐
│                         Workspace                                │
├─────────────────────────────────────────────────────────────────┤
│  Title Bar (no call UI)                                         │
├──────────────┬──────────────────────────────────┬───────────────┤
│              │                                  │               │
│  Left Dock   │         Center Panes             │  Right Dock   │
│  ┌────────┐  │                                  │               │
│  │Project │  │                                  │               │
│  │Panel   │  │                                  │               │
│  ├────────┤  │                                  │               │
│  │Collab  │  │                                  │               │
│  │Overlay │◄─┼── appears below whichever dock   │               │
│  │Panel   │  │   contains the CollabPanel       │               │
│  └────────┘  │                                  │               │
├──────────────┴──────────────────────────────────┴───────────────┤
│  Status Bar                     [CollabOverlayMicro] │ ...  │   │
│  (micro version when docks closed)                              │
└─────────────────────────────────────────────────────────────────┘
```

---

## Implementation Phases

Each phase is designed to be **independently testable** - you can verify progress after completing each one.

---

### Phase 1: Workspace Integration with Placeholder
**Location:** `crates/workspace/src/workspace.rs`, `crates/collab_ui/src/collab_overlay_panel.rs`

Create the basic infrastructure and make something visible immediately.

**Tasks:**
- [*] Create minimal `CollabOverlayPanel` struct in `crates/collab_ui/src/collab_overlay_panel.rs`
- [*] Implement basic `Render` that shows a placeholder: "In call: {channel_name}"
- [*] Export from `crates/collab_ui/src/collab_ui.rs`
- [*] Update `workspace.rs` to create `CollabOverlayPanel` when `ActiveCall` has a room
- [*] Replace hardcoded `CollabOverlay` in `render_dock_with_collab_secondary` with real entity
- [*] Subscribe to `ActiveCall` to show/hide overlay when call starts/ends

```rust
// Minimal initial implementation
pub struct CollabOverlayPanel {
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

impl Render for CollabOverlayPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(room) = ActiveCall::global(cx).read(cx).room() else {
            return div();
        };
        let room = room.read(cx);
        let channel_name = room.channel_id()
            .and_then(|id| /* get channel name */)
            .unwrap_or("Call".into());

        div()
            .p_2()
            .bg(cx.theme().colors().panel_background)
            .child(format!("In call: {}", channel_name))
    }
}
```

**✅ How to Test:**
1. Join a channel call
2. Verify placeholder text appears below the dock containing CollabPanel
3. Leave call, verify placeholder disappears
4. Move CollabPanel to right dock, rejoin call, verify placeholder follows

---

### Phase 2: Display Real Participant Data (Read-Only)
**Location:** `crates/collab_ui/src/collab_overlay_panel.rs`, `crates/ui/src/components/collab/`

Show real participant information using the existing UI skeleton components. No actions yet - just display.

**Tasks:**
- [*] Subscribe to `Room` events for live updates
- [*] Read participant data from `Room::local_participant()` and `Room::remote_participants()`
- [*] Wire up `ParticipantItem` avatar prop (currently ignored)
- [*] Display muted/speaking states on avatars
- [*] Show channel name in header
- [*] Compose `CollabOverlay`, `CollabOverlayHeader`, `ParticipantItem`, `CollabOverlayControls`

```rust
impl Render for CollabOverlayPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(room) = ActiveCall::global(cx).read(cx).room() else {
            return div();
        };
        let room = room.read(cx);

        let participants = self.collect_participants(room, cx);

        CollabOverlay::new()
            .header(CollabOverlayHeader::new(channel_name).is_open(true))
            .children(participants.into_iter().map(|p| {
                ParticipantItem::new(&p.user.github_login)
                    .avatar(p.user.avatar_uri.clone())
                    .muted(p.muted)
                    .speaking(p.speaking)
                    .into_any_element()
            }).collect())
            .controls(CollabOverlayControls::new(current_user_avatar)
                .is_muted(room.is_muted())
                .is_deafened(room.is_deafened().unwrap_or(false))
                .is_screen_sharing(room.is_sharing_screen()))
    }
}
```

**✅ How to Test:**
1. Join a call with another participant
2. Verify both participants appear with correct names and avatars
3. Have other participant mute - verify mute indicator appears
4. Have other participant speak - verify speaking indicator appears
5. Verify your own mute/deafen/screen share states shown in controls

---

### Phase 3: Wire Up Control Actions
**Location:** `crates/collab_ui/src/collab_overlay_panel.rs`, `crates/ui/src/components/collab/collab_overlay_controls.rs`

Make the control buttons functional by wiring up callbacks.

**Tasks:**
- [*] Add callback props to `CollabOverlayControls`: `on_toggle_mute`, `on_toggle_deafen`, `on_toggle_screen_share`, `on_leave`
- [*] Wire buttons to callbacks in the UI component
- [*] In `CollabOverlayPanel`, pass callbacks that call existing functions:
  - `toggle_mute()` from `title_bar::collab`
  - `toggle_deafen()` from `title_bar::collab`
  - `toggle_screen_sharing()` from `title_bar::collab`
  - `ActiveCall::hang_up()` for leave

**✅ How to Test:**
1. Join a call
2. Click mute button - verify you become muted (check title bar indicator as reference)
3. Click deafen button - verify audio muted
4. Click screen share button - verify screen share starts
5. Click leave button - verify you leave the call and overlay disappears

---

### Phase 4: Header Features (Collapse + Channel Notes)
**Location:** `crates/ui/src/components/collab/collab_overlay_header.rs`, `crates/collab_ui/src/collab_overlay_panel.rs`

Add collapse/expand and channel notes navigation.

**Tasks:**
- [*] Add `collapsed` state to `CollabOverlayPanel`
- [*] Add `on_toggle` callback to `CollabOverlayHeader`
- [*] Make header clickable to toggle collapse
- [*] Add channel notes icon to header (left of channel name)
- [*] Add `on_channel_notes` callback
- [*] Wire up channel notes to open `ChannelView`

**✅ How to Test:**
1. Join a call
2. Click header - verify participant list collapses (only header + controls visible)
3. Click header again - verify list expands
4. Click channel notes icon - verify channel notes open

---

### Phase 5: Participant Actions (Follow, Projects, Screens)
**Location:** `crates/collab_ui/src/collab_overlay_panel.rs`, `crates/ui/src/components/collab/participant_item.rs`

Add interactivity to participant items.

**Tasks:**
- [ ] Add `on_click` callback to `ParticipantItem` for following
- [ ] Implement following indicator (colored background when following)
- [ ] Create `ParticipantProject` component with click handler
- [ ] Create `ParticipantScreen` component with click handler
- [ ] Show projects/screens under each participant
- [ ] Wire up:
  - Click participant → `Workspace::follow()` / `Workspace::unfollow()`
  - Click project → `workspace::join_in_room_project()`
  - Click screen → `Workspace::open_shared_screen()`

**✅ How to Test:**
1. Join a call with participant who has shared project
2. Click their name - verify you start following them (pane border changes)
3. Click again - verify you stop following
4. Click their shared project - verify you join it
5. Have them share screen, click it - verify screen opens

---

### Phase 6: Status Bar Micro Version
**Location:** `crates/collab_ui/src/collab_overlay_status_item.rs` (new)

Create condensed status bar view for when docks are closed.

**Design:**
```
┌─────────────────────────────────────────┐
│ [Avatar1][Avatar2][Avatar3] [🎤][🚪]   │
└─────────────────────────────────────────┘
```

**Tasks:**
- [ ] Create `CollabOverlayStatusItem` struct
- [ ] Implement `StatusItemView` trait
- [ ] Show max 3 avatars + "+N" overflow indicator
- [ ] Add mute and leave buttons
- [ ] Show only when: call active AND dock with CollabPanel is closed
- [ ] Click avatars → open the dock
- [ ] Register in `crates/zed/src/zed.rs`

**✅ How to Test:**
1. Join a call
2. Close all docks (or just the one with CollabPanel)
3. Verify micro version appears in status bar
4. Verify avatars show (up to 3)
5. Click mute in status bar - verify mute toggles
6. Click leave - verify you leave call
7. Click avatar area - verify dock opens with full overlay

---

### Phase 7: Remove Title Bar Call UI ✅
**Location:** `crates/title_bar/src/collab.rs`, `crates/title_bar/src/title_bar.rs`

Clean up the old UI now that the new one is fully functional.

**Tasks:**
- [x] Remove `render_collaborator_list()` call from title bar render
- [x] Remove `render_call_controls()` call from title bar render
- [x] Keep functions for now (used by overlay via imports)
- [ ] Update any title bar tests

**✅ How to Test:**
1. Join a call
2. Verify title bar has NO avatars or call controls
3. Verify all functionality still works via overlay and status bar
4. Test with multiple participants

---

### Phase 8: Clean Up Collab Panel ✅
**Location:** `crates/collab_ui/src/collab_panel.rs`

Remove duplicate UI from collab panel.

**Tasks:**
- [x] Remove `ListEntry::CallParticipant` rendering
- [x] Remove `ListEntry::ParticipantProject` rendering
- [x] Remove `ListEntry::ParticipantScreen` rendering
- [x] Simplify or remove `Section::ActiveCall` (kept header with channel name/notes, removed participant entries)
- [x] Keep channel list, contacts, etc.

**✅ How to Test:**
1. Join a call
2. Verify collab panel does NOT show participant list (overlay shows it)
3. Verify no duplicate information between panel and overlay
4. Verify channels/contacts still work in panel

---

## Data Flow

```
ActiveCall (global)
    │
    ├──► CollabOverlayPanel (Entity in collab_ui)
    │        │
    │        │ composes UI components from ui crate:
    │        │
    │        ├──► CollabOverlayHeader
    │        │        └── channel name, notes icon, toggle
    │        │
    │        ├──► ParticipantItem (for each participant)
    │        │        ├── avatar, name, mute/speaking state
    │        │        ├── ParticipantProject (for each shared project)
    │        │        └── ParticipantScreen (if screen sharing)
    │        │
    │        └──► CollabOverlayControls
    │                 └── local user avatar + action buttons
    │
    └──► CollabOverlayStatusItem (StatusBar)
             └── condensed view when docks closed
```

---

## File Changes Summary

### New Files
| File | Description |
|------|-------------|
| `crates/collab_ui/src/collab_overlay_panel.rs` | Main Entity - manages state, composes UI |
| `crates/collab_ui/src/collab_overlay_status_item.rs` | Status bar micro version |
| `crates/ui/src/components/collab/participant_project.rs` | Shared project item component |
| `crates/ui/src/components/collab/participant_screen.rs` | Screen share item component |

### Modified Files
| File | Changes |
|------|---------|
| `crates/ui/src/components/collab/collab_overlay.rs` | Keep as layout skeleton, may add slots |
| `crates/ui/src/components/collab/collab_overlay_header.rs` | Add callbacks, notes icon |
| `crates/ui/src/components/collab/collab_overlay_controls.rs` | Add state props, callbacks |
| `crates/ui/src/components/collab/participant_item.rs` | Wire up all props, support children |
| `crates/ui/src/components/collab/mod.rs` | Export new components |
| `crates/workspace/src/workspace.rs` | Integrate real overlay entity |
| `crates/title_bar/src/collab.rs` | Remove call UI rendering |
| `crates/title_bar/src/title_bar.rs` | Remove call UI from render |
| `crates/collab_ui/src/collab_panel.rs` | Remove participant rendering |
| `crates/collab_ui/src/collab_ui.rs` | Export new modules |
| `crates/zed/src/zed.rs` | Register status bar item |

---

## Testing Plan

### Manual Testing
- [ ] Join a call and verify overlay appears below correct dock
- [ ] Move CollabPanel to right dock, verify overlay follows
- [ ] Toggle collapse/expand on overlay header
- [ ] Click channel notes icon, verify navigation
- [ ] Test all control buttons (mute, deafen, screen share, leave)
- [ ] Click participant to follow, verify following state
- [ ] Click shared project to join
- [ ] Click shared screen to view
- [ ] Close all docks, verify status bar micro version appears
- [ ] Click status bar item, verify dock opens
- [ ] Verify title bar has no call UI
- [ ] Test with multiple participants
- [ ] Test speaking/muted indicators update in real-time

### Edge Cases
- [ ] Call with only self (no other participants)
- [ ] Participant joins/leaves during call
- [ ] Screen share starts/stops
- [ ] Following participant who leaves
- [ ] Rapid toggle of controls
- [ ] Overlay behavior when CollabPanel not registered

---

## Migration Strategy

1. **Phase 1-4**: Build new system in parallel with existing UI
2. **Phase 5**: Add status bar item (additive, no conflicts)
3. **Phase 6**: Remove title bar UI (can be feature-flagged)
4. **Phase 7**: Clean up collab panel (after overlay is stable)

Each phase can be merged independently, allowing for incremental review and testing.

---

## Open Questions

1. **Persistence**: Should collapsed state persist across sessions?
  - Yes
2. **Animations**: What animations for collapse/expand, participant join/leave?
  - None for now.
3. **Keyboard shortcuts**: Should there be shortcuts for overlay actions?
  - Yes eventually
4. **Accessibility**: Screen reader support for call state changes?
  - Yes, if they exist currently. No if not.

---

## Dependencies

### Crate Dependencies
- `call` crate - ActiveCall, Room, participant types
- `collab_ui` crate - Will host CollabOverlayPanel entity
- `ui` crate - Pure presentation components (no new deps needed)
- `workspace` crate - Integration point, status bar
- `title_bar` crate - Reuse `toggle_mute`, `toggle_deafen`, `toggle_screen_sharing` functions

### Key Types
- `ActiveCall` (`crates/call/src/call_impl/mod.rs`)
- `Room` (`crates/call/src/call_impl/room.rs`)
- `LocalParticipant`, `RemoteParticipant` (`crates/call/src/call_impl/participant.rs`)
- `StatusItemView` (`crates/workspace/src/status_bar.rs`)
- `ChannelView` (`crates/collab_ui/src/channel_view.rs`) - for opening channel notes

### Room Events to Subscribe

From `crates/call/src/call_impl/room.rs` - these events trigger UI updates:

```rust
pub enum Event {
    RoomJoined { channel_id: Option<ChannelId> },
    RoomLeft { channel_id: Option<ChannelId> },
    ParticipantLocationChanged { participant_id: proto::PeerId },
    RemoteVideoTracksChanged { participant_id: proto::PeerId },
    RemoteAudioTracksChanged { participant_id: proto::PeerId },
    LocalTrackPublished,
    LocalTrackUnpublished,
    RemoteProjectShared { owner: Arc<User>, project_id: u64, worktree_root_names: Vec<String> },
    RemoteProjectUnshared { project_id: u64 },
    RemoteProjectJoined { project_id: u64 },
    RemoteProjectInvitationDiscarded { project_id: u64 },
}
```

### Subscription Pattern

From `collab_panel.rs` - how to subscribe to room changes:

```rust
// In CollabOverlayPanel::new()
let active_call = ActiveCall::global(cx);
let subscriptions = vec![
    cx.observe(&active_call, |this, _, cx| {
        this.update_from_room(cx);
        cx.notify();
    }),
    cx.subscribe(&active_call, |this, _, event, cx| {
        // Handle ActiveCall events
    }),
];

// When room exists, also subscribe to room events:
if let Some(room) = active_call.read(cx).room() {
    subscriptions.push(cx.subscribe(room, |this, _, event, cx| {
        match event {
            Room::Event::ParticipantLocationChanged { .. } => { /* update UI */ }
            Room::Event::RemoteVideoTracksChanged { .. } => { /* update screen share */ }
            // etc.
        }
        cx.notify();
    }));
}
```

No new external dependencies required.

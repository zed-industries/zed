---
title: Channels
description: "Persistent collaboration rooms in Zed for sharing projects, voice chat, and real-time code editing."
---

# Channels {#channels}

Channels are persistent rooms for team collaboration. Each channel can contain shared projects, voice chat, and collaborative notes.

Channels support:

- Pairing – each collaborator keeps their own screen, mouse, and keyboard.
- Mentoring – jump into someone else's context and help without asking them to hand over control.
- Refactoring – multiple people can join the same large refactor in real time.
- Ambient awareness – see what teammates are working on without status meetings.

Each channel usually maps to an ongoing project or workstream.
You can see who's in a channel because their avatars appear in the Collaboration Panel.

Create a channel by clicking the `+` icon next to the `Channels` text in the Collaboration Panel.
Create a subchannel by right-clicking an existing channel and selecting `New Subchannel`.

You can keep both work and side-project channels in the Collaboration Panel.

Joining a channel adds you to a shared room where you can work on projects together.

_Join [our channel tree](https://zed.dev/channel/zed-283) to get an idea of how you can organize yours._

## Inviting People

By default, channels you create can only be accessed by you.
You can invite collaborators by right-clicking and selecting `Manage members`.

When you have subchannels nested under others, permissions are inherited.
For instance, adding people to the top-level channel in your channel tree will automatically give them access to its subchannels.

Once you have added someone, they can either join your channel by clicking on it in their Collaboration Panel, or you can share the link to the channel so that they can join directly.

## Voice Chat

You can mute/unmute your microphone via the microphone icon in the upper right-hand side of the window.

> **Note:** When joining a channel, Zed automatically shares your microphone with other users in the call, if your OS allows it. To start muted, use the [`mute_on_join`](../reference/all-settings.md#calls) setting.

## Sharing Projects

After joining a channel, you can share a project over the channel via the `Share` button in the upper right-hand side of the window.
This will allow channel members to edit the code hosted on your machine as though they had it checked out locally.

When you edit someone else's project, your editor features still work: jump to definition, use AI features, and view diagnostics.
For pairing, one person can implement while the other reads and validates nearby code.
Because you keep your own local configuration, the session still feels like your normal setup.

Collaborators can open, edit, and save files, perform searches, and interact with language servers.
Guests have a read-only view of the project, including access to language server info.

### Unsharing a Project

You can remove a project from a channel by clicking on the `Unshare` button in the title bar.

Collaborators that are currently in that project will be disconnected from the project and will not be able to rejoin it unless you share it again.

## Channel Notes

Each channel has a Markdown notes file associated with it to keep track of current status, new ideas, or to collaborate on building out the design for the feature that you're working on before diving into code.

This works like a shared Markdown document backed by Zed's collaboration service.

Open channel notes by clicking the document icon to the right of the channel name in the Collaboration Panel.

> **Note:** You can view a channel's notes without joining the channel.

## Following Collaborators

To follow a collaborator, click on their avatar in the top left of the title bar.
You can also cycle through collaborators using {#kb workspace::FollowNextCollaborator} or `workspace: follow next collaborator` in the command palette.

When you join a project, you'll immediately start following the collaborator that invited you.

When a pane is following a collaborator, it will:

- follow their cursor and scroll position
- follow them to other files in the same project
- instantly swap to viewing their screenshare in that pane, if they are sharing their screen and leave the project

To stop following, simply move your mouse or make an edit via your keyboard.

### How Following Works

Following is confined to a particular pane.
When a pane is following a collaborator, it is outlined in their cursor color.

Collaborators in the same project appear in color and include a cursor color.
Collaborators in other projects are shown in gray.

This pane-specific behavior allows you to follow someone in one pane while navigating independently in another and can be an effective layout for some collaboration styles.

### Following a Terminal

Following in terminals is not currently supported the same way it is in the editor.
As a workaround, collaborators can share their screen and you can follow that instead.

## Screen Sharing

Share your screen with collaborators in the current channel by clicking on the `Share screen` (monitor icon) button in the top right of the title bar.
If you have multiple displays, you can choose which one to share via the chevron to the right of the monitor icon.

After you've shared your screen, others can click the `Screen` entry under your name in the Collaboration Panel to open a tab that keeps it visible.
If they are following you, Zed will automatically switch between following your cursor in their Zed instance and your screen share, depending on whether you are focused on Zed or another application, like a web browser.

> **Warning:** Collaborators can see your entire screen when sharing. Stop screen sharing when finished.

## Livestreaming & Guests

A channel can also be made public.
This allows anyone to join the channel by clicking on the link.

Guest users in channels can hear and see everything that is happening, and have read-only access to projects and channel notes.

If you'd like to invite a guest to participate in a channel for the duration of a call, you can do so by right-clicking them in the Collaboration Panel.
"Allowing Write Access" will allow them to edit any projects shared into the call, and to use their microphone and share their screen if they wish.

## Leaving a Call

You can leave a channel by clicking on the `Leave call` button in the upper right-hand side of the window.

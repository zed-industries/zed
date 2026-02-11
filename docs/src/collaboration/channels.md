# Channels

Channels provide a way to streamline collaborating for software engineers in many ways, but particularly:

- Pairing – when working on something together, you both have your own screen, mouse, and keyboard.
- Mentoring – it's easy to jump in to someone else's context, and help them get unstuck, without the friction of pushing code up.
- Refactoring – you can have multiple people join in on large refactoring without fear of conflict.
- Ambient awareness – you can see what everyone else is working on with no need for status emails or meetings.

Each channel corresponds to an ongoing project or work-stream.
You can see who's in a channel as their avatars will show up in the sidebar.
This makes it easy to see what everyone is doing and where to find them if needed.

Create a channel by clicking the `+` icon next to the `Channels` text in the collab panel.
Create a subchannel by right clicking an existing channel and selecting `New Subchannel`.

You can mix channels for your day job, as well as side-projects in your collab panel.

Joining a channel adds you to a shared room where you can work on projects together.

_Join [our channel tree](https://zed.dev/channel/zed-283) to get an idea of how you can organize yours._

## Inviting People

By default, channels you create can only be accessed by you.
You can invite collaborators by right clicking and selecting `Manage members`.

When you have subchannels nested under others, permissions are inherited.
For instance, adding people to the top-level channel in your channel tree will automatically give them access to its subchannels.

Once you have added someone, they can either join your channel by clicking on it in their Zed sidebar, or you can share the link to the channel so that they can join directly.

## Voice Chat

You can mute/unmute your microphone via the microphone icon in the upper right-hand side of the window.

> Note: When joining a channel, Zed will automatically share your microphone with other users in the call, if your OS allows it.
> If you'd prefer your microphone to be off when joining a channel, you can do so via the [`mute_on_join`](../reference/all-settings.md#calls) setting.

## Sharing Projects

After joining a channel, you can share a project over the channel via the `Share` button in the upper right-hand side of the window.
This will allow channel members to edit the code hosted on your machine as though they had it checked out locally.

When you are editing someone else's project, you still have the full power of the editor at your fingertips; you can jump to definitions, use the AI assistant, and see any diagnostic errors.
This is extremely powerful for pairing, as one of you can be implementing the current method while the other is reading and researching the correct solution to the next problem.
And, because you have your own config running, it feels like you're using your own machine.

We aim to eliminate the distinction between local and remote projects as much as possible.
Collaborators can open, edit, and save files, perform searches, interact with the language server, etc.
Guests have a read-only view of the project, including access to language server info.

### Unsharing a Project

You can remove a project from a channel by clicking on the `Unshare` button in the title bar.

Collaborators that are currently in that project will be disconnected from the project and will not be able to rejoin it unless you share it again.

## Channel Notes

Each channel has a Markdown notes file associated with it to keep track of current status, new ideas, or to collaborate on building out the design for the feature that you're working on before diving into code.

This is similar to a Google Doc, except powered by Zed's collaborative software and persisted to our servers.

Open the channel notes by clicking on the document icon to the right of the channel name in the collaboration panel.

> Note: You can view a channel's notes without joining the channel, if you'd just like to read up on what has been written.

## Following Collaborators

To follow a collaborator, click on their avatar in the top left of the title bar.
You can also cycle through collaborators using {#kb workspace::FollowNextCollaborator} or `workspace: follow next collaborator` in the command palette.

When you join a project, you'll immediately start following the collaborator that invited you.

When you are in a pane that is following a collaborator, you will:

- follow their cursor and scroll position
- follow them to other files in the same project
- instantly swap to viewing their screenshare in that pane, if they are sharing their screen and leave the project

To stop following, simply move your mouse or make an edit via your keyboard.

### How Following Works

Following is confined to a particular pane.
When a pane is following a collaborator, it is outlined in their cursor color.

Avatars of collaborators in the same project as you are in color, and have a cursor color.
Collaborators in other projects are shown in gray.

This pane-specific behavior allows you to follow someone in one pane while navigating independently in another and can be an effective layout for some collaboration styles.

### Following a Terminal

Following is not currently supported in the terminal in the way it is supported in the editor.
As a workaround, collaborators can share their screen and you can follow that instead.

## Screen Sharing

Share your screen with collaborators in the current channel by clicking on the `Share screen` (monitor icon) button in the top right of the title bar.
If you have multiple displays, you can choose which one to share via the chevron to the right of the monitor icon.

After you've shared your screen, others can click on the `Screen` entry under your name in the collaboration panel to open a tab that always keeps it visible.
If they are following you, Zed will automatically switch between following your cursor in their Zed instance and your screen share, depending on whether you are focused on Zed or another application, like a web browser.

> Note: Collaborators can see your entire screen when you are screen sharing, so be careful not to share anything you don't want to share.
> Remember to stop screen sharing when you are finished.

## Livestreaming & Guests

A Channel can also be made Public.
This allows anyone to join the channel by clicking on the link.

Guest users in channels can hear and see everything that is happening, and have read only access to projects and channel notes.

If you'd like to invite a guest to participate in a channel for the duration of a call you can do so by right clicking on them in the Collaboration Panel.
"Allowing Write Access" will allow them to edit any projects shared into the call, and to use their microphone and share their screen if they wish.

## Leaving a Call

You can leave a channel by clicking on the `Leave call` button in the upper right-hand side of the window.

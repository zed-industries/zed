# Collaboration

Only collaborate with people that you trust. Since sharing a project gives them access to your local file system, you should not share projects with people you do not trust; they could potentially do some nasty things.

In the future, we will do more to prevent this type of access beyond the shared project and add more control over what collaborators can do, but for now, only collaborate with people you trust.

Note: we are working on a new version of this feature called [Channels](channels/). If you'd like to be part of the private beta, please contact us!

## Adding a collaborator to a call

Before you can collaborate, you'll need to add a collaborator to your contacts. To do this:

1. Open the contacts menu by clicking on the `Show contacts menu` button in the upper right-hand corner of the window or by running `collab: toggle contacts menu` (`cmd-shift-c`).
2. Click the add button to the right of the search box.
3. Search for the contact you want to add using their GitHub handle. Note: the person you are trying to add as a contact must be an existing Zed user.

### Inviting a collaborator

You can add an existing Zed user as a contact from the contacts menu, deployed from the `Show contacts menu` button in the upper right-hand corner of the window or by `collab: toggle contacts menu` (`cmd-shift-c`) and then clicking the `Search for new contact` button to the right of the search box.

![Inviting a collaborator to the current project](https://zed.dev/img/collaboration/add-a-collaborator.png)

When you invite a collaborator to a project not in a call they will receive a notification to join, and a new call is created.

![Receiving an invite to join a call](https://zed.dev/img/collaboration/receiving-an-invite.jpg)

### Inviting non-Zed users

If someone you want to collaborate with has not yet signed up for Zed, they will need to [download the app](https://zed.dev/download) and sign in for the first time before you can add them.

## Collaborating on a project

### Share a project

When you invite a collaborator to join your project, a new call begins. Your Zed windows will show the call participants in the top right of the window.

![A new Zed call with two collaborators](https://zed.dev/img/collaboration/new-call.png)

Collaborators in the same project as you are in color, and have a cursor color. Collaborators in other projects are shown in gray. Collaborators that have access to the current project will have their own cursor color under their avatar.

We aim to eliminate the distinction between local and remote projects as much as possible. Guests can open, edit, and save files, perform searches, interact with the language server, etc.

#### Unshared Projects

If a collaborator is currently in a project that is not shared, you will not be able to jump to their project or follow them until they either share the project or return to a project that is shared.

If you are in a project that isn't shared, others will not be able to join it or see its contents.

### Follow a collaborator

To follow a collaborator, click on their avatar in the top right of the window. You can also cycle through collaborators using `workspace: follow next collaborator` (`ctrl-alt-cmd-f`).

When you join a project, you'll immediately start following the collaborator that invited you.

![Automatically following the person inviting us to a project](https://zed.dev/img/collaboration/joining-a-call.png)

When you are in a pane that is following a collaborator, you will:

- follow their cursor and scroll position
- follow them to other files in the same project
- instantly swap to viewing their screen in that pane, if they are sharing their screen and leave the project

If you move your cursor or make an edit in that pane, you will stop following.

To start following again, you can click on a collaborator's avatar or cycle through following different participants by pressing `workspace: follow next collaborator` (`ctrl-alt-cmd-f`).

#### How following works

Following is confined to a particular pane. When a pane is following a collaborator, it is outlined in their cursor color.

This pane-specific behavior allows you to follow someone in one pane while navigating independently in another and can be an effective layout for some collaboration styles.

### Sharing your screen

Share your screen with collaborators in the current call by clicking on the `Share screen` button in the top right of the window.

Collaborators will see your screen if they are following you and you start viewing a window outside Zed or a project that is not shared.

Collaborators can see your entire screen when you are screen sharing, so be careful not to share anything you don't want to share. Remember to stop screen sharing when you are finished.

Call participants can open a dedicated tab for your screen share by opening the contacts menu in the top right and clicking on the `Screen` entry if you are sharing your screen.

### Adding a project

You can add a project to a call by clicking on the `Share` button next to the project name in the title bar.

### Removing a project

You can remove a project from a call by clicking on the `Unshare` button next to the project name in the title bar.

Collaborators that are currently in that project will be disconnected from the project and will not be able to rejoin it unless you share it again.

### Following a collaborator's terminal

You can follow what a collaborator is doing in their terminal by having them share their screen and following it.

In the future, we plan to allow you to collaborate in the terminal directly in a shared project.

### Leave call

You can leave a call by opening the contacts menu in the top right and clicking on the `Leave call` button.

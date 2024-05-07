# Channels

At Zed we believe that great things are built by great people working together. We have designed Zed to help every individual work faster and to help teams of people work together more effectively.

### Overview

Channels provide a way to streamline collaborating for software engineers in many ways, but particularly:

- Pairing – when working on something together, you both have your own screen, mouse, and keyboard.
- Mentoring – it’s easy to jump in to someone else’s context, and help them get unstuck, without the friction of pushing code up.
- Refactoring – you can have multiple people join in on large refactoring without fear of conflict.
- Ambient awareness – you can see what everyone else is working on with no need for status emails or meetings.

### Channels

To open the collaboration panel hit `cmd-shift-c` (or `cmd-shift-p “collab panel: toggle focus”`).

<figure><img src="../.gitbook/assets/channels-1.png" alt=""><figcaption></figcaption></figure>

Each channel corresponds to an ongoing project or work-stream. You can see who’s in a channel as their avatars will show up in the sidebar. This makes it easy to see what everyone is doing and where to find them if needed.

You can create as many channels as you need. As in the example above, you can mix channels for your day job, as well as side-projects in one instance of Zed.

Joining a channel adds you to a shared room where you can work on projects together.

### Sharing projects

After joining a channel, you can `Share` a project with the other people there. This will enable them to edit the code hosted on your machine as though they had it checked out locally.

<figure><img src="../.gitbook/assets/channels-2.png" alt=""><figcaption></figcaption></figure>

When you are editing someone else’s project, you still have the full power of the editor at your fingertips, you can jump to definitions, use the AI assistant, and see any diagnostic errors. This is extremely powerful for pairing, as one of you can be implementing the current method while the other is reading and researching the correct solution to the next problem. And, because you have your own config running, it feels like you’re using your own machine.

### Following

You can follow someone by clicking on their avatar in the top bar, or their name in the collaboration panel. When following, your pane will show you what they are looking at, even if they are jumping between different files in the project. If you want to stop following them, you can by scrolling around, or clicking in a different part of the file.

Following is incredibly useful when you’re learning a new codebase, or trying to debug together. Because you can always see what each person is looking at, there’s no confusion as to what is being talked about.

As a bonus, if the other person is sharing their screen, you can follow them out of Zed and see what is going on, so you can see if the code you wrote together really works.

### Notes & Chat

Each channel has a notes file associated with it to keep track of current status, new ideas, or to collaborate on building out the design for the feature that you’re working on before diving into code.

<figure><img src="../.gitbook/assets/channels-3.png" alt=""><figcaption></figcaption></figure>

The chat is also there for quickly sharing context, or getting questions answered, that are more ephemeral in nature.

Between the two, you can use Zed’s collaboration mode for large-scale changes with multiple people tackling different aspects of the problem. Because you’re all working on the same copy of the code, there are no merge conflicts, and because you all have access to the same notes, it’s easy to track progress and keep everyone in the loop.

### Inviting people

By default, channels you create can only be accessed by you. You can invite collaborators by right clicking and selecting `Manage members`.

When you have channels nested under each other, permissions are inherited. For instance, in the example above, we only need to add people to the `#zed` channel, and they will automatically gain access to `#core-editor`, `#new-languages`, and `#stability`.

Once you have added someone, they can either join your channel by clicking on it in their Zed sidebar, or you can share the link to the channel so that they can join directly.

### Livestreaming & Guests

A Channel can also be made Public. This allows anyone to join the channel by clicking on the link.

Guest users in channels can hear and see everything that is happening, and have read only access to projects and channel notes. They can use the Chat as normal.

If you'd like to invite a guest to participate in a channel for the duration of a call you can do so by right clicking on them in the Collaboration Panel. "Allowing Write Access" will allow them to edit any projects shared into the call, and to use their microphone and share their screen if they wish.

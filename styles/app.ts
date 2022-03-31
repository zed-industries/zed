import chatPanel from "./chat-panel";
import { backgroundColor, border, borderColor, player, text } from "./components";
import editor from "./editor";
import projectPanel from "./project-panel";
import selectorModal from "./selector-modal";
import Theme from "./theme";
import workspace from "./workspace";

export const panel = {
  padding: { top: 12, left: 12, bottom: 12, right: 12 },
};

export default function app(theme: Theme): Object {
  return {
    selector: selectorModal(theme),
    workspace: workspace(theme),
    editor: editor(theme),
    projectDiagnostics: {
      background: backgroundColor(theme, 300),
      tabIconSpacing: 4,
      tabIconWidth: 13,
      tabSummarySpacing: 10,
      emptyMessage: {
        ...text(theme, "sans", "primary", { size: "lg" }),
      },
      statusBarItem: {
        ...text(theme, "sans", "muted"),
        margin: {
          right: 10,
        },
      },
    },
    projectPanel: projectPanel(theme),
    chatPanel: chatPanel(theme),
    contactsPanel: {
      extends: "$panel",
      hostRowHeight: 28,
      treeBranchColor: borderColor(theme, "muted"),
      treeBranchWidth: 1,
      hostAvatar: {
        cornerRadius: 10,
        width: 18,
      },
      hostUsername: {
        ...text(theme, "mono", "muted"),
        padding: {
          left: 8,
        },
      },
      hoveredSharedProject: {
        extends: "$contacts_panel.sharedProject",
        background: theme.editor.line.active.value,
        cornerRadius: 6,
      },
      hoveredUnsharedProject: {
        extends: "$contacts_panel.unsharedProject",
        background: theme.editor.line.active.value,
        cornerRadius: 6,
      },
      project: {
        guestAvatarSpacing: 4,
        height: 24,
        guestAvatar: {
          cornerRadius: 8,
          width: 14,
        },
        name: {
          extends: text(theme, "mono", "secondary"),
          margin: {
            right: 6,
          },
        },
        padding: {
          left: 8,
        },
      },
      sharedProject: {
        extends: "$contactsPanel.project",
        name: {
          color: text(theme, "mono", "primary"),
        },
      },
      unsharedProject: {
        extends: "$contactsPanel.project",
      },
    },
    search: {
      background: backgroundColor(theme, 300),
      matchBackground: theme.editor.highlight.match,
      tabIconSpacing: 4,
      tabIconWidth: 14,
      activeHoveredOptionButton: {
        extends: "$search.option_button",
        background: backgroundColor(theme, 100),
      },
      activeOptionButton: {
        extends: "$search.option_button",
        background: backgroundColor(theme, 100),
      },
      editor: {
        background: backgroundColor(theme, 500),
        cornerRadius: 6,
        maxWidth: 400,
        placeholderText: text(theme, "mono", "placeholder"),
        selection: player(theme, 1).selection,
        text: text(theme, "mono", "primary"),
        border: border(theme, "primary"),
        margin: {
          bottom: 5,
          left: 5,
          right: 5,
          top: 5,
        },
        padding: {
          bottom: 3,
          left: 13,
          right: 13,
          top: 3,
        },
      },
      hoveredOptionButton: {
        extends: "$search.optionButton",
        background: backgroundColor(theme, 100),
      },
      invalidEditor: {
        extends: "$search.editor",
        border: border(theme, "error"),
      },
      matchIndex: {
        ...text(theme, "mono", "secondary"),
        padding: 6,
      },
      optionButton: {
        ...text(theme, "mono", "secondary"),
        background: backgroundColor(theme, 300),
        cornerRadius: 6,
        border: border(theme, "primary"),
        margin: {
          left: 1,
          right: 1,
        },
        padding: {
          bottom: 1,
          left: 6,
          right: 6,
          top: 1,
        },
      },
      optionButtonGroup: {
        padding: {
          left: 2,
          right: 2,
        },
      },
      resultsStatus: {
        ...text(theme, "mono", "primary"),
        size: 18,
      },
    },
  };
}

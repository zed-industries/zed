import { backgroundColor, text } from "./components";
import selectorModal from "./selector-modal";
import workspace from "./workspace";
import editor from "./editor";
import Theme from "./theme";
import projectPanel from "./project-panel";
import chatPanel from "./chat-panel";

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
      treeBranchColor: "$surface.100",
      treeBranchWidth: 1,
      hostAvatar: {
        cornerRadius: 10,
        width: 18,
      },
      hostUsername: {
        extends: "$text.primary",
        padding: {
          left: 8,
        },
      },
      hoveredSharedProject: {
        background: "$state.hover",
        cornerRadius: 6,
        extends: "$contacts_panel.sharedProject",
      },
      hoveredUnsharedProject: {
        background: "$state.hover",
        cornerRadius: 6,
        extends: "$contacts_panel.unsharedProject",
      },
      project: {
        guestAvatarSpacing: 4,
        height: 24,
        guestAvatar: {
          cornerRadius: 8,
          width: 14,
        },
        name: {
          extends: "$text.secondary",
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
          color: "$text.primary.color",
        },
      },
      unsharedProject: {
        extends: "$contactsPanel.project",
      },
    },
    search: {
      background: backgroundColor(theme, 300),
      matchBackground: "$state.highlightedLine",
      tabIconSpacing: 4,
      tabIconWidth: 14,
      activeHoveredOptionButton: {
        background: "$surface.100",
        extends: "$search.option_button",
      },
      activeOptionButton: {
        background: "$surface.100",
        extends: "$search.option_button",
      },
      editor: {
        background: "$surface.500",
        cornerRadius: 6,
        maxWidth: 400,
        placeholderText: "$text.muted",
        selection: "$selection.host",
        text: "$text.primary",
        border: {
          color: "$border.primary",
          width: 1,
        },
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
        background: "$surface.100",
        extends: "$search.optionButton",
      },
      invalidEditor: {
        extends: "$search.editor",
        border: {
          color: "$status.bad",
          width: 1,
        },
      },
      matchIndex: {
        extends: "$text.secondary",
        padding: 6,
      },
      optionButton: {
        background: backgroundColor(theme, 300),
        cornerRadius: 6,
        extends: "$text.secondary",
        border: {
          color: "$border.primary",
          width: 1,
        },
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
        extends: "$text.primary",
        size: 18,
      },
    },
  };
}

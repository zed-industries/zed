import chatPanel from "./chat-panel";
import { backgroundColor, borderColor, text } from "./components";
import editor from "./editor";
import projectPanel from "./project-panel";
import search from "./search";
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
      ...panel,
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
    search: search(theme),
  };
}

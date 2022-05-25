import Theme from "../themes/common/theme";
import chatPanel from "./chatPanel";
import { text } from "./components";
import contactFinder from "./contactFinder";
import contactsPanel from "./contactsPanel";
import commandPalette from "./commandPalette";
import editor from "./editor";
import projectPanel from "./projectPanel";
import search from "./search";
import picker from "./picker";
import workspace from "./workspace";
import projectDiagnostics from "./projectDiagnostics";
import contactNotification from "./contactNotification";

export const panel = {
  padding: { top: 12, bottom: 12 },
};

export default function app(theme: Theme): Object {
  return {
    picker: picker(theme),
    workspace: workspace(theme),
    editor: editor(theme),
    projectDiagnostics: projectDiagnostics(theme),
    commandPalette: commandPalette(theme),
    projectPanel: projectPanel(theme),
    chatPanel: chatPanel(theme),
    contactsPanel: contactsPanel(theme),
    contactFinder: contactFinder(theme),
    search: search(theme),
    breadcrumbs: {
      ...text(theme, "sans", "secondary"),
      padding: {
        left: 6,
      },
    },
    contactNotification: contactNotification(theme),
  };
}

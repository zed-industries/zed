import { text } from "./components";
import contactFinder from "./contactFinder";
import contactsPanel from "./contactsPanel";
import contactsPopover from "./contactsPopover";
import commandPalette from "./commandPalette";
import editor from "./editor";
import projectPanel from "./projectPanel";
import search from "./search";
import picker from "./picker";
import workspace from "./workspace";
import contextMenu from "./contextMenu";
import projectDiagnostics from "./projectDiagnostics";
import contactNotification from "./contactNotification";
import updateNotification from "./updateNotification";
import tooltip from "./tooltip";
import terminal from "./terminal";
import { ColorScheme } from "../themes/common/colorScheme";

export const panel = {
  padding: { top: 12, bottom: 12 },
};

export default function app(colorScheme: ColorScheme): Object {
  return {
    meta: {
      name: colorScheme.name,
      isLight: colorScheme.isLight
    },
    picker: picker(colorScheme),
    workspace: workspace(colorScheme),
    contextMenu: contextMenu(colorScheme),
    editor: editor(colorScheme),
    projectDiagnostics: projectDiagnostics(colorScheme),
    commandPalette: commandPalette(colorScheme),
    projectPanel: projectPanel(colorScheme),
    contactsPopover: contactsPopover(colorScheme),
    contactsPanel: contactsPanel(colorScheme),
    contactFinder: contactFinder(colorScheme),
    search: search(colorScheme),
    breadcrumbs: {
      ...text(colorScheme.lowest.top, "sans", "base", "variant"),
      padding: {
        left: 6,
      },
    },
    contactNotification: contactNotification(colorScheme),
    updateNotification: updateNotification(colorScheme),
    tooltip: tooltip(colorScheme),
    terminal: terminal(colorScheme.lowest),
  };
}

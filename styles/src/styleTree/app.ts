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

// export const panel = {
//   padding: { top: 12, bottom: 12 },
// };

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
      ...text(colorScheme.lowest.top, "sans"),
      padding: {
        left: 6,
      },
    },
    contactNotification: contactNotification(colorScheme),
    updateNotification: updateNotification(colorScheme),
    tooltip: tooltip(colorScheme),
    terminal: terminal(colorScheme.lowest),
    colorScheme: {
      ...colorScheme,
      lowest: {
        ...colorScheme.lowest,
        ramps: {
          neutral: colorScheme.lowest.ramps.neutral.colors(100, "hex"),
          red: colorScheme.lowest.ramps.red.colors(100, "hex"),
          orange: colorScheme.lowest.ramps.orange.colors(100, "hex"),
          yellow: colorScheme.lowest.ramps.yellow.colors(100, "hex"),
          green: colorScheme.lowest.ramps.green.colors(100, "hex"),
          cyan: colorScheme.lowest.ramps.cyan.colors(100, "hex"),
          blue: colorScheme.lowest.ramps.blue.colors(100, "hex"),
          violet: colorScheme.lowest.ramps.violet.colors(100, "hex"),
          magenta: colorScheme.lowest.ramps.magenta.colors(100, "hex"),
        }
      },
      middle: {
        ...colorScheme.middle,
        ramps: {
          neutral: colorScheme.middle.ramps.neutral.colors(100, "hex"),
          red: colorScheme.middle.ramps.red.colors(100, "hex"),
          orange: colorScheme.middle.ramps.orange.colors(100, "hex"),
          yellow: colorScheme.middle.ramps.yellow.colors(100, "hex"),
          green: colorScheme.middle.ramps.green.colors(100, "hex"),
          cyan: colorScheme.middle.ramps.cyan.colors(100, "hex"),
          blue: colorScheme.middle.ramps.blue.colors(100, "hex"),
          violet: colorScheme.middle.ramps.violet.colors(100, "hex"),
          magenta: colorScheme.middle.ramps.magenta.colors(100, "hex"),
        }
      },
      highest: {
        ...colorScheme.highest,
        ramps: {
          neutral: colorScheme.highest.ramps.neutral.colors(100, "hex"),
          red: colorScheme.highest.ramps.red.colors(100, "hex"),
          orange: colorScheme.highest.ramps.orange.colors(100, "hex"),
          yellow: colorScheme.highest.ramps.yellow.colors(100, "hex"),
          green: colorScheme.highest.ramps.green.colors(100, "hex"),
          cyan: colorScheme.highest.ramps.cyan.colors(100, "hex"),
          blue: colorScheme.highest.ramps.blue.colors(100, "hex"),
          violet: colorScheme.highest.ramps.violet.colors(100, "hex"),
          magenta: colorScheme.highest.ramps.magenta.colors(100, "hex"),
        }
      },
      players: [
        colorScheme.players["0"],
        colorScheme.players["1"],
        colorScheme.players["2"],
        colorScheme.players["3"],
        colorScheme.players["4"],
        colorScheme.players["5"],
        colorScheme.players["6"],
        colorScheme.players["7"],
      ]
    }
  };
}

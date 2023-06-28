import contactFinder from "./contactFinder"
import contactsPopover from "./contactsPopover"
import commandPalette from "./commandPalette"
import editor from "./editor"
import projectPanel from "./projectPanel"
import search from "./search"
import picker from "./picker"
import workspace from "./workspace"
import contextMenu from "./contextMenu"
import sharedScreen from "./sharedScreen"
import projectDiagnostics from "./projectDiagnostics"
import contactNotification from "./contactNotification"
import updateNotification from "./updateNotification"
import simpleMessageNotification from "./simpleMessageNotification"
import projectSharedNotification from "./projectSharedNotification"
import tooltip from "./tooltip"
import terminal from "./terminal"
import contactList from "./contactList"
import toolbarDropdownMenu from "./toolbarDropdownMenu"
import incomingCallNotification from "./incomingCallNotification"
import { ColorScheme } from "../theme/colorScheme"
import feedback from "./feedback"
import welcome from "./welcome"
import copilot from "./copilot"
import assistant from "./assistant"
import { titlebar } from "./titlebar"

export default function app(colorScheme: ColorScheme): any {
    return {
        meta: {
            name: colorScheme.name,
            isLight: colorScheme.isLight,
        },
        commandPalette: commandPalette(colorScheme),
        contactNotification: contactNotification(colorScheme),
        projectSharedNotification: projectSharedNotification(colorScheme),
        incomingCallNotification: incomingCallNotification(colorScheme),
        picker: picker(colorScheme),
        workspace: workspace(colorScheme),
        titlebar: titlebar(colorScheme),
        copilot: copilot(colorScheme),
        welcome: welcome(colorScheme),
        contextMenu: contextMenu(colorScheme),
        editor: editor(colorScheme),
        projectDiagnostics: projectDiagnostics(colorScheme),
        projectPanel: projectPanel(colorScheme),
        contactsPopover: contactsPopover(colorScheme),
        contactFinder: contactFinder(colorScheme),
        contactList: contactList(colorScheme),
        toolbarDropdownMenu: toolbarDropdownMenu(colorScheme),
        search: search(colorScheme),
        sharedScreen: sharedScreen(colorScheme),
        updateNotification: updateNotification(colorScheme),
        simpleMessageNotification: simpleMessageNotification(colorScheme),
        tooltip: tooltip(colorScheme),
        terminal: terminal(colorScheme),
        assistant: assistant(colorScheme),
        feedback: feedback(colorScheme),
        colorScheme: {
            ...colorScheme,
            players: Object.values(colorScheme.players),
            ramps: {
                neutral: colorScheme.ramps.neutral.colors(100, "hex"),
                red: colorScheme.ramps.red.colors(100, "hex"),
                orange: colorScheme.ramps.orange.colors(100, "hex"),
                yellow: colorScheme.ramps.yellow.colors(100, "hex"),
                green: colorScheme.ramps.green.colors(100, "hex"),
                cyan: colorScheme.ramps.cyan.colors(100, "hex"),
                blue: colorScheme.ramps.blue.colors(100, "hex"),
                violet: colorScheme.ramps.violet.colors(100, "hex"),
                magenta: colorScheme.ramps.magenta.colors(100, "hex"),
            },
        },
    }
}

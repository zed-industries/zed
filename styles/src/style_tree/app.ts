import contact_finder from "./contact_finder"
import contacts_popover from "./contacts_popover"
import command_palette from "./command_palette"
import project_panel from "./project_panel"
import search from "./search"
import picker from "./picker"
import workspace from "./workspace"
import context_menu from "./context_menu"
import shared_screen from "./shared_screen"
import project_diagnostics from "./project_diagnostics"
import contact_notification from "./contact_notification"
import update_notification from "./update_notification"
import simple_message_notification from "./simple_message_notification"
import project_shared_notification from "./project_shared_notification"
import tooltip from "./tooltip"
import terminal from "./terminal"
import contact_list from "./contact_list"
import toolbar_dropdown_menu from "./toolbar_dropdown_menu"
import incoming_call_notification from "./incoming_call_notification"
import { ColorScheme } from "../theme/color_scheme"
import welcome from "./welcome"
import copilot from "./copilot"
import assistant from "./assistant"
import { titlebar } from "./titlebar"
import editor from "./editor"
import feedback from "./feedback"

export default function app(theme: ColorScheme): any {
    return {
        meta: {
            name: theme.name,
            is_light: theme.is_light,
        },
        command_palette: command_palette(theme),
        contact_notification: contact_notification(theme),
        project_shared_notification: project_shared_notification(theme),
        incoming_call_notification: incoming_call_notification(theme),
        picker: picker(theme),
        workspace: workspace(theme),
        titlebar: titlebar(theme),
        copilot: copilot(theme),
        welcome: welcome(theme),
        context_menu: context_menu(theme),
        editor: editor(theme),
        project_diagnostics: project_diagnostics(theme),
        project_panel: project_panel(theme),
        contacts_popover: contacts_popover(theme),
        contact_finder: contact_finder(theme),
        contact_list: contact_list(theme),
        toolbar_dropdown_menu: toolbar_dropdown_menu(theme),
        search: search(theme),
        shared_screen: shared_screen(theme),
        update_notification: update_notification(theme),
        simple_message_notification: simple_message_notification(theme),
        tooltip: tooltip(theme),
        terminal: terminal(theme),
        assistant: assistant(theme),
        feedback: feedback(theme),
        color_scheme: {
            ...theme,
            players: Object.values(theme.players),
            ramps: {
                neutral: theme.ramps.neutral.colors(100, "hex"),
                red: theme.ramps.red.colors(100, "hex"),
                orange: theme.ramps.orange.colors(100, "hex"),
                yellow: theme.ramps.yellow.colors(100, "hex"),
                green: theme.ramps.green.colors(100, "hex"),
                cyan: theme.ramps.cyan.colors(100, "hex"),
                blue: theme.ramps.blue.colors(100, "hex"),
                violet: theme.ramps.violet.colors(100, "hex"),
                magenta: theme.ramps.magenta.colors(100, "hex"),
            },
        },
    }
}

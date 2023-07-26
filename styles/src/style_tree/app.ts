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
import contact_finder from "./contact_finder"
import collab_panel from "./collab_panel"
import toolbar_dropdown_menu from "./toolbar_dropdown_menu"
import incoming_call_notification from "./incoming_call_notification"
import welcome from "./welcome"
import copilot from "./copilot"
import assistant from "./assistant"
import { titlebar } from "./titlebar"
import editor from "./editor"
import feedback from "./feedback"
import { useTheme } from "../common"
import channels_panel from "./channels_panel"

export default function app(): any {
    const theme = useTheme()

    return {
        meta: {
            name: theme.name,
            is_light: theme.is_light,
        },
        command_palette: command_palette(),
        contact_notification: contact_notification(),
        project_shared_notification: project_shared_notification(),
        incoming_call_notification: incoming_call_notification(),
        picker: picker(),
        workspace: workspace(),
        titlebar: titlebar(),
        copilot: copilot(),
        welcome: welcome(),
        context_menu: context_menu(),
        editor: editor(),
        project_diagnostics: project_diagnostics(),
        project_panel: project_panel(),
        channels_panel: channels_panel(),
        collab_panel: collab_panel(),
        contact_finder: contact_finder(),
        toolbar_dropdown_menu: toolbar_dropdown_menu(),
        search: search(),
        shared_screen: shared_screen(),
        update_notification: update_notification(),
        simple_message_notification: simple_message_notification(),
        tooltip: tooltip(),
        terminal: terminal(),
        assistant: assistant(),
        feedback: feedback(),
    }
}

import Theme from "../themes/theme";
import chatPanel from "./chatPanel";
import { backgroundColor, borderColor, text } from "./components";
import contactsPanel from "./contactsPanel";
import editor from "./editor";
import projectPanel from "./projectPanel";
import search from "./search";
import selectorModal from "./selectorModal";
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
        contactsPanel: contactsPanel(theme),
        search: search(theme),
        breadcrumbs: {
            ...text(theme, "sans", "primary"),
            padding: {
                left: 6,
            },
        }
    };
}

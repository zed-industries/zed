import Theme from "../themes/theme";
import { panel } from "./app";
import { borderColor, text } from "./components";

export default function(theme: Theme) {
    const project = {
        guestAvatarSpacing: 4,
        height: 24,
        guestAvatar: {
            cornerRadius: 8,
            width: 14,
        },
        name: {
            ...text(theme, "mono", "secondary"),
            margin: {
                right: 6,
            },
        },
        padding: {
            left: 8,
        },
    };

    const sharedProject = {
        ...project,
        name: {
            ...project.name,
            ...text(theme, "mono", "primary"),
        },
    };

    return {
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
        project,
        sharedProject,
        hoveredSharedProject: {
            ...sharedProject,
            background: theme.editor.line.active.value,
            cornerRadius: 6,
        },
        unsharedProject: project,
        hoveredUnsharedProject: {
            ...project,
            background: theme.editor.line.active.value,
            cornerRadius: 6,
        },
    }
}
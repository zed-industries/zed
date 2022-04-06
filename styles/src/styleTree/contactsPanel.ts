import Theme from "../themes/theme";
import { panel } from "./app";
import { backgroundColor, borderColor, text } from "./components";

export default function(theme: Theme) {
    const project = {
        guestAvatarSpacing: 4,
        height: 24,
        guestAvatar: {
            cornerRadius: 8,
            width: 14,
        },
        name: {
            ...text(theme, "mono", "placeholder", { size: "sm" }),
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
        background: backgroundColor(theme, 300),
        cornerRadius: 6,
        name: {
            ...project.name,
            ...text(theme, "mono", "secondary", { size: "sm" }),
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
            ...text(theme, "mono", "primary", { size: "sm" }),
            padding: {
                left: 8,
            },
        },
        project,
        sharedProject,
        hoveredSharedProject: {
            ...sharedProject,
            background: backgroundColor(theme, 300, "hovered"),
            cornerRadius: 6,
        },
        unsharedProject: project,
        hoveredUnsharedProject: {
            ...project,
            cornerRadius: 6,
        },
    }
}

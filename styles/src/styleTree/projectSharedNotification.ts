import Theme from "../themes/common/theme";
import { backgroundColor, borderColor, text } from "./components";

export default function projectSharedNotification(theme: Theme): Object {
  const avatarSize = 48;
  return {
    windowHeight: 72,
    windowWidth: 380,
    background: backgroundColor(theme, 300),
    ownerContainer: {
      padding: 12,
    },
    ownerAvatar: {
      height: avatarSize,
      width: avatarSize,
      cornerRadius: avatarSize / 2,
    },
    ownerMetadata: {
      margin: { left: 10 },
    },
    ownerUsername: {
      ...text(theme, "sans", "active", { size: "sm", weight: "bold" }),
      margin: { top: -3 },
    },
    message: {
      ...text(theme, "sans", "secondary", { size: "xs" }),
      margin: { top: -3 },
    },
    worktreeRoots: {
      ...text(theme, "sans", "secondary", { size: "xs", weight: "bold" }),
      margin: { top: -3 },
    },
    buttonWidth: 96,
    openButton: {
      background: backgroundColor(theme, "info", "active"),
      border: { left: true, bottom: true, width: 1, color: borderColor(theme, "primary") },
      ...text(theme, "sans", "info", { size: "xs", weight: "extra_bold" })
    },
    dismissButton: {
      border: { left: true, width: 1, color: borderColor(theme, "primary") },
      ...text(theme, "sans", "secondary", { size: "xs", weight: "extra_bold" })
    },
  };
}

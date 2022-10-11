import Theme from "../themes/common/theme";
import { backgroundColor, borderColor, text } from "./components";

export default function incomingCallNotification(theme: Theme): Object {
  const avatarSize = 32;
  return {
    windowHeight: 74,
    windowWidth: 380,
    background: backgroundColor(theme, 300),
    callerContainer: {
      padding: 12,
    },
    callerAvatar: {
      height: avatarSize,
      width: avatarSize,
      cornerRadius: avatarSize / 2,
    },
    callerMetadata: {
      margin: { left: 10 },
    },
    callerUsername: {
      ...text(theme, "sans", "active", { size: "sm", weight: "bold" }),
      margin: { top: -3 },
    },
    callerMessage: {
      ...text(theme, "sans", "secondary", { size: "xs" }),
      margin: { top: -3 },
    },
    worktreeRoots: {
      ...text(theme, "sans", "secondary", { size: "xs", weight: "bold" }),
      margin: { top: -3 },
    },
    buttonWidth: 96,
    acceptButton: {
      background: backgroundColor(theme, "ok", "active"),
      border: { left: true, bottom: true, width: 1, color: borderColor(theme, "primary") },
      ...text(theme, "sans", "ok", { size: "xs", weight: "extra_bold" })
    },
    declineButton: {
      border: { left: true, width: 1, color: borderColor(theme, "primary") },
      ...text(theme, "sans", "error", { size: "xs", weight: "extra_bold" })
    },
  };
}
